//! # in-summer-we-render
//!
//! In summer, we render
//!
//! [Github](https://github.com/hungkhoaitay/in-summer-we-render)

//#![warn(missing_docs)]

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

#[macro_use]
extern crate error_chain;

/// Error handling mod
pub mod errors {
    error_chain! {
        foreign_links {
            Io(std::io::Error);
            Float(std::num::ParseFloatError);
            Int(std::num::ParseIntError);
        }
    }
}
pub use errors::*;

#[macro_use]
extern crate approx;

mod io;
mod logic;
/// Module handling PLY
mod ply;
/// Module handling directory of ply files
mod ply_dir;
mod pointcloud;
mod processing;
mod render;

pub use filter_and_transform::{fat, filter, transform};
pub use io::{reader, writer};
pub use logic::{ui_controller, ui_controller_manager};
pub use ply_dir::PlyDir;
pub use pointcloud::{color, coordinate, params, point, points};
pub use processing::{filter_and_transform, interpolate, interpolate_controller};
pub use render::{gui, renderer, ui, ui_manager};

use std::time::Instant;
