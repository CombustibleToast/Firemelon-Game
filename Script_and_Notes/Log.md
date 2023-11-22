Started this log around day 3 so the previous days may be inaccurate.

## 0

Installed the crate by looking at documentation. Cloned the [template](https://github.com/agbrs/template) into my own repository.

Looked at the linked function agb::no_game() to get a clue on what to do but I had no idea what I was looking at.

Gave up for the day

## 1

Booted it up again and searched around more.

Decided that I should get some Rust fundamentals down so I did some of Rust by example. I think up to borrowing. I tried to cross reference some of the things the compiler was telling me about the program with some parts of Rust by example, but I couldn’t make any sense of it.

One of the best things about programming is that very often someone else has walked where you have and had the same problem as you. This is not the case for an obscure package that nobody has ever used before.

I found a guide! It’s incredibly short and unfinished but there’s a guide that gets you started with a very basic game. It has you import a sprite, draw that sprite to the screen, then attach button controls to it. Incredible

Something I really enjoy is that it directly imports from an aseprite file, all done by a rust macro!() If the file is incompatible, it will let you know at compiletime, which is incredible. I found this out when I made a gup sprite and found out that there could only be specific sizes of sprites. The size I had was wrong and rust_analyzer immediately told me in the file. Incredible work. 

I added some basic gravity, drag, and jump physics and implemented a few helper functions to clean up the tutorial code. 

Another great thing is building (almost) directly to a .gba file. You need to use a powershell command to do it but I’ve just put that in a batch file and can run it super quickly. With the mGBA emulator, you can run the file directly on the computer, but you can also just put the gba file in a flash cart and play it on actual hardware. You are also supposed to be able to use cargo run to build and immediately run the built file, but I can’t figure it out as it’s giving errors. Also, the built file can output logs to mGBA which is awesome.

## 2

Idr

## 3

I learned about numbers today. From what I can gather, gba doesn’t support floats by default, only fixed precision numbers. agb has a trait called “Number” that supports use of gba’s fixed numbers. My program did still work using floating points, but I think those are all calculated manually and slowly, so probably best to switch to a supported type early on.

I’ve also begun to look at some of the example files. Now that I have a decent foundation of agb and background knowledge (from unity), I can kind of make heads and tails of what the code is trying to do. This is how I found out about the use of the Number trait, in fact. I saw the num!() macro and had no idea what it was doing, so I looked at the documentation to learn more. I can do that now because I’ve become more comfortable with looking at rust docs and the agb package as a whole. 

I converted all of my use of floats to FixedNum<32>. Now the game crashes on launch with no stack trace available lol, but i’ll debug this in the future.

## 4

I’m doing some Good ProgrammingTM by creating a Player module (another file to hold all of the player stuff). 

Now that I am doing some external functions I am learning more things about rust. Not limitations, just things that I’ve never had to consider in the past. 

## 5 

Contiuning to move the player. Numbers continue to confuse. I learned that the num! macro only takes literals, which makes sense because it’s a macro. I’ve had to work around converting a Tri (enum) into a FixedNum to use as input detection. Turns out FixedNum accepts i32 as a type that it can store, which I never considered for some reason. 

Considering making an “animation” module that handles sprite changes and animation.

## 6

WATERMELON GAME

I watched Ludwig streaming watermelon game and I thought that it would be a decently simple game to implement. It’s just a bunch of circles falling and filling up a space. 

Some problems I can see on the horizon: Inefficient collision detection and rotation physics.

Collision: I will need to learn a more efficient collision algorithm. The simple brute force n2 one will probably not run well on the gba lol. 

Rotation: I will need to learn how to impart torque on an object, which I have an idea for but it has yet to be implemented. 

## 7

Not as simple as I thought. Rust’s borrow checker is causing me a lot of issues. At first, I wanted to have a variable point to the currently held fruit and put it in the vec of active fruits, but the [borrow checker disallows that](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move).

I decided to actually read the book after someone insulted someone else for asking the same question on Reddit, and it was actually quite helpful. (explain borrowing and ownership here, linked above).

I now knew that I had to store the fruit in a variable while it was being held, and move it into the vec once dropped. Then we can create a new fruit to be held. 

The next problem: updating fruits. Every frame, we need to update each fruit’s properties like their position, velocity, and whether or not they are touching a wall or another fruit. To do this, we need to create a function that takes all of the fruits and updates them one by one. Once again, the borrow checker tries to make sure we are being safe. (explain [single mutability](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references))

This is solved by passing in the whole vector (moving it into the update function), taking a single fruit out, and checking it against all other fruits. This does actually make sense, and it seems to me like the rust compiler just wants me to write better code lol.

8afb049:

The fruits now fall properly, each one being updated in the loop. I implemented the collision with other fruits but it doesn’t seem to work; it jitters around a lot. Probably going to need some kind of continuous collision. Also, they fall just offscreen, so you can’t see some of the smaller ones. 

## 8 

I’ve looked at Collider and Rapier2d packages, but I think they both won’t work. They both seem to specifically use core::std, which agb doesn’t allow use of. (explain how the GBA doesn’t have an operating system that we can request resources from easily.)

## 9

master ?? - Fruits now bounce against walls and eachother

master 08b2154 - Fruits now stay on screen

master 9842fa5 - Fruits’ hitboxes are now accurate

I tweaked the physics a little bit. Now the fruits bounce off of the walls accurately, and off of eachother not so accurately. I am also no longer getting a divide by zero error and crash, which is good.

I realized a few days ago while not working on this that the “center” of the fruit is actually the top left corner of the sprite, which is an 8x8px rectangle. This is important because I was calculating collisions as if that was the center of the fruit, which is not the case. This is a simple correction (made more difficult by my lack of understanding of rust’s strict type definitions) to use a slightly different position when calculating physics. 

I have also modified the code such that the fruits now are properly confined to the screen, though they do jitter up and down at the bottom of the screen, which is not the best to look at.

Now with all of these fruits on screen, we begin to see big problem. The lag. The algorithm I’m using is horrible. For every fruit on screen, i’m checking it against every other fruit on screen (explain O(n2)). I’ve been thinking about some potentially better algorithms, but haven’t landed on anything concrete yet. Perhaps something to do with graphs? Maybe a union-find?

## 10

master 197ab97 - fixed fruits teleporting around when colliding

master f358a6f - Fixed fruit jittering on the bottom of the screen

Today I have once again put off an assignment after realizing a mistake that I made in calculating fruits’ collisions with one another. Previously, they would “overcorrect” their position, causing them to teleport around when touching. The problem was that I was improperly calculating the amount the fruits were overlapping, and they would adjust their position by that incorrect overlap amount. In the image below the red vector showed how much they were moving before, and the green scalar is how much they should move (and the amount they move now). I think this whole thing is a discrete collision detection issue as the fruits would never be overlapping had their collisions been detected continuously. But I’m sure there are other complications that continuous detection has as well.

Also, I fixed fruit jittering on the bottom of the screen by making their y velocities 0 if they were close to 0

![](https://lh7-us.googleusercontent.com/1B259JkTLjyegBmJEZacvtJXgpQqbppb-HGXNmEw5BaVL-HknWznIxFPaQkmXHkiTudjFiXdrwI9cKUA3n7yfEJRRAoteG1VHAV_9B4V8gYrV6xPLNHGh8fSMqGebOeQgWjhDfVkd4RaDiq6-B8unZM)

## 11
11/17
master 6f782dc - Attempting to compile a vec of collisions not working because borrow checker
It's been like 3 weeks because I had so many assignments and projects due.
I started to look into the dynamic physics of collisions today with the help of [this video](https://www.youtube.com/watch?v=LPzyNOHY3A4) which was super helpful. However, the borrow checker strikes again. 
The big issue I was having today was compiling all collisions into a vector or any other data structure. My approach to this was creating a `Vec<(&Fruit, &Fruit)>` to hold pairs/tuples of fruit that experienced a collision that frame. The problem came from trying to take a reference from the Vec with all active fruits and placing them in the new Vec.
Originally, I was passing in a mutable slice using `Vec.as_slice` into the method that finds all collisions. This worked for dealing with the static collisions only, but did not work when I also tried to push the two fruit into the collisions vector. The problem here, likely among other deeper problems, was lifetimes. The compiler kept telling me that the lifetime of the collisions vec needed to outlive some other lifetime and it all just confused me. I went to [the book](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html) (this is not actually the book, i mistook it for the book) to get a baseline understanding of lifetimes which helped a little, but not enough to salvage this approach.
There was also an issue with using `for fruit in fruits` and nest in `for other in fruits` loops as the `in` keyword makes the slice into an iterator, which mut borrows it, which disallows the other loop to use it as well. I got around this by just using indexes for the slice, i.e. `for fruit_index in 0..fruits_len` which is clunkier but works. 
Back to the main issue, my second and current attempt uses what I now think is bad design and will probably be scrapping it. Instead of passing in the `fruits.as_slice`, I *move* in the whole vec and move it out on return. This is very clunky and highly unnecessary; the reason borrows exists is specifically to alleviate the need to move variables in and out of function calls. Anyways, this approach still doesn't work because doing `fruits.get_mut(index)` borrows fruits as mutable itself. This causes an issue when I try to get the other fruit within the nested loop, as we can only have one mutable borrow at a time. 
This problem can probably be fixed in one of two ways:
1. Split up the collision steps into compile collisions -> merge fruit (which i don't even have implemented right now) -> static collisions -> dynamic collisions
	- This may still make the borrow checker unhappy during the compile collisions phase. I'd have to pass in an immutable borrow of the fruits vec, which may solve the issue but who knows.
2. Do all steps in part 1 in a single function
	- This would be horribly unreadable but may appease the borrow checker. I don't really want to live to appease the borrow checker, I would really rather learn how to use and harness it properly, which is part of the reason I'm doing this project in the first place. 

# 12
11/20
master 5a96f83 - Removed static collision logic from collision function and the borrow checker allows it now.
master c536ed0 - Began implementing collision-merge, which requires GBA and graphics structs to work.
master 89661d1 - Refactoring done but still need to figure out how to change the properties of both during collision.

# 13
master 029d462 - Implemented dynamic collisions but some math is wrong
master 028cd94 - Math is now right. Forgot to add the merged fruit to the vec containing all fruits, which is causing me some lifetime headaches.
master 4a542d9 - Fixed the above problem. Solved by changing `fruits: &'a mut Vec\<Fruit\<'a>>` to `fruits: &mut Vec\<Fruit\<'a>>`. Huge.

Trying to properly make sprites, here's some math
![[Pasted image 20231121191519.png]]

Figured it out by computing each fruit's size as a percentage to the watermelon, then converting that to pixel art pixels. The above image wasn't used but it looks cool lol

The screen/box boundary calculation is wrong, redoing it. Here's some math:
(it should be W+(32-f.s/2), forgot the /2 part)
![[Pasted image 20231121205507.png]]