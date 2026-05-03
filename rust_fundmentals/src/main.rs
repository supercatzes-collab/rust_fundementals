
mod helper_functions;
use crate::helper_functions::*;
mod projects;
use projects::*;
fn main() {
    loop {
        let input = get_i32("exit[0] bank[1] shape[2] option1[3] option1[4] option1[5]: ");

        match input {
            0 => return,
            1 => bank(),
            2 => shape_menu(),
            3 => println!("option 3"),
            4 => println!("option 4"),
            5 => println!("option 5"),
            _ => println!("end"),
        }
    }
}
