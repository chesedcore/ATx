//context.rs 
use std::sync::{Arc, Mutex};
use crate::prelude::configs::{ApiConfig, PromptConfig};
use crate::prelude::flags::ParserFlags;
use crate::prelude::count::Count;

///the state that the translation process is in.
///think of it as the overall context.
///a "master node" suffices for your thought process too, probably.
pub struct Context {
    pub api: ApiConfig,                   //api stuff
    pub prompt_config: PromptConfig,      //prompt stuff
    pub flags: ParserFlags,               //game specific stuff
    pub threads: usize,                   //track thread pool size
    pub token_count: Arc<Count>,          //track shared usage
    pub lock: Arc<Mutex<()>>              //to synchronise stdout
}
