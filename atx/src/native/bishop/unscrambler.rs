//bishop/unscrambler.rs 
use log::{error, info};
use crate::prelude::engine::Engine;

//how the fuck does this shit work?
//so a typical bsxx.dat looks like this:
/*
┌──────────────────────────────┐
│  Header (0x00 - 0x0F)        │ <- "BSXScript 3.0\0\0\0" or whatever the fuck
├──────────────────────────────┤
│  Field Block (0x10 - ~0xA3)  │ <- offsets and sizes for everything else
├──────────────────────────────┤
│  Code Block                  │ <- script opcodes (0x1D = "show dialogue/name")
├──────────────────────────────┤
│  NameOffsetList              │ <- Vec<u32>: offsets into NameBlock
├──────────────────────────────┤
│  NameBlock                   │ <- UTF-16LE strings like "Shohei\0", "Azusa\0"
├──────────────────────────────┤
│  MessageOffsetList           │ <- List of u32s: offsets into MessageBlock
├──────────────────────────────┤
│  MessageBlock                │ <- UTF-16LE dialogue: "Kyaa!\0\0", "My balls hurt\0\0"
└──────────────────────────────┘
*/

//what you do is try to parse the offsets and sizes. this indicates the 
//number of bytes to step forward upon encountering a non-display opcode.
//you keep walking forward depending on the opcode until you encounter the 
//very amazing 0x1D opcode. incredible. this is the DISPLAY opcode: it gives
//you the message and/or character name once you follow the offset given by it.
//do this enough times, and eventually you've extracted the message/charnames.
//wahoo!

const VALID_SIGNATURES: [&[u8]; 4] = [
    b"BSXScript 3.0\0\0\0",
    b"BSXScript 3.1\0\0\0",
    b"BSXScript 3.2\0\0\0",
    b"BSXScript 3.3\0\0\0",
];

///an unscrambler backend designed for BISHOP's scripting engine, BSXScript.
///thanks to Crsky's tool for the logic involved:
///https://github.com/crskycode/BSXScript_Tool/blob/main/BSXScript.cs/
///CS might be ass to look at, but at least their code isn't. 

pub struct BSXDecoder {
    script_buffer: Vec<u8>,   //borrows the pointer to the vec<u8> from load. 
    code_block_size: u32,
    code_block_offset: u32,  
    name_offsets: Vec<u32>,
    message_offsets: Vec<u32>,
}

impl BSXDecoder {
    
    pub fn new(script_buffer: Vec<u8>) -> std::io::Result<Self>{
        let header = &script_buffer[..16];
        
        //check header validity: you can't work on shit if it's not valid bsx
        if VALID_SIGNATURES.contains(&header) {
            info!("{}", format!("Header: {:?} verified!", &header));
        } else { 
            error!("{}", format!("No valid header found, instead found {:?}", header)) 
        }
        
        //******************************************CODE BLOCK*******************//
        let code_block_offset = Self::read_four_le_bytes_from(&script_buffer, 0x2C)?;
        let code_block_size = Self::read_four_le_bytes_from(&script_buffer, 0x30)?;
        info!("Code block headers extracted!");



        //***************************************NAME BLOCK***************************//
        //i can explain, i can explain!
        let name_offset_list_offset = Self::read_four_le_bytes_from(&script_buffer, 0x88)? as usize;
        //we're getting the OFFSET to the OFFSET of the name list. we do this because these offsets
        //are relative to the block start, so the final formula for the actual offset required is
        //name_list_offset = start + name_offset_list_offset * 2 
        //hopefully that made sense. :sob:
        
        //sizes are a 16 bit thing, so we deflate it to 16bit by divyying by 2
        let name_offset_list_size = Self::read_four_le_bytes_from(&script_buffer, 0x8C)? >> 2;
        let name_block_start = Self::read_four_le_bytes_from(&script_buffer, 0x90)?;
        info!("Name block headers extracted!");


        //********************************************MESSAGE BLOCK******// 
        //same logic as before
        let message_offset_list_offset = Self::read_four_le_bytes_from(&script_buffer, 0x98)? as usize;
        let message_offset_list_size = Self::read_four_le_bytes_from(&script_buffer, 0x9C)? >> 2;
        let message_block_start = Self::read_four_le_bytes_from(&script_buffer, 0xA0)?;
        info!("Message block headers extracted!");


        //*************************NAME OFFSETS************************// 
        let mut name_offsets = Vec::new();
        for i in 0..name_offset_list_size {
            //offset is i*4 (the list offset!) bytes AHEAD of the indicated name offset offsets
            //refer back to the diagram if you don't get this
            let offset = Self::read_four_le_bytes_from(&script_buffer, name_offset_list_offset + (i*4) as usize)?;
            name_offsets.push(name_block_start + offset*2);
        }
        info!("Name offsets extracted!");


        //*******************MESSAGE OFFSETS**************************// 
        let mut message_offsets = Vec::new();
        for i in 0..message_offset_list_size {
            let offset = Self::read_four_le_bytes_from(&script_buffer, message_offset_list_offset + (i*4) as usize)?;
            message_offsets.push(message_block_start + offset*2);
        }
        info!("Message offsets extracted!");


        let decoder = BSXDecoder {
            script_buffer,
            code_block_offset,
            code_block_size,
            name_offsets,
            message_offsets
        };
        
        Ok(decoder)
    }

    pub fn unscramble_text(&self) -> std::io::Result<()> {
        info!("Unscrambling started.");
        let code_start = self.code_block_offset as usize;
        let code_end = code_start + self.code_block_size as usize;
        let code = &self.script_buffer[code_start..code_end];


        let mut current_address: usize = 0;
        while current_address < code.len() {
            match code[current_address] {
                
                //don't understand shit? well, fuck me, i don't either :D
                //way too many magic numbers just floatin around...
                0x00..=0x02 | 0x0A | 0x0B | 0x1E..=0x32 | 0x35 | 0x37 | 0x39 | 0x3F..=0x41 => current_address += 1,
                0x03..=0x06 | 0x07 | 0x08 | 0x09 | 0x34 | 0x36 | 0x38 => current_address += 5,
                0x0C..=0x11 | 0x13..=0x19 | 0x3A..=0x3C | 0x12 => current_address += 18,
                0x1A..=0x1C | 0x3D => current_address += 12,
                0x33 => current_address += 13,
                0x3E => {
                    //why?? i don't fucking know why. 
                    let count = Self::read_four_le_bytes_from(code, current_address+1)?;
                    current_address += (5 + 4 * count) as usize;
                },

                //the actual magic happens HERE
                0x1D => {
                    info!("Display opcode found at 0x{:X}!", current_address);
                    
                    if current_address+2 > code.len() {break;}

                    let message_type = code[current_address+1];
                    let mut message_id = None;
                    let mut name_id = None;

                    match message_type {
                        0 => {message_id = Some(Self::read_four_le_bytes_from(code, current_address+2)?);}
                        1 | 2 | 3 => {
                            message_id = Some(Self::read_four_le_bytes_from(code, current_address+2)?);
                            name_id = Some(Self::read_four_le_bytes_from(code, current_address+6)?);
                        }

                        other => { error!("Unknown message type discovered at 0x{:X}! Type: {}", current_address, other); }
                    }

                    if let Some(id) = name_id {
                        if id < self.name_offsets.len() as u32 {
                            if let Some(name) = self.get_string_at(self.name_offsets[id as usize] as usize) {
                                info!("[{}]", name);
                            }
                        }
                    }

                    if let Some(id) = message_id {
                        if id < self.message_offsets.len() as u32 {
                            if let Some(message) = self.get_string_at(self.message_offsets[id as usize] as usize){
                                info!("{}", message);
                            }
                        }
                    }

                    let v2 = code[current_address + 1] as u32;
                    let mut v1 = 0;
                    if v2 > 1 && current_address+14 < code.len() {
                        v1 = 4 * Self::read_four_le_bytes_from(code, current_address+10)?;
                    }

                    current_address += (v1 + 4*v2 + 6) as usize;
                }


                _ => {
                    error!("Unknown opcode at 0x{:X}!", current_address);
                    break;
                }
            }
        }
        
        Ok(())
    }
    

    ///why four bytes? it's because every 'atom' (individual unit) of the binary is created with
    ///four LE bytes in mind
    fn read_four_le_bytes_from(script_buffer: &[u8], offset: usize) -> std::io::Result<u32> {
        let offset = offset as usize;
        //                  ^^^^^^^^^ this will crash if the script buffer is bigger than 4gb 
        //                  i sure am glad they'll need to create the world's biggest fucking VN 
        //                  to do that. to anyone reading this: don't use `as` like i am, lmao

        match script_buffer.get(offset..offset + 4) {
            Some(bytes) => match bytes.try_into() {
                Ok(array) => Ok(u32::from_le_bytes(array)),
                Err(_) => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Couldn't convert 4 bytes at offset {} into array!", offset),
                )),
            },
            None => Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!("Out of bounds at offset {}!", offset),
            )),
        }
    }

    ///walks forward from the given offset, attempting to carve out a double-null terminated
    ///UTF-16LE string. stores values in a vec<u16> BECAUSE GUESS WHAT IT'S FUCKING UTF16
    fn get_string_at(&self, offset: usize) -> Option<String> {
        let pos = offset; //crash warning for obnoxiously huge data!!!!
        let mut end = pos;

        let buffer = &self.script_buffer;
        while end + 1 < buffer.len() {
            if buffer[end] == 0 && buffer[end + 1] == 0 {
                break;
            }
            end += 2;
        }

        let utf16: Vec<u16> = (pos..end)
            .step_by(2)
            .map(
                |i| u16::from_le_bytes([buffer[i], buffer[i + 1]])
            ).collect();

        String::from_utf16(&utf16).ok()
    }
}
