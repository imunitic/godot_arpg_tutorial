use gdnative::api::*;
use gdnative::prelude::*;

/// The Player "class"
#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_builder)]
pub struct Player {
    velocity: Vector2,
}

#[methods]
impl Player {
    const FRICTION: f32 = 500.0;
    const ACCELERATION: f32 = 500.0;
    const MAX_SPEED: f32 = 80.0;

    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Player builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &KinematicBody2D) -> Self {
        godot_print!("Player is created!");
        Player {
            velocity: Vector2::zero(),
        }
    }

    #[export]
    fn _ready(&self, _owner: &KinematicBody2D) {
        godot_print!("Player is ready!")
    }

    #[export]
    fn _physics_process(&mut self, _owner: &KinematicBody2D, _delta: f64) {
        let input = Input::godot_singleton();
        let mut input_vector = Vector2::zero();

        let right = input.get_action_strength("ui_right");
        let left = input.get_action_strength("ui_left");
        let down = input.get_action_strength("ui_down");
        let up = input.get_action_strength("ui_up");

        input_vector.x = (right - left) as f32;
        input_vector.y = (down - up) as f32;

        if input_vector != Vector2::zero() {
            self.velocity = self.velocity.move_towards(
                input_vector * Player::MAX_SPEED,
                Player::ACCELERATION * _delta as f32,
            );
        } else {
            self.velocity = self
                .velocity
                .move_towards(Vector2::zero(), Player::FRICTION * _delta as f32);
        }

        self.velocity =
            _owner.move_and_slide(self.velocity, Vector2::zero(), false, 4, 0.785398, true);
    }
}
