use std::sync::{Arc, Mutex, Condvar};
#[derive(PartialEq)]
pub enum Mode {
    OFF,
    BEAM,
    BALL,
}

pub struct ModeMonitor{
    pub mode: Arc<(Mutex<Mode>, Condvar)>,
}

impl ModeMonitor {
    pub fn new()-> Self {
        Self {
            mode: Arc::new((Mutex::new(Mode::OFF), Condvar::new())),
        }
    }

    pub fn set_mode(&mut self, m: Mode) {
        let (muter, cvar) = &*self.mode;
        let mut muter = muter.lock().unwrap();
        *muter = m;
        cvar.notify_all();
    }

    /*pub fn get_mode(&mut self) -> Mode {
        let (muter, _) = &*self.mode;
        return *muter.lock().unwrap();

    } */
}
