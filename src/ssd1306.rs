use rppal::i2c::I2c;
use std::error::Error;
use crate::i2cSupport;

#[allow(non_camel_case_types)]
pub struct ssd1306{
    address: u16,
    i2c_instance: I2c
}

impl ssd1306{
    //class setup
    pub fn get_struct(address: u16, instance: I2c) -> ssd1306{
        ssd1306 { address: address, i2c_instance: instance }
    }

    //hardware setup
    pub fn init(&mut self) -> Result<(), Box<dyn Error>>{
       
        self.i2c_instance.set_slave_address(self.address)?;
        
        match Self::config(&self){
            Ok(()) => println!("OLED setup"),
            Err(error) => panic!("SSD1306 OLED failed config {:?}: ", error),
        };
        
        Ok(())
    }

    //OLED setup
    fn config(&self) -> Result<(), Box<dyn Error>>{
        i2cSupport::write_cmd(&self.i2c_instance, 0xAE)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xD5)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x80)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xA8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x3F)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xD3)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x00)?;

        i2cSupport::write_cmd(&self.i2c_instance, 0x40)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x8D)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x14)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x20)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x00)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xA1)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xC8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xDA)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x12)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x81)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x80)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xD9)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xF1)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xDB)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x20)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xA4)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xA6)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x2E)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xAF)?;

        Ok(())
    }

    //class exit
    pub fn close(&self) -> Result<(), Box<dyn Error>>{
        i2cSupport::write_cmd(&self.i2c_instance, 0xAE)?;
        
        Ok(())
    }

}