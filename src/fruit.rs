use agb::{
    display::object::{Object, SpriteVram, Graphics, include_aseprite, Sprite, OamManaged},
    display::{HEIGHT, WIDTH},
    fixnum::{FixedNum, Vector2D, num, Num}, println,
    rng, syscall::sqrt,
};
use alloc::{vec::Vec, slice};

// const GRAVITY: FixedNum<8> = num!(0.5);
// const UNIT_VECTOR: Vector2D<FixedNum<8>> = Vector2D {x: num!(1.0), y: num!(1.0)}; // this is NOT the unit vector lmao
const SPRITE_SIZE: i32 = 8;

static mut NEXT_FRUIT_ID: i32 = 0;

pub struct Fruit<'a>{
    id: i32,
    pos: Vector2D<FixedNum<8>>,
    vel: Vector2D<FixedNum<8>>,
    stage: i32,
    size: i32,
    is_freefall: bool,
    sprites: &'a [SpriteVram],
    pub object: Object<'a>,
    popping: bool
}

pub fn create_fruit<'a>(pos: Vector2D<FixedNum<8>>, oam: &'a OamManaged, sprites: &'a [SpriteVram], stage: i32) -> Fruit<'a>{
    println!("Creating fruit!!");
    //Create oam object
    let object = oam.object(sprites[stage as usize].clone());

    //for testing, create a random velocity
    let randvel: Vector2D<FixedNum<8>> = Vector2D { x: (rng::gen()%6 - 3).into(), y: (rng::gen()%6 - 3).into() };
    println!("generated a random vel {}, {}", randvel.x, randvel.y);

    let mut fruit: Fruit;
    unsafe { //unfortunately necessary for using mutable static NEXT_FRUIT_ID. Would be good to change in the future
        fruit = Fruit{
            id: NEXT_FRUIT_ID.clone(),
            pos: pos.clone(),
            //vel: Vector2D::<FixedNum<8>> {x: num!(0.0), y: num!(0.0)},
            vel: randvel,
            stage: stage,
            size: stage + 3,
            is_freefall: false,
            sprites: sprites,
            object: object,
            popping: false
        };
        NEXT_FRUIT_ID += 1;
    }

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
        // if self.id == 1 {
        //     println!("Updating fruit");
        //     println!("Pos: {}, {}", self.pos.x, self.pos.y);
        //     println!("Vel: {}, {}", self.vel.x, self.vel.y);
        // }
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

fn find_all_fruit_collisions(fruits: &[Fruit]) -> Vec<(usize, usize)>{
    //Storage
    let mut collisions : Vec<(usize,usize)> = Vec::new();
    let unit_vector: Vector2D<FixedNum<8>> = Vector2D {x: num!(1.0), y: num!(1.0)};

    //Really bad algorithm: check all other fruits to see if they're in touching distance
    let num_fruits = fruits.len();
    for fruit_index in 0..num_fruits{
        //don't process if this is a popped fruit
        if fruits[fruit_index].popping {
            continue;
        }
        let fruit = fruits.get(fruit_index).unwrap();
        let fruit_phsyic_center: Vector2D<FixedNum<8>> = fruit.pos + unit_vector * (<i32 as Into<FixedNum<8>>>::into(SPRITE_SIZE)/num!(2.0));

        for other_index in 0..num_fruits{
            //don't collide with self or popped fruits
            if fruit_index == other_index || fruits[other_index].popping{
                continue;
            }

            let other = fruits.get(other_index).unwrap();
            let other_physic_center: Vector2D<FixedNum<8>> = other.pos + unit_vector * (<i32 as Into<FixedNum<8>>>::into(SPRITE_SIZE)/num!(2.0));

            //Find vector pointing from other to fruit
            let difference_vector = fruit_phsyic_center - other_physic_center;

            //They are touching when the magnitude <= sum of radii
            let overlap = -(difference_vector.fast_magnitude() - fruit.size/2 - other.size/2);
            if overlap > 0.into() && !collisions.contains(&(other_index, fruit_index)){
                collisions.push((fruit_index,other_index));
                println!("Collision between {}, {}", fruit_index, other_index);
            }
        }
    }
    return collisions;
}

fn try_merge_collisions<'a,'b>(collisions: &mut Vec<(usize, usize)>, fruits: &'b mut Vec<Fruit<'a>>, oam: &'a OamManaged, sprites: &'a [SpriteVram]){
    //Each tuple in collisions is (fruit1_index, fruit2_index) experiencing a collision
    for collision_index in 0..collisions.len(){
        let (fruit1_index, fruit2_index) = collisions.get(collision_index).unwrap();
        let fruit1 = fruits.get(*fruit1_index).unwrap();
        let fruit2 = fruits.get(*fruit2_index).unwrap();

        //Skip if the two fruits are not the same stage
        if fruit1.stage != fruit2.stage {
            continue;
        }

        //The two fruits are the same stage, merge them.
        //Create new fruit inbetween the two
        let new_fruit_pos = (fruit1.pos - fruit1.pos)/2 + fruit1.pos; // this probably isn't right, might need to get physic center and convert back
        //new_fruits.push(create_fruit(new_fruit_pos, oam, sprites, fruit1.stage + 1));
        fruits.push(create_fruit(new_fruit_pos, oam, sprites, fruit1.stage + 1));

        //Mark the two fruits as deleted and play its disappearing animation
        pop_fruit(fruit1_index, fruits); //TODO: these can be done with less borrow jank
        pop_fruit(fruit2_index, fruits);
        
        //Remove the collision from the collisions list
        collisions.remove(collision_index);
    }
}

fn pop_fruit(index: &usize, fruits: &mut Vec<Fruit>){
    let fruit = fruits.get_mut(*index).unwrap();
    //Mark fruit for deletion (disables collision)
    fruit.popping = true;

    //disable phsyics
    fruit.is_freefall = false;

    //start animation
    //TODO: implement animation
    //For now, just hide the fruit
    fruit.object.hide();
}

fn resolve_collisions(collisions: &mut Vec<(usize, usize)>, fruits: &mut [Fruit]){
        let unit_vector: Vector2D<FixedNum<8>> = Vector2D::new(sqrt(1).into(), sqrt(1).into());
        for (fruit1_index, fruit2_index) in collisions{
            //Resolve static collision
            let move_vector: Vector2D<Num<i32, 8>>;
            { //This scope is necessary to isolate the immutable fruit borrows to get mutable borrows later.
                let fruit1 = &fruits[*fruit1_index]; 
                let fruit2 = &fruits[*fruit2_index]; //Regular borrows to do calculations. Mutable borrows happen when mutation happens
                let fruit_phsyic_center: Vector2D<FixedNum<8>> = fruit1.pos + unit_vector * (<i32 as Into<FixedNum<8>>>::into(SPRITE_SIZE)/num!(2.0));
                let other_physic_center: Vector2D<FixedNum<8>> = fruit2.pos + unit_vector * (<i32 as Into<FixedNum<8>>>::into(SPRITE_SIZE)/num!(2.0));
                let difference_vector = fruit_phsyic_center - other_physic_center;
                let overlap = -(difference_vector.fast_magnitude() - fruit1.size/2 - fruit2.size/2);
                move_vector = difference_vector.fast_normalise() * overlap;
            }
        
            //The one with the lowest y pos needs to move away from other by the amount they are overlapping (push higher one up)
            fruits[*fruit1_index].pos += move_vector/2;
            fruits[*fruit2_index].pos -= move_vector/2; //if the other one needs to move, it should be in the other direction

            //Resolve dynamic collision
            let new_v1: Vector2D<FixedNum<8>>;
            let new_v2: Vector2D<FixedNum<8>>;
            {
                let fruit1 = &fruits[*fruit1_index]; 
                let fruit2 = &fruits[*fruit2_index];
                let normal_vector = (fruit2.pos - fruit1.pos).normalise();
                let tangent_vector = Vector2D::new(-normal_vector.y, normal_vector.x);
                let dot_tan1 = dot_product(&fruit1.vel, &tangent_vector);
                let dot_tan2 = dot_product(&fruit2.vel, &tangent_vector);
                let dot_norm1 = dot_product(&fruit1.vel, &normal_vector);
                let dot_norm2 = dot_product(&fruit2.vel, &normal_vector);
                let mass1: Num<i32, 8> = num!(1.333333333) * num!(3.14159265359) * fruit1.size * fruit1.size * fruit1.size;
                let mass2: Num<i32, 8> = num!(1.333333333) * num!(3.14159265359) * fruit2.size * fruit2.size * fruit2.size;
                let momentum1 = (dot_norm1 * (mass1 - mass2) + num!(2.0) * mass2 * dot_norm2) / (mass1 + mass2);
                let momentum2 = (dot_norm2 * (mass2 - mass1) + num!(2.0) * mass1 * dot_norm1) / (mass1 + mass2);

                new_v1 = Vector2D::new(
                    tangent_vector.x * dot_tan1 + normal_vector.x * momentum1, 
                    tangent_vector.y * dot_tan1 + normal_vector.y * momentum1);
                new_v2 = Vector2D::new(
                    tangent_vector.x * dot_tan2 + normal_vector.x * momentum2, 
                    tangent_vector.y * dot_tan2 + normal_vector.y * momentum2);
            }

            fruits[*fruit1_index].vel = new_v1;
            fruits[*fruit2_index].vel = new_v2;
    }
}


pub fn update_all_fruits<'a>(fruits: &'a mut Vec<Fruit>, oam: &'a OamManaged, sprites: &'a [SpriteVram]){
    let mut collisions = find_all_fruit_collisions(fruits.as_slice());
    try_merge_collisions(&mut collisions, fruits, oam, sprites);
    resolve_collisions(&mut collisions, fruits);

    for _i in 0..fruits.len(){
        let mut fruit = fruits.remove(0);
        fruit.update(fruits.as_mut_slice());
        fruits.push(fruit);
    }
}

//This could very easily be a macro, I just don't want to learn macros right now
pub fn dot_product(v1: &Vector2D<FixedNum<8>>, v2: &Vector2D<FixedNum<8>>) -> FixedNum<8> {
    return v1.x * v2.x + v1.y * v2.y;
}
