use crate::{structs::{Context}, configs::bot_configs::Config, mprober_api::api::MProberAPI};
use super::{support::Support, data::Data, Bot, manage_user::ManageUser, manage_target_server::ManageTargetServer};
use std::fmt::Error;

#[async_trait::async_trait]
pub trait Load {
    async fn on_setup() -> Result<Bot, Error> {
        let configs = Config::load().await;

        let data = Data { 
            mprober_api: None,
            target_server: None,
            user: None,
        };
    
        Ok(Bot {
            configs,
            data,
            support: Support{},
        })
    }

    async fn on_pre_command(ctx: &Context<'_>) -> Result<Bot, Error> {

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

        let mprober_api = MProberAPI::load(&target_server);
        
        let bot_data = Data {
            mprober_api: Some(mprober_api),
            target_server: Some(target_server),
            user: Some(user),
        };
        
        Ok(Bot {
            data: bot_data, 
            support: ctx.data().support.clone(),
            configs: ctx.data().configs.clone(),
        })
    }
}
