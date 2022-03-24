pub struct Regul{
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
    fn limit(&mut self, u: f64){
        if(u < self.uMin){
            return self.uMin;
        } else if(u > self.uMax) {
            return self.uMax;
        }
        return u;
    }

    fn run(){
        let mut duration = 0;
        let mut t = SystemTime::now();

        loop {
            let y0 = analogInPosition.get();
            let yref = refGen.getRef();


            let vO = OutC.calculateOutput(y0,yref);
            let uO = limit(vO);
            OutC.updateState(uO);
            
            let yI = analogInAngle.get();
            let vI = InC.calculateOutput(yI,uO);
            let uI = limit(vI);
            InC.updateState(uI);
            analogOut.set(uI);

        }

    }
}