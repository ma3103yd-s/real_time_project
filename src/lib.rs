
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_code)]
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub mod iobox;
pub mod sim;
pub mod mode;
pub mod pid;
pub mod regul;



#[cfg(test)]
mod tests {
    use super::*;
    use iobox::ComediDevice;
    use iobox::AnalogChannel;
    use iobox::AnalogType::{AnalogIn, AnalogOut};

    use std::thread;
    use std::time;
    
    #[test]
    fn test_virtual_analog() {
       
        let it = ComediDevice::init_device().unwrap();

        //let dev = ComediDevice::new(1, 0, AREF_GROUND, &it);

        let write_dev = ComediDevice::new(1, 0, AREF_GROUND, it);

        let write_chan = AnalogChannel::new(AnalogOut(0), write_dev);

        //let read_channel = AnalogChannel::new(AnalogIn(1), dev);


        write_chan.write(5000).unwrap();

        //println!("Value read is {}", res);



        assert!(true);
        
    }

    
}
