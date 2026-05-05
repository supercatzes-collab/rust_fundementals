//use crate::{helper_functions::*, projects::shape};
use std::f64::consts::PI;

pub fn shape_menu() {
    //So like instead of writing multiple print statements for every type, we can just use a vect which kinda works like an array except it doesn't have a limit in how many types in can store.
    //However we have different types and vec does not like storing multiple types, so to get around this we'll use a type called "Box".
    //It's a type that points to other types. That's all it does as far as i am aware.
    //Essentially we can get around the annoying limitation of vec by not storing the different types inside but instead just storing pointers that points to those different types. Since all pointers are the same type, it works.
    let shapes_list: Vec<Box<dyn Shape>> = vec![
    Box::new(Rectangle { width: 3.0, height: 4.0 }),  
    Box::new(Circle { radius: 5.0 }),                  
    Box::new(Triangle { a: 6.0, b: 5.0, c: 5.0, height: 4.0 }),
    ];

    for shapes in shapes_list.iter() {
        println!("area: {:.2}", shapes.area());
        println!("perimeter: {:.2}", shapes.perimeter());
    }

    //this function makes a new vec where it stores all the filtered contents from the previous vec.
    //"iter()" method automatically turns all the data into references.
        let filter_min: Vec<&Box<dyn Shape>> = shapes_list.iter()
            .filter(|shape| shape.area() >=  12.0) //This is where we call the values of the shapes then do thing to it. In this case it checks if the area is greater or equal than 12.0. If it doesn't meet the criteria it gets ignored.
            .collect();                                                                        //It stores the data into the vec the runs it.

        let largest_area = filter_min.iter()
            .map(|shape| shape.area())//We need to convert the shape instance into plain f64 (unlike filter). For some reason it can't just read "shape.area()" directly.
            .fold(f64::NEG_INFINITY, f64::max); //Idk man ts hard to understand.

        let sum_area: f64 =  filter_min.iter()
            .map(|shape| shape.area())
            .sum(); //basically returns the sum of all the area. Shrimple enuff.

        let count = filter_min.iter()
            .filter(|shape| shape.area() >= 50.0)
            .count(); //It counts only the shapes that met the criteria then finores the rest. Honestly i don't wsee why you won't just use "collect()" for this.

    println!("Largest area: {}", largest_area);
    println!("Sum of areas: {}", sum_area);
    println!("Count of shapes with area >= 50: {}", count);
        
}

//A trait kinda works like a struct in a way that it stores expected value types in which you can manipulate with impl.
//Not sure if you can also store functions in a struct.
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Triangle {
    a: f64,
    b: f64,
    c: f64,
    height: f64,
}

//We use implement to manipulate the trait then use the struct as the structure. So like every single impl's here share the same trait but does
//different functions to them. So i guess its a way to bundle up stuff with the same expected values but they all each have different functions or
//a specific contraints. Like having different struct names.
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
       2.0 * (self.width + self.height)
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * (self.a * self.height)
    }

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c       
    }
}