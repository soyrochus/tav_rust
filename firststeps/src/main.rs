mod basic;
mod structures;
mod gps;
mod calculations;
mod collections;

fn main() {
    basic::basic();

    collections::collections();
    
    structures::structures();

    calculations::calculate();

    gps::plot();
  
}
