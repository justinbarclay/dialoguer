//! dialogue is a library for Rust that helps you build useful small
//! interactive user inputs for the command line.  It provides utilities
//! to render various simple dialogs like confirmation prompts, text
//! inputs and more.
//!
//! Best paired with other libraries in the family:
//!
//! * [indicatif](https://crates.io/crates/indicatif)
//! * [console](https://crates.io/crates/console)
//!
//! # Crate Contents
//!
//! * Confirmation prompts
//! * Input prompts
//! * Menu selections
extern crate console;

pub use prompts::{Confirmation, Input, PasswordInput};
pub use select::Select;

mod prompts;
mod select;