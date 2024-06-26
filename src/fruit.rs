use core::array::from_fn;

use agb::{
    display::object::{Object, SpriteVram, OamManaged, self, AffineMatrixInstance},
    display::affine::AffineMatrix,
    display::{HEIGHT, WIDTH},
    fixnum::{FixedNum, Vector2D, num, Num}, println,
    syscall::sqrt
};
use alloc::vec::Vec;

// const GRAVITY: FixedNum<8> = num!(0.5);
// const UNIT_VECTOR: Vector2D<FixedNum<8>> = Vector2D {x: num!(1.0), y: num!(1.0)}; // this is NOT the unit vector lmao
pub const FRUIT_GENERATION_TIME: i32 = 25;
pub struct FruitStaticInfo{
    pub fruit_affine_matricies: [AffineMatrixInstance; FRUIT_GENERATION_TIME as usize],
    pub next_fruit_id: i32,
    pub current_score: i32,
    pub previous_score: i32,
}

enum PhsyicsSimulationStateStep{
    Complete,
    FindingCollisions,
    MergingCollisions,
    ResolvingCollisions
}

struct PhsyicsSimulationState{
    step: PhsyicsSimulationStateStep,
    index: i32,
}

static PHYS: PhsyicsSimulationState = PhsyicsSimulationState {
    step: PhsyicsSimulationStateStep::Complete,
    index: 0,
};


const SPRITE_SIZE: i32 = 64;
const FRUIT_DIAMETERS: [i32; 11] = [9, 11, 15, 18, 22, 29, 32, 39, 42, 53, 64];
// const FRUIT_VALUES: [i32; 11] = [1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66]; //Idk if this is right, taken from https://gaming.stackexchange.com/questions/405265/how-does-scoring-work-in-suika-game
const FRUIT_VALUES: [i32; 11] = [1, 4, 11, 26, 57, 120, 247, 502, 1013, 2036, 4083];
pub const CONTAINER_WIDTH: i32 = 110;
const LEFT_WALL: i32 = 110; //based on background image
const RIGHT_WALL: i32 = 219; 

pub struct Fruit<'a>{
    id: i32,
    pub pos: Vector2D<FixedNum<8>>,
    vel: Vector2D<FixedNum<8>>,
    stage: i32,
    size: i32,
    is_freefall: bool,
    pub object: Object<'a>,
    popping: bool,
    generating_frames_remaining: i32,
    popping_frames_remaining: i32,
}

pub fn create_fruit<'a>(pos: Vector2D<FixedNum<8>>, oam: &'a OamManaged, sprites: &'a [SpriteVram], stage: i32, fruit_static_info: &mut FruitStaticInfo) -> Fruit<'a>{
    //Create oam object
    let object = oam.object(sprites[stage as usize].clone());

    //for testing, create a random velocity
    //let randvel: Vector2D<FixedNum<8>> = Vector2D { x: (rng::gen()%6 - 3).into(), y: (rng::gen()%6 - 3).into() };

    let mut fruit: Fruit;
    fruit = Fruit{
        id: fruit_static_info.next_fruit_id,
        pos: pos.clone(),
        vel: Vector2D::<FixedNum<8>> {x: num!(0.0), y: num!(0.0)},
        // vel: randvel,
        stage: stage,
        size: FRUIT_DIAMETERS[stage as usize],
        is_freefall: false,
        object: object,
        popping: false,
        generating_frames_remaining: FRUIT_GENERATION_TIME - 1,
        popping_frames_remaining: -1,
    };
    fruit_static_info.next_fruit_id+=1;

    //Apply initial conditions
    fruit.object.set_position(fruit.pos.trunc());
    fruit.object.show();

    return fruit;
}

impl Fruit<'_>{
    pub fn drop(&mut self) -> bool{
        if self.generating_frames_remaining > 0 {
            return false;
        }
        self.is_freefall = true;
        true
    }

    pub fn update(&mut self, fruit_static_info: &FruitStaticInfo){
        //Update popping and generating fruit sizes.
        if self.generating_frames_remaining >= 0 || self.popping_frames_remaining >= 0 {
            self.update_size(fruit_static_info);
            return;
        }
        else {
            //Turn off affine mode if unnecessary
            self.object.show();
        }

        //Don't process non-phsyic'd fruits
        if !self.is_freefall {
            return;
        }

        //Update velocity
        self.update_velocity();

        //Detect Collisions
        self.check_wall_collisions();
        //Collisions with other fruit handled in updateallfruit
        //let mut fruitCollisions : Vec<(&Fruit,&Fruit)> = check_other_fruit_collisions(self, others, &mut fruitCollisions); 
        
        //Apply velocity
        self.polish_velocity();
        self.apply_velocity();

        //Try to merge with other fruit

        //Set oam object new position
        self.set_sprite_pos();
    }

    fn update_size(&mut self, fruit_static_info: &FruitStaticInfo){
        //Scale value is the denominator, e.g. vector(2,2) will scale things by half.
        //For generating fruit, scale should be 0 when frames remaining = 0, and 10 when frames remaining = FRUIT_GENERATION_TIME
        if self.generating_frames_remaining >= 0 {
            self.object.set_affine_matrix(fruit_static_info.fruit_affine_matricies[self.generating_frames_remaining as usize].clone()); //4294967295
            self.object.show_affine(object::AffineMode::Affine);
            self.generating_frames_remaining -= 1;
            return;
        }

        //Should be like generating fruit scale but reversed
        //For popping fruit, scale should be 10 when frames remaining = 0, and 0 when frames remaining = FRUIT_GENERATION_TIME
        if self.popping_frames_remaining >= 0 {
            self.object.set_affine_matrix(fruit_static_info.fruit_affine_matricies[(FRUIT_GENERATION_TIME - self.popping_frames_remaining) as usize].clone());
            self.object.show_affine(object::AffineMode::Affine);
            self.popping_frames_remaining -= 1;
            return;
        }
    }

    fn update_velocity(&mut self){
        let maxvel: FixedNum<8> = num!(5.0);
        //Apply gravity
        self.vel.y += num!(0.1); //gravity because I cant do const = num!(0.5) for some reason
        //Clamp crashes so we do it manually
        if self.vel.y > maxvel {
            println!("Fruit {} exceeded max velocity!", self.id);
            self.vel.y = maxvel;
        }

        //Apply drag
        let drag_vector = num!(0.99);
        self.vel *= drag_vector;
    }

    fn check_wall_collisions(&mut self){
        //Calculate max x and max y values of the sprite location
        let x_min = LEFT_WALL + self.size/2 - SPRITE_SIZE/2; //negative number
        let x_max = RIGHT_WALL - SPRITE_SIZE/2 - self.size/2;
        let y_min = self.size/2 - SPRITE_SIZE/2;
        let y_max = HEIGHT - SPRITE_SIZE/2 - self.size/2;
        let restitution = num!(0.1);

        if self.pos.x <= x_min.into(){
            self.vel.x = -self.vel.x * restitution;
            self.pos.x = x_min.into();
        }
        if self.pos.x >= x_max.into(){
            self.vel.x = -self.vel.x * restitution;
            self.pos.x = x_max.into();
        }
        if self.pos.y <= y_min.into(){
            self.vel.y = -self.vel.y * restitution;
            self.pos.y = y_min.into();
        }
        //Remember that max height is the bottom of the screen
        if self.pos.y >= y_max.into(){
            self.vel.y = -self.vel.y * restitution;
            self.pos.y = y_max.into();
        }
        //No need to check the top of the screen yet, that's the loss condition.    
    }

    fn polish_velocity(&mut self){
        if self.vel.y.abs() < num!(0.09) {
            self.vel.y = num!(0.0);
        }
    }

    fn apply_velocity(&mut self){
        self.pos += self.vel;
    }

    pub fn set_sprite_pos(&mut self){
        self.object.set_position(self.pos.trunc());
    }

    fn get_phsyic_center(&self) -> Vector2D<FixedNum<8>>{
        let unit_vector: Vector2D<FixedNum<8>> = Vector2D::new(sqrt(1).into(), sqrt(1).into());
        return self.pos + unit_vector * (<i32 as Into<FixedNum<8>>>::into(SPRITE_SIZE)/num!(2.0));
    }
}

fn remove_all_popped_fruits(fruits: &mut Vec<Fruit>){
    for _i in 0..fruits.len(){
        let mut fruit = fruits.remove(0);
        if !fruit.popping || fruit.popping_frames_remaining > 0 {
            fruits.push(fruit);
        } else {
            fruit.object.hide();
        }
    }
}

fn find_all_fruit_collisions(fruits: &[Fruit]) -> Vec<(usize, usize)>{
    //Storage
    let mut collisions : Vec<(usize,usize)> = Vec::new();

    //Really bad algorithm: check all other fruits to see if they're in touching distance
    let num_fruits = fruits.len();
    for fruit_index in 0..num_fruits{
        //don't process if this is a popped fruit
        if fruits[fruit_index].popping {
            continue;
        }
        let fruit = fruits.get(fruit_index).unwrap();
        let fruit_phsyic_center = fruit.get_phsyic_center();

        for other_index in fruit_index+1..num_fruits{
            //don't collide with self or popped fruits
            if fruit_index == other_index || fruits[other_index].popping{
                continue;
            }

            let other = fruits.get(other_index).unwrap();
            let other_physic_center = other.get_phsyic_center();

            //Find vector pointing from other to fruit
            let difference_vector = fruit_phsyic_center - other_physic_center;

            //They are touching when the magnitude <= sum of radii
            let overlap = -(difference_vector.fast_magnitude() - fruit.size/2 - other.size/2);
            // if overlap > 0.into() && !collisions.contains(&(other_index, fruit_index)){
            if overlap > 0.into() {
                collisions.push((fruit_index,other_index));
                // println!("Collision between {}, {}", fruit_index, other_index);
            }
        }
    }
    return collisions;
}

fn try_merge_collisions<'a>(collisions: &mut Vec<(usize, usize)>, fruits: &mut Vec<Fruit<'a>>, oam: &'a OamManaged, sprites: &'a [SpriteVram], fruit_static_info: &mut FruitStaticInfo){
    //Each tuple in collisions is (fruit1_index, fruit2_index) experiencing a collision
    for _i in 0..collisions.len(){
        //Guard clause, if there are no collisions in the vec, break out of this loop lest we crash
        if collisions.len() == 0 {
            return;
        }

        let (fruit1_index, fruit2_index) = collisions.remove(0);
        let fruit1 = fruits.get(fruit1_index).unwrap();
        let fruit2 = fruits.get(fruit2_index).unwrap();

        //Skip if the two fruits are not the same stage or one is max size and add back the collision
        if fruit1.stage != fruit2.stage || fruit1.stage as usize == sprites.len()-1 || fruit2.stage as usize == sprites.len()-1 {
            collisions.push((fruit1_index, fruit2_index));
            continue;
        }

        //The two fruits are the same stage, merge them.
        //Create new fruit inbetween the two
        let new_fruit_pos = (fruit1.pos - fruit1.pos)/2 + fruit1.pos; // this probably isn't right, might need to get physic center and convert back
        //new_fruits.push(create_fruit(new_fruit_pos, oam, sprites, fruit1.stage + 1));
        let mut new_fruit = create_fruit(new_fruit_pos, oam, sprites, fruit1.stage + 1, fruit_static_info);
        new_fruit.is_freefall = true;
        fruits.push(new_fruit);

        //Mark the two fruits as deleted and play its disappearing animation
        pop_fruit(&fruit1_index, fruits); //TODO: these can be done with less borrow jank
        pop_fruit(&fruit2_index, fruits);
        
        //Do not add back the collision

        //Remove all collisions containing popped fruit
        //While this would be more performant if run as a separate loop, we can't wait for all merges to be checked
        //   because invalid merges may occur.
        //Not a great solution because the outer for loop doesn't update to account for the removed tuples
        for _j in 0..collisions.len(){
            let (check1, check2) = collisions.remove(0);
            //There's probably a better way to write this:
            if !(check1 == fruit1_index || check1 == fruit2_index || check2 == fruit1_index || check2 == fruit2_index) {
                //Neither fruit appeared in the tuple, so the collision is still valid. Push it back.
                collisions.push((check1,check2));
            }
        }
    }
}

fn pop_fruit(index: &usize, fruits: &mut Vec<Fruit>){
    let fruit = fruits.get_mut(*index).unwrap();
    //Mark fruit for deletion (disables collision)
    fruit.popping = true;

    //disable phsyics
    fruit.is_freefall = false;

    //start animation
    fruit.generating_frames_remaining = -1; //truncate generation animation in case it immediately merges into a new fruit; popping takes prio
    fruit.popping_frames_remaining = FRUIT_GENERATION_TIME - 1;
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

pub fn update_all_fruits<'a>(fruits: &mut Vec<Fruit<'a>>, oam: &'a OamManaged, sprites: &'a [SpriteVram], fruit_static_info: &mut FruitStaticInfo){
    remove_all_popped_fruits(fruits);
    let mut collisions = find_all_fruit_collisions(fruits.as_slice());
    try_merge_collisions(&mut collisions, fruits, oam, sprites, fruit_static_info);
    resolve_collisions(&mut collisions, fruits);

    //recalculate score
    fruit_static_info.previous_score = fruit_static_info.current_score;
    fruit_static_info.current_score = 0; //recalculate score
    for _i in 0..fruits.len(){
        let mut fruit = fruits.remove(0);
        fruit.update(&fruit_static_info);
        if !fruit.popping {
            fruit_static_info.current_score += FRUIT_VALUES[fruit.stage as usize];
        }
        fruits.push(fruit);
    }    
}

//This could very easily be a macro, I just don't want to learn macros right now
pub fn dot_product(v1: &Vector2D<FixedNum<8>>, v2: &Vector2D<FixedNum<8>>) -> FixedNum<8> {
    return v1.x * v2.x + v1.y * v2.y;
}

//There's definitely a faster algorithm for this
// pub fn pow(base: FixedNum<8>, power: i32) -> FixedNum<8>{
//     let mut product = base.clone();
//     for _i in 1..power{
//         product *= base;
//     }
//     println!("{}^{} = {}", base, power, product);
//     return product;
// }

pub fn pregenerate_affine_matricies() -> [AffineMatrixInstance; FRUIT_GENERATION_TIME as usize]{
    fn generate_matrix(i: usize) -> AffineMatrixInstance{
        let scale: FixedNum<8> = (num!(1.0) + num!(10.0) * <i32 as Into<FixedNum<8>>>::into(i as i32)/FRUIT_GENERATION_TIME).into();
        let matrix = AffineMatrix::from_scale(Vector2D { x: scale, y: scale });
        return object::AffineMatrixInstance::new(matrix.to_object_wrapping());
    }

    from_fn(generate_matrix)

    //Initialize array with dummy values
    // let scale: FixedNum<8> = (1).into();
    // let matrix = AffineMatrix::from_scale(Vector2D { x: scale, y: scale });
    // let matrix_instance = object::AffineMatrixInstance::new(matrix.to_object_wrapping());
    // let mut matricies: [AffineMatrixInstance; FRUIT_GENERATION_TIME as usize] = [matrix_instance; FRUIT_GENERATION_TIME as usize];

    // for frame_number in 0..FRUIT_GENERATION_TIME as usize {
    //     let scale: FixedNum<8> = (num!(1.0) + num!(10.0) * <i32 as Into<FixedNum<8>>>::into(frame_number as i32)/FRUIT_GENERATION_TIME).into();
    //     let matrix = AffineMatrix::from_scale(Vector2D { x: scale, y: scale });
    //     let matrix_instance = object::AffineMatrixInstance::new(matrix.to_object_wrapping());
    //     matricies[frame_number] = matrix_instance;
    // }
    // return matricies;
}