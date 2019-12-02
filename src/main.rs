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

    //Create a new mcp2517 object and Set the slave select as d6
    let mut _spi_master = mcp2517::spi::Controller::new(raw_spi_master,d6);

    //data[i] == GPIO are outputs with logic high, refer to IOCON
    //data[0] == GPIO0 low && GPIO1 low
    //data[1] == GPIO0 low && GPIO1 high
    //data[2] == GPIO0 high && GPIO1 low
    //data[3] == GPIO0 high && GPIO1 high
    let data = [(0b00000011000000110000000000000000 as u32),
                (0b00000011000000110000000100000000 as u32),
                (0b00000011000000110000001000000000 as u32),
                (0b00000011000000110000001100000000 as u32)];
   
    let mut _incrementer = 0;

    //Start loop, change LED configuration every second
    loop {
        //Delay for a second
        delay.delay_ms(1000u32);
        //Send a new data packet
        _spi_master.write_sfr(mcp2517::generic::SFRAddress::IOCON,data[_incrementer]).unwrap();
        //cycle through the data packets
        //Make the common case fast by allowing 
        //  most loops to ignore the else
        if _incrementer < 3 {
            _incrementer += 1;
        } else {
            _incrementer = 0;
        } //end if
    } //end loop
} //end main()
