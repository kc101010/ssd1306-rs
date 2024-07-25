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

//See sect. 9 of https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum Commands{
    DISPLAY_ON_RESUME_RAM = 0xA4,
    DISPLAY_ON_IGNORE_RAM = 0xA5,
    DISPLAY_NORMAL = 0xA6,
    DISPLAY_INVERSE = 0xA7,
    DISPLAY_OFF = 0xAE,
    DISPLAY_ON = 0xAF,
    DISPLAY_CLK_FREQ = 0xD5,

    SEG_REMAP_NORMAL= 0xA0,
    SEG_REMAP_REVERSE = 0xA1,

    COMOUT_SCAN_NORMAL = 0xC0,
    COMOUT_SCAN_REVERSE = 0xC8,

    SET_COM_PIN = 0xDA,
    SET_DISPLAY_OFFSET = 0xD3,
    SET_PRECHARGE = 0xD9,
    SET_MULTIPLEX = 0xA8,
    SET_PAGE_ADDRESS = 0x22,
    SET_VCOMH_LEVEL = 0xDB,

    SET_PAGE_START_0 = 0xB0,
    SET_PAGE_START_1 = 0xB1,
    SET_PAGE_START_2 = 0xB2,
    SET_PAGE_START_3 = 0xB3,
    SET_PAGE_START_4 = 0xB4,
    SET_PAGE_START_5 = 0xB5,
    SET_PAGE_START_6 = 0xB6,
    SET_PAGE_START_7 = 0xB7,

}

impl ssd1306{
    //class setup
    pub fn get_struct(address: u16, instance: I2c) -> ssd1306{
        
        ssd1306 { address: address, i2c_instance: instance, display_buffer: [0; DISPLAY_BUFFER_SZ] }
    }

    //hardware init
    pub fn init(&mut self) -> Result<(), Box<dyn Error>>{
       
        self.i2c_instance.set_slave_address(self.address)?;
        
        match Self::config(&self){
            Ok(()) => println!("OLED setup"),
            Err(error) => panic!("SSD1306 OLED failed config {:?}: ", error),
        };

        self.fill(0x00)?;
        self.display_buffer[0] = 0x40;
        
        Ok(())
    }

    //set OLED config
    fn config(&self) -> Result<(), Box<dyn Error>>{
        i2cSupport::write_cmd(&self.i2c_instance, Commands::DISPLAY_OFF as u8)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::DISPLAY_CLK_FREQ as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x80)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_MULTIPLEX as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x3F)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_DISPLAY_OFFSET as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x00)?;

        i2cSupport::write_cmd(&self.i2c_instance, 0x40)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x8D)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x14)?;

        i2cSupport::write_cmd(&self.i2c_instance, 0x20)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x00)?;
        i2cSupport::write_cmd(&self.i2c_instance, Commands::SEG_REMAP_REVERSE as u8)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::COMOUT_SCAN_REVERSE as u8)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_COM_PIN as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x12)?;

        i2cSupport::write_cmd(&self.i2c_instance, 0x81)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x80)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_PRECHARGE as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xF1)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_VCOMH_LEVEL as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x20)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::DISPLAY_ON_RESUME_RAM as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, Commands::DISPLAY_NORMAL as u8)?;

        i2cSupport::write_cmd(&self.i2c_instance, 0x2E)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::DISPLAY_ON as u8)?;

        Ok(())
    }
    
    //draw a pixel to the OLED
    pub fn draw_pixel(&mut self, x: u8, y: u8) -> Result<(), Box<dyn Error>>{

        /*let offset: u8 = std::mem::size_of_val(&0x40) as u8; 
       
        let row: u8 = y / 8;
        let cell: u8 = ((x + row * 64) + offset);
        let bit: u8 = 1 << y  % 8;

        let mut tmp: u8 = self.display_buffer[cell as usize];
        tmp |= bit;
        self.display_buffer[cell as usize] = tmp;*/

        self.display_buffer[(x + (y / 8) * 64 as u8) as usize ] |= 1 << (y % 8);

        i2cSupport::write_data(&self.i2c_instance, &self.display_buffer)?;

        Ok(())
    }


    //fill the OLED screen
    pub fn fill(&mut self, data: std::os::raw::c_uchar) -> Result<(), Box<dyn Error>>{

        self.display_buffer.fill(data as u8);

        for _data in self.display_buffer{
            i2cSupport::write_data(&self.i2c_instance, &self.display_buffer)?;
        }

        Ok(())
    }

    //class exit
    pub fn close(&mut self) -> Result<(), Box<dyn Error>>{

        self.fill(0x00)?;
        i2cSupport::write_cmd(&self.i2c_instance, Commands::DISPLAY_OFF as u8)?;
        
        Ok(())
    }
 
}

