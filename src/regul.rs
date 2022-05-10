use std::time::{SystemTime, Duration};
use ref_mode::{RefModeMonitor, RefMode};
const UMAX: f32 = 10.0;
const UMIN: f32 = -10.0;

pub struct ReferenceGenerator{
    h: f32,
    timebase: SystemTime,
    last_time: SystemTime,
    setpoint: f32,
    new_setpoint: f32,
    u0: f32,
    distance: f32,
    now: f32,
    t: f32,
    ts: SystemTime,
    T: f32,
    zf: f32,
    z0: f32,
    amp: f32,
    uff: f32,
    phiff: f32,
    K_PHI: f32,
    K_V: f32,
    period: f32,
    rmode: RefModeMonitor,
    sign: f32
}
impl ReferenceGenerator {

    pub fn new(val: f32, rmode: RefModeMonitor) -> Self {
        
        Self {
            h: 10.0,
            timebase: SystemTime::now(),
            last_time: SystemTime::now(),
            setpoint: 0.0,
            new_setpoint: 0.0,
            distance: 0.0,
            t: 0.0,
            now: 0.0,
            u0: 0.0,
            ts: SystemTime::now(),
            T: 0.0,
            zf: 0.0,
            z0: 0.0,
            amp: val,
            uff: 0.0,
            phiff: 0.0,
            K_PHI: 4.5,
            K_V: 10.0,
            period: 10.0,
            sign: 1.0,
            rmode,        
        }

    }

    /*pub fn new(&self, val: f64, set_r_mode: ref_Mode) {
        self.amp = val;
        self.h = 10.0;
        self.timebase = SystemTime::now();
        self.timeleft = 0.0;
        self.setpoint = 0.0;
        self.new_setpoint;
        self.u0 = 0.0;
        self.ts = self.timebase;
        self.T = 0.0;
        self.zf = 0.0;
        self.z0 = 0.0;
        self.timeleft = 0.0;
        self.K_PHI = 4.5;
        self.K_V = 10.0;
        self.period = 15.0;
        self.sign = 1.0;

        self.rmode = RefModeMonitor::new(set_r_mode);
    }*/

    pub fn get_ref(&mut self) -> f32 {
        return self.amp*self.sign;
    }

    pub fn get_amp(&self) -> f32 {
        return self.amp;
    }

    pub fn set_amp(&mut self, val: f32) {
        self.amp = val;
    }

    pub fn get_phiff(&self) -> f32 {
        return self.phiff;
    }
    pub fn get_uff(&self) -> f32 {
        return self.uff;
    }

    pub fn run(&mut self) {

        let (lock, cvar) = &*self.rmode.mode;
        let mut mode_change = lock.lock().unwrap();
        match *mode_change {



        RefMode::MANUAL => {
            //self.setpoint = 0.0;
            //self.amp = self.setpoint;

        },

        RefMode::SQUARE => {
            let now = SystemTime::now();
            if  now.duration_since(self.last_time).unwrap_or(Duration::ZERO).as_secs_f32() > self.period  {
                self.last_time = now;
                self.sign = -self.sign;
            }

            self.new_setpoint = self.sign * self.amp;
            self.setpoint = self.new_setpoint;

            //self.amp = self.setpoint;
        },

        RefMode::OPTIMAL => {
            let now = SystemTime::now();
            if  now.duration_since(self.last_time).unwrap_or(Duration::ZERO).as_secs_f32() > self.period  {
                self.last_time = now;
                self.sign = -self.sign;
            }

            self.new_setpoint = self.sign * self.amp;


            if self.new_setpoint != self.setpoint{
                self.ts = SystemTime::now();
                self.z0 = self.amp;
                self.zf = self.new_setpoint;
                self.distance = self.zf - self.z0;
                self.u0 = self.distance.signum() * 0.1;
                self.T = (self.distance.abs() / (2.0 * self.K_PHI * self.K_V * 0.1)).cbrt();
                self.setpoint = self.new_setpoint;
            }
            if self.amp != self.setpoint {
                let t = SystemTime::now().duration_since(self.ts).unwrap().as_secs_f32();
                let T = self.T;
                if t <= T {
                    self.uff = self.u0;
                    self.phiff = -self.K_PHI * self.u0 * t;
                    self.amp = self.z0 + self.K_PHI * self.K_V * self.u0 * t*t*t/6.0;
                } else if t <= 3.0*self.T {
                    self.uff = -self.u0;
                    self.phiff = self.K_PHI * self.u0 * (t - 2.0*T);
                    self.amp = self.z0 - self.K_PHI * self.K_V * self.u0 * (t*t*t/6.0 - T*t*t + T*T*t - T*T*T/3.0);
                } else if t <= 4.0*T {
                    self.uff = self.u0;
                    self.phiff = -self.K_PHI * self.u0 * (t - 4.0*T);
                    self.amp = self.z0 + self.K_PHI * self.K_V * self.u0 * (t*t*t/6.0 - 2.0*T*t*t + 8.0*T*T*t - 26.0*T*T*T/3.0);
                } else {
                    self.uff = 0.0;
                    self.phiff = 0.0;
                    self.amp = self.setpoint;
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
        Condvar,
        Mutex,
        mpsc,
    },
    thread,
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
    ref_gen: Arc<RwLock<ReferenceGenerator>>,
    analog_pos: AnalogChannel,
    analog_angle: AnalogChannel,
    analog_out: AnalogChannel,
    tx: mpsc::Sender<f64>,
    tx_pos: mpsc::Sender<f32>,
    tx_angle: mpsc::Sender<f32>,
}


fn limit( u: f32) -> f32 {
    if u < UMIN {
        return UMIN;
    } else if u > UMAX {
        return UMAX;
    }
    return u;
}


impl Regul {


    pub fn new(outer: Arc<RwLock<PID>>, mode: Arc<(Mutex<Mode>, Condvar)>,
               inner: Arc<RwLock<PID>>,ref_gen: Arc<RwLock<ReferenceGenerator>>,
               tx: mpsc::Sender<f64>, tx_pos: mpsc::Sender<f32>, tx_angle: mpsc::Sender<f32>)
        -> Self {
        let it = ComediDevice::init_device().unwrap();


        let com_pos = ComediDevice::new(0, 0, AREF_GROUND, it);
        let com_ang = ComediDevice::new(0, 1, AREF_GROUND, com_pos.clone_dev());
        let com_write = ComediDevice::new(1, 0, AREF_GROUND, com_pos.clone_dev());

        let analog_pos = AnalogChannel::new(AnalogIn(1), com_pos);
        let analog_angle = AnalogChannel::new(AnalogIn(0), com_ang);
        let analog_out = AnalogChannel::new(AnalogOut(1), com_write);

        Self {
            outer,
            mode,
            inner,
            ref_gen,
            analog_pos,
            analog_angle,
            analog_out,
            tx,
            tx_pos,
            tx_angle,
        }

    }

    pub fn clone_inner(&self) -> Arc<RwLock<PID>> {
        Arc::clone(&self.inner)
    }

    pub fn clone_outer(&self) -> Arc<RwLock<PID>> {
        Arc::clone(&self.outer)
    }



    pub fn run(&mut self) {
        let mut is_sat: bool = false;
        loop {
            let mut t = SystemTime::now();
            let mut _H = 0.0;
            {
                let (lock, cvar) = &*self.mode;
                let mut mode_change = lock.lock().unwrap();
                let mut inner = &mut (*self.inner).write().unwrap();
                let mut outer = &mut (*self.outer).write().unwrap();

                _H = inner.get_sampling_time();

                let mut ref_gen = &mut (*self.ref_gen).write().unwrap();
                ref_gen.run();
                match *mode_change {
                    Mode::OFF => {
                        self.analog_out.write(0.0);
                        while *mode_change == Mode::OFF {
                            mode_change = cvar.wait(mode_change).unwrap();
                        }

                    },

                    Mode::BEAM => {
                        let y = self.analog_angle.read().unwrap(); // Handle result later

                        //println!("yref is {}", y);
                        let yRef = ref_gen.get_ref();
                        //println!("yref is {}", yRef);

                        //Synchronize inner
                        //let mut inner = &mut (*self.inner).write().unwrap();
                        let u = limit(inner.calculate_output(y, yRef));
                        //println!("y is {}", y);
                        //println!("u is {}", u);
                        let w_val = self.analog_out.write(u).unwrap();
                        //println!("Value written is {}", w_val);
                        inner.update_state(u);
                        self.tx.send(y as f64).ok();
                        self.tx_angle.send(y).ok();
                        self.tx_pos.send(0.0).ok();



                    },

                    Mode::BALL => {
        
                        let y0 = self.analog_pos.read().unwrap()+0.3;
                        let yref = ref_gen.get_ref();
                        let phiFF = ref_gen.get_phiff();
                        let uFF = ref_gen.get_uff();

                        let yI = self.analog_angle.read().unwrap();

                        //Synchronize Outer

                        let vO = outer.calculate_output(y0, yref) + phiFF;
                        let uO = limit(vO);               
                        if (!is_sat) {
                            outer.update_state(uO-phiFF);
                        } else {
                            outer.update_state(yI-phiFF);
                        }


                        //Synchronize Inner


                        let vI = inner.calculate_output(yI, uO) + uFF;
                        let uI = limit(vI);
                        is_sat = uI == UMIN || uI == UMAX;
                        //println!("pos is {}", y0);
                        //println!("u is {}", uI);
                        //println!("yref is {}", yI);
                        inner.update_state(uI-uFF);
                        self.analog_out.write(uI);
                        self.tx.send(uI as f64).ok();
                        self.tx_pos.send(y0).ok();
                        self.tx_angle.send(yI).ok();

                            //analog_ref.set(refGen.getRef());


                    },
                };
            }
            t = t + Duration::from_millis(_H as u64);
            let duration = t.duration_since(SystemTime::now());

            if let Ok(duration) = duration {
                //println!("Duration is {:?}", duration);
                thread::sleep(duration);
            }

        };
    }
}
