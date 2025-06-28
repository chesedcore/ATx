//bishop.rs 

use crate::native::bishop::unscrambler::BSXDecoder;
use crate::prelude::engine::Engine;
use crate::prelude::saveload::Saveloader;
use log::info;

pub struct BishopEngine {
    unscrambler: BSXDecoder,
    saveloader: Saveloader,
}

impl BishopEngine {
    pub fn new() -> std::io::Result<Self> {
        let saveloader = Saveloader::new()?;
        let bytes = saveloader.load_raw("bsxx.dat")?;
        let unscrambler = BSXDecoder::new(bytes)?;
        
        let engine = BishopEngine{unscrambler, saveloader};
        Ok(engine)
    }
}

impl Engine for BishopEngine {
    fn unscramble(&self) -> std::io::Result<()> {
        let result_data = self.unscrambler.unscramble_text()?;
        self.saveloader.save_from("bsxx.txt", result_data)?;
        let name_data = self.unscrambler.collect_names();
        self.saveloader.save_from("name_list.txt", name_data)?;
        Ok(())
    }
}
