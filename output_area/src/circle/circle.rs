use std::f64::consts::PI;
use crate::graph::*;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius: radius }
    }
}

impl graph::Graph for Circle {
    fn get_area(&self) -> f64 {
        PI * self.radius.powi(2)
    }
}