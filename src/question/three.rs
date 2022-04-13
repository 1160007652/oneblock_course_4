// 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束
extern crate num_traits;

use num_traits::float::Float;
use std::ops::{Div, Mul};

trait Shape<T> {
    fn area(&self) -> T;
}

struct Rectangle<T> {
    pub width: T,
    pub height: T,
}
struct Triangle<T> {
    pub side_1: T,
    pub side_2: T,
    pub side_3: T,
}
struct Circle<T> {
    pub radius: T,
}

impl<T: Mul<Output = T> + Copy> Shape<T> for Rectangle<T> {
    fn area(&self) -> T {
        self.width * self.height
    }
}

impl<T: Mul<f32, Output = T> + Float + Copy> Shape<T> for Circle<T> {
    fn area(&self) -> T {
        self.radius * self.radius * std::f32::consts::PI
    }
}

impl<T: Div<f32, Output = T> + Float + Copy> Shape<T> for Triangle<T> {
    fn area(&self) -> T {
        let mut p = (self.side_1 + self.side_2 + self.side_3) / 2.0;
        p = p * (p - self.side_1) * (p - self.side_2) * (p - self.side_3);
        (p).sqrt()
    }
}

pub fn calculate_graphics_area() {
    let rectangle = Rectangle {
        width: 10,
        height: 2,
    };
    println!("矩型面积: {}", rectangle.area());

    let triangle = Triangle {
        side_1: 6.0,
        side_2: 6.0,
        side_3: 6.0,
    };
    println!("三角形面积: {}", triangle.area());

    let circle = Circle { radius: 1.0 };
    println!("原型面积: {}", circle.area());
}
