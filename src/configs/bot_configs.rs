use dotenv::{ dotenv };
use gnomeutils::serenity::{GuildId};

pub struct BotConfig {
    pub token: String,
    pub environment: String,
    pub prefix: String,
    pub guild_id: GuildId,
}

impl BotConfig {

    pub fn load () -> BotConfig {

        let args:Vec<String> = Self::env_vars();

        let environment = Self::mode(&args);
        let guild_id = Self::guild_id();
        let prefix = Self::prefix(&environment);
        let token = Self::token(&environment);

        let parsed_guild_id = GuildId(guild_id);

        println!("running in {} mode, with a command prefix of {}", environment, prefix);

        BotConfig {
            environment,
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
        args
    }

    fn prefix(mode:&String) -> String {
        match mode.as_str() {
            "dev" => "~~".to_string(),
            "prod" => "~".to_string(),
            _ => panic!("Error parsing environment arg")  
        }
    }

    fn mode(args: &Vec<String>) -> String {
        let environment = args.iter().find(|ele| match ele.as_str() {
            "dev" => true,
            "prod" => true,
            _ => false
        });

        return match environment {
            Some(env) => match env.as_str() {
                "dev" => "dev".to_string(),
                "prod" => "prod".to_string(),
                _ => panic!("Error parsing environment arg")                   
            }
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

    fn token(environment:&String) -> String {    
        return match environment.as_str() {
            "dev" => match std::env::var("DEV_DISCORD_TOKEN") {
                Ok(token) => token,
                Err(_) => {
                    panic!("Error accessing bot token in .env")
                }
            }
            "prod" => match std::env::var("PROD_DISCORD_TOKEN") {
                Ok(token) => token,
                Err(_) => {
                    panic!("Error accessing bot token in .env")
                }
            }
            _ =>  panic!("Error parsing environment arg")                   
        };
    }

}