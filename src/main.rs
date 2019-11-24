#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate embedded_hal;
extern crate feather_m0 as hal;
extern crate panic_halt;

use hal::clock::GenericClockController;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use hal::entry;

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
    let mut pins = hal::Pins::new(peripherals.PORT);
    let d13 = pins.d13.into_open_drain_output(&mut pins.port);

    let raw_spi3_master = spi::spi_master3(
        &mut clocks,
        3u32.mhz(),
        peripherals.SERCOM3,
        &mut peripherals.PM,
        pins.d12,
        pins.d10,
        pins.d11,
        &mut pins.port,
    );

    let data = &[5];

    let mut spi3_master = spi::SPI::new(raw_spi3_master, d13);

    spi3_master.send(data).unwrap();

    loop {}
}
