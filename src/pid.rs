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
}

impl PID{
    pub fn new() -> Self {
        let p = PIDParam::new(-0.1, 0.0, 0.0, 1.0, 5.0, 1.0, 0.1, false);
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
        temp.setParameters(p);
        return temp;


    }

    pub fn calculateOutput(&mut self, y: f64, yref: f64) -> f64{
        self.ynew = y;
        self.e = yref - y;
        self.D = self.ad*self.D - self.bd*(y - self.yold);
        self.v = param.K*(param.Beta*yref - y) + self.I + self.D;     
        return self.v;
    }

    pub fn updateState(&mut self, u: f64){
        if (self.p.integratorOn) {
            // Forward Euler approximation
            self.I += (self.p.K * self.p.H / self.p.Ti) *
                self.e + (self.p.H / self.p.Tr) * (u - self.v);
        } else {
            self.I = 0;
        }
        self.eold = self.e;
    }

    pub fn getHMillis() -> f64{
        return (self.p.H * 1000);
    }

    pub fn setParameters(&mut self, params: PIDparams) {
        self.p = params.copy();
        if(self.p.integratorOn) {
            self.I = 0.0;
        }
        self.ad = p.Td/(p.Td + p.N * p.H);
        self.bd = p.K*p.Td*p.N/(p.Td + p.N * p.H);
    }

    pub fn with_parameters(mut self, params: PIDparams) -> Self {
        self.setParameters(params);
        self

    }

}
