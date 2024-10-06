/// main.rs 
/// 
/// Written by Kyle Christie (kc101010), 05/10/2024
/// 
/// Project point of entry. Generally used for testing.
/// 
/// https://nitschinger.at/Writing-an-embedded-display-driver-in-Rust/
/// 
use rppal::i2c::I2c;
use std::error::Error;
use std::time::Duration;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

mod i2cSupport;
mod ssd1306;

/// Main function, used for functional testing during development. 
fn main() -> Result<(), Box<dyn Error>> {

    let i2c;

    //construct i2c bus then i2c class
    match I2c::with_bus(1){
        Ok(_) => {i2c = I2c::new()?},
        Err(error) => panic!("SSD1306 OLED I2C bus {:?}: ", error),

    };
    
    //setup oled class with I2C address and instance
    let mut oled = ssd1306::ssd1306::get_struct(ssd1306::OLED_ADDR, i2c);
    
    //TEST CODE
    match oled.init(){
        Ok(_) => {println!("OLED new");},
        Err(error) => panic!("SSD1306 OLED Could not init {:?}: ", error),

    };
    
    /*match oled.fill(0xFF){
        Ok(_) => {println!("OLED fill");},
        Err(error) => panic!("SSD1306 OLED Could not fill {:?} ", error),

    };

    match oled.fill(0x00){
        Ok(_) => {println!("OLED fill");},
        Err(error) => panic!("SSD1306 OLED Could not fill {:?} ", error),

    };*/

    println!("Attempting to draw pixels");

    /*for x in 1..1024 {
        match oled.draw_pixel(x, 0){
            Ok(_) => {println!("Pixel drawn");},
            Err(error ) => panic!("SSD1306 OLED Could not draw: {:?} ", error),
        };
    }*/

    match oled.draw_pixel(0, 0){
        Ok(_) => {println!("Pixel drawn");},
        Err(error ) => panic!("SSD1306 OLED Could not draw: {:?} ", error),
    };

    match oled.draw_pixel(1, 0){
        Ok(_) => {println!("Pixel drawn");},
        Err(error ) => panic!("SSD1306 OLED Could not draw: {:?} ", error),
    };
    
    match oled.draw_pixel(2, 0){
        Ok(_) => {println!("Pixel drawn");},
        Err(error ) => panic!("SSD1306 OLED Could not draw: {:?} ", error),
    };

    match oled.draw_pixel(3, 0){
        Ok(_) => {println!("Pixel drawn");},
        Err(error ) => panic!("SSD1306 OLED Could not draw: {:?} ", error),
    };

    match oled.draw_pixel(4, 0){
        Ok(_) => {println!("Pixel drawn");},
        Err(error ) => panic!("SSD1306 OLED Could not draw: {:?} ", error),
    };

    match oled.draw_pixel(5, 0){
        Ok(_) => {println!("Pixel drawn");},
        Err(error ) => panic!("SSD1306 OLED Could not draw: {:?} ", error),
    };

    match oled.draw_pixel(6, 0){
        Ok(_) => {println!("Pixel drawn");},
        Err(error ) => panic!("SSD1306 OLED Could not draw: {:?} ", error),
    };

    match oled.draw_pixel(7, 0){
        Ok(_) => {println!("Pixel drawn");},
        Err(error ) => panic!("SSD1306 OLED Could not draw: {:?} ", error),
    };

    match oled.draw_pixel(8, 0){
        Ok(_) => {println!("Pixel drawn");},
        Err(error ) => panic!("SSD1306 OLED Could not draw: {:?} ", error),
    };

    match oled.draw_pixel(9, 0){
        Ok(_) => {println!("Pixel drawn");},
        Err(error ) => panic!("SSD1306 OLED Could not draw: {:?} ", error),
    };

    match oled.draw_pixel(10, 0){
        Ok(_) => {println!("Pixel drawn");},
        Err(error ) => panic!("SSD1306 OLED Could not draw: {:?} ", error),
    };
   
    println!("End pixel attempt");

    std::thread::sleep(Duration::from_secs(3));

    match oled.close(){
        Ok(_) => {println!("OLED close");},
        Err(error) => panic!("SSD1306 OLED Could not close {:?}: ", error),

    };

    Ok(())
}

/// Collection of unit tests to prove driver functionality
#[cfg(test)]
mod tests{
    use super::*;

    /// Unit test to check whether init function succeeds
    #[test]
    fn test_init(){
        let i2c = I2c::new().expect("I2C err");
        let mut oled = ssd1306::ssd1306::get_struct(ssd1306::OLED_ADDR, i2c);

        assert_eq!(oled.init().ok(), Some(()));

        match oled.close(){
            Ok(_) => {println!("OLED close");},
            Err(error) => panic!("SSD1306 OLED Could not close {:?}: ", error),
    
        };
    }

    /// Unit test for fill function with zeroed data
    #[test]
    fn test_fill_blank(){
        let i2c = I2c::new().expect("I2C err");
        let mut oled = ssd1306::ssd1306::get_struct(ssd1306::OLED_ADDR, i2c);
        
        match oled.init(){
            Ok(_) => {},
            Err(error) => panic!("SSD1306 OLED Could not init {:?}: ", error),
    
        };

        assert_eq!(oled.fill(0x00).ok(), Some(()));

        match oled.close(){
            Ok(_) => {},
            Err(error) => panic!("SSD1306 OLED Could not close {:?}: ", error),
    
        };

    }

    /// Unit test for fill function with maxed data
    #[test]
    fn test_fill_full(){
        let i2c = I2c::new().expect("I2C err");
        let mut oled = ssd1306::ssd1306::get_struct(ssd1306::OLED_ADDR, i2c);
        
        match oled.init(){
            Ok(_) => {},
            Err(error) => panic!("SSD1306 OLED Could not init {:?}: ", error),
    
        };

        assert_eq!(oled.fill(0xFF).ok(), Some(()));

        match oled.close(){
            Ok(_) => {},
            Err(error) => panic!("SSD1306 OLED Could not close {:?}: ", error),
    
        };
    }

    /// Unit test to check close function succeeds
    #[test]
    fn test_exit(){
        let i2c = I2c::new().expect("I2C err");
        let mut oled = ssd1306::ssd1306::get_struct(ssd1306::OLED_ADDR, i2c);

        match oled.init(){
            Ok(_) => {},
            Err(error) => panic!("SSD1306 OLED Could not init {:?}: ", error),
    
        };

        assert_eq!(oled.close().ok(), Some(()));
    }
    
    /// Unit test checks for panic if driver is closed without proper init
    #[test]
    #[should_panic]
    fn test_bad_exit(){
        let i2c = I2c::new().expect("I2C err");
        let mut oled = ssd1306::ssd1306::get_struct(ssd1306::OLED_ADDR, i2c);

        match oled.close(){
            Ok(_) => {},
            Err(error) => panic!("SSD1306 OLED Could not close {:?}: ", error),
    
        };

    }

    
}