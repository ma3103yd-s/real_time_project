use std::sync::RwLock;


pub struct PIDparameters {

}

pub struct PID {
    p: Arc<RwLock<PIDparameters>,
    v: f64, 
    ynew: f64, 
    yold: f64,
    eold: f64,
    e: f64,
    D: f64,
    I: f64,
    ad: f64,
    bd: f64,
}

impl PID {
    //TODO: Implement a PID controller with methods to calculate the output
    //and update the state
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn calculate_output(&mut self, y: f64, yref: f64) -> f64{
        unimplemented!();
    }

    pub fn update_state(&mut self, u: f64){
        unimplemented!();
    }

    pub fn get_sampling_time() -> f64{
       unimplemented!();
    }

    pub fn set_parameters() {
       unimplemented!()
    }
}