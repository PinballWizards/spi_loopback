use core::iter::{FromIterator, Iterator};

use nb;

use hal::clock::GenericClockController;
use hal::gpio::*;
use hal::pac::{PM, SERCOM3};
use hal::sercom::*;
use hal::time::*;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::FullDuplex;

pub struct SPI<T, SS> {
    spi_master: T,
    slave_select: SS,
}

impl<T, SS> SPI<T, SS>
where
    T: FullDuplex<u8>,
    SS: OutputPin,
    <SS as OutputPin>::Error: core::fmt::Debug,
{
    pub fn new(spi_master: T, mut slave_select: SS) -> SPI<T, SS> {
        slave_select
            .set_low()
            .expect("could not control slave select");
        Self {
            spi_master,
            slave_select,
        }
    }

    pub fn read(&mut self) -> Result<u8, T::Error> {
        block!(self.spi_master.read())
    }

    pub fn send(&mut self, data: &[u8]) -> Result<(), <T as FullDuplex<u8>>::Error> {
        data.iter()
            .try_for_each(|v| block!(self.spi_master.send(*v)))
    }

    pub fn free(mut self) -> (T, SS) {
        self.slave_select
            .set_high()
            .expect("could not control slave select");
        (self.spi_master, self.slave_select)
    }
}

pub fn spi_master3<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    bus_speed: F,
    sercom3: SERCOM3,
    pm: &mut PM,
    sck: Pa19<Input<Floating>>,
    mosi: Pa18<Input<Floating>>,
    miso: Pa16<Input<Floating>>,
    port: &mut Port,
) -> SPIMaster3<Sercom3Pad0<Pa16<PfD>>, Sercom3Pad2<Pa18<PfD>>, Sercom3Pad3<Pa19<PfD>>> {
    let gclk0 = clocks.gclk0();
    SPIMaster3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        bus_speed.into(),
        embedded_hal::spi::Mode {
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
            polarity: embedded_hal::spi::Polarity::IdleLow,
        },
        sercom3,
        pm,
        (miso.into_pad(port), mosi.into_pad(port), sck.into_pad(port)),
    )
}
