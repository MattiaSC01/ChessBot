# Python Prototype

This is a prototype written in python that we developped in order to familiarize with the main ideas and challenges that building a chess engine entails. It implements the UCI protocol, and it already displays many of the elements that can be found in the final Rust version. 

However, it's not as strong a chess player, mainly due to the employment of a rudimentary static evaluation function that limits its positional understanding considerably. It also occasionally misses tactical themes that are beyond its horizon, because the tree exploration is a simple min-max search with alpha-beta pruning, without quiescence search.
