use output_area::{self, Graph};
use sum_slice;
use traffic::{self, Indicator};

fn main() {
    // 枚举信号灯
    println!(
        "green light will continue:{}s",
        traffic::Traffic::Green.stop_time()
    );
    println!(
        "yellow light will continue:{}s",
        traffic::Traffic::Yellow.stop_time()
    );
    println!(
        "red light will continue:{}s",
        traffic::Traffic::Red.stop_time()
    );

    // u32整数集合求和
    if let Some(value) = sum_slice::sum(&vec![1u32, 2u32, 3u32]) {
        println!("the sum of serial:{}", value)
    } else {
        println!("the sum of serial was overflow")
    }

    if let Some(value) = sum_slice::sum(&vec![1u32, u32::MAX, 3u32]) {
        println!("the sum of serial:{}", value)
    } else {
        println!("the sum of serial was overflow")
    }

    // 打印图形面积
    let circle = output_area::Circle::new(1.0);
    println!("circle area is:{}", circle.get_area());

    let triangle = output_area::Triangle::new(1.0, 2.0);
    println!("triangle area is:{}", triangle.get_area());

    let square = output_area::Square::new(1.0);
    println!("square area is:{}", square.get_area());
}
