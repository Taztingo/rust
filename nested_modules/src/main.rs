pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green
}

use a::series::of::nested_modules;

//Import only red and yellow
//use TrafficLight::{Red, Yellow};

//Import all
use TrafficLight::*;

fn main() {
    nested_modules();
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
