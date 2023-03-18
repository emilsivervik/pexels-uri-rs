use crate::{PEXEL_HOST, PEXEL_SCHEME, PEXEL_VERSION};
const PEXEL_GET_PHOTO_PATH: &str = "photos";

/// Retrieve a specific Photo from its id.
pub struct GetPhoto {
    id: usize,
}

impl GetPhoto {
    /// Creates [`GetPhotoBuilder`] for building URI's.
    pub fn builder() -> GetPhotoBuilder {
        GetPhotoBuilder::default()
    }

    /// Create URI from inputed vales from the [`GetPhotoBuilder`].
    pub fn create_uri(&self) -> crate::BuilderResult {
        let uri = format!(
            "{}://{}/{}/{}/{}",
            PEXEL_SCHEME, PEXEL_HOST, PEXEL_VERSION, PEXEL_GET_PHOTO_PATH, self.id
        );

        let url = url::Url::parse(uri.as_str())?;

        Ok(url.into())
    }
}

/// Builder for [`GetPhoto`].
#[derive(Default)]
pub struct GetPhotoBuilder {
    id: usize,
}

impl GetPhotoBuilder {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    /// The id of the photo you are requesting.
    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    /// Create [`GetPhoto`] from the [`GetPhotoBuilder`]
    pub fn build(self) -> GetPhoto {
        GetPhoto { id: self.id }
    }
}

#[cfg(test)]
mod tests {
    use crate::photos::get_photo::GetPhotoBuilder;

    #[test]
    fn test_id() {
        let uri = GetPhotoBuilder::new().id(123).build();
        assert_eq!(
            "https://api.pexels.com/v1/photos/123",
            uri.create_uri().unwrap()
        );
    }
}
