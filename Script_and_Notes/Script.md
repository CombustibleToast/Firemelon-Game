# Intro - 9P, 1T
I've recently had a fascination with the Gameboy Advance. ...
# Basics - 8P, 2T

# The Start - 6P, 2T
I started looking around for ways to develop for the GBA. There are some high level game engines that work more like unity, but they all use C and C++ #research. I'm specifically looking to develop using Rust. Thankfully, it's a very modern language and has a package manager. This means I'll be able to just install a package for GBA development. 

After in depth research (1 google search), I was left with two options: The crate [gba](https://github.com/rust-console/gba) which provides very low-level control of the GBA's hardware, and the crate [agb](https://github.com/agbrs/agb) which manages all of the GBA's hardware. I decided to go with the latter option because it'll be simpler to use and learn for my first ever GBA *and* Rust project.

The agb crate provides a template that I can build off of to start my game. Downloading that and spinning it up, I'm greeted with this "no game" screen. Already I'm hopeful for what I'll be able to do with this, but I've got a long way to go. Evidenced even further by not knowing a lick of Rust nor this package that I've never used before. 
# The Surge - 7P, 2T
Day 0 was mostly a bust. I did get it to build and work on an emulator, I had no idea what I was doing. The next day, day 1, I searched around some more and actually found a guide! I have no idea why it's not linked anywhere on the official documentation or GitHub, but it was super helpful. It's barely even a full guide, just showing you how to import some sprites and taking in inputs from buttons. But, that was just enough to get me started.

I quickly swapped out the given sprites for one of my own: A little fish guy that was used in a game jam some friends and I were in. I added some gravity, collision with the walls and ground, and a button to jump. Boom, I already had a very basic working prototype of a game!
## Testing the game 8P, 2T
The AGB crate has a really nice and easy way of building and testing game. Cargo, Rust's package manager, provides many useful tools for automating building and testing programs. Two of which are `cargo build` and `cargo run`, which automatically compile and build your game using a preset command. To be able to run the game, I have mGBA, an emulator for GBA games. `cargo run` launches mGBA automatically and lets me immediately go from coding to testing with a single command. 
# Getting Lost and Finding Anew 10P, 0T
At this point, I kept tweaking my code for the gup. Just a few things here and there, learning a few things about how rust works, but I wasn't sure what kind of game I actually wanted to make. I've had the idea for a while now to make a demake of Downwell, which is one of my favorite games. Its style and simplicity seems like a good fit for the GBA, but also it seemed like too daunting of a task to take on. It's a simple game on the surface but does have a lot of things going on under the hood like terrain generation and enemy AI. Also, someone else has already made a prototype of one. I haven't tried it because I don't know how to build it, but it's there.

Then one evening I got a notification from Ludwig's server, just a normal stream announcement where he was playing Suika Game trying to beat ConnorEatsPants's score. I knew about the game at that point but hadn't actually seen any gameplay. I picked up the core mechanics of it pretty quick... and also began to consider it as my GBA project. It seemed simple enough to make...
# The Plan (is simple) 7P, 3T
Here's the things I'll need to make this work: Firstly, some sprites. Otherwise I won't be able to actually see what's going on in the game. Second, basic gravity and collision. The fruits need to be able to bounce off of each other and the walls of the box. Third, a way for the player to spawn the fruit. 
These are very basic requirements for the game to function and quickly spiral out into more subrequirements, but we'll get there when we get there. The best way to make something complicated is to make something simple first, then build on it. It's bad for your mental to spend a long time making something that doesn't work until the very end and have nothing to show for until then. Also you need to be able to test and debug as you go or else you'll have a gordian knot of problems. 
# The Execution 3P, 7T
## Sprites 6, 4
Alright, sprites. I made these in Aseprite, which is an excellent program, by the way. Not only that, but because the agb crate has *native Aseprite* importing, meaning I can just save my `.ase` file and have the program automaticlly import all of my sprites from that file. Amazing stuff. The sprites need to be of a certain size to be imported. If this is violated, the import macro will panic even before the program is compiled and VSCode lets me know about it which is sooo cool. 
## Spawning Fruit 4, 6 
Next, a way to spawn fruit. For now, the code just spawns a fruit that is "held" by the player and released when A is pressed. This is done with a simple `if` statement and listening to agb's input struct for new values. In the future, this needs a few more things: a cooldown so the player can't just spam A and release infinite fruit into the game, and a character to move left and right to drop fruit from.
We have another problem, though. 
### Fruit storage 0, 10
Once the player drops the fruit, the code needs to be able to reference it in the future to check things like its speed and if it's colliding with anything. We need to store it somehow. A simple array could work, but fruits will be constantly created and destroyed, which could lead to a lot of difficulty in managing them. A dynamic storage type like Rust's Vec is perfect for this. 

I was initially confused on how to use it. Since the GBA doesn't have an operating system, we can't call on it to ask for memory allocation and things like that, so we can't use Rust's `std` (standard) library. Looking into it more, though, the agb crate does let us use the `alloc` trait #research instead, which will allow us to allocate memory for ourselves. In that, we can use the Vec structure.

So, when a fruit is released from the player's grasp, it is moved into the Vec for use later, and when one is destroyed, it is just moved out. Once moved out, Rust's memory management system will automatically dispose of it, which stops memory leaks.
## Collisions 1, 9
Handling collisions is the main focus of this game. It's quite a complicated subject requiring math and physics, so let's break it down.
### Off the shelf options 8, 2
I initially wanted to avoid doing physics at all by just downloading a package for it. That is how most things are done in the modern age, after all. Unsurprisingly, there were quite a few options available, two of which were promising: [Collider](https://docs.rs/collider/latest/collider/) is a package for handling 2D collisions with continuous calculations, and [Rapier2D](https://docs.rs/rapier2d/latest/rapier2d/) is a general purpose physics engine with 2- and 3D counterparts. Both seemed to be very powerful, but each had the same three issues. 
One, the lack of the `std` library meant that they couldn't do operations that they assumed most systems would be able to do. The GBA is clearly not most systems. Two, they are both general purpose and quite heavyweight for what I'm trying to do. Three, I didn't want to learn how to use the packages because they seemed complicated. That's not without merit though; I'd have to tussle with these new packages while also fighting the GBA side of things and making the two talk to eachother. Not ideal.
### Bespoke Engine 1, 9
So, I need to make a bespoke physics engine for this game. It probably won't be that bad, circles are the easiest things to implement physics for 

# Topic list
## Outside of programming
- [ ] The game's on itch rn
- [ ] mGBA
- [ ] Flash cart playing
## General programming agb/rust
- [ ] Numbers
- [ ] no std
- [ ] dpad tri
- [ ] Animations
- [ ] Affine matrices **BASICS**!
- [ ] The borrow checker
- [ ] Lifetimes and how I still don't understand them
- [x] Sprites and importing
## Fruit
- [x] Inspiration to convert to fruit game
- [ ] Physics engine woes
- [ ] Static collisions and measurement inaccuracies 
- [ ] Dynamic collisions
- [x] storage vec
- [ ] 


## Day 0
### In the bininging
After in depth research (1 google search), there were two options that became clear: The crate [gba](https://github.com/rust-console/gba) which provides very low-level control of the GBA's hardware, and the crate [agb](https://github.com/agbrs/agb) which manages all of the low-level aspects of this kind of work. I decided to go with the latter option because there's no need to overcomplicate my work when I had both never used Rust before and never tried to make anything for the GBA before. 
### Delving into the code
The repository comes with a template build #reword which was easy to clone and build immediately. All of the necessary non-source code files were pre-set up to make building quick and easy without needing to fiddle around with the TOML files. The initial state of the game just calls the prebuilt `agb::no_game()` function *as* the "game", which just shows the words "NO GAME" in big letters. I tried to look through that function's code to see if I could glean #edit anything, but I had no idea what I was looking at so I promptly gave up for the day.
## Day 1
### Buh??
I returned the next day with a renewed passion #reword to getting this done #reword. I still had no idea what I was looking at, and remembered that I don't even know how to write any Rust. Thankfully, Rust has some of the best *modern* tools for learning its language. 
### Learning Rust
Most older languages I find have very archaic physical books printed for them that are meant to be viewed and read as physical media. With modern tech and the internet, though, that's far from the optimal way to go about learning something. "The Book" on Rust and *Rust by Example* are wonderful resources if you want to learn Rust. They feature well thought out chapters and subchapters and code that you can compile and play with *in your browser* #confirm, all for free!
### Learning agb
The same can't be said with AGB though. One of the best things about programming is that very often someone else has walked where you have and had the same problem as you. This is not the case for an obscure package that nobody has ever used before. I was struggling still to understand anything, despite having a grasp on Rust's syntax and readability. 
### The sump
As time went on, I started to wonder how anyone did anything with software without documentation. Did they learn from a friend? Is everything an insider secret? Surely not for open source software. But if someone makes an nice piece of software without teaching anyone to use it, then what's the point? 
### The catalyst
But then I found it. A guide on how to use the agb crate. It's not a good guide, only consisting of a single chapter with three subchapters, but it's just enough to bootstrap my knowledge of the crate enough to know what I need to look for. The guide introduced importing sprites from Aseprite, storing x and y values for its position, and input detection and handling. That all was just enough for me to make my own thing! After all, I already had plenty of experience from past Unity projects (unity moment), and converting the math with positions and vectors over to this project was a breeze.
### Gup
I'd like you to meet my good friend fishest_gup. He was drawn by my friend Sam #consent #attribution for a game jam four of us did. I've linked it down below #link if you want to check it out, but it's not a very good game and moreover not relevant at all to this discussion. In here, he's a physic'd out boy with jumping, gravity, drag, and collisions. Granted, these are all very simple operations with static predefined walls and a single physics object ((foreshadowing)), but he's there! He's on the GBA!
### Aseprite awesomeness
Something I really enjoy is that the agb crate allows for direct imports from an Aseprite file, all done by a rust macro!(). If the file is incompatible because of sprite size or something, the macro will panic and alert you of this at compile time which is incredible. (VSCode will also tell you of this before you even compile it!). In fact, I discovered this when I made the gup sprite and found out that there could only be specific sizes of sprites. The size I had was wrong and rust_analyzer immediately told me in the file. Incredible.
## Day 2
I don't remember what happened today; there is actually a break on this day in my commits, so maybe I mistakenly wrote this day down without having done anything. Moving on.
## Day 3
### Numbers
I learned about numbers today. From what I can gather, the Gameboy Advance doesn’t support floats by default, only fixed precision numbers. agb has a trait called “Number” that supports use of gba’s fixed numbers. My program did still work using floating points, but I think those are all handled and calculated without optimization or extra operations, so I converted all of my number to the supported type. #tense
### Exploration
I also began to look at some of the example files. Now that I had a decent foundation of agb and background knowledge (from unity), I can kind of make heads and tails of what the code is trying to do. This is how I found out about the use of the Number trait, in fact. I saw the num!() and looked it up in the documentation, which I was now able to do after getting more comfortable with it. #tense

Also all of my conversions to the fixed point numbers don't work, but that's a problem for the future.
## Day 4 and 5
### Refactoring
Day 4 and 5 I decided to move all of the player logic to another file to keep everything readable. Creating new functions for simpler logic started to teach me some things about Rust. Not limitations, but a way of looking at things that I hadn't experienced in the past ((foreshadowing)).
### Numbers, again
The numbers continue to confuse. The num! macro apparently only takes literals, which I think makes sense because it's a macro which are resolved at compile time. I also had to work around converting one of agb's enums, the Tri type, into a fixed point number. A Tri holds either a -1, 0, or 1 and is returned when asking for the horizontal or vertical axis of the dpad input. Seems like more hassle than its worth but I don't know how to make the best use of enums either. I also learned that the fixed point floating number "FixedNum" type accepts a type to store its value in, which I thought was neat; you can choose what data type it should store its values into. 

## Day 6
### Prelude
Day 6 was the pivotal day. Up to day 5, I had long considered what kind of gameplay elements I actually wanted to make a game out of. For a while, I really wanted to make a Downwell #attribution clone or demake. Downwell seems simple enough, but I knew that there's a lot more complexity that goes into a game than what it seems on the surface. Randomly generating a level, interactions with different enemies, keeping track of all the pieces on screen, all of it can just become a mess.
### The switch
This all changed when I got a notification on Discord from Ludwig's #attribution  server. Usually I just open the announcement channel to clear the notification and ignore it, but this time I actually decided to watch for a bit. This stream he was trying to beat a score set by his friend ConnerEatsPants #attribution, but that's not important. I stayed and watched what he was doing for a bit before I realized I could probably easily make this into gba game. Sure, I'd have to figure out the physics simulation, but that's really the only thing I'd have to do. That couldn't be so hard right? ((foreshadowing))
### The plan (is simple (boys))
Here's the plan: Suika game at its core is very simple in design. Each fruit while looking a bit different are actually just circles, and these circles are the easiest shape to do physics on. So, I would just need a simple physics engine to handle the collisions of circles and that's it! Sure there are a few more things like scoring and merging fruit, but that all can't that hard, right?
## Day 7
### It is that hard
Day 7 saw me pick my first fight with the borrow checker. Let me explain. When a fruit is first created, it's put into the player's "hand" before it's dropped. In that state, it's immobile and, in the future, can be moved around by the player. I had set up a variable to store the held fruit separate from all other fruits currently in play. In any other language, it would be super simple to have both the variable pointing to the fruit and the fruit existing in the list of all in play fruits. Rust does not allow this.
### :ferrisReading:
After tussling for a bit, I went to the only place that programmers go when they need help: Google into StackOverflow (name a more iconic duo). From there, I got second hand insulted by the classic toxic StackOverflow user and decided to take their recommendation to actually read the book. From there, I really began to understand the concept of ownership and borrowing. I had to create a separate variable to store the held fruit, then *move* it into the list when the player dropped it.
### More problems
Great, that solves that. But there's another problem more central to the game: Updating the fruits every frame. A function needs to be able to get the list of in-play fruits and update each one. I tried several ways using my not-yet refined knowledge of the borrow checker but the only one I got working was not great. I moved the whole vector into the function for processing and returning it back out one once finished. The function owning the list appeased the borrow checker by preventing double mutability, which is when you try to create two simultaneous mutable references to the same data. This is a big no-no in Rust.

At the end of the day, though, the fruits were falling and kinda colliding with eachother. 
## Day 8
### A *fruit*less search (haha)
Speaking of collisions, day 8 was when I had the core update loop down and began to actually look into handling collisions. I looked around for Rust physics crates, but all of them seem too heavyweight and general purpose for what I needed. Two notable ones were Collider #attribution and Rapier2d #attribution. Both seem like great crates, but not suitable for my use case.
## Day 9
### If you want something done right...
It was clear that I had to build a bespoke physics engine for this game. It won't be very complicated, mind you, but still one from scratch. 