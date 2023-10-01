# Four six-sided dice

Did you know that if you roll four six-sided dice, then it is aways possible to combine
them using the basic mathematical operations of addition, subtraction, multiplication and
division in such a way that the result is 1? I learned this from a friend. In this repository,
we explore this fact in two ways.

## A proof of the claim

The folder `proofs` contains a proof of this claim, formalized in the interactive theorem prover
[Lean 4](https://leanprover-community.github.io/). The proof basically consists of three steps:

1. Define and prove a few heuristics that apply in a large number of cases,
2. make a large case distinction to show that in almost all cases, one of the heuristics applies,
3. solve the remaining cases by hand.

It turns out that there are exactly 10 of these "exotic" cases that have to be proved by hand.

Of course, it would be possible to use Lean metaprogramming to have Lean automatically generate
a full case bash of all 1296 cases. Hopefully, this shouldn't pose a performance problem. Maybe
a nice project for the future.

## A game

The rest of this repository is a small terminal-based game written in Rust. In the game, you are
presented with -- you guessed it -- four random numbers between 1 and 6 and you have to combine them
in such a way that the result is 1. This was mainly a way for me to learn the basics of Rust.

There are two command-line switches: you can use `--rounds` or `-r` to control the number of rounds
per game, and `--seed` or `-s` to set the seed for random generation of the rounds. Setting the
seed is useful if you want to host a little tournament...