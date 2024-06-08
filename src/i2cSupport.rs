use rppal::i2c::I2c;

use std::error::Error;

pub fn write_cmd(instance : &I2c, cmd: u8) -> Result<(), Box<dyn Error>>{
    
    Ok( instance.block_write(
         0x00,
         &[cmd]
     ) ?)
 }
 