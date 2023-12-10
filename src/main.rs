use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::time::Duration;
use std::{io, process, thread};
mod error;
pub use error::*;

//pub struct SDS011 {
//    /// Link to a sensor, must be open via new()
//    port: Box<dyn SerialPort>,
//}

fn main() {
    /*
     * Open port with baudrate at 9600
     */
    let mut port = serialport::new("/dev/ttyUSB0", 9600)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .open()
        .unwrap_or_else(|error| {
            if error.kind() == serialport::ErrorKind::NoDevice {
                eprintln!("Error: Device Not found");
                process::exit(1);
            }
            if error.kind() == serialport::ErrorKind::Io(io::ErrorKind::NotFound) {
                eprintln!("Error: Device Not Connected");
                process::exit(1);
            } else {
                eprintln!("Error: {}", error);
                process::exit(1);
            }
        });

    //let mut sensor = SDS011 {
    //    port: port.try_clone().expect(""),
    //};
    //
    /*
     * Set delay to read data from sensor at regular intervals
     */
    let delay = Duration::from_secs(10);

    println!("Receiving data on {} at {} baud:", "/dev/ttyUSBO", 9600);
    loop {
        let mut buffer = [0u8; 10];
        /*
         * Read data from port into buffer of
         * u8 bytes of size 10 bytes as sensor
         * send exact 10 bytes only.
         */
        match port.read_exact(buffer.as_mut()) {
            Ok(_) => {
                /*
                 * extract second lowbyte and third hightbyte
                 * as it's pm2.5 data
                 */
                let pm25_ar = [buffer[2], buffer[3]];
                /*
                 * extract fourth lowbyte and fifith hightbyte
                 * as it's pm10 data
                 */
                let pm10_ar = [buffer[4], buffer[5]];
                let pm10: u16 = u16::from_le_bytes(pm10_ar);
                let pm25: u16 = u16::from_le_bytes(pm25_ar);
                let pmtwofive = pm25 as f32 / 10.0;
                let pmten = pm10 as f32 / 10.0;
                println!("PM10={} PM25={}", pmten, pmtwofive);
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
        /*
         * clear input/read buffer data
         * otherwise it overflows if delay is required
         */

        port.clear(serialport::ClearBuffer::Input)
            .expect("Clear buffer err");
        thread::sleep(delay);
    }
}

//impl SDS011 {
//    fn get_reply(&mut self) -> Result<[u8; 10]> {
//        let mut buf = [0u8; 10];
//        self.port.read_exact(buf.as_mut()).expect("");
//
//        let data = &buf[2..8];
//        if data.len() == 0 {
//            return Err(Error::EmptyDataFrame);
//        }
//
//        let mut checksum: u32 = 0;
//        for i in data.iter() {
//            checksum += *i as u32;
//        }
//        checksum = checksum & 255;
//        println!("checksum as u8 {:?}", checksum as u8);
//        println!("buf[8]: {:?}", buf[8]);
//
//        if checksum as u8 != buf[8] {
//            return Err(Error::BadChecksum);
//        }
//
//        Ok(buf)
//    }
//}
