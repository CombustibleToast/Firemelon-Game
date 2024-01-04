use core::{fmt::Write, array::from_fn};

use agb::{
    display::object::{Object, Size, SpriteVram, Graphics, Sprite, OamManaged},
    println, include_aseprite, fixnum::{Vector2D, FixedNum}
};

const NUMBER_SPRITESHEET: &Graphics = include_aseprite!("graphics/Numbers.aseprite");
const NUMBER_SPRITELIST: &[Sprite] = NUMBER_SPRITESHEET.sprites();

const DIGIT_AMOUNT: usize = 5;
const DIGIT_SPREAD_PX: i32 = 2;
const DIGIT_SPRITE_WIDTH: i32 = 8;

pub struct Writer<'a>{
    pos: Vector2D<i32>,
    digit_objects: [Object<'a>; DIGIT_AMOUNT],
    sprites: [SpriteVram; 10]
}

pub fn create_writer<'a>(pos: Vector2D<i32>, oam: &'a OamManaged) -> Writer<'a>{
    //Load sprites
    //There's gotta be a better way to do this, but from_fn doesn't work because I can't pass in oam
    let loaded_sprites: [SpriteVram; 10] = [ 
        oam.sprite(&NUMBER_SPRITELIST[0]),
        oam.sprite(&NUMBER_SPRITELIST[1]),
        oam.sprite(&NUMBER_SPRITELIST[2]),
        oam.sprite(&NUMBER_SPRITELIST[3]),
        oam.sprite(&NUMBER_SPRITELIST[4]),
        oam.sprite(&NUMBER_SPRITELIST[5]),
        oam.sprite(&NUMBER_SPRITELIST[6]),
        oam.sprite(&NUMBER_SPRITELIST[7]),
        oam.sprite(&NUMBER_SPRITELIST[8]),
        oam.sprite(&NUMBER_SPRITELIST[9]),
    ];

    //Create objects
    //Extra jank because I need to update this when I update digit_amount
    let objects: [Object<'a>; DIGIT_AMOUNT] = [
        oam.object(loaded_sprites[0].clone()),
        oam.object(loaded_sprites[0].clone()),
        oam.object(loaded_sprites[0].clone()),
        oam.object(loaded_sprites[0].clone()),
        oam.object(loaded_sprites[0].clone()),
    ];

    let mut new_writer = Writer{
        pos: pos,
        digit_objects: objects,
        sprites: loaded_sprites,
    };

    //Bake digit positions
    new_writer.set_position(pos);

    //Display digits
    new_writer.show();

    return new_writer;
}

impl Writer<'_>{
    pub fn write_new_score(&mut self, new_score: &i32){
        let mut working_copy = new_score.clone() as usize;
        for i in 0..DIGIT_AMOUNT{
            //Extract digit and use that as an index to the spritelist
            self.digit_objects[DIGIT_AMOUNT - i - 1].set_sprite(self.sprites[working_copy%10].clone());
            working_copy /= 10;
        }
    }

    pub fn set_position(&mut self, new_position: Vector2D<i32>){
        self.pos = new_position;
        for i in 0..DIGIT_AMOUNT{
            let pos = Vector2D{x: (self.pos.x + (i as i32 * DIGIT_SPREAD_PX) + (i as i32 * DIGIT_SPRITE_WIDTH)), y: self.pos.y};
            self.digit_objects[i].set_position(pos);
        }
    }

    pub fn show(&mut self){
        for i in 0..DIGIT_AMOUNT{
            self.digit_objects[i].show();
        }
    }

    pub fn hide(&mut self){
        for i in 0..DIGIT_AMOUNT{
            self.digit_objects[i].hide();
        }
    }
}