use toml::Value;
use std::fs;
use crate::prelude::notifs::*;
use dotenvy::dotenv;
use std::env;

const PARSER_CONFIG_LOAD_PATH: &str = "ParserConfig.toml";
const REQUIRED_ENV_VARS: &[&str] = &[
    "api", "key", "organization", "model", "language", "note_width", "batchsize",
    "timeout", "file_threads", "threads", "width", "list_width", ""
];

pub fn load_parser_config() -> Result<Success<Value>, Disaster> {
    
    //grab the toml string
    let contents = fs::read_to_string(PARSER_CONFIG_LOAD_PATH)
        .map_err(|err| Disaster(format!("Failed to read ParserConfig.toml: {}", err)))?;
    
    //turn that string into a table of toml::Value
    let table = toml::from_str(&contents)
        .map_err(|err| Disaster(format!("Failed to parse ParserConfig.toml: {}", err)))?;

    Ok(
        Success::new(table)
            .log("Successfully loaded ParserConfig.toml")
    )
}
