use iobox::Analog::{AnalogRead, AnalogWrite};


pub struct Regul {
    OutC: PID,
    InC: PID,
    refGen: referenceGenerator,
    uMin: f64,
    uMax: f64,
    analogInPosition: Analog,
    analogInAngle: Analog,
    analogOut: Analog,
    analogRef: Analog,
}
impl Regul {
    //TODO: Implement the Regul-thread that reads and writes to the I/O-channels
    fn run() {
        unimplemented!();
    }
}
