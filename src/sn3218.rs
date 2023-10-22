use std::error::Error;
use rppal::i2c::I2c;

const I2C_ADDRESS: u16 = 0x54;
const CMD_ENABLE_OUTPUT: u8 = 0x00;
const CMD_SET_PWM_VALUES: u8 = 0x01;
const CMD_ENABLE_LEDS: u8 = 0x13;
const CMD_UPDATE: u8 = 0x16;
const CMD_RESET: u8 = 0x17;

pub struct SN3218 {
    i2c: I2c,
    channel_gamma_table: Vec<Vec<i32>>
}

#[allow(dead_code)]
impl SN3218 {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // creating the channel gamma table
        let mut default_gamma_table: Vec<i32> = Vec::new();

        for i in 0..256 {
            let exponent = (i - 1) as f64 / 255.0;
            let pow_result = f64::powf(255.0, exponent);
            default_gamma_table.push(pow_result as i32);
        }

        let mut channel_gamma_table: Vec<Vec<i32>> = Vec::new();
        for _ in 0..18 {
            channel_gamma_table.push(default_gamma_table.clone());
        }

        //setting up i2c
        let mut i2c = I2c::new()?;
        i2c.set_slave_address(I2C_ADDRESS).expect("Error writing to i2c");
        

        Ok(Self {
            i2c,
            channel_gamma_table
        })
    }

    pub fn reset(&mut self) {
        self.i2c.block_write(CMD_RESET, &[0xFF]).expect("Error writing to i2c");
    }

    pub fn update(&mut self) {
        self.i2c.block_write(CMD_RESET, &[0xFF]).expect("Error writing to i2c");
    }

    pub fn enable(&mut self) {
        self.i2c.block_write(CMD_ENABLE_OUTPUT, &[0x01]).expect("Error writing to i2c");
    }

    pub fn disable(&mut self) {
        self.i2c.block_write(CMD_ENABLE_OUTPUT, &[0x00]).expect("Error writing to i2c");
    }

    pub fn output(&mut self, buffer: &[u8]) {
        let result: Vec<u8> = (0..18).map(|i| self.channel_gamma_table[i][buffer[i] as usize] as u8).collect();
        self.i2c.block_write(CMD_SET_PWM_VALUES, &result).expect("Error writing to i2c");
        self.i2c.block_write(CMD_UPDATE, &[0xFF]).expect("Error writing to i2c");
    }

    pub fn enable_leds(&mut self, mask: u32) {
        let processed_mask: &[u8] = &[
            (mask & 0x3F) as u8,
            ((mask >> 6) & 0x3F) as u8,
            ((mask >> 12) & 0x3F) as u8,
        ];
        self.i2c.block_write(CMD_ENABLE_LEDS, processed_mask).expect("Error writing to i2c");
        self.i2c.block_write(CMD_UPDATE, &[0xFF]).expect("Error writing to i2c");
    }
}