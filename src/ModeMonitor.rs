use std::sync::{Arc, Mutex};
pub enum Mode {
    OFF,
    Beam,
    Ball,
}

pub struct ModeMonitor{
    mode: Arc,
}

impl ModeMonitor {
    pub fn construct()-> Self{
        return ModeMonitor(mutex: Arc::new(Mutex::new(OFF)));
    }

    pub fn setMode(&mut self, Mode newMode){
        let mut muter = self.mode.lock().unwrap();
        *muter = newMode;
    }

    pub fn getMode(&mut self)->Mode{
        return = self.mode.lock().unwrap();
    }
}
