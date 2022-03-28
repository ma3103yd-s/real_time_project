use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub struct Regul {
    OutC: PID,
    InC: PID,
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
                    break;
                }

                "BEAM" => {
                    y = readInput(analogInAngle);
                    yRef = refGen.getRef();

                    //Synchronize inner
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
                        let vO = OutC.calculateOutput(y0, yref) + phiFF;
                        let uO = limit(vO);
                        OutC.updateState(uO-phiFF);

                        //Synchronize Inner
                        let yI = analogInAngle.get();
                        let vI = InC.calculateOutput(yI, uO) + uFF;
                        let uI = limit(vI);
                        InC.updateState(uI-uFF);
                        analogOut.set(uI);

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
