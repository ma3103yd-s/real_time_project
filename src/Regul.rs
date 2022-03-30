use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub struct Regul {
    OutC: Arc<Rwlock<PID>>,
    ModeMon:ModeMonitor,
    InC: Arc<Rwlock<PID>>,
    refGen: referenceGenerator,
    uMin: f64,
    uMax: f64,
    analogInPosition: AnalogSource,
    analogInAngle: AnalogSource,
    analogOut: AnalogSink,
    analogRef: AnalogSink,
}
impl Regul {
    fn limit(&mut self, u: f64) {
        if (u < self.uMin) {
            return self.uMin;
        } else if (u > self.uMax) {
            return self.uMax;
        }
        return u;
    }

    fn run() {
        while () {
            CASE = modeMon.getMode();
            match CASE {
                "OFF" => {
                    self.u = 0.0;
                    writeOutput(self.u);
                    let mut mode = &*self.ModeMon.mode;
                    let cvar = &*self.ModeMon.cond;
                    while(cvar.wait(mode).unwrap()==OFF){
                    }
                    break;
                }

                "BEAM" => {
                    y = readInput(analogInAngle);
                    yRef = refGen.getRef();

                    //Synchronize inner
                    inner = inC.lock.unwrap();
                    u = limit(InC.calculateOutput(y, yRef));
                    writeOutput(u);
                    InC.updateState(u);
                }

                "BALL" => {
                    let mut duration = 0;
                    let mut t = SystemTime::now();

                    loop {
                        let y0 = analogInPosition.get();
                        let yref = refGen.getRef();
                        let phiFF = refGen.getPhiFF();
                        let uFF = refGen.getUff();

                        //Synchronize Outer
                        {
                            outer = outC.lock.unwrap();
                            let vO = outer.calculateOutput(y0, yref) + phiFF;
                            let uO = limit(vO);
                            outer.updateState(uO-phiFF);
                        }

                        //Synchronize Inner
                        {
                            inner = inC.lock.unwrap();
                            let yI = analogInAngle.get();
                            let vI = inner.calculateOutput(yI, uO) + uFF;
                            let uI = limit(vI);
                            inner.updateState(uI-uFF);
                            analogOut.set(uI);
                        }
                        analogRef.set(refGen.getRef());

                        t = t + InC.getHMillis();
                        let duration = SystemTime::now().duration_since(t);
                        if (duration > 0) {
                            thread::sleep(duration)
                        }
                    }
                }
            }
        }
    }
}
