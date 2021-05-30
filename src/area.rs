//作业三：实现一个打印图形面积的函数，接收一个可以计算面积的类型作为参数，需要用到泛型和泛型约束

use std::f64::consts::PI;

pub struct Circle {
  pub r: f64
}

pub struct Triangle {
  pub b: f64,
  pub h: f64,
}

pub struct Square {
  pub a: f64,
}

pub trait Area {
  fn area(&self) -> f64;
}

impl Area for Circle {
  fn area(&self) -> f64 {
    PI * self.r * self.r
  }
}

impl Area for Triangle {
  fn area(&self) -> f64 {
    self.b * self.h / 2.0
  }
}

impl Area for Square {
  fn area(&self) -> f64 {
    self.a * self.a
  }
}

pub fn get_area<T: Area>(item: T) -> f64 {
  item.area()
}