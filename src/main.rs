// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
extern crate alloc;

use agb::{
    display::object::{SpriteVram, Graphics, include_aseprite, Sprite},
    fixnum::{FixedNum, Vector2D, num}, input::Button,
    rng::gen
};
use fruit::{create_fruit, Fruit, update_all_fruits};
use player::create_player;
use alloc::vec::Vec;

const FRUIT_SPRITESHEET: &Graphics = include_aseprite!("graphics/Fruits.ase");
const FRUIT_SPRITELIST: &[Sprite] = FRUIT_SPRITESHEET.sprites();

const GUP_SPRITESHEET: &Graphics = include_aseprite!("graphics/gup.ase");
const GUP_SPRITELIST: &[Sprite] = GUP_SPRITESHEET.sprites();

mod fruit;
mod player;

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    //Get OAM, VBlank, and inputs
    let oam = gba.display.object.get_managed();
    let vblank = agb::interrupt::VBlank::get();
    let mut input = agb::input::ButtonController::new();

    //Load Fruit sprites
    //let fruit_sprites: [SpriteVram; FRUIT_SPRITELIST.len()] = [oam.sprite(&FRUIT_SPRITELIST[i]); FRUIT_SPRITELIST.len()];
    let mut fruit_sprites: Vec<SpriteVram> = Vec::new();
    for sprite in FRUIT_SPRITELIST {
        fruit_sprites.push(oam.sprite(sprite));
    }

    //Load Gup/player sprites
    let mut gup_sprites: Vec<SpriteVram> = Vec::new();
    for sprite in GUP_SPRITELIST {
        gup_sprites.push(oam.sprite(sprite));
    }

    //Create fruit object storage
    let mut fruit_objects: Vec<fruit::Fruit> = Vec::new();
    //Bootstrap fruit engine lol
    let initial_pos: Vector2D<FixedNum<8>> = Vector2D::new(num!(50.0), num!(50.0));
    let mut held_fruit: Fruit = create_fruit(initial_pos, &oam, fruit_sprites.as_slice(), (gen()%4).abs());

    //Create player/gup
    let mut player = create_player(gup_sprites.as_slice(), &oam);
    
    //Core Loop
    loop {
        //Collect player input
        if input.is_pressed(Button::LEFT){
            player.walk_left();
        } else if input.is_pressed(Button::RIGHT){
            player.walk_right();
        } else {
            player.stop_walk();
        }

        //Update player sprite
        player.update_animation();

        //Update held fruit position
        held_fruit.pos = player.get_hold_vector();
        held_fruit.set_sprite_pos();
        
        if input.is_just_pressed(Button::A){
            //Drop Fruit and move it to the vec
            held_fruit.drop();
            fruit_objects.push(held_fruit);

            //Fruit was just dropped, create new fruit
            let initial_pos = player.get_hold_vector();
            //held_fruit = create_fruit(initial_pos, &oam, fruit_sprites.as_slice(), (fruit_objects.len() as i32)%11);
            held_fruit = create_fruit(initial_pos, &oam, fruit_sprites.as_slice(), (gen()%4).abs());
            held_fruit.object.show();
        }

        //update all fruits
        update_all_fruits(&mut fruit_objects, &oam, fruit_sprites.as_slice());

        //Commit objects, wait for vblank, update inputs
        oam.commit();
        vblank.wait_for_vblank();
        input.update();
    }
}
