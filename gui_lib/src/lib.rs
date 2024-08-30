

pub trait Draw {
    fn draw(&self);
}


// Generic version
// Can only have one type of component
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }


// Trait objects with dynamic dispatch
// When use Traits objects, Rust uses dynamic dispatch(compiler doesn't know the concrete type at compile time)
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // dynamic dispatch
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button: {}", self.label);
    }
}