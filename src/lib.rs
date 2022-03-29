
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_code)]
#![allow(deref_nullptr)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub mod iobox;



#[cfg(test)]
mod tests {
    use super::*;
    use iobox::AnalogType;
    use iobox::ComediDevice;
    
    #[test]
    fn test_analog() {
        let it = ComediDevice::init_device().expect("Failed to init device");
        let dev = ComediDevice::new(0, 30000, AREF_GROUND, it);
        let analog_read = iobox::AnalogChannel::new(AnalogType::AnalogIn(1), dev);
        let result = analog_read.read().expect("Failed to read data");

        assert!(true);
        
    }
}
