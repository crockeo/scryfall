use super::price::Price;
use super::uri::Uri;
use super::uuid::Uuid;
use serde::Deserialize;
use std::collections::HashSet;
use std::time::SystemTime;

/// Possible colors that a card can be. Note that cards who do not have a color are not automatically colorless, e.g.
/// conspiracies.
#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    #[serde(rename = "W")]
    White,

    #[serde(rename = "U")]
    Blue,

    #[serde(rename = "B")]
    Black,

    #[serde(rename = "R")]
    Red,

    #[serde(rename = "G")]
    Green,
}

/// The kind of card, e.g. normal / split / etc.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    Normal,
    Split,
    Flip,
    Transform,
    Meld,
    Leveler,
    Saga,
    Planar,
    Scheme,
    Vanguard,
    Token,
    DoubleFacedToken,
    Emblem,
    Augment,
    Host,
}

/// Frame effects that are applied over the primary Frame kinds.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FrameEffect {
    Legendary,
    Miracle,
    NyxTouched,
    Draft,
    Devoid,
    Tombstone,
    ColorShifted,
    SunMoonDfc,
    CompassLandDfc,
    OriginPwDfc,
    MoonEldraziDfc,
}

/// Main Frame kind, e.g. '93, '97, etc.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum Frame {
    #[serde(rename = "1993")]
    Year1993,

    #[serde(rename = "1997")]
    Year1997,

    #[serde(rename = "2003")]
    Year2003,

    #[serde(rename = "2015")]
    Year2015,

    #[serde(rename = "future")]
    Future,
}

/// The different kinds of MTG this can be played on. E.g. paper MTG, Arena, and MTG online.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Game {
    Paper,
    Arena,
    Mtgo,
}

/// Rarity levels that a card can be.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythic,
}

/// The legality status of this card in different formats.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Legality {
    NotLegal,
    Legal,
    Banned,
    Restricted,
}

/// Primary card object
#[derive(Debug, Deserialize, PartialEq)]
pub struct Card {
    /// This card’s Arena ID, if any. A large percentage of cards are not available on Arena and do not have this ID.
    pub arena_id: Option<u32>,

    /// A unique ID for this card in Scryfall’s database.
    pub id: Uuid,

    /// A language code for this printing.
    pub lang: String,

    /// This card’s Magic Online ID (also known as the Catalog ID), if any. A large percentage of cards are not
    /// available on Magic Online and do not have this ID.
    pub mtgo_id: Option<u32>,

    /// This card’s foil Magic Online ID (also known as the Catalog ID), if any. A large percentage of cards are not
    /// available on Magic Online and do not have this ID.
    pub mtgo_foil_id: Option<u32>,

    /// This card’s multiverse IDs on Gatherer, if any, as an array of integers. Note that Scryfall includes many promo
    /// cards, tokens, and other esoteric objects that do not have these identifiers.
    pub multiverse_ids: Option<Vec<u32>>,

    /// This card’s ID on TCGplayer’s API, also known as the productId.
    pub tcgplayer_id: Option<u32>,

    /// A unique ID for this card’s oracle identity. This value is consistent across reprinted card editions, and unique
    /// among different cards with the same name (tokens, Unstable variants, etc).
    pub oracle_id: Uuid,

    /// A link to where you can begin paginating all re/prints for this card on Scryfall’s API.
    pub prints_search_uri: Uri,

    /// A link to this card’s rulings list on Scryfall’s API.
    pub rulings_uri: Uri,

    /// A link to this card’s permapage on Scryfall’s website.
    pub scryfall_uri: Uri,

    /// A link to this card object on Scryfall’s API.
    pub uri: Uri,

    /// Gameplay fields
    /// If this card is closely related to other cards, this property will be an array with Related Card Objects.
    pub all_parts: Option<Vec<CardFace>>,

    /// An array of Card Face objects, if this card is multifaced.
    pub card_face: Option<Vec<RelatedCard>>,

    /// The card’s converted mana cost. Note that some funny cards have fractional mana costs.
    pub cmc: u32,

    /// This card’s colors, if the overall card has colors defined by the rules. Otherwise the colors will be on the
    /// card_faces objects, see below.
    pub colors: Option<HashSet<Color>>,

    /// This card’s color identity.
    pub color_identity: HashSet<Color>,

    /// The colors in this card’s color indicator, if any. A null value for this field indicates the card does not have
    /// one.
    pub color_indicator: Option<HashSet<Color>>,

    /// This card’s overall rank/popularity on EDHREC. Not all cards are ranked.
    pub edhrec_rank: Option<u32>,

    /// True if this printing exists in a foil version.
    pub foil: bool,

    /// A code for this card’s layout.
    pub layout: Layout,

    /// An object describing the legality of this card across play formats. Possible legalities are legal, not_legal,
    /// restricted, and banned.
    pub legalities: Legalities,

    /// This loyalty if any. Note that some cards have loyalties that are not numeric, such as X.
    pub loyalty: Option<String>,

    /// The mana cost for this card. This value will be any empty string "" if the cost is absent. Remember that per
    /// the game rules, a missing mana cost and a mana cost of {0} are different values. Multi-faced cards will report
    /// this value in card faces.
    pub mana_cost: Option<String>,

    /// The name of this card. If this card has multiple faces, this field will contain both names separated by ' // '.
    pub name: String,

    /// True if this printing exists in a nonfoil version.
    pub nonfoil: bool,

    /// The Oracle text for this card, if any.
    pub oracle_text: Option<String>,

    /// True if this card is oversized.
    pub oversized: bool,

    /// This card’s power, if any. Note that some cards have powers that are not numeric, such as *.
    pub power: Option<String>,

    /// True if this card is on the Reserved List.
    pub reserved: bool,

    /// This card’s toughness, if any. Note that some cards have toughnesses that are not numeric, such as *.
    pub toughness: Option<String>,

    /// The type line of this card.
    pub type_line: String,

    /// The name of the illustrator of this card. Newly spoiled cards may not have this field yet.
    pub artist: Option<String>,

    /// Whether this card is found in boosters.
    pub booster: bool,

    /// This card’s border color: black, borderless, gold, silver, or white.
    pub border_color: String,

    /// The Scryfall ID for the card back design present on this card.
    pub card_back_id: Uuid,

    /// This card’s collector number. Note that collector numbers can contain non-numeric characters, such as letters
    /// or ★.
    pub collector_number: String,

    /// True if this is a digital card on Magic Online.
    pub digital: bool,

    /// The flavor text, if any.
    pub flavor_text: Option<String>,

    /// This card’s frame effect, if any.
    pub frame_effect: Option<FrameEffect>,

    /// This card’s frame layout.
    pub frame: Frame,

    /// True if this card’s artwork is larger than normal.
    pub full_art: bool,

    // A list of games that this card print is available in, paper, arena, and/or mtgo.
    pub games: Vec<Game>,

    /// True if this card’s imagery is high resolution.
    pub highres_image: bool,

    /// A unique identifier for the card artwork that remains consistent across reprints. Newly spoiled cards may not
    /// have this field yet.
    pub illustration_id: Option<Uuid>,

    /// An object listing available imagery for this card. See the Card Imagery article for more information.
    pub image_uris: Option<ImageUris>,

    /// An object containing daily price information for this card, including usd, usd_foil, eur, and tix prices, as
    /// strings.
    pub prices: Prices,

    /// The localized name printed on this card, if any.
    pub printed_name: Option<String>,

    /// The localized text printed on this card, if any.
    pub printed_text: Option<String>,

    /// The localized type line printed on this card, if any.
    pub printed_type_line: Option<String>,

    /// True if this card is a promotional print.
    pub promo: bool,

    /// An array of strings describing what categories of promo cards this card falls into.
    pub promo_types: Vec<String>,

    /// An object providing URIs to this card’s listing on major marketplaces.
    pub purchase_uris: PurchaseUris,

    /// This card’s rarity. One of common, uncommon, rare, or mythic.
    pub rarity: Rarity,

    /// An object providing URIs to this card’s listing on other Magic: The Gathering online resources.
    pub related_uris: RelatedUris,

    /// The date this card was first released.
    pub released_at: SystemTime,

    /// True if this card is a reprint.
    pub reprint: bool,

    /// A link to this card’s set on Scryfall’s website.
    pub scryfall_set_uri: Uri,

    /// This card’s full set name.
    pub set_name: String,

    /// A link to where you can begin paginating this card’s set on the Scryfall API.
    pub set_search_uri: Uri,

    /// The type of set this printing is in.
    pub set_type: String,

    /// A link to this card’s set object on Scryfall’s API.
    pub set_uri: String,

    /// This card’s set code.
    pub set: String,

    /// True if this card is a Story Spotlight.
    pub story_spotlight: bool,

    /// True if the card is printed without text.
    pub textless: bool,

    /// Whether this card is a variation of another printing.
    pub variation: bool,

    /// The printing ID of the printing this card is a variation of.
    pub variation_of: Option<Uuid>,

    /// This card’s watermark, if any.
    pub watermark: Option<String>,
}

/// Card face object, used within the card object in the card_faces field.
#[derive(Debug, Deserialize, PartialEq)]
pub struct CardFace {
    /// The name of the illustrator of this card face. Newly spoiled cards may not have this field yet.
    pub artist: Option<String>,

    /// The colors in this face’s color indicator, if any.
    pub color_indicator: Option<HashSet<Color>>,

    /// This face’s colors, if the game defines colors for the individual face of this card.
    pub colors: Option<HashSet<Color>>,

    /// The flavor text printed on this face, if any.
    pub flavor_text: Option<String>,

    /// A unique identifier for the card face artwork that remains consistent across reprints. Newly spoiled cards may
    /// not have this field yet.
    pub illustration_id: Option<Uuid>,

    /// An object providing URIs to imagery for this face, if this is a double-sided card. If this card is not
    /// double-sided, then the image_uris property will be part of the parent object instead.
    pub image_uris: Option<ImageUris>,

    /// This face’s loyalty, if any.
    pub loyalty: Option<String>,

    /// The mana cost for this face. This value will be any empty string "" if the cost is absent. Remember that per
    /// the game rules, a missing mana cost and a mana cost of {0} are different values.
    pub mana_cost: String,

    /// The name of this particular face.
    pub name: String,

    /// The Oracle text for this face, if any.
    pub oracle_text: Option<String>,

    /// This face’s power, if any. Note that some cards have powers that are not numeric, such as *.
    pub power: Option<String>,

    /// The localized name printed on this face, if any.
    pub printed_name: Option<String>,

    /// The localized text printed on this face, if any.
    pub printed_text: Option<String>,

    /// The localized type line printed on this face, if any.
    pub printed_type_line: Option<String>,

    /// This face’s toughness, if any.
    pub toughness: Option<String>,

    /// The type line of this particular face.
    pub type_line: String,

    /// The watermark on this particulary card face, if any.
    pub watermark: Option<String>,
}

/// Related card object, used within the card object in the all_parts field.
#[derive(Debug, Deserialize, PartialEq)]
pub struct RelatedCard {
    /// An unique ID for this card in Scryfall’s database.
    pub id: Uuid,

    /// A field explaining what role this card plays in this relationship, one of token, meld_part, meld_result, or
    /// combo_piece.
    pub component: String,

    /// The name of this particular related card.
    pub name: String,

    /// The type line of this card.
    pub type_line: String,

    /// A URI where you can retrieve a full object describing this card on Scryfall’s API.
    pub uri: Uri,
}

/// Contains legalities for this card in each format.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Legalities {
    pub standard: Legality,
    pub future: Legality,
    pub modern: Legality,
    pub legacy: Legality,
    pub pauper: Legality,
    pub vintage: Legality,
    pub penny: Legality,
    pub commander: Legality,
    pub brawl: Legality,
    pub duel: Legality,
    pub oldschool: Legality,
}

/// Contains all of the possible URIs for each kind of image Scryfall stores.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ImageUris {
    pub small: Option<Uri>,
    pub normal: Option<Uri>,
    pub large: Option<Uri>,
    pub png: Option<Uri>,
    pub art_crop: Option<Uri>,
    pub border_crop: Option<Uri>,
}

/// Contains prices in different markets for this card.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Prices {
    pub usd: Option<Price>,
    pub usd_foil: Option<Price>,
    pub eur: Option<Price>,
    pub tix: Option<Price>,
}

/// Contains URIs to this card on sites where you can purchase this card
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct PurchaseUris {
    pub tcgplayer: Option<Uri>,
    pub cardmarket: Option<Uri>,
    pub cardhoarder: Option<Uri>,
}

/// Contains URIs to this card on related sites.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RelatedUris {
    pub tcgplayer_decks: Option<Uri>,
    pub edhrec: Option<Uri>,
    pub mtgtop8: Option<Uri>,
}

mod tests {
    #[test]
    fn test_color() {
        use super::Color;

        let color_pairs = vec![
            ("\"W\"", Color::White),
            ("\"U\"", Color::Blue),
            ("\"B\"", Color::Black),
            ("\"R\"", Color::Red),
            ("\"G\"", Color::Green),
        ];

        for color_pair in color_pairs {
            let val: Color = serde_json::from_str(color_pair.0).unwrap();
            assert_eq!(color_pair.1, val);
        }
    }

    #[test]
    fn test_layout() {
        use super::Layout;

        let layout_pairs = vec![
            ("\"normal\"", Layout::Normal),
            ("\"split\"", Layout::Split),
            ("\"flip\"", Layout::Flip),
            ("\"transform\"", Layout::Transform),
            ("\"meld\"", Layout::Meld),
            ("\"leveler\"", Layout::Leveler),
            ("\"saga\"", Layout::Saga),
            ("\"planar\"", Layout::Planar),
            ("\"scheme\"", Layout::Scheme),
            ("\"vanguard\"", Layout::Vanguard),
            ("\"token\"", Layout::Token),
            ("\"double_faced_token\"", Layout::DoubleFacedToken),
            ("\"emblem\"", Layout::Emblem),
            ("\"augment\"", Layout::Augment),
            ("\"host\"", Layout::Host),
        ];

        for layout_pair in layout_pairs {
            let val: Layout = serde_json::from_str(layout_pair.0).unwrap();
            assert_eq!(layout_pair.1, val);
        }
    }

    #[test]
    fn test_frame_effect() {
        use super::FrameEffect;

        let frame_effect_pairs = vec![
            ("\"legendary\"", FrameEffect::Legendary),
            ("\"miracle\"", FrameEffect::Miracle),
            ("\"nyxtouched\"", FrameEffect::NyxTouched),
            ("\"draft\"", FrameEffect::Draft),
            ("\"devoid\"", FrameEffect::Devoid),
            ("\"tombstone\"", FrameEffect::Tombstone),
            ("\"colorshifted\"", FrameEffect::ColorShifted),
            ("\"sunmoondfc\"", FrameEffect::SunMoonDfc),
            ("\"compasslanddfc\"", FrameEffect::CompassLandDfc),
            ("\"originpwdfc\"", FrameEffect::OriginPwDfc),
            ("\"mooneldrazidfc\"", FrameEffect::MoonEldraziDfc),
        ];

        for frame_effect_pair in frame_effect_pairs {
            let val: FrameEffect = serde_json::from_str(frame_effect_pair.0).unwrap();
            assert_eq!(frame_effect_pair.1, val);
        }
    }

    #[test]
    fn test_frame() {
        use super::Frame;

        let frame_pairs = vec![
            ("\"1993\"", Frame::Year1993),
            ("\"1997\"", Frame::Year1997),
            ("\"2003\"", Frame::Year2003),
            ("\"2015\"", Frame::Year2015),
            ("\"future\"", Frame::Future),
        ];

        for frame_pair in frame_pairs {
            let val: Frame = serde_json::from_str(frame_pair.0).unwrap();
            assert_eq!(frame_pair.1, val);
        }
    }

    #[test]
    fn test_game() {
        use super::Game;

        let game_pairs = vec![
            ("\"paper\"", Game::Paper),
            ("\"arena\"", Game::Arena),
            ("\"mtgo\"", Game::Mtgo),
        ];

        for game_pair in game_pairs {
            let val: Game = serde_json::from_str(game_pair.0).unwrap();
            assert_eq!(game_pair.1, val);
        }
    }

    #[test]
    fn test_rarity() {
        use super::Rarity;

        let rarity_pairs = vec![
            ("\"common\"", Rarity::Common),
            ("\"uncommon\"", Rarity::Uncommon),
            ("\"rare\"", Rarity::Rare),
            ("\"mythic\"", Rarity::Mythic),
        ];

        for rarity_pair in rarity_pairs {
            let val: Rarity = serde_json::from_str(rarity_pair.0).unwrap();
            assert_eq!(rarity_pair.1, val);
        }
    }

    #[test]
    fn test_legality() {
        use super::Legality;

        let legality_pair = vec![
            ("\"not_legal\"", Legality::NotLegal),
            ("\"legal\"", Legality::Legal),
            ("\"banned\"", Legality::Banned),
            ("\"restricted\"", Legality::Restricted),
        ];

        for legality_pair in legality_pair {
            let val: Legality = serde_json::from_str(legality_pair.0).unwrap();
            assert_eq!(legality_pair.1, val);
        }
    }

    #[test]
    fn test_card_face() {}

    #[test]
    fn test_related_card() {}

    #[test]
    fn test_legalities() {}

    #[test]
    fn test_image_uris() {
        use super::ImageUris;
        use super::Uri;
        use http;

        let target_image_uris = ImageUris {
            small: Some(Uri("https://testuri.com/path/1"
                .parse::<http::Uri>()
                .unwrap())),
            normal: Some(Uri("https://testuri.com/path/2"
                .parse::<http::Uri>()
                .unwrap())),
            large: Some(Uri("https://testuri.com/path/3"
                .parse::<http::Uri>()
                .unwrap())),
            png: None,
            art_crop: None,
            border_crop: None,
        };

        let image_uris: ImageUris = serde_json::from_str(
            r#"
            {
                "small": "https://testuri.com/path/1",
                "normal": "https://testuri.com/path/2",
                "large": "https://testuri.com/path/3"
            }
            "#,
        )
        .unwrap();

        assert_eq!(target_image_uris, image_uris);
    }

    #[test]
    fn test_prices() {
        use super::Price;
        use super::Prices;

        let target_prices = Prices {
            usd: Some(Price(15.44)),
            usd_foil: Some(Price(37.12)),
            eur: None,
            tix: None,
        };

        let prices = serde_json::from_str(
            r#"
            {
                "usd": "15.44",
                "usd_foil": "37.12"
            }
            "#,
        )
        .unwrap();

        assert_eq!(target_prices, prices);
    }

    #[test]
    fn test_purchase_uris() {
        use super::PurchaseUris;
        use super::Uri;
        use http;

        let target_purchase_uris = PurchaseUris {
            tcgplayer: Some(Uri("https://testuri.com/path/1"
                .parse::<http::Uri>()
                .unwrap())),
            cardmarket: None,
            cardhoarder: None,
        };

        let purchase_uris = serde_json::from_str(
            r#"
            {
                "tcgplayer": "https://testuri.com/path/1"
            }
            "#,
        )
        .unwrap();

        assert_eq!(target_purchase_uris, purchase_uris);
    }

    #[test]
    fn test_related_uris() {
        use super::RelatedUris;
        use super::Uri;
        use http;

        let target_related_uris = RelatedUris {
            tcgplayer_decks: Some(Uri("https://testuri.com/path/1"
                .parse::<http::Uri>()
                .unwrap())),
            edhrec: None,
            mtgtop8: None,
        };

        let related_uris = serde_json::from_str(
            r#"
            {
                "tcgplayer_decks": "https://testuri.com/path/1"
            }
            "#,
        )
        .unwrap();

        assert_eq!(target_related_uris, related_uris);
    }
}
