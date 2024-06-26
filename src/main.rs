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
    display::{object::{SpriteVram, Graphics, include_aseprite, Sprite, OamUnmanaged, SpriteLoader}, window, tiled::{RegularBackgroundSize, TileFormat, TiledMap}},
    fixnum::{FixedNum, Vector2D, num}, input::Button,
    rng::gen, 
    sound::mixer::Frequency, println, include_background_gfx,
    // include_font
};
use fruit::{create_fruit, Fruit, update_all_fruits, FruitStaticInfo, pregenerate_affine_matricies};
use player::create_player;
use alloc::vec::Vec;
use score_writer::{Writer, create_writer};
use sounds::start_bgm;

static FRUIT_SPRITESHEET: &Graphics = include_aseprite!("graphics/Fruits.ase");
static FRUIT_SPRITELIST: &[Sprite] = FRUIT_SPRITESHEET.sprites();

static GUP_SPRITESHEET: &Graphics = include_aseprite!("graphics/gup.ase");
static GUP_SPRITELIST: &[Sprite] = GUP_SPRITESHEET.sprites();

include_background_gfx!(background_sprite, tiles_source => "graphics/Background.aseprite");

mod fruit;
mod player;
mod sounds;
mod score_writer;
mod test;

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // crate::test::window(gba);

    //Get OAM, VBlank, and inputs
    let oam = gba.display.object.get_managed();
    let (gfx, mut vram) = gba.display.video.tiled0();
    let vblank = agb::interrupt::VBlank::get();
    let mut input = agb::input::ButtonController::new();

    //Set up background
    let background_tileset = &background_sprite::tiles_source.tiles;
    vram.set_background_palettes(background_sprite::PALETTES);
    let mut background_map = gfx.background(
        agb::display::Priority::P0,
        RegularBackgroundSize::Background32x32,
        background_tileset.format(),
    );
    background_map.fill_with(&mut vram, &background_sprite::tiles_source);
    background_map.commit(&mut vram);
    background_map.set_visible(true);

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

    //Create fruit static/global info struct
    let mut fruit_static_info = FruitStaticInfo{
        next_fruit_id: 0, 
        fruit_affine_matricies: pregenerate_affine_matricies(),
        current_score: 0,
        previous_score: -1
    };

    //Create fruit object storage
    let mut fruit_objects: Vec<fruit::Fruit> = Vec::new();
    //Bootstrap fruit engine lol
    let initial_pos: Vector2D<FixedNum<8>> = Vector2D::new(num!(50.0), num!(50.0));
    let mut held_fruit: Fruit = create_fruit(initial_pos, &oam, fruit_sprites.as_slice(), (gen()%4).abs(), &mut fruit_static_info);

    //Create player/gup
    let mut player = create_player(gup_sprites.as_slice(), &oam);

    //Create music stuff
    let mut sounds = start_bgm(gba.mixer.mixer(Frequency::Hz10512));

    //Create score writer
    let mut score_writer = create_writer(Vector2D{x:0, y:0}, &oam);

    //Create performance timer
    // let mut timer = gba.timers.timers().timer2;
    // timer.set_divider(agb::timer::Divider::Divider256);
    // timer.set_enabled(true);
    
    //Core Loop
    loop {
        //Start debug timer
        // let start_time = timer.value();

        //Collect player input
        if input.is_pressed(Button::LEFT){
            player.walk_left();
        } else if input.is_pressed(Button::RIGHT){
            player.walk_right();
        } else {
            player.stop_walk();
        }

        if input.is_just_pressed(Button::B) {
            sounds.play_random_song();
        }
        if input.is_just_pressed(Button::SELECT) {
            sounds.mute();
        }

        //Update player sprite
        player.update_animation();

        //Update held fruit position
        held_fruit.pos = player.get_hold_vector();
        held_fruit.set_sprite_pos();
        
        if input.is_just_pressed(Button::A) && held_fruit.drop() {
            //Drop Fruit and move it to the vec
            fruit_objects.push(held_fruit);

            //Fruit was just dropped, create new fruit
            let initial_pos = player.get_hold_vector();
            //held_fruit = create_fruit(initial_pos, &oam, fruit_sprites.as_slice(), (fruit_objects.len() as i32)%11);
            held_fruit = create_fruit(initial_pos, &oam, fruit_sprites.as_slice(), (gen()%4).abs(), &mut fruit_static_info);
            held_fruit.object.show();
        }

        //update all fruits
        update_all_fruits(&mut fruit_objects, &oam, fruit_sprites.as_slice(), &mut fruit_static_info);
        held_fruit.update(&fruit_static_info);

        //Write new score
        if fruit_static_info.current_score != fruit_static_info.previous_score {
            println!("Score: {}", fruit_static_info.current_score);
            score_writer.write_new_score(&fruit_static_info.current_score);
        }


        //Collect timer and print
        // let end_time = timer.value();
        // println!("Update took {}", end_time.wrapping_sub(start_time));

        //Commit objects, wait for vblank, update inputs, mixer computer
        let _ = gen();
        oam.commit();
        input.update();
        sounds.frame();
        vblank.wait_for_vblank();
    }
}

