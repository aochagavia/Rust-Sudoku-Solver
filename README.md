Rust-Sudoku-Solver
==================

A basic application to solve sudokus using the Rust programming language.


##Why?

Because I needed an excuse to program something in Rust, and I had curiosity to know if I could make a non-bruteforcing algorithm to solve a sudoku.


##How does it work?

###Algorithm

It tries first applying a fast_solve algorithm, which for most sudokus is enough.
If this does not succeed, then the program proceeds with brute forcing it.

Detailed description is provided as comments at the beginning of each source file inside "src/sudoku".

###Input format

The input format is very intuitive and you will surely understand it when you take a look at the files inside the directory "samples"


##Hey! Something is wrong in your code!

Then you can open a pull request. I promise I will look at it.
