use std::{
    sync::{
        RwLock,
        Arc,
    },thread};
use regul::{ModeMonitor, ReferenceGenerator, Regul};
use pid::PID;

pub fn main() {
    let inner = Arc::new(RwLock::new(PID::new()));
    let outer = Arc::new(RwLock::new(PID::new()));
    let ref_gen = ReferenceGenerator(1);
    let monitor = ModeMonitor::new();

    monitor.set_mode(BEAM);

    let regul = Regul::new(&outer, monitor,&inner,ref_gen);

    thread::new(move||regul.run());
}
