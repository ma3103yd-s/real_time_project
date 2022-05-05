use std::sync::{Arc, Mutex, Condvar};
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RefMode {
    MANUAL,
    SQUARE,
    OPTIMAL,
}

pub struct RefModeMonitor{
    pub mode: Arc<(Mutex<RefMode>, Condvar)>,
}

impl RefModeMonitor {
    pub fn new(rmode: RefMode)-> Self {
        Self {
            mode: Arc::new((Mutex::new(rmode), Condvar::new())),
        }
    }
    pub fn from(rmode: Arc<(Mutex<RefMode>, Condvar)>) -> Self {
        Self {
            mode: rmode,
        }
    }

    pub fn set_mode(&mut self, m: RefMode) {
        let (muter, cvar) = &*self.mode;
        let mut muter = muter.lock().unwrap();
        *muter = m;
        cvar.notify_all();
    }

    pub fn get_ref(&self) -> &Arc<(Mutex<RefMode>, Condvar)> {
        &self.mode
    }

    /*pub fn get_mode(&mut self) -> RefMode {
        let (muter, _) = &*self.mode;
        return *muter.lock().unwrap();

    }*/
}
