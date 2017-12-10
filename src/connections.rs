pub struct ConnectionStatus {
    pub idx1: usize,
    pub idx2: usize,
    pub strength: f64
}

impl ConnectionStatus {
   pub fn new(idx1: usize, idx2: usize) -> ConnectionStatus {
        ConnectionStatus {
            idx1, idx2,
            strength: 0.0
        }
    }
}