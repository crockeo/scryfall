use super::card::Card;
use super::set::Set;
use super::uri::Uri;
use serde::Deserialize;

/// A type-generic List object
#[derive(Deserialize)]
pub struct List<T> {
    /// An array of the requested objects, in a specific order.
    pub data: Vec<T>,

    /// True if this List is paginated and there is a page beyond the current page.
    pub has_more: bool,

    /// If there is a page beyond the current page, this field will contain a full API URI to that page. You may
    /// submit a HTTP GET request to that URI to continue paginating forward on this List.
    pub next_page: Option<Uri>,

    /// If this is a list of Card objects, this field will contain the total number of cards found across all pages.
    pub total_card: Option<u32>,

    /// An array of human-readable warnings issued when generating this list, as strings. Warnings are non-fatal
    /// issues that the API discovered with your input. In general, they indicate that the List will not contain the
    /// all of the information you requested. You should fix the warnings and re-submit your request.
    pub warnings: Option<Vec<String>>,
}

/// A card-specific List object.
pub type CardList = List<Card>;

/// A set-specific List object.
pub type SetList = List<Set>;
