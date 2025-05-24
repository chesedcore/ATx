//configs.rs 

//[STRUCTS]

///a struct that holds the context required for the api to work.
pub struct ApiConfig {
    pub model: String,         //"gpt-4", "gpt-3.5-turbo"
    pub timeout: u32,          //time before death (sec)
    pub input_cost: f32,       //cost per 1m tokens given
    pub output_cost: f32,      //cost per 1m tokens received
    pub batch_size: u8,        //max items per api call
    pub frequency_penalty: f32, //inversely related to repeats
    
    pub api: String,            //who would've guessed
    pub key: String,            //WHO would've guessed
    pub org: String,            //does this even fucking matter
}

///stores prompt inputs and format args.
pub struct PromptConfig {
    pub language: String,      //"english", "japanese"
    pub prompt: String,        //the actual fucking prompt
    pub vocab: String,         //vocab needed to infer nouns
    pub general_width: u8,     //that but for dialogue
}


