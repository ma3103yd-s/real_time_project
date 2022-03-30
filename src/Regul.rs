
const UMAX: f64 = 10.0;
const UMIN: f64 = -10.0;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub struct Regul {
    outer: Arc<Rwlock<PID>>,
    mode:ModeMonitor,
    inner: Arc<Rwlock<PID>>,
    ref_gen: referenceGenerator,
    analog_pos: AnalogChannel,
    analog_angle: AnalogChannel,
    analog_out: AnalogChannel,
    analog_ref: AnalogChannel,
}
impl Regul {
    fn limit(&mut self, u: f64) {
        if (u < UMIN) {
            return UMIN;
        } else if (u > UMAX) {
            return UMAX;
        }
        return u;
    }

    fn run() {
        while () {
            
            match mode.get_mode() {
                OFF => {
                    self.u = 0.0;
                    writeOutput(self.u);
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
                    u = limit(inner.calculateOutput(y, yRef));
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
                            outer.updateState(uO-phiFF);
                        }

                        //Synchronize Inner
                        {
                            inner = &*self.inner.lock.unwrap();
                            let yI = analog_angle.get();
                            let vI = inner.calculateOutput(yI, uO) + uFF;
                            let uI = limit(vI);
                            inner.updateState(uI-uFF);
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
