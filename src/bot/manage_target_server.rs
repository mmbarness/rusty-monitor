use crate::{structs::Context, models::{target_server::Find}, bot::manage_user::ManageUser};
use entity::target_server::ActiveModel as TargetServer;

use super::support::Support;

#[async_trait::async_trait]
pub trait ManageTargetServer {
    async fn get_server_if_exists(ctx: &Context<'_>) -> Option<entity::target_server::Model> {
        let db_connection = &ctx.data().configs.database.connection;
        let user = match Support::get_user_if_exists(&ctx).await {
            Some(u) => u,
            None => {
                panic!("user not registered to db");
            }
        };
        TargetServer::find_by_user_model(&user, db_connection).await
    }
}

impl ManageTargetServer for Support {}