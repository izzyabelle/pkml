use core::result::Result;
use std::error::Error;

type EmptyResult = Result<(), Box<dyn Error>>;

pub mod app;
pub mod bounded_i32;
pub mod event;
pub mod game;
pub mod handler;
pub mod moves;
pub mod player;
pub mod pokemon;
pub mod poketype;
pub mod preset;
pub mod selvec;
pub mod stat;
pub mod status;
pub mod trigger;
pub mod tui;
pub mod ui;
