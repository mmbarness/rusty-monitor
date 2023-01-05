use dotenv::{ dotenv };
use gnomeutils::serenity::{GuildId};
use crate::database::initialize::Database;

#[derive(Debug, Clone)]
pub struct Config {
    pub token: String,
    pub environment: Environment,
    pub database: Database,
    pub prefix: String,
    pub guild_id: GuildId,
}

#[derive(Debug, Clone)]
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
    pub fn to_string(env: Environment) -> String {
        match env {
            Environment::Dev => "dev".to_string(),
            Environment::Prod => "prod".to_string(),
            Environment::Test => "test".to_string(),
        }
    }
}

impl Config {

    pub async fn load () -> Config {
        let args:Vec<String> = Self::env_vars();

        let environment = Self::environment(&args);
        let guild_id = Self::guild_id();
        let prefix = Self::prefix(&environment);
        let token = Self::token(&environment);
        let db_url = Self::db_url(&environment);
        let database = Database::load(&db_url).await;

        let parsed_guild_id = GuildId(guild_id);

        println!("running in {} mode, with a command prefix of {}", Environment::to_string(environment.clone()), prefix);

        Config {
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

    fn db_url(environment: &Environment) -> String {
        match environment {
            Environment::Dev => match std::env::var("DEV_DATABASE_URL") {
                Ok(url) => url,
                Err(_) => {
                    panic!("Error accessing DATABASE_USERNAME in .env")
                }
            },
            Environment::Test => match std::env::var("TEST_DATABASE_URL") {
                Ok(url) => url,
                Err(_) => {
                    panic!("Error accessing DATABASE_USERNAME in .env")
                }
            },
            Environment::Prod => match std::env::var("PROD_DATABASE_URL") {
                Ok(url) => url,
                Err(_) => {
                    panic!("Error accessing DATABASE_USERNAME in .env")
                }
            }
        }
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