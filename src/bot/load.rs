use sea_orm::IntoActiveModel;

use crate::{structs::{Context, Error}, configs::bot_configs::Config, resource_api::api::ResourceApi, database::initialize::Database};
use super::{support::Support, invocation_data::InvocationData, Bot, query_db::QueryDb};

#[async_trait::async_trait]
pub trait Load {
    async fn on_setup() -> Result<Bot, Error> {
        let configs = Config::load().await;
        let database = Database::load(&configs.database_credentials, &configs.environment).await;

        Ok(Bot {
            configs,
            database,
            support: Support{},
        })
    }

    async fn on_pre_command(ctx: &Context<'_>) -> Result<InvocationData, Error> {

        let user = match Support::get_user_if_exists(ctx).await {
            Some(user) => user.into_active_model(),
            None => {
                return Err("unable to find user in pre_command".to_string().into())
            }
        };
        let target_server = match Support::get_server_if_exists(ctx).await {
            Some(target_server) => target_server.into_active_model(),
            None => {
                return Err("unable to find server in pre_command".to_string().into())
            }
        };

        let resource_api = match ResourceApi::load(&target_server) {
            Ok(api) => api,
            Err(e) => {
                return Err(e);
            }
        };
        
        let invocation_data = InvocationData {
            resource_api: resource_api,
            target_server: target_server,
            user: user,
        };
        
        Ok(invocation_data)
    }
}
