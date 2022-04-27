use crate::{DataSource, Point, Result, Time, Value};

const GEN_POINTS: u32 = 200;
const GEN_T_INTERVAL: Time = 20;

const UMAX: f64 = 10.0;
const UMIN: f64 = -10.0;

pub struct ReferenceGenerator{
    h: f64,
    timebase: Time,
    timeleft: f64,
    setpoint: f64,
    new_setpoint: f64,
    u0: f64,
    disctance: f64,
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


impl DataSource for Regul{
    fn get_data(&mut self) -> Result<Vec<Point>>{
        Ok(rv)
    }
    fn get_num_values(&self) -> Result<usize>{
        return Ok(4);
    }
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
    time::Duration,
};

use pid::PID;
use iobox::{ComediDevice, Analog::{
    AnalogChannel,
    AnalogType::{AnalogIn, AnalogOut}
}};
use iobox::Digital;

pub struct Regul {
    outer: Arc<RwLock<PID>>,
    mode: ModeMonitor,
    inner: Arc<RwLock<PID>>,
    ref_gen: ReferenceGenerator,
    analog_pos: AnalogChannel,
    analog_angle: AnalogChannel,
    analog_out: AnalogChannel,
    rv: Vec<Point>,
    curr_time: Time,
    interval: Time
}
impl Regul {

    pub fn new(outer: &Arc<RwLock<PID>>, mode: ModeMonitor, inner: &Arc<RwLock<PID>>,
    ref_gen: ReferenceGenerator) -> Self {
        let outer = Arc::clone(outer);
        let inner = Arc::clone(inner);
        
        let mut curr_time = Duration::from_secs(0);
        let interval = Duration::from_secs(outer.get_sampling_time);

        let it = ComediDevice::init_device().unwrap();


        let com_pos = ComediDevice::new(0, 0, AREF_GROUND, &it);
        let com_ang = ComediDevice::new(0, 1, AREF_GROUND, &it);
        let com_write = ComediDevice::new(1, 0, AREF_GROUND, &it);

        let analog_pos = AnalogChannel::new(AnalogRead(0), com_pos);
        let analog_angle = AnalogChannel::new(AnalogRead(1), com_ang);
        let analog_out = AnalogChannel::new(AnalogWrite(1), com_write);

        Self {
            outer,
            mode,
            inner,
            ref_gen,
            analog_pos,
            analog_angle,
            analog_out,
        }
        
    }

    pub fn clone_inner(&self) -> Arc<RwLock<PID>> {
        Arc::clone(&self.inner);
    }

    fn addP(&self, ang: f32, pos: f32,u: f32,r: f32) {
        let t = self.curr_time;
        rv.push(Point{
            t,
            vs: [ang,pos,u,r]
        }
        );
        self.curr_time += self.interval;
    }

    pub fn clone_outer(&self) -> Arc<RwLock<PID>> {
        Arc::clone(&self.outer)
    }

    fn limit(&mut self, u: f32) {
        if (u < UMIN) {
            return UMIN;
        } else if (u > UMAX) {
            return UMAX;
        }
        return u;
    }

    pub fn run() {
        loop {
            
            match mode.get_mode() {
                OFF => {
                    self.u = 0.0;
                    self.analog_out.write(u);
                    let (lock, cvar) = &*self.mode.mode;
                    let mut mode_change = lock.lock().unwrap();
                    while(*mode_change == OFF){
                        mode_change = cvar.wait(mode_change).unwrap();
                    }
                    writeP(0.0,0.0,0.0);
                },

                BEAM => {
                    y = self.analog_ang.read();

                    yRef = self.ref_gen.get_ref();

                    //Synchronize inner
                    let mut inner = &*self.inner.lock().unwrap();
                    u = limit(inner.calculate_output(y, yRef));
                    self.analog_out.write(u);
                    inner.update_state(u);
                    writeP(y,0.0,u,yRef);
                },

                 BALL => {
                    /*
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
                    } */
                },
            }
        }
    }
}
