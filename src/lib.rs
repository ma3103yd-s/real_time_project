
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_code)]
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub mod iobox;
//pub mod sim;
pub mod mode;
pub mod pid;
pub mod regul;

pub use iobox::ComediDevice;
pub use iobox::AnalogChannel;
pub use iobox::AnalogType::{AnalogIn, AnalogOut};
pub use regul::{Regul, ReferenceGenerator};
pub use mode:: {Mode, ModeMonitor};
pub use pid::{PIDparam, PID};


#[cfg(test)]
mod tests {
    use super::*;
    use iobox::ComediDevice;
    use iobox::AnalogChannel;
    use iobox::AnalogType::{AnalogIn, AnalogOut};

    use std::thread;
    use std::time;
    use std::sync::{RwLock, Arc};
    /*
    #[test]
    fn test_virtual_analog() {
       
        let it = ComediDevice::init_device().unwrap();

        //let dev = ComediDevice::new(1, 0, AREF_GROUND, &it);

        let write_dev = ComediDevice::new(1, 0, AREF_GROUND, it);

        let write_chan = AnalogChannel::new(AnalogOut(0), write_dev);

        //let read_channel = AnalogChannel::new(AnalogIn(1), dev);


        write_chan.write(0.0).unwrap();

        //println!("Value read is {}", res);



        assert!(true);
        
    }*/

    #[test]
    fn test_regul() {
        let inner = Arc::new(RwLock::new(PID::new()));
        let outer = Arc::new(RwLock::new(PID::new()));
        let ref_gen = ReferenceGenerator(0.0);
        let mut monitor = ModeMonitor::new();

        monitor.set_mode(Mode::BEAM);



        let regul_thread = thread::Builder::new();

        let handler = regul_thread.spawn(move|| {
            let mut regul = Regul::new(&outer, monitor,&inner,ref_gen);
            regul.run();
        }).unwrap();

        handler.join().unwrap();
        assert!(true);

    }

    
}


