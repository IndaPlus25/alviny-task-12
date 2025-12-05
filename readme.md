# alviny-task-12

It's a Raycaster.

## Controls

Use WASD to strafe around the level, and left and right arrow keys to look around. Use SHIFT to sprint.

## Map making

Maps are stored in *.lvl files. These files are plaintext representations of a level map. In such a representation, every period `.` represents a walkable space, and every wall is represented by a `#` symbol.

## How to run

First, clone the repository:

```bat
git clone https://github.com/IndaPlus25/alviny-task-12.git
```

Then, navigate to the program folder and run the application:

```bat
cd alviny-task-12
./raycaster.exe
```

### Running with user-defined maps

To run with a custom map, use

```bat
./raycaster.exe "path/to/your/level-file.lvl"
```
