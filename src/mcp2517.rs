pub mod generic {
    use bitfield::*;
    bitfield! {
        pub struct Instruction(u16);
        impl Debug;
        u16;
        pub op_code, set_op_code: 15, 12;
        pub address, set_address: 10, 0;
    }

    pub struct OpCode;
    impl OpCode {
        pub const RESET: u16 = 0b0000 << 12;
        pub const READ_SFR: u16 = 0b0011 << 12;
        pub const WRITE_SFR: u16 = 0b0010 << 12;
    }

    pub enum SFRAddress {
        OSC = 0xE00,
        IOCON = 0xE04,
        CRC = 0xE08,
        ECCCON = 0xE0C,
        ECCSTAT = 0xE10,

        C1CON = 0x0,
        C1NBTCFG = 0x4,
        C1DBTCFG = 0x8,
        C1TDC = 0xC,
        C1TBC = 0x10,
        C1TSCON = 0x14,
        C1VEC = 0x18,
        C1INT = 0x1C,
        C1RXIF = 0x20,
        C1TXIF = 0x24,
        C1RXOVIF = 0x28,
        C1TXATIF = 0x2C,
        C1TXREQ = 0x30,
        C1TREC = 0x34,
        C1BDIAG0 = 0x38,
        C1BDIAG1 = 0x3C,
        C1TEFCON = 0x40,
        C1TEFSTA = 0x44,
        C1TEFUA = 0x48,
        // 0x4C is reserved
        C1TXQCON = 0x50,
        C1TXQSTA = 0x54,
        C1TXQUA = 0x58,

        C1FIFOCON1 = 0x5C,
        C1FIFOSTA1 = 0x60,
        C1FIFOUA1 = 0x64,

        C1FIFOCON2 = 0x68,
        C1FIFOSTA2 = 0x6C,
        C1FIFOUA2 = 0x70,

        C1FIFOCON3 = 0x74,
        C1FIFOSTA3 = 0x78,
        C1FIFOUA3 = 0x7C,

        C1FIFOCON4 = 0x80,
        C1FIFOSTA4 = 0x84,
        C1FIFOUA4 = 0x88,

        C1FIFOCON5 = 0x8C,
        C1FIFOSTA5 = 0x90,
        C1FIFOUA5 = 0x94,

        C1FIFOCON6 = 0x98,
        C1FIFOSTA6 = 0x9C,
        C1FIFOUA6 = 0xA0,

        C1FIFOCON7 = 0xA4,
        C1FIFOSTA7 = 0xA8,
        C1FIFOUA7 = 0xAC,

        C1FIFOCON8 = 0xB0,
        C1FIFOSTA8 = 0xB4,
        C1FIFOUA8 = 0xB8,

        C1FIFOCON9 = 0xBC,
        C1FIFOSTA9 = 0xC0,
        C1FIFOUA9 = 0xC4,

        C1FIFOCON10 = 0xC8,
        C1FIFOSTA10 = 0xCC,
        C1FIFOUA10 = 0xD0,

        C1FIFOCON11 = 0xD4,
        C1FIFOSTA11 = 0xD8,
        C1FIFOUA11 = 0xDC,

        C1FIFOCON12 = 0xE0,
        C1FIFOSTA12 = 0xE4,
        C1FIFOUA12 = 0xE8,

        C1FIFOCON13 = 0xEC,
        C1FIFOSTA13 = 0xF0,
        C1FIFOUA13 = 0xF4,

        C1FIFOCON14 = 0xF8,
        C1FIFOSTA14 = 0xFC,
        C1FIFOUA14 = 0x100,

        C1FIFOCON15 = 0x104,
        C1FIFOSTA15 = 0x108,
        C1FIFOUA15 = 0x10C,

        C1FIFOCON16 = 0x110,
        C1FIFOSTA16 = 0x114,
        C1FIFOUA16 = 0x118,

        C1FIFOCON17 = 0x11C,
        C1FIFOSTA17 = 0x120,
        C1FIFOUA17 = 0x124,

        C1FIFOCON18 = 0x128,
        C1FIFOSTA18 = 0x12C,
        C1FIFOUA18 = 0x130,

        C1FIFOCON19 = 0x134,
        C1FIFOSTA19 = 0x138,
        C1FIFOUA19 = 0x13C,

        C1FIFOCON20 = 0x140,
        C1FIFOSTA20 = 0x144,
        C1FIFOUA20 = 0x148,

        C1FIFOCON21 = 0x14C,
        C1FIFOSTA21 = 0x150,
        C1FIFOUA21 = 0x154,

        C1FIFOCON22 = 0x158,
        C1FIFOSTA22 = 0x15C,
        C1FIFOUA22 = 0x160,

        C1FIFOCON23 = 0x164,
        C1FIFOSTA23 = 0x168,
        C1FIFOUA23 = 0x16C,

        C1FIFOCON24 = 0x170,
        C1FIFOSTA24 = 0x174,
        C1FIFOUA24 = 0x178,

        C1FIFOCON25 = 0x17C,
        C1FIFOSTA25 = 0x180,
        C1FIFOUA25 = 0x184,

        C1FIFOCON26 = 0x188,
        C1FIFOSTA26 = 0x18C,
        C1FIFOUA26 = 0x190,

        C1FIFOCON27 = 0x194,
        C1FIFOSTA27 = 0x198,
        C1FIFOUA27 = 0x19C,

        C1FIFOCON28 = 0x1A0,
        C1FIFOSTA28 = 0x1A4,
        C1FIFOUA28 = 0x1A8,

        C1FIFOCON29 = 0x1AC,
        C1FIFOSTA29 = 0x1B0,
        C1FIFOUA29 = 0x1B4,

        C1FIFOCON30 = 0x1B8,
        C1FIFOSTA30 = 0x1BC,
        C1FIFOUA30 = 0x1C0,

        C1FIFOCON31 = 0x1C4,
        C1FIFOSTA31 = 0x1C8,
        C1FIFOUA31 = 0x1CC,

        C1FLTCON0 = 0x1D0,
        C1FLTCON1 = 0x1D4,
        C1FLTCON2 = 0x1D8,
        C1FLTCON3 = 0x1DC,
        C1FLTCON4 = 0x1E0,
        C1FLTCON5 = 0x1E4,
        C1FLTCON6 = 0x1E8,
        C1FLTCON7 = 0x1EC,

        C1FLTOBJ0 = 0x1F0,
        C1MASK0 = 0x1F4,

        C1FLTOBJ1 = 0x1F8,
        C1MASK1 = 0x1FC,
        C1FLTOBJ2 = 0x200,
        C1MASK2 = 0x204,

        C1FLTOBJ3 = 0x208,
        C1MASK3 = 0x20C,

        C1FLTOBJ4 = 0x210,
        C1MASK4 = 0x214,

        C1FLTOBJ5 = 0x218,
        C1MASK5 = 0x21C,

        C1FLTOBJ6 = 0x220,
        C1MASK6 = 0x224,

        C1FLTOBJ7 = 0x228,
        C1MASK7 = 0x22C,

        C1FLTOBJ8 = 0x230,
        C1MASK8 = 0x234,

        C1FLTOBJ9 = 0x238,
        C1MASK9 = 0x23C,

        C1FLTOBJ10 = 0x240,
        C1MASK10 = 0x244,

        C1FLTOBJ11 = 0x248,
        C1MASK11 = 0x24C,

        C1FLTOBJ12 = 0x250,
        C1MASK12 = 0x254,

        C1FLTOBJ13 = 0x258,
        C1MASK13 = 0x25C,

        C1FLTOBJ14 = 0x260,
        C1MASK14 = 0x264,

        C1FLTOBJ15 = 0x268,
        C1MASK15 = 0x26C,

        C1FLTOBJ16 = 0x270,
        C1MASK16 = 0x274,

        C1FLTOBJ17 = 0x278,
        C1MASK17 = 0x27C,

        C1FLTOBJ18 = 0x280,
        C1MASK18 = 0x284,

        C1FLTOBJ19 = 0x288,
        C1MASK19 = 0x28C,

        C1FLTOBJ20 = 0x290,
        C1MASK20 = 0x294,

        C1FLTOBJ21 = 0x298,
        C1MASK21 = 0x29C,

        C1FLTOBJ22 = 0x2A0,
        C1MASK22 = 0x2A4,

        C1FLTOBJ23 = 0x2A8,
        C1MASK23 = 0x2AC,

        C1FLTOBJ24 = 0x2B0,
        C1MASK24 = 0x2B4,

        C1FLTOBJ25 = 0x2B8,
        C1MASK25 = 0x2BC,

        C1FLTOBJ26 = 0x2C0,
        C1MASK26 = 0x2C4,

        C1FLTOBJ27 = 0x2C8,
        C1MASK27 = 0x2CC,

        C1FLTOBJ28 = 0x2D0,
        C1MASK28 = 0x2D4,

        C1FLTOBJ29 = 0x2D8,
        C1MASK29 = 0x2DC,

        C1FLTOBJ30 = 0x2E0,
        C1MASK30 = 0x2E4,

        C1FLTOBJ31 = 0x2E8,
        C1MASK31 = 0x2EC,
    }

    const MESSAGE_IDENTIFIER_MASK: u16 = 0b0000_0111_1111_1111;
    pub enum MessageIdentifier {
        Solenoid = 0x0,
    }

    bitfield! {
        pub struct TransmitMessageHeader([u8]);
        impl Debug;
        u8;
        // T0
        u16, standard_identifier, set_standard_identifier: 10, 0;
        u32, extended_identifier, set_extended_identifier: 28, 10;
        sid11, _: 29, 29;
        // T1
        pub data_length_code, set_data_length_code: 35, 32;
        pub identifier_extension, _: 36;
        pub remote_transmission_request, _: 37;
        pub bit_rate_switched, _: 38;
        pub fd_frame, _: 39;
        pub error_status_indicator, _: 40;
        pub sequence, set_sequence: 47, 41;
    }

    impl TransmitMessageHeader<[u8; 2]> {
        pub fn new(identifier: MessageIdentifier) -> Self {
            let mut msg = TransmitMessageHeader([0; 2]);
            msg.set_standard_identifier((identifier as u16) & MESSAGE_IDENTIFIER_MASK);
            msg
        }
    }

    bitfield! {
        pub struct ReceiveMessageHeader([u8]);
        impl Debug;
        u8;
        // T0
        pub u16, standard_identifier, set_standard_identifier: 10, 0;
        pub u32, extended_identifier, set_extended_identifier: 28, 10;
        sid11, _: 29, 29;
        // T1
        pub data_length_code, set_data_length_code: 35, 32;
        pub identifier_extension, _: 36;
        pub remote_transmission_request, _: 37;
        pub bit_rate_switched, _: 38;
        pub fd_frame, _: 39;
        pub error_status_indicator, _: 40;
        pub filter_hit, _: 47, 43;
        // T2
        pub u32, timestamp, _: 95, 64;
    }

    impl ReceiveMessageHeader<[u8; 3]> {
        pub fn new(identifier: MessageIdentifier) -> Self {
            let mut msg = ReceiveMessageHeader([0; 3]);
            msg.set_standard_identifier((identifier as u16) & MESSAGE_IDENTIFIER_MASK);
            msg
        }
    }
}

pub mod spi {
    use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin};
    use embedded_hal::spi::FullDuplex;

    use super::generic::*;

    pub struct Controller<T, SS> {
        spi_master: T,
        slave_select: SS,
    }

    impl<T, SS> Controller<T, SS>
    where
        T: FullDuplex<u8>,
        SS: StatefulOutputPin,
        <SS as OutputPin>::Error: core::fmt::Debug,
    {
        pub fn new(spi_master: T, mut slave_select: SS) -> Controller<T, SS> {
            slave_select.set_low().unwrap();
            Self {
                spi_master,
                slave_select,
            }
        }

        fn ready_slave_select(&mut self) -> () {
            if self.slave_select.is_set_low().unwrap() {
                self.slave_select.set_high().unwrap();
            }
            self.slave_select.set_low().unwrap();
        }

        pub fn reset(&mut self) -> Result<(), T::Error> {
            self.ready_slave_select();

            let instruction = Instruction(OpCode::RESET);
            match self.send(&instruction.0.to_be_bytes()) {
                Ok(_) => Ok(()),
                Err(err) => {
                    self.slave_select.set_low().unwrap();
                    Err(err)
                }
            }
        }

        pub fn read_sfr(&mut self, address: SFRAddress) -> Result<u32, T::Error> {
            self.ready_slave_select();
            let mut instruction = Instruction(OpCode::READ_SFR);
            instruction.set_address(address as u16);
            match self.send(&instruction.0.to_be_bytes()) {
                Ok(_) => (),
                Err(err) => {
                    self.slave_select.set_low().unwrap();
                    return Err(err);
                }
            }

            let mut read_value: u32 = 0;

            // Read four bytes back
            for i in 0..4 {
                match self.read() {
                    Ok(val) => read_value |= (val as u32) << 8 * i,
                    Err(val) => {
                        self.slave_select.set_low().unwrap();
                        return Err(val);
                    }
                }
            }
            self.slave_select.set_high().unwrap();
            Ok(read_value)
        }

        pub fn write_sfr(&mut self, address: SFRAddress, value: u32) -> Result<u32, T::Error> {
            self.ready_slave_select();
            let mut instruction = Instruction(OpCode::WRITE_SFR);
            instruction.set_address(address as u16);
            let ret = match self.send(&instruction.0.to_be_bytes()) {
                Ok(_) => Ok(value),
                Err(err) => Err(err),
            };
            self.slave_select.set_high().unwrap();
            ret
        }

        fn read(&mut self) -> Result<u8, T::Error> {
            block!(self.spi_master.read())
        }

        fn send(&mut self, data: &[u8]) -> Result<(), <T as FullDuplex<u8>>::Error> {
            data.iter()
                .try_for_each(|v| block!(self.spi_master.send(*v)))
        }

        pub fn free(mut self) -> (T, SS) {
            self.slave_select.set_high().unwrap();
            (self.spi_master, self.slave_select)
        }
    }
}
