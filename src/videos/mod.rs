//! # Building URI for Pexels Videos
//!
//! # API Documentation
//! `https://www.pexels.com/api/documentation/#videos`
//!
//! # Example
//!
//! ```rust
//! use pexels_uri::{videos, Orientation, Size};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let uri_builder = videos::Search::builder()
//!        .query("Dogs running")
//!        .orientation(Orientation::Landscape)
//!        .per_page(25)
//!        .size(Size::Small)
//!        .build();
//!
//!   assert_eq!(
//!       "https://api.pexels.com/videos/search?query=Dogs+running&per_page=25&orientation=landscape&size=small",
//!       uri_builder.create_uri()?
//!   );
//!     Ok(())
//! }
//!
//! ```

mod get_video;
mod popular;
mod search;
pub use get_video::{GetVideo, GetVideoBuilder};
pub use popular::{Popular, PopularBuilder};
pub use search::{Search, SearchBuilder};

const PEXEL_VIDEO_PATH: &str = "videos";
