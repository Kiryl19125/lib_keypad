#![no_std]

use stm32f1xx_hal::gpio::*;

#[derive(Debug)]
pub enum Key {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Night,
    Zero,
    Star,
    Hash,
    A,
    B,
    C,
    D,
}

pub struct KeyPad {
    column_pins: [ErasedPin<Output>; 4],
    row_pins: [ErasedPin<Input<PullDown>>; 4],
}

impl KeyPad {
    pub fn new(
        column_pins: [ErasedPin<Output>; 4],
        row_pins: [ErasedPin<Input<PullDown>>; 4],
    ) -> Self {
        Self {
            column_pins,
            row_pins,
        }
    }

    pub fn key_pooling(&mut self) -> Option<Key> {
        for i in 0..self.column_pins.len() {
            self.column_pins[i].set_high();
            for j in 0..self.row_pins.len() {
                if self.row_pins[j].is_high() {
                    if j == 0 {
                        if i == 0 {
                            return Some(Key::One);
                        } else if i == 1 {
                            return Some(Key::Two);
                        } else if i == 2 {
                            return Some(Key::Three);
                        } else if i == 3 {
                            return Some(Key::A);
                        }
                    }
                    if j == 1 {
                        if i == 0 {
                            return Some(Key::Four);
                        } else if i == 1 {
                            return Some(Key::Five);
                        } else if i == 2 {
                            return Some(Key::Six);
                        } else if i == 3 {
                            return Some(Key::B);
                        }
                    }
                    if j == 2 {
                        if i == 0 {
                            return Some(Key::Seven);
                        } else if i == 1 {
                            return Some(Key::Eight);
                        } else if i == 2 {
                            return Some(Key::Night);
                        } else if i == 3 {
                            return Some(Key::C);
                        }
                    }
                    if j == 3 {
                        if i == 0 {
                            return Some(Key::Star);
                        } else if i == 1 {
                            return Some(Key::Zero);
                        } else if i == 2 {
                            return Some(Key::Hash);
                        } else if i == 3 {
                            return Some(Key::D);
                        }
                    }
                }
            }
            self.column_pins[i].set_low();
        }

        None
    }
}

pub struct ShiftRegister {
    lock_pin: ErasedPin<Output>,
    clock_pin: ErasedPin<Output>,
    data_pin: ErasedPin<Output>,
}

impl ShiftRegister {
    pub fn new(
        lock_pin: ErasedPin<Output>,
        clock_pin: ErasedPin<Output>,
        data_pin: ErasedPin<Output>,
    ) -> Self {
        Self {
            lock_pin,
            clock_pin,
            data_pin,
        }
    }

    pub fn write_array(&mut self, array: &[u8]) {
        self.lock_pin.set_low();
        for &num in array.iter().rev() {
            self.clock_pin.set_low();
            if num == 0 {
                self.data_pin.set_low();
            } else {
                self.data_pin.set_high();
            }
            self.clock_pin.set_high();
        }
        self.lock_pin.set_high();
    }
}
