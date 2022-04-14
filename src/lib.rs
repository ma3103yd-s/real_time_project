
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
    use iobox::{MAX_VAL, RANGE_1, RANGE_2};
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
        let inner_param = PIDparam::new(3.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.05, false);

        let mut inner = 
            Arc::new(RwLock::new(PID::new().with_parameters(inner_param)));

        let outer = Arc::new(RwLock::new(PID::new()));
        let ref_gen = ReferenceGenerator(1.0);
        let mut monitor = ModeMonitor::new();

        monitor.set_mode(Mode::BALL);



        let regul_thread = thread::Builder::new();

        let zero_val_d = iobox::from_physical(0.0, MAX_VAL, &RANGE_1);
        let zero_val_p = iobox::to_physical(MAX_VAL/2, MAX_VAL, &RANGE_1);


        //assert_eq!(zero_val_d, MAX_VAL/2);
        //assert_eq!(zero_val_p, 0.0);
        let zero_val_test = iobox::from_physical(zero_val_p, MAX_VAL, &RANGE_1);


        let handler = regul_thread.spawn(move|| {
            let mut regul = Regul::new(&outer, monitor,&inner,ref_gen);
            regul.run();
        }).unwrap();

        handler.join().unwrap();
        assert!(true);

    }

    
}


