//flags.rs 
use toml::Value;
use std::marker::PhantomData;
use crate::prelude::notifs::*;
use crate::prelude::engines;

///determines which data should be processed or skipped.
#[derive(Debug)]
pub struct ParserFlags<Engine> {
    table: Value,
    engine: PhantomData<Engine>,
}

impl <Engine> ParserFlags<Engine> {
    ///assumes ownership of the table provided to the flag.
    pub fn from_toml(table: Value) -> Self {
        ParserFlags { table, engine: PhantomData }
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

impl ParserFlags<engines::RPGMaker> {
    ///attempts to grab the code from the table.
    pub fn get_code(&self, field: &str) -> Result<Success<bool>, Disaster> {
        let codes = self.get_raw("RPGMaker"); 

        
        let table = match codes.val{
            //grab the rpgmaker table
            Value::Table(t) => t,
            _ => return Err(Disaster("No RPGMaker table found in ParserConfig.toml!".to_string())),
        };
        
        //define green and red paths
        match table.get(field) {
            Some(Value::Boolean(b)) => Ok(
                codes.chain(*b).log(&format!("Fetched bool {} from RPGM field {}.", b, field))
            ),
            Some(other) => Err(Disaster(
                    format!("RPGM field {} is not a boolean! Found {:?} instead.", field, other)
            )),
            None => Ok(
                codes.chain(false).warn(&format!("No bool found for field {}, defaulting to false.", field))
            )
        }
    }
}
