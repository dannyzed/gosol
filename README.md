[![Build Status](https://travis-ci.org/dannyzed/gosol.svg?branch=master)](https://travis-ci.org/dannyzed/gosol)

gosol
=====
gosol is a tsumego solver written in rust.  The focus of this project is primarily to learn rust rather than create the best possible tsumego solver.

Current State
-------------
gosol can currently solve closed tsumego problems that do not require a large amount of moves.  There
is currently no user friendly interface exposing this functionality.

Building
--------
As of March 26 2016, gosol compiles on the beta and nightly versions of the
rust compiler.  

Planned Improvements/Features
-----------------------------
 * Rewrite the board/game code.  In order to get something functioning I have "circumvented" the rust borrow checker by using indicies instead of references in some places.
 * Minimax improvements.  Right now a very simple minimax algorithm is implemented, adding heuristics and alpha-beta pruning should increase the speed of the solver.
 * Zobrist Hashing.
 * CLI for sgf files.  It is intended to be able to input an sgf file and have gosol output a new sgf file with the principal variations included.
