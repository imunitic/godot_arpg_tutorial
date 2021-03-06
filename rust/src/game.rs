use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[register_with(Self::register_builder)]
pub struct Game {
    name: String,
}

#[methods]
impl Game {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Game builder is registered!");
    }

    fn new(_owner: &Node2D) -> Self {
        godot_print!("Game is created!");
        Game {
            name: "".to_string(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: &Node2D) {
        self.name = "Game".to_string();
        godot_print!("{} is ready!", self.name);
    }

    #[export]
    unsafe fn _process(&self, _owner: &Node2D, _delta: f64) {
    }
}
