use crate::{helper_functions::*, projects::shape};
use std::f64::consts::PI;

pub fn shape_menu() {

}

//Declare structs for each type because they all have different fields.
//Do the same with impl
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}
#[derive(Debug)]
struct Triangle {
    height: f64,
    base: f64,
}
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Triangle {
    fn area(&self) -> f64 {
        self.base * self.height
    }
}

impl Circle {
    fn area(&self) -> f64 {
        PI * self.radius
    }
}