//! Deals with transforming data between the SC2 API and types moe suitable for use in ECS systems. 

pub mod action;
pub mod entity;
mod player;
pub mod state;
mod position;

pub use player::PlayerResources;
pub use state::ApiObservation;
