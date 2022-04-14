use crate::{DataSource, Point, Result, Time, Value};

const GEN_POINTS: u32 = 200;
const GEN_T_INTERVAL: Time = 20;

const UMAX: f64 = 10.0;
const UMIN: f64 = -10.0;

pub struct ReferenceGenerator(f64);


impl DataSource for Regul{
    fn get_data(&mut self) -> Result<Vec<Point>>{
        Ok(rv)
    }
    fn get_num_values(&self) -> Result<usize>{
        return Ok(4);
    }
}

impl ReferenceGenerator {
    pub fn new(val: f64) {
        Self(val)
    }

    pub fn get_ref(&self) -> f64 {
        self.0
    }

    pub fn run(&self) {
        let h = 10;
        let mut timebase = SystemTime::now();
        let mut timeleft = 0;
        let mut duration;

        let mut setpoint = 0.0;
        let mut new_setpoint;
        let mut u0 = 0.0; 
        let mut distance;
        let mut now;
        let mut t;
        let mut tf = 0.001 * timebase as f32;
        let mut ts = tf;
        let mut T = 0.0;
        let mut zf = 0.0;
        let mut z0 = 0.0;



        let mut timeleft = 0;
        loop{
            now = 0.001 * timebase as f32;
            timeleft -= self.h;
            if (timeleft <= 0) {
                timeleft += (long) (500.0 * self.period);
            }
            new_setpoint = -get_ref();
            ts = now;
            z0 = get_ref();
            zf = new_setpoint;
            distance = zf - z0;
            u0 = Math.signum(distance) * max_ctrl;
            T = Math.cbrt(Math.abs(distance) / (2.0 * K_PHI * K_V * max_ctrl));
            tf = ts + 4.0 * T;
            setpoint = new_setpoint;


            if (get_ref() != setpoint) {
                t = now - ts;	
                if (t <= T) {
                    uff = u0;
                    phiff = -K_PHI * u0 * t;
                    new(z0 + K_PHI * K_V * u0 * t*t*t/6);
                } else if (t <= 3.0*T) {
                    uff = -u0;
                    phiff = K_PHI * u0 * (t - 2*T);
                    new(z0 - K_PHI * K_V * u0 * (t*t*t/6 - T*t*t + T*T*t - T*T*T/3));
                } else if (t <= 4.0*T) {
                    uff = u0;
                    phiff = -K_PHI * u0 * (t - 4*T);
                    new(z0 + K_PHI * K_V * u0 * (t*t*t/6 - 2*T*t*t + 8*T*T*t - 26*T*T*T/3));
                } else {
                    uff = 0.0;
                    phiff = 0.0;
                    new(setpoint);
                }
            }


            timebase += h;
            duration = timebase - SystemTime::now();
            if (duration > 0) {
                sleep(duration);			
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

    fn addP(ang: f32, pos: f32,u: f32,r: f32) {
        let t = curr_time;
        rv.push(Point{
            t,
            vs: [ang,pos,u,r]
        }
        );
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
