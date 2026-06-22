//! The runtime loops. [`main_loop`] is the synchronous game loop (input → update
//! → render); [`player_loop`] holds the player-movement logic.

pub mod main_loop;
pub mod player_loop;

//there may need to be some loop consolidation done