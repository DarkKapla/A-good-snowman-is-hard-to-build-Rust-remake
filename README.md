# A good snowcrab is hard to build

⛄️⛄️⛄️🦀️⛄️⛄️⛄️

A remake of the little puzzle video game « *A good snowman is hard to build* » in rust. Built upon the piston game library.

The original game is here https://store.steampowered.com/app/316610/A_Good_Snowman_Is_Hard_To_Build/. The goal is to push snow balls on the grid in order to build snowmen with three balls of different sizes.

## How to use

### Installation

1. Have a rust compiler and toolchain set up. I recommend installing the rustup tool from the official website.
2. Download this repository and go to the project's root folder.
3. Start the compilation with `$ cargo run`.
4. You may need to install additional libraries for piston to work. The compiler errors should tell more about it.
5. ???
6. Profit.

### Controls

Use Z/Q/S/D or the arrow keys to move around.  
Press E or R to rewind one turn.  
Press space bar to recenter the view. Hold to have the cam follow the player.  
Press ESC to quit.  

### Features

* The complete map of the original game, embedded in the binary. Not the bonus levels though.
* Same gameplay, same puzzle rules.
* Rewind: revert back one step. It keeps a limited but big enough history of the player's moves.
* The game is saved in a file `save.txt` in the process' directory, and will be loaded at the next start.
* Memory leaks, according to valgrind. This has to originate from the piston library, as my code doesn't contain unsafe code, reference-counting pointers, std::forget(), etc.

The graphics are made entirely with piston's geometrical shapes functions. I had more trouble than expected adjusting the colors of the elements in the game. The objective was to make the overall screen look nice and smooth to the eyes, but it always seemed a bit too flashy, and the saturation levels looked unbalanced. The handling of color in games is not always that easy.

## Ethical concerns

So this is a remake of a proprietary game. I hope I do not offense the developers of the original *A good snowman is hard to build* too much. The worst part of this repository is probably that it contains the whole map (apart from bonus levels) to play. But on the other hand, if the map were not in the repo, people could still watch youtube playthrough of the original game and recreate the map in the text file by themselves. Just like they could redraw any level on paper and then solve it without buying the game. In any case, the real game's experience is still far better.

## Possible improvements

A "reset" action that resets an entire level whould be more powerful than the rewind button. With the rewind, you may have to press the key many times to get back to the starting point of the puzzle. An idea is to track when the player walks on an empty tile (they separate levels) and to use those to maintain a stack of pointers to the rewind stack. Another strategy would be to read a level's starting state in the map's starting state, using a graph search to explore only the current level. Of course, to reset a level will mess up with the current rewind and save features that keep a history of the player's movements.
