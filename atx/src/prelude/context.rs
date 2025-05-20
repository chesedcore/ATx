//context.rs 
use std::sync::{Arc, Mutex};
use regex::Regex;
use crate::prelude::configs::{ApiConfig, PromptConfig};
use crate::prelude::flags::ParserFlags;
use crate::prelude::count::Count;

///the state that the translation process is in.
///think of it as the overall context.
pub struct Context<Engine> {
    pub api: ApiConfig,            //api stuff
    pub prompt: PromptConfig,      //prompt stuff
    pub flags: ParserFlags<Engine>,//game specific stuff
    pub regex: Regex,              //for japanese text detection
    pub threads: usize,            //track thread pool size
    pub token_count: Arc<Count>,   //track shared usage
    pub lock: Arc<Mutex<()>>       //to synchronise stdout
}

