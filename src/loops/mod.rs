//! The runtime loops. [`main_loop`] is the synchronous game loop: input →
//! update → render. Player movement lives in the loop and in
//! [`Game::step_player`](crate::game::Game::step_player); the old standalone
//! `player_loop` was superseded and removed.

pub mod main_loop;