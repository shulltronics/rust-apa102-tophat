#![no_std]
#![no_main]

mod dotstar;
/**** low-level imports *****/
use panic_halt as _;
use cortex_m::prelude::*;
use cortex_m_rt::entry;
use embedded_hal::{
        digital::v2::{OutputPin, InputPin},
        spi,
    };
use embedded_time::rate::*;

/***** board-specific imports *****/
use adafruit_feather_rp2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        watchdog::Watchdog,
        Sio,
        gpio::{FunctionUart, FunctionSpi},
        uart,
        Spi,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

/***** imports for external devices *****/
use apa102_spi::Apa102;
use smart_leds::{RGB8, SmartLedsWrite};

use dotstar::{DotStar_Pulse, DotStar_Static, DotStar_Wheel, DotStar_Beacon};

const WHITE: (u8, u8, u8) = (0xFF, 0xFF, 0xFF);
const RED: (u8, u8, u8)   = (0xFF, 0x00, 0x00);
const GRN: (u8, u8, u8)   = (0x00, 0xFF, 0x00);
const BLUE: (u8, u8, u8)  = (0x00, 0x00, 0xFF);

#[entry]
fn main() -> ! {
    // Grab the singleton object
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    // Init the watchdog timer, to pass into the clock init
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();
    
    // initialize the Single Cycle IO
    let sio = Sio::new(pac.SIO);
    // initialize the pins to default state
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // These are implicitly used by the spi driver if they are in the correct mode
    let _spi_sclk = pins.sclk.into_mode::<FunctionSpi>();
    let _spi_mosi = pins.mosi.into_mode::<FunctionSpi>();
    let _spi_miso = pins.miso.into_mode::<FunctionSpi>();
    let spi = Spi::<_, _, 8>::new(pac.SPI0);

    // Exchange the uninitialised SPI driver for an initialised one
    let mut spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        8_000_000u32.Hz(),
        &embedded_hal::spi::MODE_2,
    );

    // QUESTION: Why does this only take two arguments?
    // QUESTION: Why aren't the pins already declared in the board crate?
    let _uart_tx = pins.tx.into_mode::<FunctionUart>();
    let _uart_rx = pins.rx.into_mode::<FunctionUart>();
    let mut uart = uart::UartPeripheral::new(pac.UART0, &mut pac.RESETS)
        .enable(
            uart::common_configs::_9600_8_N_1,
            clocks.peripheral_clock.into(),
        ).unwrap();

    let mut timer = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let mut button_pin = pins.d4.into_pull_up_input();
    let mut last_button_state: bool = button_pin.is_low().unwrap();

    let mut dotstar = Apa102::new(spi);

    // Define the modes here
    const NUM_MODES: u8 = 6;
    let mut mode: u8 = 1;
    let mut ds_static_black = DotStar_Static::new(RGB8::new(0,0,0));
    let mut ds_static_white = DotStar_Static::new(RGB8::new(255, 255, 255));
    let mut ds_static_red   = DotStar_Static::new(RED.into());
    let mut ds_pulse = DotStar_Pulse::new(RGB8::new(50, 0, 50));
    let mut ds_wheel = DotStar_Wheel::new();
    let mut ds_beacon = DotStar_Beacon::new(BLUE.into());

/*
Loop Section
    Todo: make the "modes" impliment the iterator trait
    and call <mode>.iter_mut().next() each loop, then display
    the [RGB8; NUM_PX] array to the DotStar
*/
    let mut delay: u8 = 12;   // loop delay in ms
    loop {
        // Button logic
        let button_state = button_pin.is_low().unwrap();
        if button_state && (button_state != last_button_state) {
            if mode == (NUM_MODES-1) {
                mode = 0;
            } else {
                mode = mode + 1;
            }
        }
        last_button_state = button_state;

        // iterate through the applicable modes (static doesn't impl Iterator)
        ds_pulse.next();
        ds_wheel.next();
        ds_beacon.next();
        
        // select the list based on current mode
        let ds: [RGB8; dotstar::NUM_PX] = match mode {
            0 => ds_static_black.to_list(),   // off
            1 => ds_static_white.to_list(),                   // static white
            2 => ds_static_red.to_list(),                   // static white
            3 => ds_pulse.to_list(),                    // pulsing magenta (FIX!)
            4 => ds_wheel.to_list(),                    // color wheel
            5 => ds_beacon.to_list(),                    // color wheel
            _ => [RGB8::new(0,0,0); dotstar::NUM_PX],
        };

        // write to the leds
        dotstar.write(ds.iter().cloned()).unwrap();

        timer.delay_ms(delay as u32);
    }
}

