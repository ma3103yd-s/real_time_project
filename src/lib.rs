
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_code)]
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub mod iobox;
pub mod sim;
pub mod pid;
pub mod Regul;



#[cfg(test)]
mod tests {
    use super::*;
    use sim::AnalogType;
    use sim::ComediDevice;
    use sim::VirtualWriter;
    use std::thread;
    use std::time;
    use pid::PID;
    use mode::ModeMonitor;
    
    #[test]
    fn test_virtual_analog() {
        let ang_writer = VirtualWriter::new();
        let ang_writer = ang_writer.start_writing("/tmp/ang", 10).unwrap();
        let pos_writer = VirtualWriter::new();
        let pos_writer = pos_writer.start_writing("/tmp/pos", 10).unwrap();
        let sampler = thread::Builder::new();
        let it = ComediDevice::init_device("/tmp/read").expect("Failed to init device");
        let it_2 = ComediDevice::init_device("/tmp/write").expect("Failed to init device");
        let dev = ComediDevice::new(0, 30000, AREF_GROUND, it);
        let dev_2 = ComediDevice::new(0, range, AREF_GROUND, it_2);
        let analog_read = sim::AnalogChannel::new(AnalogType::AnalogIn(1), dev);
        let analog_write = sim::AnalogChannel::new(AnalogType::AnalogOut(1),dev_2);
        let inner = Arc::new(RwLock::new(PID::new()));
        let outer = Arc::new(RwLock::new(PID::new()));


        let mode = ModeMonitor::init();
        let regul_mode = ModeMonitor::new(mode.clone_arc());

        let mut regul = Regul::new(Arc::clone(&inner), Arc::clone(&outer), 
                    regul_mode);
        
        let t = sampler.spawn(move || {
            regul.run();
        }).unwrap();  

        ang_writer.join();
        t.join();



        assert!(true);
        
    }

    
}
