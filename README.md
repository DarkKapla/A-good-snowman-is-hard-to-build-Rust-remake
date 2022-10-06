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
Press T to reset the current level.  
Press space bar to recenter the view. Hold to have the cam follow the player.  
Press ESC to quit.  

### Features

* The complete map of the original game, embedded in the binary. Not the bonus levels though.
* Same gameplay, same puzzle rules.
* Rewind: revert back one step. It keeps a limited but big enough history of the player's moves.
* Reset a level: if the player is inside a level, that level can be re-initialized in order to restart the puzzle from scratch. That feature is quite poorly implemented and erases the rewind queue, so use with caution. Because the player's position isn't tracked, the reset feature doesn't bother and let the player still upon resetting, even if that means putting a snowball in the player.
* The game is saved in a file `save.txt` in the process' directory, which will be loaded at the next start so the player did not lose progress.
* Memory leaks, according to valgrind. This has to originate from the piston library, as my code doesn't contain unsafe code, reference-counting pointers, std::forget(), etc.

The graphics are made entirely with piston's geometrical shapes functions. I had more trouble than expected adjusting the colors of the elements in the game. The objective was to make the overall screen look nice and smooth to the eyes, but it always seemed a bit too flashy, and the saturation levels looked unbalanced. The handling of color in games is not always that easy.

## Ethical concerns

So this is a remake of a proprietary game. I hope I do not offense the developers of the original *A good snowman is hard to build* too much. The worst part of this repository is probably that it contains the whole map (apart from bonus levels) to play. But on the other hand, if the map were not in the repo, people could still watch youtube playthrough of the original game and recreate the map in the text file by themselves. Just like they could redraw any level on paper and then solve it without buying the game. In any case, the real game's experience is still far better.
