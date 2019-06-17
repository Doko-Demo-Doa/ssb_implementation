//! Parser component of subtitle format implementation.
//! ```
//! // Imports
//! use std::{
//!     convert::TryFrom,
//!     fs::File,
//!     io::{BufReader,Cursor}
//! };
//! use ssb_parser::data::{Ssb,SsbRender};
//! // Data
//! let ssb_reader1 = Cursor::new("...");
//! let ssb_reader2 = BufReader::new(File::open("/foo/bar.ssb").unwrap());
//! // Parsing
//! let ssb = Ssb::default()
//!     .parse_owned(ssb_reader1).unwrap()
//!     .parse_owned(ssb_reader2).unwrap();
//! let ssb_render = SsbRender::try_from(ssb).unwrap();
//! // Print
//! println!("{:#?}", ssb_render);
//! ```
#![doc(
    html_logo_url = "https://substation-beta.github.io/assets/img/logo.png",
    html_favicon_url  = "https://substation-beta.github.io/assets/img/logo.png",
    html_root_url = "https://substation-beta.github.io"
)]


/// Minor types for data.
pub mod types;
// Internal utility structures & functions for data processing.
mod utils;
/// Data processors and main storages.
pub mod data;