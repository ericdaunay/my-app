#![no_main]
#![no_std]

use my_app as _; // global logger + panicking-behavior + memory layout

use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use nrf52840_hal::{
        self as hal,
        gpio::{p0::Parts as P0Parts, Level},
        Timer,
    };

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");
    let board = hal::pac::Peripherals::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let pins = P0Parts::new(board.P0);
    let mut _led_1 = pins.p0_13.into_push_pull_output(Level::Low);
    let mut led_2 = pins.p0_03.into_push_pull_output(Level::High);
    let mut led_3 = pins.p0_04.into_push_pull_output(Level::High);
    let mut led_4 = pins.p0_28.into_push_pull_output(Level::High);

    timer.delay_ms(1000u32);

    loop {
        led_2.set_high().unwrap();
        led_3.set_high().unwrap();
        led_4.set_high().unwrap();
        timer.delay_ms(1000_u32);       
        led_2.set_low().unwrap();
        led_3.set_low().unwrap();
        led_4.set_low().unwrap();
        timer.delay_ms(1000_u32);
    }

    }