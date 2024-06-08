use rppal::i2c::I2c;
use std::error::Error;
use std::thread;
use std::time::Duration;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

mod i2cSupport;
mod ssd1306;

const OLED_ADDR: u16 = 0x3C;

fn main() -> Result<(), Box<dyn Error>> {

    let i2c;

    //construct i2c bus then i2c class
    match I2c::with_bus(1){
        Ok(_) => {i2c = I2c::new()?},
        Err(error) => panic!("SSD1306 OLED I2C bus {:?}: ", error),

    };
    
    //setup oled class with I2C address and instance
    let mut oled = ssd1306::ssd1306::get_struct(OLED_ADDR, i2c);

    //TEST CODE - init OLED, pause 5 secs then close
    match oled.init(){
        Ok(_) => {println!("OLED new");},
        Err(error) => panic!("SSD1306 OLED Could not init {:?}: ", error),

    };
    
    thread::sleep(Duration::from_secs(5));

    match oled.close(){
        Ok(_) => {println!("OLED close");},
        Err(error) => panic!("SSD1306 OLED Could not close {:?}: ", error),

    };

    Ok(())
}