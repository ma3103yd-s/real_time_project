#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_code)]
#![allow(deref_nullptr)]
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::{
    sync::{
        RwLock,
        Arc,
    },thread};


//pub mod iobox;
//pub mod mode;
//pub mod pid;
//pub mod regul;
extern crate real_time_project;

use real_time_project::iobox;
use real_time_project::mode;
use real_time_project::pid;
use real_time_project::regul;


use iobox::ComediDevice;
use iobox::AnalogChannel;
use iobox::AnalogType::{AnalogIn, AnalogOut};
use regul::{Regul, ReferenceGenerator};
use mode:: {Mode, ModeMonitor};
use pid::{PIDparam, PID};

pub fn main() {
    let (tx, rx) = flume::unbounded(); // Channel to send data;
    let gui_receiver = rx.clone();
    let inner = Arc::new(RwLock::new(PID::new()));
    let outer = Arc::new(RwLock::new(PID::new()));
    let ref_gen = ReferenceGenerator(1.0);
    let mut monitor = ModeMonitor::new();

    monitor.set_mode(Mode::BEAM);



    thread::spawn(move|| {
        let mut regul = Regul::new(&outer, monitor.get_ref(),&inner,ref_gen);
        regul.run()
    });
}
