use std::sync::RwLock;

pub struct PID{
    p: PIDParam,
    v: f64, 
    ynew: f64, 
    yold: f64,
    e: f64,
    D: f64,
    I: f64,
    ad: f64,
    bd: f64,
    lock: RwLock
}

impl PID{
    pub fn new() -> Self {
        Self{
            lock: RwLock::new(PIDParam), 
            I: 0,
            D: 0,
            v: 0,
            e: 0,
            ynew: 0,
            yold: 0,
            ad: 0,
            bd: 0,
            p: PIDParam::new(-0.1, 0.0, 0, 1.0, 5.0, 1.0, 0.1, false)
        }
    }

    pub fn calculateOutput(&mut self, y: f64, yref: f64) -> f64{
        let param = lock.read().unwrap();
        self.ynew = y;
        self.e = yref - y;
        self.D = self.ad*self.D - self.bd*(y - self.yold);
        self.v = param.K*(param.Beta*yref - y) + self.I + self.D;     
        return self.v;
    }

    pub fn updateState(&mut self, u: f64){
        let param = lock.read().unwrap();
        if (param.integratorOn) {
            // Forward Euler approximation
            self.I += (param.K * param.H / param.Ti) * self.e + (param.H / param.Tr) * (u - self.v);
        } else {
            self.I = 0;
        }
        self.yold = self.ynew;
    }

    pub fn getHMillis() -> f64{
        return (self.p.H * 1000);
    }

    pub fn setParameters() {
        let mut w = lock.write.unwrap();
        *w = newParameters.copy() as PIDParameter;
        if (!p.integratorOn) {
            I = 0;
        }
        ad = p.Td/(p.Td + p.N * p.H);
        bd = p.K*p.Td*p.N/(p.Td + p.N * p.H);
    }
}