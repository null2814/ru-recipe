use crate::graph::*;

#[derive(Debug, Clone, Copy)]
pub struct Square {
    side: f64,
}

impl Square {
    pub fn new(side: f64) -> Self {
        Self { side: side }
    }
}

impl graph::Graph for Square {
    fn get_area(&self) -> f64 {
        self.side.powi(2)
    }
}