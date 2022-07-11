pub mod graph;
pub mod circle;
pub mod square;
pub mod triangle;

pub use graph::graph::Graph;
pub use circle::circle::*;
pub use square::square::*;
pub use triangle::triangle::*;

pub fn print_area<T: Graph>(graph: T) {
    println!("graph area is:{}", graph.get_area());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn it_works() {
        let circle = Circle::new(1.0);
        print_area(circle);
        assert_eq!(circle.get_area(), PI);

        let triangle = Triangle::new(1.0, 2.0);
        print_area(triangle);
        assert_eq!(triangle.get_area(), 1.0);

        let square = Square::new(1.0);
        print_area(square);
        assert_eq!(square.get_area(), 1.0);
    }
}
