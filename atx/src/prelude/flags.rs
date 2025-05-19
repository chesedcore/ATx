use toml::Value;
use crate::prelude::engines;
use std::marker::PhantomData;
//flags.rs 

///determines which data should be processed or skipped.
pub struct ParserFlags<Engine> {
    table: Value,
    engine: PhantomData<Engine>,
}

impl <Engine> ParserFlags<Engine> {
    fn from_toml(table: Value) -> Self {
        ParserFlags { table, PhantomData }
    }
    fn get(field: &str) ->
}
