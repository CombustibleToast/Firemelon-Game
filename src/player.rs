use agb::{
    display::object::{include_aseprite, Graphics, OamManaged, Object, Sprite, SpriteVram},
    display::{HEIGHT, WIDTH},
    fixnum::{num, FixedNum, Num, Vector2D},
    println,
};
use alloc::vec::Vec;

const WALK_SPEED: i32 = 2;
const X_MIN: i32 = WIDTH/2;
const X_MAX: i32 = WIDTH - 32;
const WALK_SEQUENCE: [usize; 4] = [0,1,0,2];


#[derive(PartialEq)]
enum AnimationState {
    Still,
    WalkLeft,
    WalkRight,
}

pub struct Player<'a> {
    pos: Vector2D<FixedNum<8>>,
    animation_state: AnimationState,
    sprites: &'a [SpriteVram],
    frame_number: usize,
    last_sprite_change: i32,
    sprite_change_rate: i32,
    flip: bool,
    object: Object<'a>,
}

pub fn create_player<'a>(sprites: &'a [SpriteVram], oam: &'a OamManaged) -> Player<'a> {
    let mut new_player = Player { 
        pos: Vector2D { x: X_MIN.into(), y: num!(20.0) }, 
        animation_state: AnimationState::Still, 
        sprites: sprites,
        frame_number: 0,
        object: oam.object(sprites[0].clone()),
        last_sprite_change: 0,
        sprite_change_rate: 5,
        flip: false
    };
    new_player.object.show();
    return new_player;
}

impl Player<'_>{
    pub fn get_hold_vector(&self) -> Vector2D<FixedNum<8>> {
        return self.pos - Vector2D { x: num!(32.0), y: num!(8.0) };
    }

    pub fn walk_left(&mut self) {
        self.pos.x -= WALK_SPEED;
        self.animation_state = AnimationState::WalkLeft;
        if self.pos.x < X_MIN.into(){
            self.pos.x = X_MIN.into()
        }
    }

    pub fn walk_right(&mut self) {
        self.pos.x += WALK_SPEED;
        self.animation_state = AnimationState::WalkRight;
        if self.pos.x > X_MAX.into(){
            self.pos.x = X_MAX.into()
        }
    }

    pub fn stop_walk(&mut self) {
        self.animation_state = AnimationState::Still;
    }

    pub fn update_animation(&mut self) {
        match self.animation_state{
            AnimationState::Still => {
                self.last_sprite_change = self.sprite_change_rate-1;
                self.frame_number = 0;
            }
            AnimationState::WalkLeft => {
                self.last_sprite_change += 1;
                self.flip = true;
            }
            AnimationState::WalkRight => {
                self.last_sprite_change += 1;
                self.flip = false;
            }
        }
        
        if self.last_sprite_change >= self.sprite_change_rate{
            self.last_sprite_change = 0;
            self.frame_number = (self.frame_number + 1) % WALK_SEQUENCE.len();
            self.object.set_sprite(self.sprites[WALK_SEQUENCE[self.frame_number]].clone());
        }

        self.object.set_hflip(self.flip);

        self.object.set_position(self.pos.trunc());
    }
}
