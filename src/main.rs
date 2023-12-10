use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::mem::transmute;
use std::time::Duration;
use std::{io, process, thread};
mod error;
pub use error::*;

//pub struct SDS011 {
//    /// Link to a sensor, must be open via new()
//    port: Box<dyn SerialPort>,
//}

fn main() {
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
    let delay = Duration::from_secs(10);

    println!("Receiving data on {} at {} baud:", "/dev/ttyUSBO", 9600);
    loop {
        let mut buffer = [0u8; 10];
        //let raw = sensor.get_reply().expect("");
        //let pm25_ar = [raw[2], raw[3]];
        //let pm10_ar = [raw[4], raw[5]];
        //let pm25: u16 = unsafe { transmute::<[u8; 2], u16>(pm25_ar) }.to_le();
        //let pm10: u16 = unsafe { transmute::<[u8; 2], u16>(pm10_ar) }.to_le();
        //let pmtwofive = pm25 as f32 / 10.0;
        //let pmten = pm10 as f32 / 10.0;
        //println!("PM10={} PM25={}", pmten, pmtwofive);
        match port.read_exact(buffer.as_mut()) {
            Ok(_) => {
                let pm25_ar = [buffer[2], buffer[3]];
                let pm10_ar = [buffer[4], buffer[5]];
                let pm25: u16 = unsafe { transmute::<[u8; 2], u16>(pm25_ar) }.to_le();
                let pm10: u16 = unsafe { transmute::<[u8; 2], u16>(pm10_ar) }.to_le();
                let pmtwofive = pm25 as f32 / 10.0;
                let pmten = pm10 as f32 / 10.0;
                println!("PM10={} PM25={}", pmten, pmtwofive);
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
        thread::sleep(delay)
    }
    //let mut serial_buf: Vec<u8> = vec![0; 32];
    //let data = port
    //    .read(serial_buf.as_mut_slice())
    //    .expect("Found no data!");
    //println!("{}", data);
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
