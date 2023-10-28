use agb::{
    display::object::{Object, SpriteVram, Graphics, include_aseprite, Sprite, OamManaged},
    display::{HEIGHT, WIDTH},
    fixnum::{FixedNum, Vector2D, num, Num}, println,
};
use alloc::vec::Vec;

//const GRAVITY: FixedNum<8> = num!(0.5);

pub struct Fruit<'a>{
    id: i32,
    pos: Vector2D<FixedNum<8>>,
    vel: Vector2D<FixedNum<8>>,
    stage: i8,
    size: i8,
    is_freefall: bool,
    sprites: &'a [SpriteVram],
    pub object: Object<'a>,
}

pub fn create_fruit<'a>(pos: Vector2D<FixedNum<8>>, oam: &'a OamManaged, sprites: &'a [SpriteVram], stage: i8, id: i32) -> Fruit<'a>{
    println!("Creating fruit!!");
    //Create oam object
    let object = oam.object(sprites[stage as usize].clone());

    let mut fruit = Fruit{
        id: id,
        pos: pos.clone(),
        vel: Vector2D::<FixedNum<8>> {x: num!(0.0), y: num!(0.0)},
        stage: stage,
        size: stage + 2,
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
        //agb::println!("y vel: {}", self.vel.y);

        //Detect Collisions
        check_wall_collisions(self);
        check_other_fruit_collisions(self, others); 
        
        //Apply velocity
        apply_velocity(self);

        //Try to merge with other fruit

        //Set oam object new position
        self.object.set_position(self.pos.trunc());
    }
}

fn update_velocity(fruit: &mut Fruit){
    let maxvel: FixedNum<8> = num!(5.0);
    fruit.vel.y += num!(0.5); //gravity because I cant do const = num!(0.5) for some reason
    //Clamp crashes so we do it manually
    if fruit.vel.y > maxvel {
        println!("Fruit Exeeded max velocity");
        fruit.vel.y = maxvel;
    }
}

fn apply_velocity(fruit: &mut Fruit){
    fruit.pos = fruit.vel + fruit.pos;
}

fn check_wall_collisions(fruit: &mut Fruit){
    //Check wall collisions, modify vel, clamp position if necessary
    if fruit.pos.x <= num!(0.0){
        fruit.vel.x = num!(0.0);
    }
    if fruit.pos.x >= (WIDTH - fruit.size as i32).into(){
        fruit.vel.x = num!(0.0);
    }
    //Remember that max height is the bottom of the screen
    if fruit.pos.y >= (HEIGHT - fruit.size as i32).into(){
        fruit.vel.y = num!(0.0);
    }
    //No need to check the top of the screen yet, that's the loss condition.    
}

fn check_other_fruit_collisions(fruit: &mut Fruit, others: &mut [Fruit]){
    //Really bad algorithm: check all other fruits to see if they're in touching distance
    for other in others{
        //Find vector pointing from other to self
        let difference_vector = fruit.pos - other.pos;

        //Move apart if they're too close. They are touching when the magnitude <= sum of radii
        let overlap = difference_vector.fast_magnitude();
        if overlap <= ((fruit.size + other.size) as i32).into(){
            //A collision has occurred
            //This needs to move away from other by the amount they are overlapping
            let move_vector = difference_vector.fast_normalise() * overlap;
            fruit.pos += move_vector;

            //Change velocity vector of both by the collision force
        }
    }
}

pub fn update_all_fruits(mut fruits: Vec<Fruit>) -> Vec<Fruit>{
    for _i in 0..fruits.len(){
        let mut fruit = fruits.remove(0);
        fruit.update(fruits.as_mut_slice());
        fruits.push(fruit);
    }

    return fruits;
}