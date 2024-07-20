use rppal::i2c::I2c;

use std::error::Error;

pub fn write_cmd(instance : &I2c, cmd: u8) -> Result<(), Box<dyn Error>>{
    
    Ok( instance.block_write(
         0x00,
         &[cmd]
     ) ?)
}

pub fn write_data(instance: &I2c, buffer: &[std::os::raw::c_uchar; 1024]) -> Result<(), Box<dyn Error>>{
    
    match instance.block_write(0x40, buffer){
        Ok(_) => {},
        Err(error) => panic!("i2cSupport error write_data {:?} ", error)
    };

    Ok(())

    
}

