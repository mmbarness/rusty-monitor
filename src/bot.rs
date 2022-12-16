use core::panic;

use log::{ info };
use serenity::{
    async_trait,
    client::{
        Context, 
        EventHandler
    },
    model::{
        gateway::Ready,
        prelude::GuildId,
        prelude::{
            command::CommandOptionType, ResumedEvent,
        },
        application::interaction::Interaction,
        application::interaction::InteractionResponseType, voice::VoiceState
    },
}; 
pub struct Bot {
    pub discord_guild_id: GuildId,
}

#[async_trait]
impl EventHandler for Bot {

    async fn resume(&self, _: Context, _:ResumedEvent) {
        info!("rusty tube is back");
    }

    async fn voice_state_update(&self, _ctx: Context, _old: Option<VoiceState>, _new: VoiceState) {
        info!("registering voice state change");
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = GuildId::set_application_commands(&self.discord_guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| { 
                    command.name("hello").description("Say hello") 
                })
                .create_application_command(|command| { 
                    command
                        .name("play")
                        .description("Play Youtube Link") 
                        .create_option(|option| {
                            option
                                .name("video")
                                .description("youtube video - id or link")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                })
        }).await.unwrap();

        info!("{:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {   
        if let Interaction::ApplicationCommand(command) = &interaction {
        
            let response_content = match command.data.name.as_str() {
                "hello" => "hello".to_owned(),
                "play" => {
                    let app_command = match interaction.to_owned().application_command() {
                        Some(app_command) => app_command,
                        None => {
                            panic!("unable to parse app command")
                        },
                    };

                    let guild_id =  match app_command.guild_id {
                        Some(guild_id) => guild_id.clone(), 
                        None => {
                            panic!("unable to get guild id")
                        },
                    };

                    let argument = command
                        .data
                        .options
                        .iter()
                        .find(|opt| opt.name == "video")
                        .cloned();
                    
                    let arg_value = argument.unwrap().value.unwrap();
                    let video = arg_value.as_str().unwrap();

                    let manager = songbird::get(&ctx).await
                        .expect("Songbird Voice client placed in at initialisation.").clone();

                    if let Some(handler_lock) = manager.get(guild_id) {
                        let mut handler = handler_lock.lock().await;
    
                        let source = match songbird::ytdl(&video).await {
                            Ok(source) => source,
                            Err(why) => {
                                println!("Err starting source: {:?}", why);
                                panic!("uh oh");
                                // check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);
                            }
                        };
                
                        handler.play_source(source);
                    }

                    let play_message = "playing";
            
                    play_message.to_string()
                }
                command => unreachable!("Unknown command: {}", command),
            };

            let create_interaction_response =
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response_content))
                });

            if let Err(why) = create_interaction_response.await {
                eprintln!("Cannot respond to slash command: {}", why);
            }
        }
    }
}