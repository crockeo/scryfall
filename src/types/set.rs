use super::uuid::Uuid;
use http::Uri;
use std::time::SystemTime;

/// Set objects
pub struct Set {
    /// A unique ID for this set on Scryfall that will not change.
    pub id: Uuid,

    /// The unique three to five-letter code for this set.
    pub code: String,

    /// The unique code for this set on MTGO, which may differ from the regular code.
    pub mtgo_code: Option<String>,

    /// This set’s ID on TCGplayer’s API, also known as the groupId
    pub tcgplayer_id: Option<u32>,

    /// The English name of the set.
    pub name: String,

    /// A computer-readable classification for this set. See below.
    pub set_type: SetType,

    /// The date the set was released or the first card was printed in the set (in GMT-8 Pacific time).
    pub released_at: Option<SystemTime>,

    /// The block code for this set, if any.
    pub block_code: Option<String>,

    /// The block or group name code for this set, if any.
    pub block: Option<String>,

    /// The set code for the parent set, if any. promo and token sets often have a parent set.
    pub parent_set_code: String,

    /// The number of cards in this set.
    pub card_count: u32,

    /// True if this set was only released on Magic Online
    pub digital: bool,

    /// True if this set contains only foil cards.
    pub foil_only: bool,

    /// A link to this set’s permapage on Scryfall’s website.
    pub scryfall_uri: Uri,

    /// A link to this set object on Scryfall’s API.
    pub uri: Uri,

    /// A URI to an SVG file for this set’s icon on Scryfall’s CDN. Hotlinking this image isn’t recommended, because
    /// it may change slightly over time. You should download it and use it locally for your particular user
    /// interface needs.
    pub icon_svg_uri: Uri,

    /// A Scryfall API URI that you can request to begin paginating over the cards in this set.
    pub search_uri: Uri,
}

pub enum SetType {
    Core,
    Expansion,
    Masters,
    Masterpiece,
    FromTheVault,
    Spellbook,
    PremiumDeck,
    DuelDeck,
    DraftInnovation,
    TrasureChest,
    Commander,
    Planechase,
    Archenemy,
    Vanguard,
    Funny,
    Starter,
    Box,
    Promo,
    Token,
    Memorabilia,
}
