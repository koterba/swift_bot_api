use std::time::Duration;
use std::time::Instant;
use std::error::Error;
use std::thread;

// interaction with pi hardware
use rppal::gpio::{Gpio, InputPin, OutputPin, Level};

// for camera
use rascam::{SimpleCamera, CameraSettings, info};
use std::fs::File;
use std::io::Write;

use crate::sn3218::SN3218;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Button {
    A, B, X, Y
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Motor {
    Left, Right
}

pub struct SwiftBot {
    button_a: InputPin,
    button_b: InputPin,
    button_x: InputPin,
    button_y: InputPin,
    button_a_led: OutputPin,
    button_b_led: OutputPin,
    button_x_led: OutputPin,
    button_y_led: OutputPin,
    motor_en: OutputPin,
    motor_left_p: OutputPin,
    motor_left_n: OutputPin,
    motor_right_p: OutputPin,
    motor_right_n: OutputPin,
    ultra_trig: OutputPin,
    ultra_echo: InputPin,

    sn3218: SN3218,
    buffer: [u8; 18],

    camera: SimpleCamera,
    camera_configured: bool
}

#[allow(dead_code)]
impl SwiftBot {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let gpio = Gpio::new()?;
        let button_a = gpio.get(5)?.into_input();
        let button_b = gpio.get(6)?.into_input();
        let button_x = gpio.get(16)?.into_input();
        let button_y = gpio.get(24)?.into_input();

        let button_a_led = gpio.get(23)?.into_output();
        let button_b_led = gpio.get(22)?.into_output();
        let button_x_led = gpio.get(17)?.into_output();
        let button_y_led = gpio.get(27)?.into_output();

        let motor_en = gpio.get(26)?.into_output();
        let motor_left_p = gpio.get(8)?.into_output();
        let motor_left_n = gpio.get(11)?.into_output();
        let motor_right_p = gpio.get(10)?.into_output();
        let motor_right_n = gpio.get(9)?.into_output();

        let ultra_trig = gpio.get(13)?.into_output();
        let ultra_echo = gpio.get(25)?.into_input();

        // underlights
        let buffer: [u8; 18] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        let mut sn3218 = SN3218::new()?;
        sn3218.reset();
        sn3218.output(&buffer);
        sn3218.enable_leds(0b111111111111111111);
        sn3218.disable();

        let camera = SimpleCamera::new(info().unwrap().cameras[0].clone()).unwrap();
        let camera_configured = false;

        Ok(Self {
            button_a,
            button_b,
            button_x,
            button_y,
            button_a_led,
            button_b_led,
            button_x_led,
            button_y_led,
            motor_en,
            motor_left_p,
            motor_left_n,
            motor_right_p,
            motor_right_n,
            ultra_trig,
            ultra_echo,
            sn3218,
            buffer,
            camera,
            camera_configured
        })
    }

    pub fn is_pressed(&self, button: Button) -> bool {
        let button = match button {
            Button::A => &self.button_a,
            Button::B => &self.button_b,
            Button::X => &self.button_x,
            Button::Y => &self.button_y
        };

        match button.read() {
            Level::High => return false,
            Level::Low  => return true
        }
    }

    // IT WORKS. in case you forgot, you got it working after setting
    // the motors to simply "set_high" and "set_low" instead of using pwm
    pub fn set_motor_speed(&mut self, motor: Motor, speed: f64) {
        self.motor_en.set_high();
        let (pwm_p, pwm_n) = match motor {
            Motor::Left => (&mut self.motor_left_n, &mut self.motor_left_p),
            Motor::Right => (&mut self.motor_right_p, &mut self.motor_right_n)
        };

        let frequency = 100.0;
        let error = "Unable to set PWM motor frequency";
        if speed > 0.0 {
            pwm_p.set_pwm_frequency(frequency, 1.0).expect(error);
            pwm_n.set_pwm_frequency(frequency, 1.0 - speed).expect(error);
        } else if speed < 0.0 {
            pwm_p.set_pwm_frequency(frequency, 1.0 + speed).expect(error);
            pwm_n.set_pwm_frequency(frequency, 1.0).expect(error);
        } else {
            pwm_p.set_pwm_frequency(frequency, 1.0).expect(error);
            pwm_n.set_pwm_frequency(frequency, 1.0).expect(error);
        }
    }

    pub fn forward(&mut self) {
        self.set_motor_speed(Motor::Left, 1.0);
        self.set_motor_speed(Motor::Right, 1.0);
    }

    pub fn backward(&mut self) {
        self.set_motor_speed(Motor::Left, -1.0);
        self.set_motor_speed(Motor::Right, -1.0);
    }

    pub fn stop(&mut self) {
        self.set_motor_speed(Motor::Left, 0.0);
        self.set_motor_speed(Motor::Right, 0.0);
    }

    pub fn distance(&mut self) -> f32 {
        self.ultra_trig.set_high();
        thread::sleep(Duration::from_micros(10));
        self.ultra_trig.set_low();

        // Measure the response time
        let mut pulse_start = Instant::now();
        let mut pulse_end = Instant::now();
        while self.ultra_echo.read() == Level::Low {
            pulse_start = Instant::now();
        }
        while self.ultra_echo.read() == Level::High {
            pulse_end = Instant::now();
        }

        // Calculate the distance based on the time difference
        let pulse_duration = pulse_end.duration_since(pulse_start);
        let distance = (pulse_duration.as_micros() as f32 * 0.034) / 2.0;
        distance
    }

    pub fn show_underlight(&mut self) {
        self.sn3218.output(&self.buffer);
        self.sn3218.enable();
    }

    pub fn set_underlight(&mut self, r: u8, g: u8, b: u8) {
        for light in 0..6 {
            self.buffer[light * 3] = r;
            self.buffer[(light * 3) + 1] = g;
            self.buffer[(light * 3) + 2] = b;
        }
        self.show_underlight();
    }

    pub fn clear_underlight(&mut self) {
        self.buffer = [0; 18];
        self.sn3218.disable();
    }

    pub fn set_button_light(&mut self, button: Button, value: f64) {
        let led = match button {
            Button::A => &mut self.button_a_led,
            Button::B => &mut self.button_b_led,
            Button::X => &mut self.button_x_led,
            Button::Y => &mut self.button_y_led
        };

        let frequency = 2000.0;
        led.set_pwm_frequency(frequency, value).expect("Unable to set PWM light frequency");
    }

    pub fn clear_button_lights(&mut self) {
        let buttons = [Button::A, Button::B, Button::X, Button::Y];
        for button in buttons {
            self.set_button_light(button, 0.0);
        }
    }

    pub fn save_photo(&mut self, filename: &str, width: u32, height: u32, exposure_time: u64) {
        let b = self.photo_buffer(width, height, exposure_time);
        File::create(filename).unwrap().write_all(&b).unwrap();
    }

    pub fn photo_buffer(&mut self, width: u32, height: u32, exposure_time: u64) -> Vec<u8> {
        if !self.camera_configured {
            self.camera.configure(CameraSettings {
                height, width,
                ..CameraSettings::default()
            });
            match self.camera.activate() {
                Err(_) => {return Vec::new()},
                Ok(()) => {}
            };
            self.camera_configured = true;
        }
        thread::sleep(Duration::from_millis(exposure_time));
        self.camera.take_one().unwrap()
    }
}