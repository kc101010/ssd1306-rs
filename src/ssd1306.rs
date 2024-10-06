/// ssd1306.rs 
/// 
/// Written by Kyle Christie (kc101010), 05/10/2024
/// 
/// Class drives the ssd1306 OLED over i2c making use of
/// rppal libraries. This code is specifically designed
/// for use on raspberry pi. 
use rppal::i2c::I2c;
use std::error::Error;
use std::mem::size_of;
use crate::i2cSupport;

/// Const value for size of display buffer
const DISPLAY_BUFFER_SZ: usize = 1024 + 1;

/// Const value for OLED i2c address
pub const OLED_ADDR: u16 = 0x3C;


/// Underlying struct represents key OLED data
/// 
/// Data;
/// 
/// ``` address ```      - stores I2C address
/// 
/// ``` i2c_instance ``` - stores instance of underlying i2c driver
/// 
/// ``` display_buffer```- stores display buffer to be written to screen
/// 
/// Class drives the ssd1306 OLED screen using rppal library to provide
/// control over i2c.  
/// 
/// Wider implementation gives various pieces of funcionality including;
/// 
/// + Filling the screen
/// + Drawing individual pixels
#[allow(non_camel_case_types)]
pub struct ssd1306{ 
    address: u16, 
    i2c_instance: I2c,
    display_buffer: [u8; DISPLAY_BUFFER_SZ],
}

/// Enum of commands which can be written to the OLED.
/// Encompasses named commands from datasheet that
/// allow configuration of the SSD1306. 
/// 
/// This is then used in the config func to clarify
/// which configs are being set and allow for easier
/// reading.
/// 
/// 
/// See sect. 9 of https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum Commands{
    DISPLAY_ON_RESUME_RAM = 0xA4,
    DISPLAY_ON_IGNORE_RAM = 0xA5,
    DISPLAY_NORMAL = 0xA6,
    DISPLAY_INVERSE = 0xA7,
    DISPLAY_OFF = 0xAE,
    DISPLAY_ON = 0xAF,
    SET_DISPLAY_CLK_FREQ = 0xD5,

    SEG_REMAP_NORMAL= 0xA0,
    SEG_REMAP_REVERSE = 0xA1,

    COMOUT_SCAN_NORMAL = 0xC0,
    COMOUT_SCAN_REVERSE = 0xC8,

    SET_COM_PIN = 0xDA,
    SET_DISPLAY_OFFSET = 0xD3,
    SET_PRECHARGE = 0xD9,
    SET_MULTIPLEX = 0xA8,
    SET_VCOMH_LEVEL = 0xDB,

    SET_MEM_ADDRESS_MODE = 0x20,
    SET_COLUMN_ADDRESS = 0x21,
    SET_PAGE_ADDRESS = 0x22,

    SET_START_LINE = 0x40,

    DEACTIVATE_SCROLL = 0x2E,
    ACTIVATE_SCROLL = 0x2F,

    SET_CONTRAST_CONTROL_BANK0 = 0x81,
    SET_CHARGE_PUMP = 0x8D,
    ENABLE_CHARGE_PUMP = 0x14,

    SET_PAGE_START_0 = 0xB0,
    SET_PAGE_START_1 = 0xB1,
    SET_PAGE_START_2 = 0xB2,
    SET_PAGE_START_3 = 0xB3,
    SET_PAGE_START_4 = 0xB4,
    SET_PAGE_START_5 = 0xB5,
    SET_PAGE_START_6 = 0xB6,
    SET_PAGE_START_7 = 0xB7,

}


/// Class implements functionality for the SSD1306 OLED
impl ssd1306{
    
    /// Function allows access to struct data 
    pub fn get_struct(address: u16, instance: I2c) -> ssd1306{
        
        //init class variables
        ssd1306 { address: address, i2c_instance: instance, display_buffer: [0; DISPLAY_BUFFER_SZ] }
    }

    //hardware init
    pub fn init(&mut self) -> Result<(), Box<dyn Error>>{
       
        //set I2C slave address
        self.i2c_instance.set_slave_address(self.address)?;
        
        //If config succeeds, print message and move otherwise panic
        match Self::config(&self){
            Ok(()) => println!("OLED setup"),
            Err(error) => panic!("SSD1306 OLED failed config {:?}: ", error),
        };

        //Zero screen data - ensure screen is blank on startup
        self.fill(0x00)?;
        self.display_buffer[0] = 0x40 as u8;
        
        Ok(())
    }

    /// Set configuration for OLED over i2c
    /// 
    /// https://github.com/Dev4Embedded/ssd1306 - this driver helped figure some things out!
    /// 
    /// Note that when setting page and column addresses, that a start AND end address should
    /// be provided!! If this is not provided then the display will give unexpected behaviour!
    fn config(&self) -> Result<(), Box<dyn Error>>{
       
        i2cSupport::write_cmd(&self.i2c_instance, Commands::DISPLAY_OFF as u8)?;
        
        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_DISPLAY_CLK_FREQ as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x80)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_MULTIPLEX as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x3F)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_DISPLAY_OFFSET as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x00)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_START_LINE as u8)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_CHARGE_PUMP as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, Commands::ENABLE_CHARGE_PUMP as u8)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_MEM_ADDRESS_MODE as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x00)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_COLUMN_ADDRESS as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x00)?; //Start addr
        i2cSupport::write_cmd(&self.i2c_instance, 127)?;  //End addr


        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_PAGE_ADDRESS as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x00)?; //Start addr
        i2cSupport::write_cmd(&self.i2c_instance, 7)?;    //End addr

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SEG_REMAP_REVERSE as u8)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::COMOUT_SCAN_REVERSE as u8)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_COM_PIN as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x12)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_CONTRAST_CONTROL_BANK0 as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x9F)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_PRECHARGE as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0xF1)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::SET_VCOMH_LEVEL as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, 0x20)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::DISPLAY_ON_RESUME_RAM as u8)?;
        i2cSupport::write_cmd(&self.i2c_instance, Commands::DISPLAY_NORMAL as u8)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::DEACTIVATE_SCROLL as u8)?;

        i2cSupport::write_cmd(&self.i2c_instance, Commands::DISPLAY_ON as u8)?;

        Ok(())
    }
    
    //draw a pixel to the OLED
    pub fn draw_pixel(&mut self, x: i32, y: i32) -> Result<(), Box<dyn Error>>{
        
        const OFFSET: i32 = size_of::<u8>() as i32;
        
        let row: i32 = y / 8;
        let cell: usize = (((x  + row  * 64) + OFFSET) - 1 ) as usize;
        let bit: u8 = 1 << (y as u8 % 8);

        println!("cell: {:?}, bit: {:?}", cell, bit );
        self.display_buffer[cell] |= bit;

        //println!("{:?}", self.display_buffer);

        i2cSupport::write(&mut self.i2c_instance, &self.display_buffer)?;

        Ok(())
    }


    //fill the OLED screen
    pub fn fill(&mut self, data: std::os::raw::c_uchar) -> Result<(), Box<dyn Error>>{

        //fill display buffer with given data
        self.display_buffer.fill(data as u8);

        //write display buffer over bus
        i2cSupport::write(&mut self.i2c_instance, &self.display_buffer)?;

        Ok(())
    }

    //class exit
    pub fn close(&mut self) -> Result<(), Box<dyn Error>>{

        //Empty RAM then turn display off
        self.fill(0x00)?;
        i2cSupport::write_data(&self.i2c_instance, &self.display_buffer)?;
        i2cSupport::write_cmd(&self.i2c_instance, Commands::DISPLAY_OFF as u8)?;
        
        
        Ok(())
    }
 
}

