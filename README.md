## Block the Pig

This is a remake of the [Block the
Pig](https://www.mathplayground.com/mobile_block_the_pig/) game in Rust.
I played this game originally on the phone and, after many hours of
playing with friends, found the optimal strategy (and achieved a high score
of 198 rounds beaten). Having done that, I was interested in creating a
clone of the game--and adding more features in the process--as well as
an agent to solve the game algorithmically. 

### Directory Structure

- `src/`
    - `board.rs` The representation of the game board, agnostic of the
      pig; it just stores the tiles on the map.
    - `game.rs` The representation of the state of the game, i.e. the
      pig and the map.
    - `main.rs` The WIP driver for the game
    - `maps.rs` The implementations of various board configurations
      (currently just the classic 11x5 board)


### Roadmap

I want this project to eventually be a well-designed,
easily-customizable game driver with which one can create basic and more
complex Block the Pig variants, potentially with multiple pigs. 

As well, I would like to implement various agents for solving the game
(though I have yet to show that this is a solved game...)

Finally, I would like to persist some leaderboard rankings, and maybe
even host this on my website (which is also WIP...)
