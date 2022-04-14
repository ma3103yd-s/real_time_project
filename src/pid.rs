#[derive(Debug, Clone, Copy)]
pub struct PIDparam {
    K: f32,
    Ti: f32,
    Td: f32,
    Tr: f32,
    N: f32,
    Beta: f32,
    H: f32,
    integrator_on: bool,

}

impl PIDparam {
    pub fn new(K: f32, Ti: f32, Td: f32,
               Tr: f32, N: f32, Beta: f32, H: f32, integrator_on: bool) -> Self {

        Self {
            K,
            Ti,
            Td,
            Tr,
            N,
            Beta,
            H,
            integrator_on,
        }

    }
}

pub struct PID{
    p: PIDparam,
    v: f32, 
    ynew: f32, 
    yold: f32,
    eold: f32,
    e: f32,
    D: f32,
    I: f32,
    ad: f32,
    bd: f32,
}

impl PID{
    pub fn new() -> Self {
        let p = PIDparam::new(-0.1, 0.0, 0.1, 1.0, 5.0, 1.0, 0.1, false);
        let mut temp  = Self {
            p,
            v: 0.0,
            ynew: 0.0,
            yold: 0.0,
            eold: 0.0,
            e: 0.0,
            D: 0.0,
            I: 0.0,
            ad: 0.0,
            bd: 0.0
        };
        temp.set_parameters(p);
        return temp;


    }

    pub fn calculate_output(&mut self, y: f32, yref: f32) -> f32 {
        self.ynew = y;
        self.e = yref - y;
        self.D = self.ad*self.D - self.bd*(y - self.yold);
        self.v = self.p.K*(self.p.Beta*yref - y) + self.I + self.D;     
        return self.v;
    }

    pub fn update_state(&mut self, u: f32){
        if (self.p.integrator_on) {
            // Forward Euler approximation
            self.I += (self.p.K * self.p.H / self.p.Ti) *
                self.e + (self.p.H / self.p.Tr) * (u - self.v);
        } else {
            self.I = 0.0;
        }
        self.eold = self.e;
    }

    pub fn get_sampling_time(&self) -> f32 {
        return (self.p.H * 1000.0);
    }

    pub fn set_parameters(&mut self, params: PIDparam) {
        self.p = params;
        if(self.p.integrator_on) {
            self.I = 0.0;
        }
        self.ad = self.p.Td/(self.p.Td + self.p.N * self.p.H);
        self.bd = self.p.K*self.p.Td*self.p.N/(self.p.Td + self.p.N * self.p.H);
    }

    pub fn with_parameters(mut self, params: PIDparam) -> Self {
        self.set_parameters(params);
        self

    }

}
