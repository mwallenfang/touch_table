#![no_std]
#![no_main]

use bsp::{entry, hal::timer::Alarm, Pins};
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin, PinState};
use embedded_time::duration::Microseconds;
use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    timer,
    watchdog::Watchdog,
};

// A look-up, to know which segments to activate for which number
const number_lookup: [[PinState; 7]; 10] = [
    [
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::Low,
    ], //0
    [
        PinState::Low,
        PinState::High,
        PinState::High,
        PinState::Low,
        PinState::Low,
        PinState::Low,
        PinState::Low,
    ], //1
    [
        PinState::High,
        PinState::High,
        PinState::Low,
        PinState::High,
        PinState::High,
        PinState::Low,
        PinState::High,
    ], //2
    [
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::Low,
        PinState::Low,
        PinState::High,
    ], //3
    [
        PinState::Low,
        PinState::High,
        PinState::High,
        PinState::Low,
        PinState::Low,
        PinState::High,
        PinState::High,
    ], //4
    [
        PinState::High,
        PinState::Low,
        PinState::High,
        PinState::High,
        PinState::Low,
        PinState::High,
        PinState::High,
    ], //5
    [
        PinState::High,
        PinState::Low,
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::High,
    ], //6
    [
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::Low,
        PinState::Low,
        PinState::Low,
        PinState::Low,
    ], //7
    [
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::High,
    ], //8
    [
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::High,
        PinState::Low,
        PinState::High,
        PinState::High,
    ], //9
];

fn draw_number(number: u8, pins: &Pins) {
    let tens_digit = number - (number % 10);
    // tens_a
    //     .set_state(number_lookup[tens_digit as usize][0])
    //     .unwrap();
}

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure GPIO25 as an output
    let mut led_pin = pins.led.into_push_pull_output();

    let mut tens_a = pins.gpio13.into_push_pull_output();
    let mut tens_b = pins.gpio12.into_push_pull_output();
    let mut tens_c = pins.gpio2.into_push_pull_output();
    let mut tens_d = pins.gpio1.into_push_pull_output();
    let mut tens_e = pins.gpio0.into_push_pull_output();
    let mut tens_f = pins.gpio14.into_push_pull_output();
    let mut tens_g = pins.gpio15.into_push_pull_output();

    let mut ones_a = pins.gpio9.into_push_pull_output();
    let mut ones_b = pins.gpio8.into_push_pull_output();
    let mut ones_c = pins.gpio6.into_push_pull_output();
    let mut ones_d = pins.gpio5.into_push_pull_output();
    let mut ones_e = pins.gpio4.into_push_pull_output();
    let mut ones_f = pins.gpio10.into_push_pull_output();
    let mut ones_g = pins.gpio11.into_push_pull_output();

    led_pin.set_high().unwrap();

    let mut timer = timer::Timer::new(pac.TIMER, &mut pac.RESETS);

    let mut alarm_0 = timer.alarm_0().unwrap();

    alarm_0.schedule(Microseconds(5000000_u32));
    let mut number = 99;

    loop {
        let tens_digit = (number - (number % 10)) / 10;
        let ones_digit = number % 10;
        tens_a
            .set_state(number_lookup[tens_digit as usize][0])
            .unwrap();
        tens_b
            .set_state(number_lookup[tens_digit as usize][1])
            .unwrap();
        tens_c
            .set_state(number_lookup[tens_digit as usize][2])
            .unwrap();
        tens_d
            .set_state(number_lookup[tens_digit as usize][3])
            .unwrap();
        tens_e
            .set_state(number_lookup[tens_digit as usize][4])
            .unwrap();
        tens_f
            .set_state(number_lookup[tens_digit as usize][5])
            .unwrap();
        tens_g
            .set_state(number_lookup[tens_digit as usize][6])
            .unwrap();

        ones_a
            .set_state(number_lookup[ones_digit as usize][0])
            .unwrap();
        ones_b
            .set_state(number_lookup[ones_digit as usize][1])
            .unwrap();
        ones_c
            .set_state(number_lookup[ones_digit as usize][2])
            .unwrap();
        ones_d
            .set_state(number_lookup[ones_digit as usize][3])
            .unwrap();
        ones_e
            .set_state(number_lookup[ones_digit as usize][4])
            .unwrap();
        ones_f
            .set_state(number_lookup[ones_digit as usize][5])
            .unwrap();
        ones_g
            .set_state(number_lookup[ones_digit as usize][6])
            .unwrap();

        delay.delay_ms(100);
        number -= 1;

        if number < -1 {
            number = 99;
        }
        // if button_pin.is_low().unwrap() {
        //     led_pin.set_high();
        //     alarm_0.schedule(Microseconds(5000000_u32));
        // }
    }
}

// End of file
