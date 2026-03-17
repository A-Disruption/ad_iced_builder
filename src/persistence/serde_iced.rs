/// Custom serde helpers for iced types that don't implement Serialize/Deserialize.
///
/// Usage: `#[serde(with = "serde_iced::color")]` on struct fields.
use iced::{Alignment, Color, Length, Padding, Point, Vector};
use serde::{Deserialize, Serialize};

// ─── Internal helper types ────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Copy)]
pub(super) enum SerLength {
    Fill,
    Shrink,
    Fixed { value: f32 },
    FillPortion { portion: u16 },
}

impl From<Length> for SerLength {
    fn from(l: Length) -> Self {
        match l {
            Length::Fill => Self::Fill,
            Length::Shrink => Self::Shrink,
            Length::Fixed(v) => Self::Fixed { value: v },
            Length::FillPortion(p) => Self::FillPortion { portion: p },
        }
    }
}

impl From<SerLength> for Length {
    fn from(l: SerLength) -> Self {
        match l {
            SerLength::Fill => Length::Fill,
            SerLength::Shrink => Length::Shrink,
            SerLength::Fixed { value } => Length::Fixed(value),
            SerLength::FillPortion { portion } => Length::FillPortion(portion),
        }
    }
}

// ─── color ────────────────────────────────────────────────────────────────────

pub mod color {
    use iced::Color;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(c: &Color, s: S) -> Result<S::Ok, S::Error> {
        [c.r, c.g, c.b, c.a].serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Color, D::Error> {
        let [r, g, b, a] = <[f32; 4]>::deserialize(d)?;
        Ok(Color { r, g, b, a })
    }
}

// ─── length ───────────────────────────────────────────────────────────────────

pub mod length {
    use iced::Length;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::SerLength;

    pub fn serialize<S: Serializer>(l: &Length, s: S) -> Result<S::Ok, S::Error> {
        SerLength::from(*l).serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Length, D::Error> {
        Ok(SerLength::deserialize(d)?.into())
    }
}

pub mod length_opt {
    use iced::Length;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::SerLength;

    pub fn serialize<S: Serializer>(v: &Option<Length>, s: S) -> Result<S::Ok, S::Error> {
        v.map(|l| SerLength::from(l)).serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Length>, D::Error> {
        Ok(Option::<SerLength>::deserialize(d)?.map(Into::into))
    }
}

// ─── padding ─────────────────────────────────────────────────────────────────

pub mod padding {
    use iced::Padding;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    struct SerPadding {
        top: f32,
        bottom: f32,
        left: f32,
        right: f32,
    }

    pub fn serialize<S: Serializer>(p: &Padding, s: S) -> Result<S::Ok, S::Error> {
        SerPadding {
            top: p.top,
            bottom: p.bottom,
            left: p.left,
            right: p.right,
        }
        .serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Padding, D::Error> {
        let s = SerPadding::deserialize(d)?;
        Ok(Padding {
            top: s.top,
            bottom: s.bottom,
            left: s.left,
            right: s.right,
        })
    }
}

// ─── vector ──────────────────────────────────────────────────────────────────

pub mod vector {
    use iced::Vector;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(v: &Vector, s: S) -> Result<S::Ok, S::Error> {
        [v.x, v.y].serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vector, D::Error> {
        let [x, y] = <[f32; 2]>::deserialize(d)?;
        Ok(Vector { x, y })
    }
}

// ─── point ───────────────────────────────────────────────────────────────────

pub mod point {
    use iced::Point;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(p: &Point, s: S) -> Result<S::Ok, S::Error> {
        [p.x, p.y].serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Point, D::Error> {
        let [x, y] = <[f32; 2]>::deserialize(d)?;
        Ok(Point { x, y })
    }
}

// ─── alignment ───────────────────────────────────────────────────────────────

pub mod alignment {
    use iced::Alignment;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    enum SerAlignment {
        Start,
        Center,
        End,
    }

    pub fn serialize<S: Serializer>(a: &Alignment, s: S) -> Result<S::Ok, S::Error> {
        let ser = match a {
            Alignment::Start => SerAlignment::Start,
            Alignment::Center => SerAlignment::Center,
            Alignment::End => SerAlignment::End,
        };
        ser.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Alignment, D::Error> {
        Ok(match SerAlignment::deserialize(d)? {
            SerAlignment::Start => Alignment::Start,
            SerAlignment::Center => Alignment::Center,
            SerAlignment::End => Alignment::End,
        })
    }
}

// ─── text_line_height ─────────────────────────────────────────────────────────

pub mod text_line_height {
    use iced::{Pixels, widget::text};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    enum SerLineHeight {
        Relative(f32),
        Absolute(f32),
    }

    pub fn serialize<S: Serializer>(lh: &text::LineHeight, s: S) -> Result<S::Ok, S::Error> {
        let ser = match lh {
            text::LineHeight::Relative(v) => SerLineHeight::Relative(*v),
            text::LineHeight::Absolute(px) => SerLineHeight::Absolute(px.0),
        };
        ser.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<text::LineHeight, D::Error> {
        Ok(match SerLineHeight::deserialize(d)? {
            SerLineHeight::Relative(v) => text::LineHeight::Relative(v),
            SerLineHeight::Absolute(v) => text::LineHeight::Absolute(Pixels(v)),
        })
    }
}

// ─── text_wrapping ────────────────────────────────────────────────────────────

pub mod text_wrapping {
    use iced::widget::text;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    enum SerWrapping {
        None,
        Word,
        Glyph,
        WordOrGlyph,
    }

    pub fn serialize<S: Serializer>(w: &text::Wrapping, s: S) -> Result<S::Ok, S::Error> {
        let ser = match w {
            text::Wrapping::None => SerWrapping::None,
            text::Wrapping::Word => SerWrapping::Word,
            text::Wrapping::Glyph => SerWrapping::Glyph,
            text::Wrapping::WordOrGlyph => SerWrapping::WordOrGlyph,
        };
        ser.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<text::Wrapping, D::Error> {
        Ok(match SerWrapping::deserialize(d)? {
            SerWrapping::None => text::Wrapping::None,
            SerWrapping::Word => text::Wrapping::Word,
            SerWrapping::Glyph => text::Wrapping::Glyph,
            SerWrapping::WordOrGlyph => text::Wrapping::WordOrGlyph,
        })
    }
}

// ─── text_shaping ─────────────────────────────────────────────────────────────

pub mod text_shaping {
    use iced::widget::text;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    enum SerShaping {
        Basic,
        Advanced,
        Auto,
    }

    pub fn serialize<S: Serializer>(sh: &text::Shaping, s: S) -> Result<S::Ok, S::Error> {
        let ser = match sh {
            text::Shaping::Basic => SerShaping::Basic,
            text::Shaping::Advanced => SerShaping::Advanced,
            text::Shaping::Auto => SerShaping::Auto,
        };
        ser.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<text::Shaping, D::Error> {
        Ok(match SerShaping::deserialize(d)? {
            SerShaping::Basic => text::Shaping::Basic,
            SerShaping::Advanced => text::Shaping::Advanced,
            SerShaping::Auto => text::Shaping::Auto,
        })
    }
}

// ─── text_alignment ───────────────────────────────────────────────────────────

pub mod text_alignment {
    use iced::widget::text;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    enum SerTextAlignment {
        Default,
        Left,
        Center,
        Right,
        Justified,
    }

    pub fn serialize<S: Serializer>(a: &text::Alignment, s: S) -> Result<S::Ok, S::Error> {
        let ser = match a {
            text::Alignment::Default => SerTextAlignment::Default,
            text::Alignment::Left => SerTextAlignment::Left,
            text::Alignment::Center => SerTextAlignment::Center,
            text::Alignment::Right => SerTextAlignment::Right,
            text::Alignment::Justified => SerTextAlignment::Justified,
        };
        ser.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<text::Alignment, D::Error> {
        Ok(match SerTextAlignment::deserialize(d)? {
            SerTextAlignment::Default => text::Alignment::Default,
            SerTextAlignment::Left => text::Alignment::Left,
            SerTextAlignment::Center => text::Alignment::Center,
            SerTextAlignment::Right => text::Alignment::Right,
            SerTextAlignment::Justified => text::Alignment::Justified,
        })
    }
}

// ─── vertical_alignment ───────────────────────────────────────────────────────

pub mod vertical_alignment {
    use iced::alignment::Vertical;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    enum SerVertical {
        Top,
        Center,
        Bottom,
    }

    pub fn serialize<S: Serializer>(v: &Vertical, s: S) -> Result<S::Ok, S::Error> {
        let ser = match v {
            Vertical::Top => SerVertical::Top,
            Vertical::Center => SerVertical::Center,
            Vertical::Bottom => SerVertical::Bottom,
        };
        ser.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vertical, D::Error> {
        Ok(match SerVertical::deserialize(d)? {
            SerVertical::Top => Vertical::Top,
            SerVertical::Center => Vertical::Center,
            SerVertical::Bottom => Vertical::Bottom,
        })
    }
}

// ─── scrollable_direction ─────────────────────────────────────────────────────
// Only stores the direction variant — scrollbar properties use defaults on load.

pub mod scrollable_direction {
    use iced::widget::scrollable::{Direction, Scrollbar};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    enum SerDirection {
        Vertical,
        Horizontal,
        Both,
    }

    pub fn serialize<S: Serializer>(d: &Direction, s: S) -> Result<S::Ok, S::Error> {
        let ser = match d {
            Direction::Vertical(_) => SerDirection::Vertical,
            Direction::Horizontal(_) => SerDirection::Horizontal,
            Direction::Both { .. } => SerDirection::Both,
        };
        ser.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Direction, D::Error> {
        Ok(match SerDirection::deserialize(d)? {
            SerDirection::Vertical => Direction::Vertical(Scrollbar::default()),
            SerDirection::Horizontal => Direction::Horizontal(Scrollbar::default()),
            SerDirection::Both => Direction::Both {
                vertical: Scrollbar::default(),
                horizontal: Scrollbar::default(),
            },
        })
    }
}

// ─── scrollable_anchor ────────────────────────────────────────────────────────

pub mod scrollable_anchor {
    use iced::widget::scrollable::Anchor;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    enum SerAnchor {
        Start,
        End,
    }

    pub fn serialize<S: Serializer>(a: &Anchor, s: S) -> Result<S::Ok, S::Error> {
        let ser = match a {
            Anchor::Start => SerAnchor::Start,
            Anchor::End => SerAnchor::End,
        };
        ser.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Anchor, D::Error> {
        Ok(match SerAnchor::deserialize(d)? {
            SerAnchor::Start => Anchor::Start,
            SerAnchor::End => Anchor::End,
        })
    }
}

// ─── text_editor_content ──────────────────────────────────────────────────────
// Serializes as the raw text string; reconstructs Content on load.

pub mod text_editor_content {
    use iced::widget::text_editor;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(c: &text_editor::Content, s: S) -> Result<S::Ok, S::Error> {
        c.text().serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<text_editor::Content, D::Error> {
        let text = String::deserialize(d)?;
        Ok(text_editor::Content::with_text(&text))
    }
}

// ─── iced_theme ──────────────────────────────────────────────────────────────
// Serializes iced::Theme as its string name; Custom theme falls back to Dark.

pub mod iced_theme_opt {
    use iced::Theme;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Public accessor for use in `main.rs` `to_project_data()`.
    pub fn theme_key_str(t: &Theme) -> &'static str {
        theme_key(t)
    }

    /// Public accessor for use in `main.rs` `from_project_data()`.
    pub fn theme_from_key_str(name: &str) -> Option<Theme> {
        theme_from_key(name)
    }

    fn theme_key(t: &Theme) -> &'static str {
        match t {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::Dracula => "Dracula",
            Theme::Nord => "Nord",
            Theme::SolarizedLight => "SolarizedLight",
            Theme::SolarizedDark => "SolarizedDark",
            Theme::GruvboxLight => "GruvboxLight",
            Theme::GruvboxDark => "GruvboxDark",
            Theme::CatppuccinLatte => "CatppuccinLatte",
            Theme::CatppuccinFrappe => "CatppuccinFrappe",
            Theme::CatppuccinMacchiato => "CatppuccinMacchiato",
            Theme::CatppuccinMocha => "CatppuccinMocha",
            Theme::TokyoNight => "TokyoNight",
            Theme::TokyoNightStorm => "TokyoNightStorm",
            Theme::TokyoNightLight => "TokyoNightLight",
            Theme::KanagawaWave => "KanagawaWave",
            Theme::KanagawaDragon => "KanagawaDragon",
            Theme::KanagawaLotus => "KanagawaLotus",
            Theme::Moonfly => "Moonfly",
            Theme::Nightfly => "Nightfly",
            Theme::Oxocarbon => "Oxocarbon",
            Theme::Ferra => "Ferra",
            Theme::Custom(_) => "Dark",
        }
    }

    pub fn serialize<S: Serializer>(t: &Option<Theme>, s: S) -> Result<S::Ok, S::Error> {
        t.as_ref().map(theme_key).serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Theme>, D::Error> {
        let name = Option::<String>::deserialize(d)?;
        Ok(name.and_then(|n| theme_from_key(&n)))
    }

    fn theme_from_key(name: &str) -> Option<Theme> {
        Some(match name {
            "Light" => Theme::Light,
            "Dark" => Theme::Dark,
            "Dracula" => Theme::Dracula,
            "Nord" => Theme::Nord,
            "SolarizedLight" => Theme::SolarizedLight,
            "SolarizedDark" => Theme::SolarizedDark,
            "GruvboxLight" => Theme::GruvboxLight,
            "GruvboxDark" => Theme::GruvboxDark,
            "CatppuccinLatte" => Theme::CatppuccinLatte,
            "CatppuccinFrappe" => Theme::CatppuccinFrappe,
            "CatppuccinMacchiato" => Theme::CatppuccinMacchiato,
            "CatppuccinMocha" => Theme::CatppuccinMocha,
            "TokyoNight" => Theme::TokyoNight,
            "TokyoNightStorm" => Theme::TokyoNightStorm,
            "TokyoNightLight" => Theme::TokyoNightLight,
            "KanagawaWave" => Theme::KanagawaWave,
            "KanagawaDragon" => Theme::KanagawaDragon,
            "KanagawaLotus" => Theme::KanagawaLotus,
            "Moonfly" => Theme::Moonfly,
            "Nightfly" => Theme::Nightfly,
            "Oxocarbon" => Theme::Oxocarbon,
            "Ferra" => Theme::Ferra,
            _ => return None,
        })
    }
}

// ─── option_color ─────────────────────────────────────────────────────────────
// Serializes Option<Color> as Option<[f32; 4]>; used by StatusColorOverride fields.

pub mod option_color {
    use iced::Color;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(c: &Option<Color>, s: S) -> Result<S::Ok, S::Error> {
        c.as_ref().map(|c| [c.r, c.g, c.b, c.a]).serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Color>, D::Error> {
        let arr = Option::<[f32; 4]>::deserialize(d)?;
        Ok(arr.map(|[r, g, b, a]| Color { r, g, b, a }))
    }
}
