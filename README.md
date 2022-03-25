Group name: Sebastian's Game of Life

Group members: Sebastian Glusak (sglus2)

Project introduction: 
This project hopes to achieve an accurate representation of Conway's Game of Life written in the Rust programming language. Some goals in mind include making an good-looking and easy to use user interface and also implementing several modifications that alter the game. This project was chosen because it is a simple concept that can further be built off of in many different ways to my own liking. The concept of the game itself and its results are interesting as well as fun to see visualized.

System overview: 
A functional project should allow the user to individually select the starting cells in Conway's Game of Life and to run the game using those selected cells. It should also allow the user to zoom in and out of the grid, select between different shapes, adjust the time between evolution cycles, and pause / resume the game at any given moment. The project should implement the rules so that the game accurately represents Conway's Game of Life. In the original Game of Life, there is an infinite, two-dimensional orthogonal grid of square cells where the cell can be in either a state of dead or alive. Every cell interacts with its eight neighboring cells: two horizontally, two vertically, four diagonally. The beginning of the game starts with the user choosing a set of squares to be alive for the initial state. With the given initial state, the game will run through a series of evolutions forever until there are no remaining live squares. The rules for each evolution follow as such: Any live cell with two or three live neighbors survives, any dead cell with three live neighbors becomes alive, all other live cells die, and all other dead cells stay dead. For purposes of this project, these rules will be subject to change when implementing different shapes or modifications.

Possible challenges: Creating a good user interface; implementing rules correctly; dealing with never ending games; creating game using different shapes / rules.

References: Inspired by John Conway's Game of Life.
