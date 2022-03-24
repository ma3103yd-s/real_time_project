
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_code)]
#![allow(deref_nullptr)]



include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ptr;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn comedi_test()  {
        unsafe  {
            let subdev: u32 = 0;
            let chan: u32  = 0;
            let range: u32 = 0;
            let data: *mut lsampl_t = ptr::null_mut() as *mut lsampl_t;
            
            let aref: u32 = AREF_GROUND;
            let mut retval:i32 = 0;

            let it: Option<&mut comedi_t> = comedi_open("/dev/comedi0".as_ptr() as *mut i8).as_mut();

            let it: &mut comedi_t = it.expect("Unable to open device");
            
            retval = comedi_data_read(it, subdev, chan, range, aref, data);

            if retval < 0 {
                panic!("Unable to read data");
                
            }

            println!("{}", *data);
            assert!(true);


        }
        
        
    }
}
