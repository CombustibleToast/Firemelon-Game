# Intro - 9P, 1T
Hey! 
This is a Gameboy Advance demake for Suika Game written entirely in Rust, but I'll get into that later. It plays pretty similarly to the original game, as best as I could do without writing a comprehensive physics engine. There are a few quirks, but hey, it's Suika on the GBA.
If you want to check it out, it's all on Github and itch.io for free. I've put links in the description for your convenience. 
# Basics - 8P, 2T
#wip
I started off just with the idea to make something for the GBA. I had the idea that a Downwell demake would be pretty neat, but I still needed to learn how to make something for the system first. Also, I wanted to learn Rust, which is a neat programming language. I won't get into why it's really cool here, but if you want to learn more about it, I've linked a video by No Boilerplate in the description. It's an excellent introduction. #attribution
# The Start - 6P, 4T
I started looking around for ways to develop for the GBA. The most popular way is in C++ using something called Tonc, but I'm specifically looking to develop using Rust. 
Thanks to its modern package manager Cargo, it's easy for people to create and use packages for the language. And wouldn't you know it, there's a package for developing for the GBA. After in depth research (1 google search), I was left with two options: The crate [gba](https://github.com/rust-console/gba) which provides very low-level control of the GBA's hardware, and the crate [agb](https://github.com/agbrs/agb) which manages all of the GBA's hardware. I decided to go with the latter option because it'll be simpler to use and learn for my first ever GBA *and* Rust project.

The agb crate provides a template that I can build off of to start my game. Downloading that and spinning it up, I'm greeted with this "no game" screen. Already I'm hopeful for what I'll be able to do with this, but I've got a long way to go. Evidenced even further by not knowing a lick of Rust nor this package that I've never used before. 
# The Surge - 7P, 2T
Day 0 was mostly a bust. I did get it to build and work on an emulator, I had no idea what I was doing. The next day, day 1, I searched around some more and actually found a guide! I have no idea why it's not linked anywhere on the official documentation or GitHub, but it was super helpful. It's barely even a full guide, just showing you how to import some sprites and taking in inputs from buttons. But, that was just enough to get me started.

I quickly swapped out the given sprites for one of my own: A little fish guy that was used in a game jam some friends and I were in. I added some gravity, collision with the walls and ground, and a button to jump. Boom, I already had a very basic working prototype!
## Testing the game 8P, 2T
The AGB crate has a really nice and easy way of building and testing game. Cargo provides many useful tools for automating building and testing programs. Two of which are `cargo build` and `cargo run`, which automatically compiles and runs the program using a preset command. To be able to run the game, I have mGBA, an emulator for GBA games. `cargo run` launches mGBA automatically and lets me immediately go from coding to testing with a single command. 
# Getting Lost and Finding Anew 10P, 0T
At this point, I kept tweaking a few things here and there, learning a few things about how rust works, but I wasn't sure what kind of game I actually wanted to make. I've had the idea for a while now to make a demake of Downwell, which is one of my favorite games. Its style and simplicity seems like a good fit for the GBA, but also it seemed like too daunting of a task to take on. It's a simple game on the surface but does have a lot of things going on under the hood like terrain generation and enemy AI. Also, someone else has already made an incomplete version. I haven't tried it because I don't know how to build it, but it's there. #attribution 

Then one evening I got a notification from Ludwig's server, just a normal stream announcement where he was playing Suika Game. I knew about the game at that point but hadn't actually seen any gameplay. I picked up the core mechanics of it pretty quick... and also began to consider it as my GBA project. It seemed simple enough to make...
# The Plan (is simple) 7P, 3T
`Smash 64 - Character Select`
Here's the things I'll need to make this work: Firstly, some sprites. Otherwise I won't be able to actually see what's going on in the game. Second, basic gravity and collision. The fruits need to be able to bounce off of each other and the walls of the box. Third, a way for the player to spawn the fruit. 
These are very basic requirements for the game to function and quickly spiral out into more subrequirements, but we'll get there when we get there. The best way to make something complicated is to make something simple first, then build on it. Having nothing to show for your work as you incrementally improve is often not helpful for keeping up a positive mentality. Also, it will be easier to test and debug iteratively, rather than all at once at the end, which would be impossible. 
# The Execution 3P, 7T
## Sprites 6, 4
Alright, sprites. I made these in Aseprite, which is an excellent program, by the way. Not only that, the agb crate has *native Aseprite* importing, meaning I can just save my `.ase` file and have the program automatically import all of my sprites from that file. Amazing stuff. The sprites need to be of a certain size to be imported. If this is violated, the import macro will panic even before the program is compiled and VSCode lets me know about it which is sooo cool. 
## Spawning Fruit 4, 6 
Next, a way to spawn fruit. For now, the code just spawns a fruit that is "held" by the player and released when A is pressed. This is done with a simple `if` statement and listening to agb's input struct for new values. In the future, this needs a few more things: a cooldown so the player can't just spam A and release infinite fruit into the game, and a character to move left and right to drop fruit from.
We have another problem, though. 
### Fruit storage 0, 10
`???`
Once the player drops the fruit, the code needs to be able to reference it in the future to check things like its speed and if it's colliding with anything. We need to store it somehow. A simple array could work, but fruits will be constantly created and destroyed, which could lead to a lot of difficulty in managing them. A dynamic storage type like Rust's Vec is perfect for this. 

I was initially confused on how to use it. Since the GBA doesn't have an operating system, we can't call on it to ask for memory allocation, so we can't use Rust's `std` (standard) library. Looking into it more, though, the agb crate does let us use the `alloc` trait #research instead, which will allow us to allocate memory for ourselves. In that, we can use Vecs.

So, when the player drops a fruit, it is moved into the Vec for use later, and when one is destroyed, it is just moved out of the Vec. Once moved out, Rust's memory management system will automatically dispose of it, preventing memory leaks.
## Collisions 1, 9
Handling collisions is the main focus of this game. It's quite a complicated subject requiring math and physics, so let's break it down.
### Off the shelf options 8, 2
I initially wanted to avoid doing physics at all by just downloading a package for it. That is how most things are done in the modern age, after all. Unsurprisingly, there were quite a few options available, two of which were promising: [Collider](https://docs.rs/collider/latest/collider/) is a package for handling 2D collisions with continuous calculations, and [Rapier2D](https://docs.rs/rapier2d/latest/rapier2d/) is a general purpose physics engine with 2- and 3D counterparts. Both seem very powerful, but each had the same three issues:
One, the lack of the `std` library meant that they couldn't do operations that they assumed most systems would be able to do. The GBA is clearly not most systems. Two, they are both general purpose and too heavyweight for what I'm trying to do. Three, I didn't want to learn how to use the packages because they seemed complicated. I'd have to tussle with these new packages while also fighting to let them talk to the GBA. Not ideal.
### Bespoke Engine 1, 9
So, I need to make a bespoke physics engine for this game. It probably won't be that bad, circles are the easiest things to implement physics for. Right?
There are a few stages for handling collisions. First, we need to get all the collisions between every fruit and store them somewhere. There's an efficient way to do this but I'm lazy and just used a naiive approach instead. Horrible performance but it works. This part actually caused a problem because I messed up my math, causing fruits to teleport around when colliding with one another. I didn't realize this was my fault for a long time and thought it was because of the GBA.
Handling fruit merging requires an additional check before the rest is calculated. It's pretty simple, though. Just need to check each collision and if the two fruits are the same, we delete both and make a new merged fruit between them. It's not that straightforward, though. We also need to go through the list of collisions and remove all that contain either of the deleted fruit, or else we will get some unwanted "double" merges. 
The next stage is handling the static collisions between fruits. Basically, we need to make sure that fruits don't overlap with one another, so we move them away from each other if they are. [pic](https://lh7-us.googleusercontent.com/1B259JkTLjyegBmJEZacvtJXgpQqbppb-HGXNmEw5BaVL-HknWznIxFPaQkmXHkiTudjFiXdrwI9cKUA3n7yfEJRRAoteG1VHAV_9B4V8gYrV6xPLNHGh8fSMqGebOeQgWjhDfVkd4RaDiq6-B8unZM)
Finally, the dynamic collisions between the fruit need to be handled. I had no idea how to do this, but there is an [excellent video covering the topic](https://www.youtube.com/watch?v=LPzyNOHY3A4) made by #attribution , so I just followed that. The dynamic collision allows fruits to transfer their momentums when they collide, making it much more realistic.
### Borrow Checker Woes 3,7
I have yet to mention that, throughout all of this, Rust's borrow checker has given me so much pain... while also being its best feature. When I try to do something that's totally normal in any other language, Rust's compiler will get angry and slap me in the face for being unsafe. I can't edit the same thing in two places, I can't create an array without fully initializing it first, etcetera. This is all really annoying, but also guarantees against undefined behavior. This means when the game is running, it's super clear when something incorrect happens because it's 100% my fault, rather than some weird thing with the language. If you've ever dealt with debugging memory in C or C++, you know the pain...
Also there's a whole thing with lifetimes in Rust, which I have almost no understanding of but listening to the compiler's helpful errors just tells me what to put where to use them.
# Final Touches 8,2

## Spriting 7,3
`???`
Alright, that's all of the heavy technical stuff done. Now we can work on the funny sprites for the game. The ones here (shown on screen) are placeholder ones I made, but aren't size-accurate to the source material. So I made a new set using ratios I calculated with pixel sizes of the original game. Then I fixed up how the sprites exist in the game ([[Log#13]]) so they'd be less jank. Like I said at the start, I can directly import aseprite files using a macro, which is just amazing. 
## Gup 10,0
`???`
Hey, who's that? It's Fishest Gup! I put him in as a placeholder sprite that the player controls, but I think I might keep him in like that. With his introduction, version 0.1.0 was released, the first "playable" release of the game. 
## Music 9,1
Next, I added some background music. I remember in Ludwig's stream that everyone was complaining the music was so boring/repetitive. To counter that (and because I was listening to her at the time), I put in femtanyl's #attribution CHASER album. For some wild reason, I'm able to import wav files directly, which is crazy. I had to compress them to the max to make them fit inside the rom file, though. 
I can also import tracker files instead which are much smaller and natively use the GBA's sound chip, which may be coming soon...
## Affine 8,2
The fruits just popping into existence is really boring, so I made use of the GBA's affine matrix implementation. If you don't know what that means, don't worry. It takes a whole university course in computer graphics to understand them. Even then I still don't actually know how they work. It basically lets me spin sprites around and change how big they are.
## Score Readout 7,3
The score is one of the most important parts of the game, and it's been missing this whole time. I tried using agb's text rendering stuff, but it was too hard so just made the text some sprites with numbers. Works fine for me. Oh and I have no idea how the scoring in suika works despite my best efforts searching around, so my version just checks how many of each fruit are on screen. 
## Background 9,1
I thought adding the background for the game was going to be hard but again, agb's import macros saved the day. The current one is super basic but it works just fine. Might commission someone to make a better one. #stc
# Ending 10,0
That's pretty much it. The game's live and downloadable on itch right now for free. You can also view all of the source code on Github, along with the script for this video and all my notes (**if you're reading this on Github right now, hi!**). You can download the rom to play on an emulator like mGBA or an actual GBA if you have a flash cartridge. Here it is on my real GBA!
In the future, I'd like to add high score saving and a simple menu system, but this is good enough for now. 
## Notes 8,2
To anyone else wanting to walk this path, be sure to check out the documentation for agb and follow the short guide someone has made for it. It's not much but it got it just barely. If you haven't used Rust before, be sure to read the first few chapters of The Book free online. It's got a lot of good information on how to use the language. 

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
- [x] The borrow checker
- [ ] Lifetimes and how I still don't understand them
- [x] Sprites and importing
## Fruit
- [x] Inspiration to convert to fruit game
- [x] Physics engine woes
- [x] Static collisions and measurement inaccuracies 
- [x] Dynamic collisions
- [x] storage vec
- [ ] 
# OLD
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