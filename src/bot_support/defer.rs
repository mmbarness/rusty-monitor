use super::bot_support::BotSupport;
use crate::structs::Context;

impl BotSupport {
    pub async fn defer(ctx: &Context<'_>) {
        match ctx.defer().await {
            Ok(_) => {
                println!("deferring response...")
            },
            Err(_) => {
                panic!("unable to defer discord resp")
            }
        }
    }
}