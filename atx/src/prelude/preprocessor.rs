//preprocessor.rs 
use crate::prelude::notifs::*;
use std::marker::PhantomData;
use std::fs::File;
use std::io::BufReader;

///preprocessor for the engine. engine specific impls live in the `native` folder.
///generic impls live here.

const SOURCE_TEXT_DIR: &str = "from";

pub struct Preprocessor<Engine> {
    marker: PhantomData<Engine>,
    reader: BufReader<File>, //the actual given input
}

impl<Engine> Preprocessor<Engine> {
    pub fn from_plaintext_file(filepath: &str) -> Result<Success<Self>, Disaster>{
        //open up that file
        let file = File::open(&format!("{}/{}", SOURCE_TEXT_DIR, filepath))
            .map_err(|err|Disaster(format!("{}",err)))?;

        //turn that file into a reader
        let reader = BufReader::new(file);
        Ok(
            Success::new(Self{marker:PhantomData, reader})
                .log(&format!("Opened {} successfully!", filepath))
        )
    }
}
