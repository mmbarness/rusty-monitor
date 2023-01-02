use dotenv::{ dotenv };
use gnomeutils::serenity::{GuildId};
use crate::database::initialize::Database;

#[derive(Debug, Clone)]
pub struct BotConfig {
    pub token: String,
    pub environment: Environment,
    pub database: Database,
    pub prefix: String,
    pub guild_id: GuildId,
}

#[derive(Debug, Clone, Copy)]
pub enum Environment {
    Dev,
    Prod,
    Test
}

impl Environment {
    pub fn match_env(unvalidated_env: &str) -> Environment {
        match unvalidated_env {
            "dev" => Environment::Dev,
            "prod" => Environment::Prod,
            "test" => Environment::Test,
            _ => panic!("Error parsing environment arg")                   
        }
    }
}

pub trait EnviromentString {
    fn to_string(env: Environment) -> String {
        match env {
            Environment::Dev => "dev".to_string(),
            Environment::Prod => "prod".to_string(),
            Environment::Test => "test".to_string(),
        }
    }
}

impl EnviromentString for Environment {}

impl BotConfig {

    pub async fn load () -> BotConfig {

        let args:Vec<String> = Self::env_vars();

        let environment = Self::environment(&args);
        let guild_id = Self::guild_id();
        let prefix = Self::prefix(&environment);
        let token = Self::token(&environment);
        let (db_password, db_username) = Self::db_creds();
        let database = Database::load(&db_password, &db_username, &environment).await;

        let parsed_guild_id = GuildId(guild_id);

        println!("running in {} mode, with a command prefix of {}", Environment::to_string(environment), prefix);

        BotConfig {
            environment,
            database,
            guild_id: parsed_guild_id,
            prefix,
            token,
        }
    }

    fn env_vars() -> Vec<String> {
        match dotenv() {
            Ok(_) => {
                println!(".env file found, using...")
            }
            Err(_) => {
                println!(".env file not found, looking elsewhere")
            }
        };
        let args = std::env::args().collect();
        println!("args: {:?}", args);
        args
    }

    fn db_creds() -> (String, String) {
        let username = match std::env::var("DATABASE_USERNAME") {
            Ok(db_username) => db_username,
            Err(_) => {
                panic!("Error accessing DATABASE_USERNAME in .env")
            }
        };
        let password = match std::env::var("DATABASE_PASSWORD") {
            Ok(db_password) => db_password,
            Err(_) => {
                panic!("Error accessing DATABASE_PASSWORD in .env")
            }
        };
        (username, password)
    }

    fn prefix(mode:&Environment) -> String {
        match mode {
            Environment::Dev => "~~".to_string(),
            Environment::Prod => "~".to_string(),
            _ => panic!("Error parsing environment arg")  
        }
    }

    fn environment(args: &Vec<String>) -> Environment {
        let environment = args.iter().find(|ele| match ele.as_str() {
            "dev" => true,
            "prod" => true,
            _ => false
        });

        return match environment {
            Some(env) => Environment::match_env(env),
            None => panic!("Did you provide an environment as an argument? Options are 'prod' or 'dev'")
        };
    }

    fn guild_id() -> u64 {
        return match std::env::var("DISCORD_GUILD_ID") {
            Ok(token) => token.parse::<u64>().expect("unable to correctly parse guild_id"),
            Err(_) => {
                panic!("Error accessing discord guild id in .env")
            }
        }
    }

    fn token(environment:&Environment) -> String {    
        return match environment {
            Environment::Dev => match std::env::var("DEV_DISCORD_TOKEN") {
                Ok(token) => token,
                Err(_) => {
                    panic!("Error accessing bot token in .env")
                }
            }
            Environment::Prod => match std::env::var("PROD_DISCORD_TOKEN") {
                Ok(token) => token,
                Err(_) => {
                    panic!("Error accessing bot token in .env")
                }
            }
            _ =>  panic!("Error parsing environment arg")                   
        };
    }

}