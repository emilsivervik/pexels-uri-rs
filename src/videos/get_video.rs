use super::PEXEL_VIDEO_PATH;
use crate::{PEXEL_HOST, PEXEL_SCHEME};
const PEXEL_GET_VIDEO_PATH: &str = "videos";

/// Retrieve a specific Video from its id.
pub struct GetVideo {
    id: usize,
}

impl GetVideo {
    /// Creates [`GetVideoBuilder`] for building URI's.
    pub fn builder() -> GetVideoBuilder {
        GetVideoBuilder::default()
    }

    /// Create URI from inputed vales from the [`GetVideoBuilder`].
    pub fn create_uri(&self) -> crate::BuilderResult {
        let uri = format!(
            "{}://{}/{}/{}/{}",
            PEXEL_SCHEME, PEXEL_HOST, PEXEL_VIDEO_PATH, PEXEL_GET_VIDEO_PATH, self.id
        );

        let url = url::Url::parse(uri.as_str())?;

        Ok(url.into())
    }
}

/// Builder for [`GetVideo`].
#[derive(Default)]
pub struct GetVideoBuilder {
    id: usize,
}

impl GetVideoBuilder {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    /// The id of the video you are requesting.
    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    /// Create [`GetVideo`] from the [`GetVideoBuilder`]
    pub fn build(self) -> GetVideo {
        GetVideo { id: self.id }
    }
}

#[cfg(test)]
mod tests {
    use crate::videos::get_video::GetVideoBuilder;

    #[test]
    fn test_id() {
        let uri = GetVideoBuilder::new().id(123).build();
        assert_eq!(
            "https://api.pexels.com/videos/videos/123",
            uri.create_uri().unwrap()
        );
    }
}
