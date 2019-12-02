#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate embedded_hal;
extern crate feather_m0 as hal;
extern crate panic_halt;

#[macro_use(block)]
extern crate nb;

#[macro_use]
extern crate bitfield;

use hal::clock::GenericClockController;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use hal::entry;

pub mod mcp2517;
mod spi;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut d13 = pins.d13.into_open_drain_output(&mut pins.port);
    let d6 = pins.d6.into_open_drain_output(&mut pins.port);

    /*let raw_spi3_master = spi::spi_master3(
        &mut clocks,
        3u32.mhz(),
        peripherals.SERCOM3,
        &mut peripherals.PM,
        pins.d12,
        pins.d10,
        pins.d11,
        &mut pins.port,
    );*/

    let raw_spi_master = hal::spi_master(
        &mut clocks,
        3u32.mhz(),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.sck,
        pins.mosi,
        pins.miso,
        &mut pins.port,
    );

    //Create two MCP2517 objects within the same SPI lane having SS d6 and d7
    //Configure both MCP2517 SPI objects
    //  IOCON should allow interrupts to be sent out of INT0 (RX) and INT1 (TX)
    //Define CAN messages within a vector for easy sending and comparing
    //Start loop to send and recieve messages
    //If unresolving loop is used, main function return type of Never (!)
    //If for loop is used, main function returns a random ass integer
    //  print out the value of the comparison, the sent message, and the recieved message as bit defined number
    //      println!(Are_Same + " | " + sent_message.binary() + " : " + recieved_message.binary());

    loop {
        delay.delay_ms(1000u32);
    }
}
