use rppal::i2c::I2c;
use std::error::Error;
use crate::i2cSupport;

const DISPLAY_BUFFER_SZ: usize = 1024;

#[allow(non_camel_case_types)]
pub struct ssd1306{
    address: u16,
    i2c_instance: I2c,
    display_buffer: [u8; DISPLAY_BUFFER_SZ],
}

impl ssd1306{
    //class setup
    pub fn get_struct(address: u16, instance: I2c) -> ssd1306{
        
        ssd1306 { address: address, i2c_instance: instance, display_buffer: [0; DISPLAY_BUFFER_SZ] }
    }

    //hardware setup
    pub fn init(&mut self) -> Result<(), Box<dyn Error>>{
       
        self.i2c_instance.set_slave_address(self.address)?;
        
        match Self::config(&self){
            Ok(()) => println!("OLED setup"),
            Err(error) => panic!("SSD1306 OLED failed config {:?}: ", error),
        };

        self.display_buffer.fill(0);
        self.display_buffer[0] = 0x40;
        
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

    pub fn draw_pixel(&mut self, x: u8, y: u8) -> Result<(), Box<dyn Error>>{

        let offset: u8 = std::mem::size_of_val(&0x40) as u8; 
       
        let row: u8 = y / 8;
        let cell: u8 = ((x + row * 64) + offset).into();
        let bit: u8 = 1 << y  % 8;

        let mut tmp: u8 = self.display_buffer[cell as usize];
        tmp |= bit;
        self.display_buffer[cell as usize] = tmp;

        i2cSupport::write_data(&self.i2c_instance, &self.display_buffer)?;

        Ok(())
    }

    pub fn fill(&mut self, data: std::os::raw::c_uchar) -> Result<(), Box<dyn Error>>{

        self.display_buffer.fill(data as u8);
        i2cSupport::write_data(&self.i2c_instance, &self.display_buffer)?;

        Ok(())
    }

    //class exit
    pub fn close(&mut self) -> Result<(), Box<dyn Error>>{

        self.display_buffer.fill(0);
        i2cSupport::write_cmd(&self.i2c_instance, 0xAE)?;
        
        Ok(())
    }

}