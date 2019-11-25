#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate embedded_hal;
extern crate feather_m0 as hal;
extern crate panic_halt;

#[macro_use(block)]
extern crate nb;

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
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    let mut pins = hal::Pins::new(peripherals.PORT);
    let d13 = pins.d13.into_push_pull_output(&mut pins.port);
    let mut d6 = pins.d6.into_push_pull_output(&mut pins.port);
    d6.set_low().unwrap();

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

    loop {
        delay.delay_ms(1000u32);
        block!(spi3_master.send(data)).unwrap();
        let result = block!(spi3_master.read()).unwrap();
        if result == data[0] {
            d6.set_low().unwrap();
        } else {
            d6.set_high().unwrap();
        }
    }
}
