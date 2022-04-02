
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_code)]
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub mod iobox;
pub mod sim;



#[cfg(test)]
mod tests {
    use super::*;
    use sim::AnalogType;
    use sim::ComediDevice;
    use sim::VirtualWriter;
    use std::thread;
    use std::time;
    
    #[test]
    fn test_virtual_analog() {
        let v = VirtualWriter::new();
        let v = v.start_writing("/tmp/write", 10).unwrap();
        let sampler = thread::Builder::new();
        
          let t = sampler.spawn(|| {
            let it = ComediDevice::init_device("/tmp/read").expect("Failed to init device");
            let dev = ComediDevice::new(0, 30000, AREF_GROUND, it);
            let analog_read = sim::AnalogChannel::new(AnalogType::AnalogIn(1), dev);
            let mut result:u32 = 0;
            let mut counter = 0;
            while counter < 20 {
                result = analog_read.read().unwrap_or(result);
                println!("Result is {}", result);
                thread::sleep(time::Duration::from_millis(20));
                counter +=1;
            }
            ()
            
        }).unwrap();  

        v.join();
        t.join();



        assert!(true);
        
    }

    
}
