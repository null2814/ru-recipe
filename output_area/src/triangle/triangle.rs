use crate::graph::*;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    base: f64,
    height: f64,
}

impl Triangle {
    pub fn new(base: f64, height: f64) -> Self {
        Self {
            base: base,
            height: height,
        }
    }
}

impl graph::Graph for Triangle {
    fn get_area(&self) -> f64 {
        self.base * self.height / 2f64
    }
}