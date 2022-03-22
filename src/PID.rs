pub struct PID{
    p: PIDParam,
    v: f64, 
    ynew: f64, 
    yold: f64,
    e: f64,
    D: f64,
    I: f64,
    ad: f64,
    bd: f64
}

impl PID{
    fn calculateOutput(&mut self, y: f64, yref: f64) -> f64{
        self.ynew = y;
        self.e = yref - y;
        self.D = self.ad*self.D - self.bd*(y - self.yold);
        self.v = self.p.K*(self.p.Beta*yref - y) + self.I + self.D;     
        return self.v;
    }

    fn updateState(&mut self, u: f64){
        if (self.p.integratorOn) {
            // Forward Euler approximation
            self.I += (self.p.K * self.p.H / self.p.Ti) * self.e + (self.p.H / self.p.Tr) * (u - self.v);
        } else {
            self.I = 0;
        }
        self.yold = self.ynew;
    }

    fn getHMillis() -> f64{
        return (self.p.H * 1000);
    }

    fn setParameters() {
        p = (PIDParameters) newParameters.clone();
        if (!p.integratorOn) {
            I = 0;
        }
        ad = p.Td/(p.Td + p.N * p.H);
        bd = p.K*p.Td*p.N/(p.Td + p.N * p.H);
    }
}