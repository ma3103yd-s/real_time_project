
const UMAX: f32 = 10.0;
const UMIN: f32 = -10.0;

pub struct ReferenceGenerator(pub f32);


impl ReferenceGenerator {
    pub fn new(val: f32) -> Self {
        Self(val)
    }

    pub fn get_ref(&self) -> f32 {
        self.0
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
    mode: ModeMonitor,
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


    pub fn new(outer: &Arc<RwLock<PID>>, mode: ModeMonitor, inner: &Arc<RwLock<PID>>,
    ref_gen: ReferenceGenerator) -> Self {
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
            mode,
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
            let (lock, cvar) = &*self.mode.mode;
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
