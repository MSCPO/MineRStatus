//! MineRStatus shared library: configuration, query logic, and the HTTP app.
//!
//! Both binary targets consume this crate:
//! - [`crate::app`] holds the axum router and handlers.
//! - [`crate::config`] loads configuration (TOML > env > defaults).
//! - [`crate::status`] queries servers and builds responses.

pub mod app;
pub mod config;
pub mod status;