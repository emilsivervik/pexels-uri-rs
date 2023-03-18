# pexels-uri
Create URI's for Pexels API using the builderpattern.

## API Documentation
This is not an official crate from Pexels, their documentation can be found [here](https://www.pexels.com/api/documentation/)

 # Examples

 ```rust
 use pexels_uri::{videos, Orientation};

 fn main() -> Result<(), Box<dyn std::error::Error>> {
     let uri_builder = videos::Search::builder()
         .query("Dogs running")
         .orientation(Orientation::Landscape)
         .per_page(25)
         .build();

     assert_eq!(
         "https://api.pexels.com/videos/search?query=Dogs+running&per_page=25&orientation=landscape",
         uri_builder.create_uri()?
     );
     Ok(())
 }

 ```