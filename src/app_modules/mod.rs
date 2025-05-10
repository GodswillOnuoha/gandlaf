/*
Module for the web application modules.
*/

mod app_state;

pub mod api;
pub mod auth;
pub mod health;

pub use app_state::AppState;
