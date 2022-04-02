
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
    it: Rc<RefCell<comedi_t>>,
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
    pub fn new(subdev: u32, range: u32, aref: u32, init: Rc<RefCell<comedi_t>>)
    -> Self {
        let dev = ComediDevice {
            subdev,
            range,
            aref,
            it: Rc::clone(&init),
        };
        return dev;
    }

    pub fn init_device() -> Result<Rc<RefCell<comedi_t>>,IOError> {
        let mut it: comedi_t = comedi_t {
            _unused:[]
        };
        unsafe {
            let temp = comedi_open(DEV_PATH.as_ptr() as *const i8).as_mut();
            it = *(temp.ok_or(IOError::DeviceError)?);
        }

        return Ok(Rc::new(RefCell::new(it)));
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
        let mut data: u32 = 0;
        if let AnalogType::AnalogIn(chan) = &self._type {
            unsafe {
                let mut data_p: *mut lsampl_t = ptr::null_mut();
                retval = comedi_data_read(&mut *(*self.dev.it).borrow_mut(), self.dev.subdev, *chan, self.dev.range,
                    self.dev.aref, data_p);
                if retval < 0 {
                    return Err(IOError::ReadError);
                }
                data = *data_p as u32;
                phys_data = convert_to_physical(data, MAXVAL, &RANGE)
            }
            return Ok(phys_data);

        } else {
            return Err(IOError::WriteOnly);
        }

    }

    fn convert_to_physical(data: u32, maxval: u32, range: &[f32;2]) -> f32 {
        let length = abs(range[1]-range[0]);
        let dx = length/maxval as f32;
        0..maxval.map(|x| {
            (x, range[0]+x*dx)
        }).filter(|(i, xx)| {
            i==data
        }).unwrap().0

    }

    fn convert_from_physical(data: f32, maxval: u32, range: &[f32;2]) -> u32 {
        let length = abs(range[1]-range[0]);
        let dx = length/maxval as f32;
        0..maxval.map(|x| {
            (x, range[0]+x*dx)
        }).filter(|(i, xx)| {
            xx>=data
        }).take(1).unwrap().1
        
    }

    pub fn write(&self, phys_data: f32) -> Result<(), IOError> {
        // TODO: write data to iobox
        if let AnalogType::AnalogOut(chan) = &self._type {
            unsafe {
                let data  = convert_from_physical(phys_data, MAXVAL, &RANGE);
                let data_p: lsampl_t = lsampl_t {data};
                retval = comedi_data_write(&mut *(*self.dev.it).borrow_mut(), self.dev.subdev, *chan, self.dev.range,
                    self.dev.aref, data_p);
                if retval < 0 {
                    return Err(IOError::WriteError);
                }
            }
            return Ok(());

        } else {
            return Err(IOError::ReadOnly);
        }


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