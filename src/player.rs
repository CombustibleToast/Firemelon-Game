use agb::{
    display::object::{include_aseprite, Graphics, OamManaged, Object, Sprite, SpriteVram},
    display::{HEIGHT, WIDTH},
    fixnum::{num, FixedNum, Num, Vector2D},
    println,
};

const WALK_SPEED: i32 = 5;
const X_MIN: i32 = WIDTH/2;
const X_MAX: i32 = WIDTH - 16;

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
    let new_player = Player { 
        pos: Vector2D { x: X_MIN.into(), y: num!(20.0) }, 
        animation_state: AnimationState::Still, 
        sprites: sprites,
        frame_number: 0,
        object: oam.object(sprites[0].clone()),
        last_sprite_change: 0,
        sprite_change_rate: 5,
        flip: false
    };

    return new_player;
}

impl Player<'_>{
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
        if self.pos.x < X_MIN.into(){
            self.pos.x = X_MIN.into()
        }
    }

    fn update_animation(&mut self) {
        match self.animation_state{
            AnimationState::Still => {
                self.last_sprite_change = self.sprite_change_rate;
                self.frame_number = 0;
            }
            AnimationState::WalkLeft => {
                s
            }
        }

        

    }
}
