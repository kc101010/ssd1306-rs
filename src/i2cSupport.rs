/// i2cSupport.rs 
/// 
/// Written by Kyle Christie (kc101010), 05/10/2024
/// 
/// Class abstracts select functions from rppal i2c lib
/// for use in ssd1306 driver.
use rppal::i2c::I2c;
use std::error::Error;

/// Function abstracts 'block_write', configured to write commands to device
pub fn write_cmd(instance : &I2c, cmd: u8) -> Result<(), Box<dyn Error>>{
    
    Ok( instance.block_write(
         0x00,
         &[cmd]
     ) ?)
}

/// Function abstracts 'block_write' configured to write data to the device
pub fn write_data(instance: &I2c, buffer: &[u8; 1025]) -> Result<(), Box<dyn Error>>{
    
    match instance.block_write(0x40, buffer){
        Ok(_) => {},
        Err(error) => panic!("i2cSupport error write_data {:?} ", error)
    };

    Ok(())
    
}

/// Function abstracts i2c driver 'write' func to allow data to be sent 'as-is'
pub fn write(instance: &mut I2c, buffer: &[u8]) -> Result<(), Box<dyn Error>>{

    match instance.write(buffer) {
        Ok(_) => {},
        Err(error) => panic!("i2cSupport error write {:?} ", error)
    }

    Ok(())
}

