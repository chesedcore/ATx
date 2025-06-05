//flags.rs 
use toml::Value;
use crate::prelude::notifs::Success;

///determines which data should be processed or skipped.
#[derive(Debug)]
pub struct ParserFlags {
    table: Value,
}

impl ParserFlags {
    ///assumes ownership of the table provided to the flag.
    pub fn from_toml(table: Value) -> Self {
        ParserFlags { table }
    }

    ///attempts to fetch a raw &Value from the toml table.
    pub fn get_raw(&self, field: &str) -> Success<&Value> {
        match &self.table.get(field) {
            Some(val) => Success::new(val),
            None => {
                Success::new(&Value::Boolean(false))
                    .warn(&format!("No field detected for query {}, defaulting to false", field))
            }
        }
    }
}

