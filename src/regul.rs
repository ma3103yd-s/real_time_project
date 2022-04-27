
const UMAX: f32 = 10.0;
const UMIN: f32 = -10.0;

pub struct ReferenceGenerator{
    h: f64,
    timebase: Time,
    timeleft: f64,
    setpoint: f64,
    new_setpoint: f64,
    u0: f64,
    distance: f64,
    now: f64,
    t: f64,
    ts: Time,
    T: f64,
    zf: f64,
    z0: f64,
    amp: f64,
    uff: f64,
    phiff: f64,
    K_PHI: f64,
    K_V: f64,
    period: f64,
    rmode: RefModeMonitor
}
impl ReferenceGenerator {
    pub fn new(&self, val: f64, set_r_mode: ref_Mode) {
        self.amp = val;
        self.h = 10;
        self.timebase = SystemTime::now();
        self.timeleft = 0.0;
        self.setpoint = 0.0;
        self.new_setpoint; 
        self.u0 = 0.0; 
        self.ts = timebase;
        self.T = 0.0;
        self.zf = 0.0;
        self.z0 = 0.0;
        self.timeleft = 0.0;
        self.K_PHI = 4.5;
        self.K_V = 10.0;
        self.period = 15.0;
        
        rmode = RefModeMonitor::new(set_r_mode);
    }

    pub fn get_ref(&mut self) -> f64 {
        return self.amp;
    }

    pub fn get_phiff(&self) -> f64 {
        return self.phiff;
    }
    pub fn get_uff(&self) -> f64 {
        return self.uff;
    }

    pub fn run(&mut self) {

        match rmode.get_mode(){

        Manual => {
            setpoint = 0.0;
            self.amp = setpoint;

        }
        
        Square => {
            timeleft -= self.h;
            if (timeleft <= 0) {
                timeleft += (long) (500.0 * self.period);
                sign = -sign;
            }
            new_setpoint = sign * self.amp;
            setpoint = new_setpoint;
            self.amp = setpoint;
        }
        
        Optimal => {
            timeleft -= self.h;
            if (timeleft <= 0) {
                timeleft += (long) (500.0 * self.period);
                sign = -sign;
            }
            new_setpoint = sign * self.amp;
            if (new_setpoint != setpoint){
                self.ts = SystemTime::now();
                self.z0 = self.amp;
                self.zf = new_setpoint;
                self.distance = zf - z0;
                self.u0 = distance.signum() * 0.1;
                self.T = (distance.abs() / (2.0 * K_PHI * K_V * 0.1)).cbrt();
                self.setpoint = new_setpoint;
            }
            if (self.amp != setpoint) {
                let t = SystemTime::now().duration_since(ts).unwrap().as_secs();	
                let T = self.T;
                if (t <= T) {
                    self.uff = self.u0;
                    self.phiff = -K_PHI * self.u0 * t;
                    self.amp = (self.z0 + self.K_PHI * self.K_V * self.u0 * t*t*t/6);
                } else if (t <= 3.0*self.T) {
                    self.uff = -self.u0;
                    self.phiff = self.K_PHI * self.u0 * (t - 2*T);
                    self.amp = (self.z0 - self.K_PHI * self.K_V * self.u0 * (t*t*t/6 - T*t*t + T*T*t - T*T*T/3));
                } else if (t <= 4.0*T) {
                    self.uff = self.u0;
                    self.phiff = -self.K_PHI * self.u0 * (t - 4*T);
                    self.amp = (self.z0 + self.K_PHI * self.K_V * self.u0 * (t*t*t/6 - 2*T*t*t + 8*T*T*t - 26*T*T*T/3));
                } else {
                    self.uff = 0.0;
                    self.phiff = 0.0;
                    self.amp = (setpoint);
                }
            }
        }
    }    
    }
}
use std::{
    sync::{
        RwLock,
        Arc,
    },
    thread,
    time::{SystemTime, Duration},
};

use pid::PID;
use crate::AREF_GROUND;
use iobox::{ComediDevice,
    AnalogChannel,
    AnalogType::{AnalogIn, AnalogOut},
};
use mode::{Mode, ModeMonitor};

//use iobox::Digital;

pub struct Regul {
    outer: Arc<RwLock<PID>>,
    mode: Arc<(Mutex<Mode>, Condvar)>,
    inner: Arc<RwLock<PID>>,
    ref_gen: ReferenceGenerator,
    analog_pos: AnalogChannel,
    analog_angle: AnalogChannel,
    analog_out: AnalogChannel,
}


fn limit( u: f32) -> f32 {
    if (u < UMIN) {
        return UMIN;
    } else if (u > UMAX) {
        return UMAX;
    }
    return u;
}
 

impl Regul {


    pub fn new(outer: &Arc<RwLock<PID>>, mode: &Arc<(Mutex<Mode>, Condvar)>,
               inner: &Arc<RwLock<PID>>,ref_gen: ReferenceGenerator)
        -> Self {
        let outer = Arc::clone(outer);
        let inner = Arc::clone(inner);
        
        let it = ComediDevice::init_device().unwrap();


        let com_pos = ComediDevice::new(0, 0, AREF_GROUND, it);
        let com_ang = ComediDevice::new(0, 1, AREF_GROUND, com_pos.clone_dev());
        let com_write = ComediDevice::new(1, 0, AREF_GROUND, com_pos.clone_dev());

        let analog_pos = AnalogChannel::new(AnalogIn(1), com_pos);
        let analog_angle = AnalogChannel::new(AnalogIn(0), com_ang);
        let analog_out = AnalogChannel::new(AnalogOut(1), com_write);

        Self {
            outer,
            mode: Arc::clone(mode),
            inner,
            ref_gen,
            analog_pos,
            analog_angle,
            analog_out,
        }
        
    }

    pub fn clone_inner(&self) -> Arc<RwLock<PID>> {
        Arc::clone(&self.inner)
    }

    pub fn clone_outer(&self) -> Arc<RwLock<PID>> {
        Arc::clone(&self.outer)
    }

   

    pub fn run(&mut self) {
        loop {
            let mut t = SystemTime::now();
            let (lock, cvar) = &*self.mode;
            let mut mode_change = lock.lock().unwrap();
            let mut inner = &mut (*self.inner).write().unwrap();
            let mut outer = &mut (*self.outer).write().unwrap();
            match *mode_change {
                Mode::OFF => {
                    self.analog_out.write(0.0);
                    while(*mode_change == Mode::OFF){
                        mode_change = cvar.wait(mode_change).unwrap();
                    }
                    
                },

                Mode::BEAM => {
                    let y = self.analog_angle.read().unwrap(); // Handle result later

                    let yRef = self.ref_gen.get_ref();
                    println!("yref is {}", yRef);

                    //Synchronize inner
                    //let mut inner = &mut (*self.inner).write().unwrap();
                    let u = limit(inner.calculate_output(y, yRef));
                    //println!("y is {}", y);
                    //println!("u is {}", u);
                    let w_val = self.analog_out.write(u).unwrap();
                    //println!("Value written is {}", w_val);
                    inner.update_state(u);





                },

                 Mode::BALL => {
                    


                
                    let y0 = self.analog_pos.read().unwrap();
                    let yref = self.ref_gen.get_ref();
                    let phiFF = 0.0;//ref_gen.getPhiFF();
                    let uFF = 0.0;//ref_gen.getUff();

                    //Synchronize Outer
                    
                    let vO = outer.calculate_output(y0, yref) + phiFF;
                    let uO = limit(vO);
                    outer.update_state(uO-phiFF);
                    

                    //Synchronize Inner
                    
                    let yI = self.analog_angle.read().unwrap();
                    let vI = inner.calculate_output(yI, uO) + uFF;
                    let uI = limit(vI);
                    println!("pos is {}", y0);
                    println!("u is {}", uI);
                    println!("yref is {}", yref);
                    inner.update_state(uI-uFF);
                    self.analog_out.write(uI);
                        
                        //analog_ref.set(refGen.getRef());

                     
                },
            };

            t = t + Duration::from_millis(inner.get_sampling_time() as u64);
            let duration = t.duration_since(SystemTime::now());

            if let Ok(duration) = duration {
                println!("Duration is {:?}", duration);
                thread::sleep(duration);
            }



        };
    }
}
