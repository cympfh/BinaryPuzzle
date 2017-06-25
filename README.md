# BinaryPuzzle Solver

## Rule of BinaryPuzzle

see [BinaryPuzzle.com](http://binarypuzzle.com/)

## Data Format

The initial game state are given through `stdin`.
The solution are displayed through `stdout`.

The initial game state includes the board size `N` and `NxN` board cells.
A cell is `.`, `0` or `1`.

```bash
BinaryPuzzleSolver <<EOM
10
.....1...1
1......0..
..0....0..
.00...0..1
1........1
...0..1...
0....1....
.......0.0
0........0
.0.0.1....
EOM
```

```bash
BinaryPuzzleSolver <<EOM
12
..1.0.......
.0........11
.0..0.......
.....1.11.0.
....0.....0.
......1.....
.......0....
1..00.1...1.
..........1.
.11.1...1..1
00..11..10.1
............
EOM
```

### sample

```bash
cargo run < ./sample-input
```

