use entity::target_server::Model;
use serenity::Error;

use crate::{configs::mprober_configs::MProberConfigs, structs::Context, bot::{Bot, invocation_data::InvocationData}};

use super::{requester::Request, client::Client};
#[derive(Debug, Clone)]
pub struct MProberAPI {
    pub configs: MProberConfigs,
    pub requester: Request,
}

impl MProberAPI {
    pub fn load(target_server: &Model) -> MProberAPI {
        let configs = MProberConfigs::load(target_server);
        let client = Self::client(&configs);
        MProberAPI { configs, requester: Self::requester(client) }
    }

    fn client(configs: &MProberConfigs) -> Client {
        Client { auth: configs.auth,  auth_key: configs.api_key.clone() }
    }

    fn requester(client: Client) -> Request {
        Request { client }
    }

    pub async fn validate_from_invocation_data(ctx: Context<'_>) -> Result<MProberAPI, Error> {
        match ctx.invocation_data::<InvocationData>().await {
            Some(bot) => Ok(bot.mprober_api.clone()),
            None => {
                ctx.say("we weren\t able to get your server info. Maybe try again.").await;
                return Err(Error::Other("error pulling data from invocation_data, which means it wasn\'t set correctly in pre-command"))
            }
        }
    }
}