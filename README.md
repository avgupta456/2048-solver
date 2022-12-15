## 2048 Solver

This is a solver for the popular 2048 game (https://play2048.co/), and my final project for CPSC 474 at Yale (Fall 2022). It uses an Expectimax algorithm to find the best move to make at each step. The solver is implemented in Rust.

### How to Run

To run the solver, first install Rust (https://www.rust-lang.org/tools/install). Then, clone this repository and run the following command:

```bash
cargo run --release <depth>
```

where `<depth>` is the depth of the search tree. We recommend starting with a depth of 3, and increasing if you want to see the solver take longer to find the best move.

### High Score

Using a depth of 6, the solver has achieved a max tile of 16384 and a high score of 250,040. See `high-score.PNG` for a screenshot of the high score (before CLI improvements). This is a work in progress, and we hope to improve the solver to achieve even higher scores!
