use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use regex::Regex;
//yeah, i know. fuck me


//[STRUCTURES]

///a struct that holds the context required for the api to work.
///intended to be built up from the `.env` file.
pub struct ApiConfig {
    pub model: String,         //"gpt-4", "gpt-3.5-turbo"
    pub timeout: u64,          //time before death (sec)
    pub input_cost: f32,       //cost per 1m tokens given
    pub output_cost: f32,      //cost per 1m tokens received
    pub batch_size: usize,     //max items per api call
    pub frequency_penalty: f32 //inversely related to repeats
}

///stores prompt inputs and format args.
pub struct PromptConfig {
    pub language: String,      //"english", "japanese"
    pub prompt: String,        //the actual fucking prompt
    pub vocab: String,         //vocab needed to infer nouns
    pub note_width: usize,     //wrap width for notes
    pub list_width: usize,     //wrap width for descriptions
    pub general_width: usize,  //that but for dialogue
}

///determines which if data should be processed or skipped.
///if these codes seem arbitrary, it's because RPGM just
///does shit this way :p 
#[derive(Debug, Clone)]
pub struct ParserFlags {
    pub code_401: bool,             //show text
    pub code_405: bool,             //scroll text
    pub code_102: bool,             //choices
    pub code_101: bool,             //name box
    pub code_408: bool,             //comments (recommended 0)
    pub code_122: bool,             //control vars
    pub code_355655: bool,          //script stuff 
    pub code_357: bool,             //plugins bs
    pub code_657: bool,             //fuck if i know
    pub code_356: bool,             //old format script
    pub code_319: bool,             //equipment change
    pub code_320: bool,             //name change
    pub code_321: bool,             //class change
    pub code_324: bool,             //nickname change
    pub code_111: bool,             //conditional logic
    pub code_108: bool,             //comments (0)
    pub first_line_speakers: bool,  //parse name lines
    pub facename_101: bool,         //infer speaker from face
    pub output_names: bool,         //print all char names
    pub br_flag: bool,              //treat <br> as line break
    pub fix_textwrap: bool,         //normalise newline/textwrap
    pub ignore_translated_text: bool,//skip repeat lines
}

///thread-safe state tracking.
///helps in computing costs and track usage across threads
#[derive(Debug, Default)]
pub struct Count {
    pub input: AtomicUsize,    //number of tokens thrown in
    pub output: AtomicUsize,   //number of tokens processed
}

///the state that the translation process is in.
///think of it as the overall context.
pub struct Context {
    pub api: ApiConfig,         //api stuff
    pub prompt: PromptConfig,   //prompt stuff
    pub flags: ParserFlags,     //game specific stuff
    pub regex: Regex,           //for japanese text detection
    pub threads: usize,         //track thread pool size
    pub token_count: Arc<Count>,//track shared usage
    pub lock: Arc<Mutex<()>>    //to synchronise stdout
}

//[IMPLEMENTATIONS]


