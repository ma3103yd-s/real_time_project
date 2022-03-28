use std::sync::RwLock;

pub struct PID{
    p: PIDParam,
    v: f64, 
    ynew: f64, 
    yold: f64,
    eold: f64,
    e: f64,
    D: f64,
    I: f64,
    ad: f64,
    bd: f64,
    lock: RwLock
}

impl PID{
    //TODO: Implement a PID controller with methods to calculate the output
    //and update the state
    pub fn new() -> Self {
        
    }

    pub fn calculateOutput(&mut self, y: f64, yref: f64) -> f64{
        
    }

    pub fn updateState(&mut self, u: f64){
        
    }

    pub fn getHMillis() -> f64{
       
    }

    pub fn setParameters() {
       
    }
}