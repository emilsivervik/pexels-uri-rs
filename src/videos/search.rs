use super::PEXEL_VIDEO_PATH;
use crate::{Locale, Orientation, Size, PEXEL_HOST, PEXEL_SCHEME};
const PEXEL_VIDEO_SEARCH_PATH: &str = "search";

/// This endpoint enables you to search Pexels for any topic that you would like. For example your query could be something broad like Nature, Tigers, People. Or it could be something specific like Group of people working.
pub struct Search<'a> {
    query: &'a str,
    page: Option<usize>,
    per_page: Option<usize>,
    orientation: Option<Orientation>,
    size: Option<Size>,
    locale: Option<Locale>,
}

impl<'a> Search<'a> {
    /// Creates [`SearchBuilder`] for building URI's.
    pub fn builder() -> SearchBuilder<'a> {
        SearchBuilder::default()
    }

    /// Create URI from inputed vales from the [`SearchBuilder`].
    pub fn create_uri(&self) -> crate::BuilderResult {
        let uri = format!(
            "{}://{}/{}/{}",
            PEXEL_SCHEME, PEXEL_HOST, PEXEL_VIDEO_PATH, PEXEL_VIDEO_SEARCH_PATH
        );

        let mut url = url::Url::parse(uri.as_str())?;

        url.query_pairs_mut().append_pair("query", self.query);

        if let Some(page) = &self.page {
            url.query_pairs_mut()
                .append_pair("page", page.to_string().as_str());
        }

        if let Some(per_page) = &self.per_page {
            url.query_pairs_mut()
                .append_pair("per_page", per_page.to_string().as_str());
        }

        if let Some(orientation) = &self.orientation {
            url.query_pairs_mut()
                .append_pair("orientation", orientation.as_str());
        }

        if let Some(size) = &self.size {
            url.query_pairs_mut().append_pair("size", size.as_str());
        }

        if let Some(locale) = &self.locale {
            url.query_pairs_mut().append_pair("locale", locale.as_str());
        }

        Ok(url.into())
    }
}

/// Builder for [`Search`].
#[derive(Default)]
pub struct SearchBuilder<'a> {
    query: &'a str,
    page: Option<usize>,
    per_page: Option<usize>,
    orientation: Option<Orientation>,
    size: Option<Size>,
    locale: Option<Locale>,
}

impl<'a> SearchBuilder<'a> {
    pub fn new() -> Self {
        Self {
            query: "",
            page: None,
            per_page: None,
            orientation: None,
            size: None,
            locale: None,
        }
    }

    /// The search query.
    pub fn query(mut self, query: &'a str) -> Self {
        self.query = query;
        self
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

    /// Desired video orientation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Minimum video size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    /// The locale of the search you are performing.
    pub fn locale(mut self, locale: Locale) -> Self {
        self.locale = Some(locale);
        self
    }

    /// Create [`Search`] from the [`SearchBuilder`]
    pub fn build(self) -> Search<'a> {
        Search {
            query: self.query,
            page: self.page,
            per_page: self.per_page,
            orientation: self.orientation,
            size: self.size,
            locale: self.locale,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Locale, Orientation, Size};

    use crate::videos::search::SearchBuilder;

    #[test]
    fn test_query() {
        let uri = SearchBuilder::new().query("bar").build();
        assert_eq!(
            "https://api.pexels.com/videos/search?query=bar",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_page() {
        let uri = SearchBuilder::new().page(1).build();
        assert_eq!(
            "https://api.pexels.com/videos/search?query=&page=1",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_per_page() {
        let uri = SearchBuilder::new().per_page(1).build();
        assert_eq!(
            "https://api.pexels.com/videos/search?query=&per_page=1",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_orientation() {
        let uri = SearchBuilder::new()
            .orientation(Orientation::Landscape)
            .build();
        assert_eq!(
            "https://api.pexels.com/videos/search?query=&orientation=landscape",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_size() {
        let uri = SearchBuilder::new().size(Size::Small).build();
        assert_eq!(
            "https://api.pexels.com/videos/search?query=&size=small",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_locale() {
        let uri = SearchBuilder::new().locale(Locale::sv_SE).build();
        assert_eq!(
            "https://api.pexels.com/videos/search?query=&locale=sv-SE",
            uri.create_uri().unwrap()
        );
    }
}
