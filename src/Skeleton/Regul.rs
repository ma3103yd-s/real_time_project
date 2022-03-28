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
    //TODO: Implement the Regul-thread that reads and writes to the I/O-channels
    fn run() {
    }
}
