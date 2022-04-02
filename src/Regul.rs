const AREF_GROUND: usize =  0;
const UMAX: f64 = 10.0;
const UMIN: f64 = -10.0;
const DEV_POS: &'static str = "/tmp/pos";
const DEV_ANG: &'static str = "/tmp/ang";
const DEV_OUT: &'static str = "/tmp/out";
const DEV_REF: &'static str = "/tmp/in";


pub struct ReferenceGenerator(f64);

impl ReferenceGenerator {

    pub fn get_ref(&self) -> f64 {
        self.0
    }

    pub fn get_phiFF(&self) -> f64 {
        self.get_ref()
    }

    pub fn get_uFF(&self) -> f64 {
        self.get_ref()
    }

}

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
    _ref: ReferenceGenerator,
}
impl Regul {


    pub fn new(inner: Arc<RwLock<PID>>, outer: Arc<RwLock<PID>>,
    ref_gen: ReferenceGenerator) -> Self {
        let pos_it = ComediDevice::init_device(DEV_POS).expect("Failed to init device");
        let ang_it = ComediDevice::init_device(DEV_ANG).expect("Failed to init device");
        let out_it = ComediDevice::init_device(DEV_OUT).expect("Failed to init device");
        let pos = ComediDevice::new(0, 30000, AREF_GROUND, pos_it);
        let ang = ComediDevice::new(0, 30000, AREF_GROUND, ang_it);
        let out = ComediDevice::new(0, 30000, AREF_GROUND, out_it);
        let analog_pos = AnalogChannel::new(AnalogRead(0), pos);
        let analog_angle = AnalogChannel::new(AnalogRead(1), ang);
        let analog_out = AnalogChannel::new(AnalogWrite(0), out);

        Self {
            outer,
            mode,
            inner,
            ref_gen,
            analog_pos,
            analog_angle,
            analog_out,
            _ref: ref_gen,
        }
        
    }

    pub fn clone_inner(&self) -> Arc<RwLock<PID>> {
        Arc::clone(&self.inner)
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

    pub fn run(&mut self) {
        loop {
            
            match mode.get_mode() {
                OFF => {

                    let (lock, cvar) = &*self.mode.mode;
                    let mut mode_change = lock.lock().unwrap();
                    {
                        let mut inner = &*self.inner.lock.unwrap();
                        inner.v = 0.0;
                        analog_out.write(0.0);
                    }

                    while(*mode_change == OFF){
                        mode_change = cvar.wait(mode_change).unwrap();
                    }
                    break;
                }

                BEAM => {
                    let y = self.analog_angle.read();
                    let yRef = _ref.get_ref();

                    //Synchronize inner
                    let mut inner = &*self.inner.lock().unwrap();
                    u = limit(inner.calculate_output(y, yRef));
                    self.analog_out.write(u);
                    inner.update_state(u);
                }

                BALL => {
                    let mut duration = 0;
                    let mut t = SystemTime::now();
                    let y0 = analog_pos.read();
                    let yref = _ref.get_ref();
                    let phiFF = _ref.get_phiFF();
                    let uFF = _ref.get_uFF();

                    //Synchronize Outer
                    let mut uo: f32 = 0.0;
                    {
                        let mut outer = &*self.outer.lock.unwrap();
                        let vO = outer.calculate_output(y0, yref) + phiFF;
                        uO = limit(vO);
                        outer.update_state(uO-phiFF);
                    }

                    //Synchronize Inner
                    let mut h = 0;
                    {
                        inner = &*self.inner.lock.unwrap();
                        h = inner.get_h_millis();
                        let yI = self.analog_angle.read();
                        let vI = inner.calculate_output(yI, uO) + uFF;
                        let uI = limit(vI);
                        inner.update_state(uI-uFF);
                        self.analog_out.write(uI);
                    }
                    analog_ref.set(refGen.getRef());

                    t = t + outer.getHMillis();
                    let duration = SystemTime::now().duration_since(t);
                    if (duration > 0) {
                        thread::sleep(duration);
                    }
                
                }
            }
        }
    }
}
