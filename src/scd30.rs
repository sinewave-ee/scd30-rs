use embedded_hal::blocking::i2c::{Read, Write};
use heapless::Vec;
use heapless::consts::*;

pub enum Command {
    StartContinuousMeasurement  = 0x0010,
    StopContinuousMeasurement   = 0x0104,
    SetMeasurementInterval      = 0x4600,
    GetDataReadyStatus          = 0x0202,
    ReadMeasurement             = 0x0300,
    SetAutomaticSelfCalibration = 0x5306,
    SetForcedRecalibrationValue = 0x5204,
    SetTemperatureOffset        = 0x5403,
    SetAltitude                 = 0x5102,
    ReadFirmwareVersion         = 0xd100,
    SoftReset                   = 0xd304,
}

const ADDRESS: u8 = 0x61 << 1;

pub struct Scd30<T> {
    i2c: T,
}

pub struct Measurement {
    pub co2:            f32,
    pub humidity:       f32,
    pub temperature:    f32,
}

impl<T, E> Scd30<T> where T: Read<Error = E> + Write<Error = E> {

    pub fn new(mut i2c: T) -> Self {
        Scd30 {
            i2c
        }
    }

    pub fn soft_reset(&mut self) -> Result<(), <T as Write>::Error> {
        self.i2c.write(ADDRESS, &(Command::SoftReset as u16).to_be_bytes())
    }

    pub fn start_measuring(&mut self) {
        self.start_measuring_with_mbar(0)
    }

    pub fn set_automatic_calibration(&mut self, enable: bool) {
        let mut vec: Vec<u8, U4> = Vec::new();
        vec.extend_from_slice(&(Command::SetAutomaticSelfCalibration as u16).to_be_bytes());
        vec.extend_from_slice(&[ 0x00, enable as u8 ]);
        self.i2c.write(ADDRESS, &vec);
    }

    pub fn start_measuring_with_mbar(&mut self, pressure: u16) {
        let mut vec: Vec<u8, U4> = Vec::new();
        vec.extend_from_slice(&(Command::StartContinuousMeasurement as u16).to_be_bytes());
        vec.extend_from_slice(&pressure.to_be_bytes());
        self.i2c.write(ADDRESS, &vec);
    }

    pub fn read(&mut self) -> Measurement {
        let mut buf = [0u8; 6 * 3];

        self.i2c.write(ADDRESS, &(Command::ReadMeasurement as u16).to_be_bytes());
        self.i2c.read(ADDRESS, &mut buf);

        Measurement {
            co2:         f32::from_bits(u32::from_be_bytes([ buf[0], buf[1], buf[3], buf[4] ])),
            temperature: f32::from_bits(u32::from_be_bytes([ buf[6], buf[7], buf[9], buf[10] ])),
            humidity:    f32::from_bits(u32::from_be_bytes([ buf[12], buf[13], buf[15], buf[16] ])),
        }
    }

}
