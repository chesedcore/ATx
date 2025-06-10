//bishop.rs 

use crate::native::bishop::unscrambler::BSXDecoder;
use crate::prelude::engine::Engine;
use crate::prelude::saveload::Saveloader;

pub struct BishopEngine {
    unscrambler: BSXDecoder,
}

impl BishopEngine {
    pub fn new() -> std::io::Result<Self> {
        let saveloader = Saveloader::new()?;
        let bytes = saveloader.load_raw("bsxx.dat")?;
        let unscrambler = BSXDecoder::new(bytes)?;
        
        let engine = BishopEngine{unscrambler};
        Ok(engine)
    }
}

impl Engine for BishopEngine {
    fn unscramble(&self) {
        self.unscrambler.unscramble_text();
    }
}
