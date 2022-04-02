
use std::{ptr, cell::RefCell, rc::Rc};
// TODO: 
// Define constants relating to subdev, ports, etc of the ioboc
use crate::*;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io;
use std::fs::{File, OpenOptions};
use std::thread;
use std::time;

const DEV_READ: &'static str = "/tmp/test_read";
const DEV_WRITE: &'static str = "/tmp/test_write";


pub struct ComediDevice {
    subdev: u32,
    range: u32,
    aref: u32,
    it: Rc<RefCell<File>>,
}

pub struct AnalogChannel {
    _type: AnalogType,
    dev: ComediDevice,
}

pub enum AnalogType {
    AnalogIn(u32),
    AnalogOut(u32),

}

pub struct DigitalChannel {
    _type: DigitalType,
    dev: ComediDevice,
}

pub enum DigitalType {
    DigitalIn(u32),
    DigitalOut(u32),
}

#[derive(Debug)]
pub enum IOError {
    ReadOnly,
    WriteOnly,
    ReadError,
    WriteError,
    PortNotOpen,
    DeviceError
}


impl ComediDevice {
    pub fn new(subdev: u32, range: u32, aref: u32, init: Rc<RefCell<File>>)
    -> Self {
        let dev = ComediDevice {
            subdev,
            range,
            aref,
            it: Rc::clone(&init),
        };
        return dev;
    }

    pub fn init_device(dev: &str) -> io::Result<Rc<RefCell<File>>> {
        let mut f = File::open(dev)?;
        Ok(Rc::new(RefCell::new(f)))
    }

}

impl AnalogChannel {

    pub fn new(_type: AnalogType, dev: ComediDevice) -> Self {
        Self {
            _type,
            dev,
        }
    }

    pub fn read(&self) -> Result<u32, IOError> {
        // TODO: read analog data from iobox.
        let mut retval: i32 = 0;
        let mut buf = [0;4];
        let mut data: u32 = 0;
        if let AnalogType::AnalogIn(chan) = &self._type {
            let mut f = (*self.dev.it).borrow_mut();
            let n = f.read(&mut buf[..]).map_err(|_| IOError::ReadError)?;
            data = u32::from_ne_bytes(buf);

        } else {
            return Err(IOError::WriteOnly);
        }
        Ok(data)

    }

    pub fn write(&self, data: u32) -> Result<(), IOError> {
        // TODO: write data to iobox
        if let AnalogType::AnalogOut(chan) = &self._type {
            let mut f = (*self.dev.it).borrow_mut();
            let write_data = data.to_ne_bytes();
            let n = f.write(&write_data[..]).map_err(|_| IOError::WriteError)?;
    

        } else {
            return Err(IOError::WriteOnly);
        }
        Ok(())

    }
    


}


pub struct VirtualWriter(thread::Builder);

impl VirtualWriter {

    pub fn new() -> Self {
        Self(thread::Builder::new())
    }

    pub fn start_writing(self, dev: &'static str, sampling_time: u64) -> io::Result<thread::JoinHandle<()>> {
        println!("INITIALIZING THREAD");
        self.0.spawn(move || {
            let mut f = OpenOptions::new().write(true).open(dev).unwrap();
            let mut writer = BufWriter::new(f);
            let data: u32 = 100;
            let mut counter = 0;
            while counter < 20 {
                //let data_str = format!("{}", data);
                //let data_buf = data_str.as_bytes();
                let data_buf = data.to_ne_bytes();
                let n = writer.write(&data_buf).unwrap();
                println!("Bytes written: {}", n);
                thread::sleep(time::Duration::from_millis(sampling_time));
                counter +=1;
            }
            

        })


    }

}

impl DigitalChannel {

    pub fn read(&self) -> Result<u32, IOError> {
        // TODO: read analog data from iobox.
        unimplemented!();
    }

    pub fn write(&self) -> Result<u32, IOError> {
        // TODO: write data to iobox
        unimplemented!();
    }


}