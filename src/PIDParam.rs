pub struct PIDParam{
    pub K: f64,
    pub Ti: f64,
    pub Tr: f64,
    pub Td: f64,
    pub N: f64,
    pub Beta: f64,
    pub H: f64,
    pub integratorOn: bool 
}
impl PIDParam {
    pub fn new(K: f64, Ti: f64, Tr: f64, Td: f64, N: f64, Beta:f64, H:f64, integratorOn: bool) -> Self {
        Self{K, Ti, Tr, Td, N, Beta, H, integratorOn}
    }
}

