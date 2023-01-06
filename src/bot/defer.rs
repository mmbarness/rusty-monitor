use super::support::Support;
use crate::structs::Context;

#[async_trait::async_trait]
pub trait Defer {
    async fn defer(ctx: &Context<'_>) {
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

impl Defer for Support {}