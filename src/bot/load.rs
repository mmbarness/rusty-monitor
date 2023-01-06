use crate::{structs::{Context}, configs::bot_configs::Config, resource_api::api::ResourceApi};
use super::{support::Support, invocation_data::InvocationData, Bot, manage_user::ManageUser, manage_target_server::ManageTargetServer};
use std::fmt::Error;

#[async_trait::async_trait]
pub trait Load {
    async fn on_setup() -> Result<Bot, Error> {
        let configs = Config::load().await;

        Ok(Bot {
            configs,
            support: Support{},
        })
    }

    async fn on_pre_command(ctx: &Context<'_>) -> Result<InvocationData, Error> {

        let user = match Support::get_user_if_exists(ctx).await {
            Some(user) => user,
            None => {
                return Err(Error)
            }
        };
        let target_server = match Support::get_server_if_exists(ctx).await {
            Some(target_server) => target_server,
            None => {
                return Err(Error)
            }
        };

        let resource_api = ResourceApi::load(&target_server);
        
        let invocation_data = InvocationData {
            resource_api: resource_api,
            target_server: target_server,
            user: user,
        };
        
        Ok(invocation_data)
    }
}
