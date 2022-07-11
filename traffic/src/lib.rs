pub trait Indicator {
    fn stop_time(&self) -> usize;
}

pub enum Traffic {
    Green,
    Yellow,
    Red,
}

impl Indicator for Traffic {
    fn stop_time(&self) -> usize {
        match &self {
            Self::Green => 55,
            Self::Yellow => 5,
            Self::Red => 60,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let green = Traffic::Green;
        assert_eq!(green.stop_time(), 55);

        let yellow = Traffic::Yellow;
        assert_eq!(yellow.stop_time(), 5);

        let red = Traffic::Red;
        assert_eq!(red.stop_time(), 60);
    }
}
