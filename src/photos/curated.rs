use crate::{PEXEL_HOST, PEXEL_SCHEME, PEXEL_VERSION};
const PEXEL_CURATED_PATH: &str = "curated";

/// This endpoint enables you to receive real-time photos curated by the Pexels team.
pub struct Curated {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl Curated {
    /// Creates [`CuratedBuilder`] for building URI's.
    pub fn builder() -> CuratedBuilder {
        CuratedBuilder::default()
    }

    /// Create URI from inputed vales from the [`CuratedBuilder`].
    pub fn create_uri(&self) -> crate::BuilderResult {
        let uri = format!(
            "{}://{}/{}/{}",
            PEXEL_SCHEME, PEXEL_HOST, PEXEL_VERSION, PEXEL_CURATED_PATH
        );

        let mut url = url::Url::parse(uri.as_str())?;

        if let Some(page) = &self.page {
            url.query_pairs_mut()
                .append_pair("page", page.to_string().as_str());
        }

        if let Some(per_page) = &self.per_page {
            url.query_pairs_mut()
                .append_pair("per_page", per_page.to_string().as_str());
        }

        Ok(url.into())
    }
}

/// Builder for [`Curated`].
#[derive(Default)]
pub struct CuratedBuilder {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl CuratedBuilder {
    pub fn new() -> Self {
        Self {
            page: None,
            per_page: None,
        }
    }

    /// The page number you are requesting.
    pub fn page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }

    /// The number of results you are requesting per page.
    pub fn per_page(mut self, per_page: usize) -> Self {
        self.per_page = Some(per_page);
        self
    }

    /// Create [`Curated`] from the [`CuratedBuilder`]
    pub fn build(self) -> Curated {
        Curated {
            page: self.page,
            per_page: self.per_page,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::photos::curated::CuratedBuilder;

    #[test]
    fn test_page() {
        let uri = CuratedBuilder::new().page(1).build();
        assert_eq!(
            "https://api.pexels.com/v1/curated?page=1",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_per_page() {
        let uri = CuratedBuilder::new().per_page(1).build();
        assert_eq!(
            "https://api.pexels.com/v1/curated?per_page=1",
            uri.create_uri().unwrap()
        );
    }
}
