//! Create URI's for Pexels API using.
//!
//! # API Documentation
//! This is not an official crate from Pexels, their documentation can be found [here](https://www.pexels.com/api/documentation/)
//!
//! # Examples
//!
//! ```
//! use pexels_uri::{videos, Orientation};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let uri_builder = videos::Search::builder()
//!         .query("Dogs running")
//!         .orientation(Orientation::Landscape)
//!         .per_page(25)
//!         .build();
//!
//!     assert_eq!(
//!         "https://api.pexels.com/videos/search?query=Dogs+running&per_page=25&orientation=landscape",
//!         uri_builder.create_uri()?
//!     );
//!     Ok(())
//! }
//!
//! ```
//!

pub mod photos;
pub mod videos;

const PEXEL_SCHEME: &str = "https";
const PEXEL_HOST: &str = "api.pexels.com";
const PEXEL_VERSION: &str = "v1";

/// Desired photo orientation.
pub enum Orientation {
    Landscape,
    Portrait,
    Square,
}

impl Orientation {
    fn as_str(&self) -> &str {
        match self {
            Orientation::Landscape => "landscape",
            Orientation::Portrait => "portrait",
            Orientation::Square => "square",
        }
    }
}

/// The locale of the search you are performing.
#[allow(non_camel_case_types)]
pub enum Locale {
    en_US,
    pt_BR,
    es_ES,
    ca_ES,
    de_DE,
    it_IT,
    fr_FR,
    sv_SE,
    id_ID,
    pl_PL,
    ja_JP,
    zh_TW,
    zh_CN,
    ko_KR,
    th_TH,
    nl_NL,
    hu_HU,
    vi_VN,
    cs_CZ,
    da_DK,
    fi_FI,
    uk_UA,
    el_GR,
    ro_RO,
    nb_NO,
    sk_SK,
    tr_TR,
    ru_RU,
}

impl Locale {
    fn as_str(&self) -> &str {
        match self {
            Locale::en_US => "en-US",
            Locale::pt_BR => "pt-BR",
            Locale::es_ES => "es-ES",
            Locale::ca_ES => "ca-ES",
            Locale::de_DE => "de-DE",
            Locale::it_IT => "it-IT",
            Locale::fr_FR => "fr-FR",
            Locale::sv_SE => "sv-SE",
            Locale::id_ID => "id-ID",
            Locale::pl_PL => "pl-PL",
            Locale::ja_JP => "ja-JP",
            Locale::zh_TW => "zh-TW",
            Locale::zh_CN => "zh-CN",
            Locale::ko_KR => "ko-KR",
            Locale::th_TH => "th-TH",
            Locale::nl_NL => "nl-NL",
            Locale::hu_HU => "hu-HU",
            Locale::vi_VN => "vi-VN",
            Locale::cs_CZ => "cs-CZ",
            Locale::da_DK => "da-DK",
            Locale::fi_FI => "fi-FI",
            Locale::uk_UA => "uk-UA",
            Locale::el_GR => "el-GR",
            Locale::ro_RO => "ro-RO",
            Locale::nb_NO => "nb-NO",
            Locale::sk_SK => "sk-SK",
            Locale::tr_TR => "tr-TR",
            Locale::ru_RU => "-ES",
        }
    }
}

/// Minimum video/photo size.
pub enum Size {
    Large,
    Medium,
    Small,
}

impl Size {
    fn as_str(&self) -> &str {
        match self {
            Size::Large => "large",
            Size::Medium => "medium",
            Size::Small => "small",
        }
    }
}
use std::fmt;

pub(crate) type BuilderResult = Result<String, PexelsError>;

/// Errors that can occurr while creating Pexels URI.
#[derive(Debug, Clone, PartialEq)]
pub enum PexelsError {
    ParseError(url::ParseError),
    HexColorCodeError(String),
}

impl fmt::Display for PexelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for PexelsError {}

impl From<url::ParseError> for PexelsError {
    fn from(err: url::ParseError) -> PexelsError {
        PexelsError::ParseError(err)
    }
}
