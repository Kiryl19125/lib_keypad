#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use lib_keypad::Key;
use lib_keypad::KeyPad;
use lib_keypad::ShiftRegister;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use stm32f1xx_hal::{
    gpio::*,
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
};

use alloc_cortex_m::CortexMHeap;
// use cortex_m::asm;

#[macro_use]
extern crate alloc;

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    rtt_init_print!();
    let dp = pac::Peripherals::take().unwrap();
    let pc = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = pc.SYST.delay(&clocks);

    let mut afio = dp.AFIO.constrain();
    let mut gpio_a = dp.GPIOA.split();
    let mut gpio_b = dp.GPIOB.split();

    // init display

    let scl = gpio_b.pb8.into_alternate_open_drain(&mut gpio_b.crh);
    let sda = gpio_b.pb9.into_alternate_open_drain(&mut gpio_b.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.Hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate180)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    let mut key_pad = KeyPad::new(
        [
            gpio_a.pa0.into_push_pull_output(&mut gpio_a.crl).erase(),
            gpio_a.pa1.into_push_pull_output(&mut gpio_a.crl).erase(),
            gpio_a.pa2.into_push_pull_output(&mut gpio_a.crl).erase(),
            gpio_a.pa3.into_push_pull_output(&mut gpio_a.crl).erase(),
        ],
        [
            gpio_a.pa4.into_pull_down_input(&mut gpio_a.crl).erase(),
            gpio_a.pa5.into_pull_down_input(&mut gpio_a.crl).erase(),
            gpio_a.pa6.into_pull_down_input(&mut gpio_a.crl).erase(),
            gpio_a.pa7.into_pull_down_input(&mut gpio_a.crl).erase(),
        ],
    );

    let lock_pin = gpio_b.pb12.into_push_pull_output(&mut gpio_b.crh).erase();
    let clock_pin = gpio_b.pb13.into_push_pull_output(&mut gpio_b.crh).erase();
    let data_pin = gpio_b.pb14.into_push_pull_output(&mut gpio_b.crh).erase();

    let mut shift_register = ShiftRegister::new(lock_pin, clock_pin, data_pin);

    shift_register.write_array(&[0, 0, 0, 0, 0, 0, 0, 0]);

    let mut flag = false;

    // say hello!

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    delay.delay_ms(3000_u32);

    display.clear(BinaryColor::Off).unwrap();
    display.flush().unwrap();

    loop {
        let key = key_pad.key_pooling();
        match key {
            Some(key) => {
                if !flag {
                    rprintln!("Key: {:?}", key);
                    flag = true;

                    match key {
                        Key::Zero => {
                            shift_register.write_array(&[1, 0, 0, 0, 0, 0, 0, 0]);

                            display.clear(BinaryColor::Off).unwrap();
                            display.flush().unwrap();
                            Text::with_baseline("0", Point::zero(), text_style, Baseline::Top)
                                .draw(&mut display)
                                .unwrap();
                            display.flush().unwrap();
                        }
                        Key::One => {
                            shift_register.write_array(&[0, 1, 0, 0, 0, 0, 0, 0]);

                            display.clear(BinaryColor::Off).unwrap();
                            display.flush().unwrap();
                            Text::with_baseline("1", Point::zero(), text_style, Baseline::Top)
                                .draw(&mut display)
                                .unwrap();
                            display.flush().unwrap();
                        },
                        Key::Two => {
                            shift_register.write_array(&[0, 0, 1, 0, 0, 0, 0, 0]);
                            display.clear(BinaryColor::Off).unwrap();
                            Text::with_baseline("2", Point::zero(), text_style, Baseline::Top)
                                .draw(&mut display)
                                .unwrap();
                            display.flush().unwrap();
                        },
                        Key::Three => {
                            shift_register.write_array(&[0, 0, 0, 1, 0, 0, 0, 0]);
                            display.clear(BinaryColor::Off).unwrap();
                            Text::with_baseline("3", Point::zero(), text_style, Baseline::Top)
                                .draw(&mut display)
                                .unwrap();
                            display.flush().unwrap();
                        },
                        Key::Four => {
                            shift_register.write_array(&[0, 0, 0, 0, 1, 0, 0, 0]);
                            display.clear(BinaryColor::Off).unwrap();
                            Text::with_baseline("4", Point::zero(), text_style, Baseline::Top)
                                .draw(&mut display)
                                .unwrap();
                            display.flush().unwrap();
                        },
                        Key::Five => {
                            shift_register.write_array(&[0, 0, 0, 0, 0, 1, 0, 0]);
                            display.clear(BinaryColor::Off).unwrap();
                            Text::with_baseline("5", Point::zero(), text_style, Baseline::Top)
                                .draw(&mut display)
                                .unwrap();
                            display.flush().unwrap();
                        },
                        Key::Six => {
                            shift_register.write_array(&[0, 0, 0, 0, 0, 0, 1, 0]);
                            display.clear(BinaryColor::Off).unwrap();
                            Text::with_baseline("6", Point::zero(), text_style, Baseline::Top)
                                .draw(&mut display)
                                .unwrap();
                            display.flush().unwrap();
                        },
                        Key::Seven => {
                            shift_register.write_array(&[0, 0, 0, 0, 0, 0, 0, 1]);
                            display.clear(BinaryColor::Off).unwrap();
                            Text::with_baseline("7", Point::zero(), text_style, Baseline::Top)
                                .draw(&mut display)
                                .unwrap();
                            display.flush().unwrap();
                        },
                        Key::A => {
                            shift_register.write_array(&[0, 0, 0, 0, 0, 0, 0, 0]);
                            display.clear(BinaryColor::Off).unwrap();
                            Text::with_baseline("Clear", Point::zero(), text_style, Baseline::Top)
                                .draw(&mut display)
                                .unwrap();
                            display.flush().unwrap();
                        },
                        _ => {}
                    }
                }
            }
            None => flag = false,
        }

        // delay.delay_ms(100_u32);
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    rprintln!("Panic!!!");
    panic!("{:#?}", ef);
}
