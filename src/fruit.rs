use core::borrow::BorrowMut;

use agb::{
    display::object::{Object, SpriteVram, Graphics, include_aseprite, Sprite, OamManaged},
    display::{HEIGHT, WIDTH},
    fixnum::{FixedNum, Vector2D, num, Num}, println,
    rng,
};
use alloc::{vec::Vec, slice};

// const GRAVITY: FixedNum<8> = num!(0.5);
// const UNIT_VECTOR: Vector2D<FixedNum<8>> = Vector2D {x: num!(1.0), y: num!(1.0)};
const SPRITE_SIZE: i32 = 8;

pub struct Fruit<'a>{
    id: i32,
    pos: Vector2D<FixedNum<8>>,
    vel: Vector2D<FixedNum<8>>,
    stage: i32,
    size: i32,
    is_freefall: bool,
    sprites: &'a [SpriteVram],
    pub object: Object<'a>,
}

pub fn create_fruit<'a>(pos: Vector2D<FixedNum<8>>, oam: &'a OamManaged, sprites: &'a [SpriteVram], stage: i32, id: i32) -> Fruit<'a>{
    println!("Creating fruit!!");
    //Create oam object
    let object = oam.object(sprites[stage as usize].clone());

    //for testing, create a random velocity
    let randvel: Vector2D<FixedNum<8>> = Vector2D { x: (rng::gen()%6 - 3).into(), y: (rng::gen()%6 - 3).into() };
    println!("generated a random vel {}, {}", randvel.x, randvel.y);

    let mut fruit = Fruit{
        id: id,
        pos: pos.clone(),
        //vel: Vector2D::<FixedNum<8>> {x: num!(0.0), y: num!(0.0)},
        vel: randvel,
        stage: stage,
        size: stage + 3,
        is_freefall: false,
        sprites: sprites,
        object: object
    };

    //Apply initial conditions
    fruit.object.set_position(fruit.pos.trunc());
    fruit.object.show();

    println!("Initial pos: {}, {}", fruit.pos.x, fruit.pos.y);
    return fruit;
}

impl Fruit<'_>{
    pub fn drop(&mut self){
        self.is_freefall = true;
    }

    pub fn update(&mut self, others: &mut [Fruit]){
        if self.id == 1 {
            println!("Updating fruit");
            println!("Pos: {}, {}", self.pos.x, self.pos.y);
            println!("Vel: {}, {}", self.vel.x, self.vel.y);
        }
        //Update velocity
        update_velocity(self);

        //Detect Collisions
        check_wall_collisions(self);
        //Collisions with other fruit handled in updateallfruit
        //let mut fruitCollisions : Vec<(&Fruit,&Fruit)> = check_other_fruit_collisions(self, others, &mut fruitCollisions); 
        
        //Apply velocity
        polish_velocity(self);
        apply_velocity(self);

        //Try to merge with other fruit

        //Set oam object new position
        self.object.set_position(self.pos.trunc());
    }
}

fn update_velocity(fruit: &mut Fruit){
    let maxvel: FixedNum<8> = num!(5.0);
    //Apply gravity
    fruit.vel.y += num!(0.1); //gravity because I cant do const = num!(0.5) for some reason
    //Clamp crashes so we do it manually
    if fruit.vel.y > maxvel {
        println!("Fruit {} exceeded max velocity!", fruit.id);
        fruit.vel.y = maxvel;
    }

    //Apply drag
    let drag_vector = num!(0.99);
    fruit.vel *= drag_vector;
}

fn polish_velocity(fruit: &mut Fruit){
    if fruit.vel.y < num!(0.05) {
        fruit.vel.y = num!(0.0);
    }
}

fn apply_velocity(fruit: &mut Fruit){
    fruit.pos = fruit.vel + fruit.pos;
}

fn check_wall_collisions(fruit: &mut Fruit){
    //Check wall collisions, modify vel, clamp position if necessary
    let x_min = (0 - SPRITE_SIZE as i32).into();
    let x_max = (WIDTH - SPRITE_SIZE - fruit.size as i32).into();
    let y_min = (0 - SPRITE_SIZE as i32).into();
    let y_max = (HEIGHT - SPRITE_SIZE - fruit.size as i32).into();
    let restitution = num!(0.5);

    if fruit.pos.x <= x_min {
        fruit.vel.x = -fruit.vel.x * restitution;
        fruit.pos.x = x_min;
    }
    if fruit.pos.x >= x_max{
        fruit.vel.x = -fruit.vel.x * restitution;
        fruit.pos.x = x_max;
    }
    if fruit.pos.y <= y_min{
        fruit.vel.y = -fruit.vel.y * restitution;
        fruit.pos.y = y_min;
    }
    //Remember that max height is the bottom of the screen
    if fruit.pos.y >= y_max{
        fruit.vel.y = -fruit.vel.y * restitution;
        fruit.pos.y = y_max;
    }
    //No need to check the top of the screen yet, that's the loss condition.    
}

fn try_merge_fruits(fruit: &mut Fruit, other: &mut Fruit, all: &mut [Fruit]) -> bool{
    if fruit.stage != other.stage {
        return false;
    }

    return true;
}

fn find_all_fruit_collisions(fruits: &Vec<Fruit>) -> Vec<(usize, usize)>{
    //Storage
    let mut collisions : Vec<(usize,usize)> = Vec::new();
    let unit_vector: Vector2D<FixedNum<8>> = Vector2D {x: num!(1.0), y: num!(1.0)};

    //Really bad algorithm: check all other fruits to see if they're in touching distance
    let num_fruits = fruits.len();
    for fruit_index in 0..num_fruits{
        let fruit = fruits.get(fruit_index).unwrap();
        let fruit_phsyic_center: Vector2D<FixedNum<8>> = fruit.pos + unit_vector * (<i32 as Into<FixedNum<8>>>::into(SPRITE_SIZE)/num!(2.0));

        for other_index in 0..num_fruits{
            //don't collide with self
            if fruit_index == other_index{
                continue;
            }

            let other = fruits.get(other_index).unwrap();
            let other_physic_center: Vector2D<FixedNum<8>> = other.pos + unit_vector * (<i32 as Into<FixedNum<8>>>::into(SPRITE_SIZE)/num!(2.0));

            //Find vector pointing from other to fruit
            let difference_vector = fruit_phsyic_center - other_physic_center;

            //They are touching when the magnitude <= sum of radii
            let overlap = -(difference_vector.fast_magnitude() - fruit.size/2 - other.size/2);
            if overlap > 0.into(){
                collisions.push((fruit_index,other_index));
            }
        }
    }
    return collisions;
}

fn try_merge_collisions(collisions: &mut Vec<(usize, usize)>, fruits: &mut Vec<Fruit>){
    //Each tuple in collisions is (fruit1_index, fruit2_index) experiencing a collision
    for (fruit1_index, fruit2_index) in collisions{
        let fruit1 = fruits.get(*fruit1_index).unwrap();
        let fruit2 = fruits.get(*fruit2_index).unwrap();

        //Skip if the two fruits are not the same stage
        if fruit1.stage != fruit2.stage {
            continue;
        }

        //The two fruits are the same stage, merge them.
        //Create new fruit inbetween the two
        let new_fruit_pos = (fruit1.pos - fruit1.pos)/2 + fruit1.pos; // this probably isn't right, might need to get physic center and convert back
        let oam = gba.display.object.get_managed();
        
        // create_fruit(pos, oam, sprites, stage, id);

        //Mark the two fruits as deleted and play its disappearing animation
        // pop_fruit(fruit1_index, fruits);
        // pop_fruit(fruit2_index, fruits);

        //Remove the collision from the collisions list and the fruits from the fruits list

    }
}

pub fn update_all_fruits(mut fruits: Vec<Fruit>) -> Vec<Fruit>{
    let mut collisions = find_all_fruit_collisions(&fruits);
    try_merge_collisions(&mut collisions, &mut fruits);

    for _i in 0..fruits.len(){
        let mut fruit = fruits.remove(0);
        fruit.update(fruits.as_mut_slice());
        fruits.push(fruit);
    }

    return fruits;
}

/* Static collision alg
        let fruit_phsyic_center: Vector2D<FixedNum<8>> = fruit.pos + unit_vector * (<i32 as Into<FixedNum<8>>>::into(SPRITE_SIZE)/num!(2.0));
            let other_physic_center: Vector2D<FixedNum<8>> = other.pos + unit_vector * (<i32 as Into<FixedNum<8>>>::into(SPRITE_SIZE)/num!(2.0));
            let difference_vector = fruit_phsyic_center - other_physic_center;
            let overlap = -(difference_vector.fast_magnitude() - fruit.size/2 - other.size/2);
if overlap > 0.into() {
                //A collision has occurred
                //Add it to the collisions Vec for dynamic processing later
                collisions.push((fruits.get_mut(fruit_index).unwrap(), fruits.get_mut(other_index).unwrap()));

                //The one with the lowest y pos needs to move away from other by the amount they are overlapping (push higher one up)
                let move_vector = difference_vector.fast_normalise() * overlap;
                if fruit.pos.y < other.pos.y {
                    fruit.pos += move_vector;
                }
                else {
                    other.pos -= move_vector; //if the other one needs to move, it should be in the other direction
                }

                //Change velocity vector of both by the collision force
                //for now just move in opposite directions with restitution, not accurate though
                fruit.vel = difference_vector.fast_normalise() * fruit.vel.fast_magnitude() * num!(0.5);
                other.vel = difference_vector.fast_normalise() * other.vel.fast_magnitude() * num!(0.5) * -1;
            }   
*/