#[macro_use]
extern crate lazy_static;

use std::io::{Result, ErrorKind};
use std::ptr;
// TODO: 
// Define constants relating to subdev, ports, etc of the ioboc

const SUBDEV: u32 = 0;
const RANGE: u32 = 0;
const AREF: u32 = AREF_GROUND;

lazy_static! {
    static ref PORT: Option<&mut comedi_t>  = {
        unsafe {
            let mut it = comedi_open("/dev/comedi0".as_ptr() as *mut i8).as_mut();
            let it: &comedi_t = it.expect("Unable to open device");
            it
        }

    }
 
}

enum AnalogChannel {
    AnalogIn(u32),
    AnalogOut(u32),

}

enum DigitalChannel {
    DigitalIn(u32),
    DigitalOut(u32),
}

enum IOError {
    ReadOnly,
    WriteOnly,
    ReadError,
    WriteError,
    PortNotOpen,
}


impl AnalogChannel {

    pub fn read(&self) -> Result<u32, IOError> {
        // TODO: read analog data from iobox.
        let mut retval: i32 = 0;
        let mut data: u32 = 0;
        if let AnalogIn(chan) = &self {
            unsafe {
                let mut data_p: *mut lsampl_t = ptr::null_mut();
                let valid_port = PORT.ok_or(PortNotOpen)?;
                retval = comedi_data_read(valid_port, SUBDEV, chan, RANGE, AREF, data_p);
                if retval < 0 {
                    return Err(ReadError);
                }
                data = *data_p as u32;
            }
            return Ok(data);

        } else {
            return Err(WriteOnly);
        }

    }

    pub fn write(&self) -> io::Result<u32> {
        // TODO: write data to iobox
        unimplemented!();
    }


}


impl DigitalChannel {

    pub fn read(&self) -> io::Result<u32> {
        // TODO: read analog data from iobox.
        unimplemented!();
    }

    pub fn write(&self) -> io::Result<u32> {
        // TODO: write data to iobox
        unimplemented!();
    }


}