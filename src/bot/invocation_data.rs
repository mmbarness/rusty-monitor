use crate::{resource_api::api::ResourceApi, structs::Context};
use serenity::Error;
#[derive(Debug, Clone)]
pub struct InvocationData {
    pub resource_api: ResourceApi,
    pub user: entity::users::Model,
    pub target_server: entity::target_server::Model,
}

impl InvocationData {
    pub async fn validate(ctx: Context<'_>) -> Result<InvocationData, Error> {
        match ctx.invocation_data::<InvocationData>().await {
            Some(invocation_data) => Ok(invocation_data.clone()),
            None => {
                ctx.say("we weren\t able to get your server info. Maybe try again.")
                    .await
                    .expect("failed to send message");
                return Err(Error::Other("error pulling data from invocation_data, which means it wasn\'t set correctly in pre-command"))
            }
        }
    }
}