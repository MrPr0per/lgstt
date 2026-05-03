//! Presentation Layer - взаимодействие с пользователем
//!
//! Содержит CLI интерфейс для ввода событий.

mod cli;

pub use cli::ask_for_settings;
pub use cli::run_mainloop;
