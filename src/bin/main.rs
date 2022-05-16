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
use real_time_project::ref_mode;
use real_time_project::ui;

use std::sync::mpsc;
use iobox::ComediDevice;
use iobox::AnalogChannel;
use iobox::AnalogType::{AnalogIn, AnalogOut};
use regul::{Regul, ReferenceGenerator};
use mode:: {Mode, ModeMonitor};
use pid::{PIDparam, PID};
use ref_mode::{RefMode, RefModeMonitor};
use ui::{App, run, BeamCanvas};

pub fn main() {
    let (tx_u, rx_u) = mpsc::channel(); // Channel to send data;
    let (tx_y, rx_y) = mpsc::channel();
    let (tx_pos, rx_pos) = mpsc::channel();
    let(tx_angle, rx_angle) = mpsc::channel();
    //let gui_receiver = rx.clone();
    let inner = Arc::new(RwLock::new(PID::new()));
    let outer_param = PID::new().with_parameters(PIDparam::new(-0.1, 15.0, 1.5, 10.0, 10.0, 1.0, 0.02, true));
    let outer = Arc::new(RwLock::new(outer_param));
    //let ref_gen = ReferenceGenerator(1.0);
    let mut monitor = ModeMonitor::new();

    monitor.set_mode(Mode::BEAM);
    let inner_param = PIDparam::new(2.0, 0.0, 0.0, 10.0, 1.0, 1.0, 0.02, false);

    let mut inner = 
        Arc::new(RwLock::new(PID::new().with_parameters(inner_param)));

    //let outer = Arc::new(RwLock::new(PID::new()));
    let ref_mon = RefModeMonitor::new(RefMode::OPTIMAL);
    let ref_mon_ui = Arc::clone(ref_mon.get_ref());

    let ref_gen = ReferenceGenerator::new(2.0, ref_mon);
    let ref_arc = Arc::new(RwLock::new(ref_gen));
    //let mut monitor = ModeMonitor::new();

    //monitor.set_mode(Mode::BALL);

    let regul_thread = thread::Builder::new();
    let ui_thread = thread::Builder::new();

    let outer_ui  = Arc::clone(&outer);
    let inner_ui = Arc::clone(&inner);
    let mode_ui = Arc::clone(monitor.get_ref());
    let ref_gen_ui = Arc::clone(&ref_arc);


    let regul_mode = Arc::clone(monitor.get_ref());
    let regul_ref_mode = Arc::clone(&ref_arc);

    //assert_eq!(zero_val_d, MAX_VAL/2);
    //assert_eq!(zero_val_p, 0.0);

    let handler = regul_thread.spawn(move || {
        let mut regul = Regul::new(outer,
        regul_mode, inner, regul_ref_mode, tx_u, tx_y, tx_pos, tx_angle);
        regul.run();
    }).unwrap();
    let ui_handler = ui_thread.spawn(move || {
        let canvas = BeamCanvas::new(rx_angle, rx_pos, 3800.0);
        let mut app = App::new(outer_ui, inner_ui, ref_gen_ui, mode_ui,
            ref_mon_ui, rx_u, rx_y, 260, canvas);
        run(app).unwrap();
    }).unwrap();

    ui_handler.join().unwrap();
    handler.join().unwrap();

    //assert!(true);

}

