# Intro
I've recently had a fascination with the Gameboy Advance. ...
## The Start
...
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