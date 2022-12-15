# 2048 Solver

This is my final project for CPSC 474 at Yale (Fall 2022). We use reinforcement learning to train an agent to play the popular 2048 game (https://play2048.co/). The agent is implemented in Rust and uses an Expectimax algorithm with a transposition table and corner heuristic to find the best move to make at each step.

## How to Run

To run the solver, first install Rust (https://www.rust-lang.org/tools/install). Then, clone this repository and run the following command:

```bash
cargo run --release <depth>
```

where `<depth>` is the depth of the search tree. We recommend starting with a depth of 3, and increasing if you want to see the solver take longer to find the best move.

Alternatively, you can run the precompiled solver with the following command:

```bash
./rust_solver <depth>
```

## High Score

Using a depth of 6, the solver has achieved a max tile of <strong>16384</strong> and a high score of <strong>250,040</strong>. See `high-score.PNG` for a screenshot of the high score (before CLI improvements). This is a work in progress, and we hope to improve the solver to achieve even higher scores!
