include!(concat!(env!("OUT_DIR"), "/proto", "/mod.rs"));

pub mod ability;
pub mod unit;

pub use ability::AbilityId;
