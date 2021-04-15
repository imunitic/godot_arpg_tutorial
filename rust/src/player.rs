use gdnative::api::*;
use gdnative::prelude::*;

/// The Player "class"
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_builder)]
pub struct Player;

#[methods]
impl Player {
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Player builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &KinematicBody2D) -> Self {
        godot_print!("Player is created!");
        Player
    }

    #[export]
    fn _ready(&self, _owner: &KinematicBody2D) {
        godot_print!("Player is ready!")
    }

    #[export]
    fn _physics_process(&self, _owner: &KinematicBody2D, _delta: f64)   {
        let input = Input::godot_singleton();
		let mut velocity = Vector2::zero();
		let mut input_vector = Vector2::zero();

		let right = input.get_action_strength("ui_right");
		let left = input.get_action_strength("ui_left");
		let down = input.get_action_strength("ui_down");
		let up = input.get_action_strength("ui_up");

		input_vector.x = (right - left) as f32;
		input_vector.y = (down - up) as f32;

		if input_vector != Vector2::zero()
		{
			velocity = input_vector;
		}

		_owner.move_and_collide(velocity, false, false, false); 
    }
}
