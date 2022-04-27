use std::sync::{Arc, Mutex, Condvar};
pub enum ref_Mode {
    MANUAL,
    SQUARE,
    OPTIMAL,
}

pub struct RefModeMonitor{
    pub mode: Arc<Mutex<Mode>, Condvar>,
}

impl RefModeMonitor {
    pub fn new(rmode: ref_Mode)-> Self {
        Self {
            mode: Arc::new((Mutex::new(rmode), Condvar::new())),
        }
    }

    pub fn set_mode(&mut self, m: ref_Mode) {
        let (muter, cvar) = &*self.mode;
        let mut muter = muter.lock().unwrap();
        *muter = m;
        cond.notify_all();
    }

    pub fn get_mode(&mut self) -> ref_Mode {
        let (muter, _) = &*self.mode;
        return *muter.lock().unwrap();

    }
}
