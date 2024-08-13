use core::result::Result;
use std::error::Error;

type EmptyResult = Result<(), Box<dyn Error>>;

pub mod app;
pub mod effect;
pub mod event;
pub mod game;
pub mod handler;
pub mod moves;
pub mod play;
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
