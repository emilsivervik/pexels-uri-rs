use crate::{Locale, Orientation, PexelsError, Size, PEXEL_HOST, PEXEL_SCHEME, PEXEL_VERSION};
const PEXEL_PHOTO_SEARCH_PATH: &str = "search";

/// Used as an input value for [`Color::Hex`] when specifying a hexadecimal color code.
///
/// #Example
///
/// ```
/// use pexels_uri::{photos, Size};
/// use pexels_uri::photos::{Color, Hex, SearchBuilder};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///        let hex_color = Hex::from_str("#FFFFFF")?;
///        let uri = SearchBuilder::new().color(Color::Hex(hex_color)).build();
///        assert_eq!(
///            "https://api.pexels.com/v1/search?query=&color=%23FFFFFF",
///            uri.create_uri()?
///        );
///        Ok(())
///  }
/// ```
///
/// # Errors
/// If string literal can't be validated as a valid hexadecimal color code [`PexelsError::HexColorCodeError`] will be propagated.
#[derive(Debug, PartialEq)]
pub struct Hex<'a>(&'a str);

impl<'a> Hex<'a> {
    pub fn from_str(v: &'a str) -> Result<Self, crate::PexelsError> {
        if v.len() != 7 {
            return Err(crate::PexelsError::HexColorCodeError(format!(
                "{} is not 7 chatacters long.",
                v
            )));
        }

        if !v.starts_with("#") {
            return Err(crate::PexelsError::HexColorCodeError(format!(
                "{} does not start with #.",
                v
            )));
        }

        let hex_digits = &v[1..];

        for c in hex_digits.chars() {
            if !c.is_ascii_hexdigit() {
                return Err(crate::PexelsError::HexColorCodeError(format!(
                    "{} have values that are not valid ASCII punctuation character.",
                    v
                )));
            }
        }

        Ok(Self { 0: v })
    }
}

/// Desired photo color.
pub enum Color<'a> {
    Red,
    Orange,
    Yellow,
    Green,
    Turquoise,
    Blue,
    Violet,
    Pink,
    Brown,
    Black,
    Gray,
    White,
    Hex(Hex<'a>),
}

impl<'a> Color<'a> {
    /// Get enum as string literal value
    fn as_str(&self) -> Result<&str, PexelsError> {
        let value = match self {
            Color::Red => "red",
            Color::Orange => "orange",
            Color::Yellow => "yellow",
            Color::Green => "green",
            Color::Turquoise => "turquoise",
            Color::Blue => "blue",
            Color::Violet => "violet",
            Color::Pink => "pink",
            Color::Brown => "brown",
            Color::Black => "black",
            Color::Gray => "gray",
            Color::White => "white",
            Color::Hex(v) => v.0,
        };

        Ok(value)
    }
}

/// This endpoint enables you to search Pexels for any topic that you would like. For example your query could be something broad like Nature, Tigers, People. Or it could be something specific like Group of people working.
pub struct Search<'a> {
    query: &'a str,
    page: Option<usize>,
    per_page: Option<usize>,
    orientation: Option<Orientation>,
    size: Option<Size>,
    color: Option<Color<'a>>,
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
            PEXEL_SCHEME, PEXEL_HOST, PEXEL_VERSION, PEXEL_PHOTO_SEARCH_PATH
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

        if let Some(color) = &self.color {
            url.query_pairs_mut().append_pair("color", color.as_str()?);
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
    color: Option<Color<'a>>,
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
            color: None,
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

    /// Desired photo orientation.
    pub fn per_page(mut self, per_page: usize) -> Self {
        self.per_page = Some(per_page);
        self
    }

    /// Desired photo orientation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Minimum photo size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    /// Desired photo color.
    pub fn color(mut self, color: Color<'a>) -> Self {
        self.color = Some(color);
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
            color: self.color,
            locale: self.locale,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        let uri = SearchBuilder::new().query("bar").build();
        assert_eq!(
            "https://api.pexels.com/v1/search?query=bar",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_page() {
        let uri = SearchBuilder::new().page(1).build();
        assert_eq!(
            "https://api.pexels.com/v1/search?query=&page=1",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_per_page() {
        let uri = SearchBuilder::new().per_page(1).build();
        assert_eq!(
            "https://api.pexels.com/v1/search?query=&per_page=1",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_orientation() {
        let uri = SearchBuilder::new()
            .orientation(Orientation::Landscape)
            .build();
        assert_eq!(
            "https://api.pexels.com/v1/search?query=&orientation=landscape",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_size() {
        let uri = SearchBuilder::new().size(Size::Small).build();
        assert_eq!(
            "https://api.pexels.com/v1/search?query=&size=small",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_color() {
        let uri = SearchBuilder::new().color(Color::Pink).build();
        assert_eq!(
            "https://api.pexels.com/v1/search?query=&color=pink",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_hex_color_code() {
        let hex_color = Hex::from_str("#FFFFFF").unwrap();
        let uri = SearchBuilder::new().color(Color::Hex(hex_color)).build();
        assert_eq!(
            "https://api.pexels.com/v1/search?query=&color=%23FFFFFF",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_locale() {
        let uri = SearchBuilder::new().locale(Locale::sv_SE).build();
        assert_eq!(
            "https://api.pexels.com/v1/search?query=&locale=sv-SE",
            uri.create_uri().unwrap()
        );
    }

    #[test]
    fn test_hex_struct_length() {
        let hex_color = Hex::from_str("#allanballan");
        assert_eq!(
            hex_color,
            Err(crate::PexelsError::HexColorCodeError(String::from(
                "#allanballan is not 7 chatacters long."
            )))
        );
    }

    #[test]
    fn test_hex_struct_box_validation() {
        let hex_color = Hex::from_str("FFFFFFF");
        assert_eq!(
            hex_color,
            Err(crate::PexelsError::HexColorCodeError(String::from(
                "FFFFFFF does not start with #."
            )))
        );
    }

    #[test]
    fn test_hex_struct_ascii_validation() {
        let hex_color = Hex::from_str("#??????");
        assert_eq!(
            hex_color,
            Err(crate::PexelsError::HexColorCodeError(String::from(
                "#?????? have values that are not valid ASCII punctuation character."
            )))
        );
    }
}
