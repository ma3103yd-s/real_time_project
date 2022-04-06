use std::borrow::BorrowMut;
use std::ffi::CString;
use std::{ptr, cell::RefCell, rc::Rc};
// TODO: 
// Define constants relating to subdev, ports, etc of the ioboc
use crate::*;
use std::sync::Once;

const DEV_PATH: &'static str = "/dev/comedi0";



pub struct ComediDevice {
    subdev: u32,
    range: u32,
    aref: u32,
    it: Rc<RefCell<*mut comedi_t>>,
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
    pub fn new(subdev: u32, range: u32, aref: u32, init: &Rc<RefCell<*mut comedi_t>>)
    -> Self {
        let dev = ComediDevice {
            subdev,
            range,
            aref,
            it: Rc::clone(init),
        };
        return dev;
    }

    pub fn init_device() -> Result<Rc<RefCell<*mut comedi_t>>,IOError> {
        unsafe {
            let c_string = CString::new(DEV_PATH).expect("CString failed");
            let temp = comedi_open(c_string.as_ptr()).as_mut();
            let temp = temp.ok_or(IOError::DeviceError)?;
            return Ok(Rc::new(RefCell::new(temp)));
        }

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
        let mut data: u32 = 10;
        if let AnalogType::AnalogIn(chan) = self._type {
            unsafe {
                let mut data_p: lsampl_t = 0;
                retval = comedi_data_read(*(*self.dev.it).borrow_mut(), self.dev.subdev, chan, self.dev.range,
                    self.dev.aref, &mut data_p);
                 if retval < 0 {
                    return Err(IOError::ReadError);
                } 
                data = data_p;
            }
            return Ok(data);

        } else {
            return Err(IOError::WriteOnly);
        }

    }

    pub fn write(&self, val: u32) -> Result<(), IOError> {
        // TODO: write data to iobox
        let mut retval: i32 = 0;
        if let AnalogType::AnalogOut( chan) = self._type {
            unsafe {
                retval = comedi_data_write(*(*self.dev.it).borrow_mut(), self.dev.subdev, chan, self.dev.range, self.dev.aref, val);
            }
            if  retval < 0 {
                return Err(IOError::WriteError);
            }
            return Ok(());
        }

        return Err(IOError::ReadOnly);
        
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
