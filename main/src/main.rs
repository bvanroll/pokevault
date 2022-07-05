use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::saves::{convert_trainer_name, Save};

mod test;
mod saves;

fn main() {

}

fn read_save(save_path: &Path) -> Result<Save, ()>{
    let f = File::open(save_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    let mut temp:Vec<u8> = Vec::new();
    for &var in &buffer[0x2598.. 0x2598 + 0xb] {

    }
    let save = Save {
        trainer_name: convert_trainer_name(buffer[0x2598 .. 0x2598 + 0xb]),
        playtime_hours: *&buffer[0x2CED]
    };
    return Ok(save); //learned something new today
}