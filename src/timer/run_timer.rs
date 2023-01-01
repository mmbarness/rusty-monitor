use crate::mprober_api::api::MProberAPI;
use crate::mprober_api_resources::shared_traits::Compute;
use crate::mprober_api_resources::cpu::{CPULoad};
use gnomeutils::serenity::{ChannelId};
use tokio::{ time };

pub async fn cpu_monitor(ctx:poise::serenity_prelude::Context, mprober_api: MProberAPI, channel_id: ChannelId, load_threshold:f64) -> () {
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(10));
        let api = mprober_api.clone();
        loop {
            let api_configs = &api.configs;
            let cpus = mprober_api.requester.cpu_load(&api_configs).await;
            interval.tick().await;
            let cpus_stat = &cpus.cpus_stat;
            let cpus_average = CPULoad::avg(cpus_stat);
            // let state =  
            match cpus_average > load_threshold {
                true => {
                    println!("current load: {:?}", &cpus_average);
                    match channel_id.send_message(&ctx, |m| {
                        m.content(format!("current cpu load of {} exceeding set threshold of {}", CPULoad::percentage(&cpus_average), CPULoad::percentage(&load_threshold)))
                    }).await {
                        Ok(_) => {
                            println!("successfully sent message")
                        },
                        Err(e) => {
                            panic!("error sending message - {}", e)
                        }
                    };
                },
                false => {
                    println!("current load of {} below threshold of {}", CPULoad::percentage(&cpus_average), CPULoad::percentage(&load_threshold));
                }
            };
            interval.tick().await;
        }
    });
}