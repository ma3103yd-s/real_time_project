use std::sync::{Arc, Mutex,Condvar};
pub enum Mode {
    OFF,
    Beam,
    Ball,
}

pub struct ModeMonitor{
    mode: Arc<Mutex<Mode>>,
    pub cond: Condvar
}

impl ModeMonitor {
    pub fn construct()-> Self{
        Self{
            mutex: Arc::new(Mutex::new(OFF))
            cond CondVar::new();
        };
    }

    pub fn setMode(&mut self, Mode newMode){
        let mut muter = self.mode.lock().unwrap();
        *muter = newMode;
        cond.notify_all();
    }

    pub fn getMode(&mut self)->Mode{
        return = self.mode.lock().unwrap();
    }
}
