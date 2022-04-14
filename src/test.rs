#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_code)]
#![allow(deref_nullptr)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::{
    sync::{
        RwLock,
        Arc,
    },thread};


pub mod mode;
pub mod iobox;
pub mod pid;
pub mod regul;


use iobox::ComediDevice;
use iobox::AnalogChannel;
use iobox::AnalogType::{AnalogIn, AnalogOut};
use regul::{Regul, ReferenceGenerator};
use mode:: {Mode, ModeMonitor};
use pid::{PIDparam, PID};

pub fn main() {
    let inner = Arc::new(RwLock::new(PID::new()));
    let outer = Arc::new(RwLock::new(PID::new()));
    let ref_gen = ReferenceGenerator(1.0);
    let mut monitor = ModeMonitor::new();

    monitor.set_mode(Mode::BEAM);



    thread::spawn(move|| {
        let mut regul = Regul::new(&outer, monitor,&inner,ref_gen);
        regul.run()
    });
}
