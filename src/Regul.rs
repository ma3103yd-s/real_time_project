
const UMAX: f64 = 10.0;
const UMIN: f64 = -10.0;

pub struct ReferenceGenerator(f64);

use std::{
    sync::{
        RwLock,
        Arc,
    },
    thread,
    time::Duration,
};

use pid::PID;
use sim::Analog::{
    AnalogChannel,
    AnalogType::{AnalogIn, AnalogOut}
};
use sim::Digital;

pub struct Regul {
    outer: Arc<RwLock<PID>>,
    mode: ModeMonitor,
    inner: Arc<RwLock<PID>>,
    ref_gen: ReferenceGenerator,
    analog_pos: AnalogChannel,
    analog_angle: AnalogChannel,
    analog_out: AnalogChannel,
    analog_ref: AnalogChannel,
}
impl Regul {


    pub fn new(outer: PID, mode: ModeMonitor, inner: PID,
    ref_gen: ReferenceGenerator) -> Self {
        let outer = Arc::new(RwLock::new(outer));
        let inner = Arc::new(RwLock::new(inner));
        let analog_pos = AnalogChannel::new(AnalogRead(0));
        let analog_angle = AnalogChannel::new(AnalogRead(1));
        let analog_out = AnalogChannel::new(AnalogWrite(0));
        let analog_ref = AnalogChannel::new(AnalogWrite(1));

        Self {
            outer,
            mode,
            inner,
            ref_gen,
            analog_pos,
            analog_angle,
            analog_out,
            analog_ref,
        }
        
    }

    pub fn clone_inner(&self) -> Arc<RwLock<PID>> {
        Arc::clone(&self.inner);
    }

    pub fn clone_outer(&self) -> Arc<RwLock<PID>> {
        Arc::clone(&self.outer)
    }

    fn limit(&mut self, u: f64) {
        if (u < UMIN) {
            return UMIN;
        } else if (u > UMAX) {
            return UMAX;
        }
        return u;
    }

    pub fn run() {
        while () {
            
            match mode.get_mode() {
                OFF => {
                    self.u = 0.0;
                    write_output(self.u);
                    let (lock, cvar) = &*self.mode.mode;
                    let mut mode_change = lock.lock().unwrap();
                    while(*mode_change == OFF){
                        mode_change = cvar.wait(mode_change).unwrap();
                    }
                    break;
                }

                BEAM => {
                    y = read_input(analogInAngle);
                    yRef = refGen.getRef();

                    //Synchronize inner
                    let mut inner = &*self.inner.lock().unwrap();
                    u = limit(inner.calculate_output(y, yRef));
                    write_output(u);
                    inner.update_state(u);
                }

                BALL => {
                    let mut duration = 0;
                    let mut t = SystemTime::now();

                    loop {
                        let y0 = analog_position.get();
                        let yref = refGen.getRef();
                        let phiFF = refGen.getPhiFF();
                        let uFF = refGen.getUff();

                        //Synchronize Outer
                        {
                            let mut outer = &*self.outer.lock.unwrap();
                            let vO = outer.calculateOutput(y0, yref) + phiFF;
                            let uO = limit(vO);
                            outer.update_state(uO-phiFF);
                        }

                        //Synchronize Inner
                        {
                            inner = &*self.inner.lock.unwrap();
                            let yI = analog_angle.get();
                            let vI = inner.calculate_output(yI, uO) + uFF;
                            let uI = limit(vI);
                            inner.update_state(uI-uFF);
                            analog_out.set(uI);
                        }
                        analog_ref.set(refGen.getRef());

                        t = t + InC.getHMillis();
                        let duration = SystemTime::now().duration_since(t);
                        if (duration > 0) {
                            thread::sleep(duration);
                        }
                    }
                }
            }
        }
    }
}
