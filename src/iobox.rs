use std::borrow::BorrowMut;
use std::ffi::CString;
use std::{ptr, cell::RefCell, rc::Rc};
// TODO: 
// Define constants relating to subdev, ports, etc of the ioboc
use crate::*;
use std::sync::Once;


pub const DEV_PATH: &'static str = "/dev/comedi0";
pub const MAX_VAL: u32 = 65535;
pub const RANGE_1: [f32;2] = [-10.0,10.0];
pub const RANGE_2: [f32;2] = [-5.0,5.0];


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

pub fn to_physical(val: u32, max_data_val: u32, range: &[f32;2])-> f32{
    let ratio = (range[0]-range[1]).abs()/max_data_val as f32;
    let new_val = ratio*(val as f32) + range[0];
    return new_val;
}

pub fn from_physical(val: f32, max_data_val: u32, range: &[f32;2])-> u32{
    let ratio = max_data_val as f32/(range[0]-range[1]).abs();
    let new_val = (ratio*(val-range[0])) as u32 ;
    return new_val;

}


impl ComediDevice {
    pub fn new(subdev: u32, range: u32, aref: u32, init: Rc<RefCell<*mut comedi_t>>)
    -> Self {
        let dev = ComediDevice {
            subdev,
            range,
            aref,
            it: init,
        };
        return dev;
    }

    pub fn clone_dev(&self) -> Rc<RefCell<*mut comedi_t>> {
        Rc::clone(&(self.it))
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

    pub fn read(&self) -> Result<f32, IOError> {
        // TODO: read analog data from iobox.
        let mut retval: i32 = 0;
        let mut data: u32 = 10;
        let mut r_data: f32 = 0.0;
        if let AnalogType::AnalogIn(chan) = self._type {
            unsafe {
                let mut data_p: lsampl_t = 0;
                retval = comedi_data_read(*(*self.dev.it).borrow_mut(), self.dev.subdev, chan, self.dev.range,
                    self.dev.aref, &mut data_p);
                 if retval < 0 {
                    return Err(IOError::ReadError);
                } 
                data = data_p;
                if(chan == 0){
                    r_data = to_physical(data,MAX_VAL,&RANGE_1);
                } else {
                    r_data = to_physical(data,MAX_VAL,&RANGE_2);
                }
            }
            return Ok(r_data);

        } else {
            return Err(IOError::WriteOnly);
        }

    }

    pub fn write(&self, val: f32) -> Result<u32, IOError> {
        // TODO: write data to iobox
        let mut retval: i32 = 0;
        let mut w_val: u32 = 0;
        if let AnalogType::AnalogOut( chan) = self._type {
            unsafe {
                w_val = from_physical(val,MAX_VAL,&RANGE_1);
                retval = comedi_data_write(*(*self.dev.it).borrow_mut(), self.dev.subdev, chan, self.dev.range, self.dev.aref, w_val);
            }
            if  retval < 0 {
                return Err(IOError::WriteError);
            }
            return Ok(w_val);
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
