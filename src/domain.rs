//! Domain Layer - ядро бизнес-логики
//!
//! Содержит несколько bounded contexts:
//! - Time Tracking - трекинг времени (TimeSegment, ActivityType)
//! - Settings - настройки приложения (AppSettings)

// Time Tracking BC
mod activity_type;
mod activity_instance;
mod time_segment;
mod tag;

// Settings BC
pub mod settings;

pub use activity_type::*;
pub use activity_instance::*;
pub use time_segment::*;
pub use tag::*;
