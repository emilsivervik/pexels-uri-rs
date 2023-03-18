//! # Building URI for Pexels Photos
//!
//! # API Documentation
//! `https://www.pexels.com/api/documentation/#photos`
//!
//! # Example
//!
//! ```
//! use pexels_uri::{photos, Size};
//! use pexels_uri::photos::Color;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!      let uri_builder = photos::Search::builder()
//!          .query("Dogs running")
//!          .color(Color::Pink)
//!          .size(Size::Medium)
//!          .build();
//!
//!      assert_eq!(
//!          "https://api.pexels.com/v1/search?query=Dogs+running&size=medium&color=pink",
//!          uri_builder.create_uri()?
//!      );
//!     Ok(())
//!  }
//! ```
//!

mod curated;
mod get_photo;
mod search;

pub use curated::{Curated, CuratedBuilder};
pub use get_photo::{GetPhoto, GetPhotoBuilder};
pub use search::{Color, Hex, Search, SearchBuilder};
