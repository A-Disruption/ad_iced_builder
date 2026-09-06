// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// 3e914891b42fdb713e5a93f21664c7c5391998f0476609bdfb802f725f16dd50

//! Icon fonts for this crate.
//!
//! Each font here is a subset containing only the icons this
//! project asks for, cut from these upstream sets:
//!
//! - **Lucide** — ISC — <https://lucide.dev/icons>
//!
//! Redistributing these fonts means carrying their licenses with you.

#![allow(dead_code)]

use iced::Font;
use iced::widget::Text;

/// Lucide subset — ISC.
pub const FONT_LUCIDE: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// Every font this module needs.
///
/// Register all of them before rendering any icon:
///
/// ```ignore
/// let mut app = iced::application(App::new, App::update, App::view);
///
/// for font in icon::FONTS {
///     app = app.font(*font);
/// }
///
/// app.run()
/// ```
// One entry per line stays readable as the list grows, and keeps a
// `cargo fmt` in the consuming crate from rewriting generated output.
#[rustfmt::skip]
pub const FONTS: &[&[u8]] = &[
    FONT_LUCIDE,
];

/// The single font this module uses.
pub const FONT: &[u8] = FONT_LUCIDE;

/// One of the icon sets this module draws from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Family {
    /// Identifier, matching [`Icon::family`].
    pub id: &'static str,
    /// Human-readable name.
    pub name: &'static str,
    /// The `iced_lucide` Cargo feature that provides this set.
    pub feature: &'static str,
    /// License the icons are published under.
    pub license: &'static str,
    /// Where to browse the set.
    pub url: &'static str,
}

/// The icon sets this module draws from.
///
/// [`Icon::family`] holds an `id` from this list, so it is both what a
/// filter UI wants and where an "about" screen finds its attribution.
#[rustfmt::skip]
pub const FAMILIES: &[Family] = &[
    Family { id: "lucide", name: "Lucide", feature: "lucide", license: "ISC", url: "https://lucide.dev/icons" },
];

const LUCIDE: Font = Font::new("lucide");

/// An icon: its name, the character that draws it, and the font that carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Icon {
    /// The upstream icon name.
    pub name: &'static str,
    /// The character to render.
    pub codepoint: &'static str,
    /// The font the character belongs to.
    pub font: Font,
    /// Identifier of the set this icon came from, matching [`FAMILIES`].
    pub family: &'static str,
}

/// Every icon in this module.
///
/// Use this with [`render`] to build a picker:
///
/// ```ignore
/// for icon in ALL_ICONS {
///     button(render(*icon)).on_press(Message::Pick(icon.name))
/// }
/// ```
// One icon per line; rustfmt would otherwise explode each struct
// literal across five lines and make this unreadable.
#[rustfmt::skip]
pub const ALL_ICONS: &[Icon] = &[
    Icon { name: "a-arrow-down", codepoint: "\u{E585}", font: LUCIDE, family: "lucide" },
    Icon { name: "a-arrow-up", codepoint: "\u{E586}", font: LUCIDE, family: "lucide" },
    Icon { name: "a-large-small", codepoint: "\u{E587}", font: LUCIDE, family: "lucide" },
    Icon { name: "accessibility", codepoint: "\u{E297}", font: LUCIDE, family: "lucide" },
    Icon { name: "activity", codepoint: "\u{E038}", font: LUCIDE, family: "lucide" },
    Icon { name: "activity-square", codepoint: "\u{E4B4}", font: LUCIDE, family: "lucide" },
    Icon { name: "ad", codepoint: "\u{E700}", font: LUCIDE, family: "lucide" },
    Icon { name: "air-vent", codepoint: "\u{E34D}", font: LUCIDE, family: "lucide" },
    Icon { name: "airplay", codepoint: "\u{E039}", font: LUCIDE, family: "lucide" },
    Icon { name: "alarm-check", codepoint: "\u{E1EC}", font: LUCIDE, family: "lucide" },
    Icon { name: "alarm-clock", codepoint: "\u{E03A}", font: LUCIDE, family: "lucide" },
    Icon { name: "alarm-clock-check", codepoint: "\u{E1EC}", font: LUCIDE, family: "lucide" },
    Icon { name: "alarm-clock-minus", codepoint: "\u{E1ED}", font: LUCIDE, family: "lucide" },
    Icon { name: "alarm-clock-off", codepoint: "\u{E23B}", font: LUCIDE, family: "lucide" },
    Icon { name: "alarm-clock-plus", codepoint: "\u{E1EE}", font: LUCIDE, family: "lucide" },
    Icon { name: "alarm-minus", codepoint: "\u{E1ED}", font: LUCIDE, family: "lucide" },
    Icon { name: "alarm-plus", codepoint: "\u{E1EE}", font: LUCIDE, family: "lucide" },
    Icon { name: "alarm-smoke", codepoint: "\u{E57B}", font: LUCIDE, family: "lucide" },
    Icon { name: "album", codepoint: "\u{E03B}", font: LUCIDE, family: "lucide" },
    Icon { name: "alert-circle", codepoint: "\u{E077}", font: LUCIDE, family: "lucide" },
    Icon { name: "alert-octagon", codepoint: "\u{E127}", font: LUCIDE, family: "lucide" },
    Icon { name: "alert-triangle", codepoint: "\u{E193}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-center", codepoint: "\u{E182}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-center-horizontal", codepoint: "\u{E26C}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-center-vertical", codepoint: "\u{E26D}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-end-horizontal", codepoint: "\u{E26E}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-end-vertical", codepoint: "\u{E26F}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-horizontal-distribute-center", codepoint: "\u{E03C}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-horizontal-distribute-end", codepoint: "\u{E03D}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-horizontal-distribute-start", codepoint: "\u{E03E}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-horizontal-justify-center", codepoint: "\u{E272}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-horizontal-justify-end", codepoint: "\u{E273}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-horizontal-justify-start", codepoint: "\u{E274}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-horizontal-space-around", codepoint: "\u{E275}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-horizontal-space-between", codepoint: "\u{E276}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-justify", codepoint: "\u{E184}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-left", codepoint: "\u{E185}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-right", codepoint: "\u{E183}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-start-horizontal", codepoint: "\u{E270}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-start-vertical", codepoint: "\u{E271}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-vertical-distribute-center", codepoint: "\u{E27E}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-vertical-distribute-end", codepoint: "\u{E27F}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-vertical-distribute-start", codepoint: "\u{E280}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-vertical-justify-center", codepoint: "\u{E277}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-vertical-justify-end", codepoint: "\u{E278}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-vertical-justify-start", codepoint: "\u{E279}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-vertical-space-around", codepoint: "\u{E27A}", font: LUCIDE, family: "lucide" },
    Icon { name: "align-vertical-space-between", codepoint: "\u{E27B}", font: LUCIDE, family: "lucide" },
    Icon { name: "ambulance", codepoint: "\u{E5BB}", font: LUCIDE, family: "lucide" },
    Icon { name: "ampersand", codepoint: "\u{E49C}", font: LUCIDE, family: "lucide" },
    Icon { name: "ampersands", codepoint: "\u{E49D}", font: LUCIDE, family: "lucide" },
    Icon { name: "amphora", codepoint: "\u{E61B}", font: LUCIDE, family: "lucide" },
    Icon { name: "anchor", codepoint: "\u{E03F}", font: LUCIDE, family: "lucide" },
    Icon { name: "angle", codepoint: "\u{E732}", font: LUCIDE, family: "lucide" },
    Icon { name: "angry", codepoint: "\u{E2FC}", font: LUCIDE, family: "lucide" },
    Icon { name: "annoyed", codepoint: "\u{E2FD}", font: LUCIDE, family: "lucide" },
    Icon { name: "antenna", codepoint: "\u{E4E2}", font: LUCIDE, family: "lucide" },
    Icon { name: "anvil", codepoint: "\u{E580}", font: LUCIDE, family: "lucide" },
    Icon { name: "aperture", codepoint: "\u{E040}", font: LUCIDE, family: "lucide" },
    Icon { name: "app-window", codepoint: "\u{E426}", font: LUCIDE, family: "lucide" },
    Icon { name: "app-window-mac", codepoint: "\u{E5D2}", font: LUCIDE, family: "lucide" },
    Icon { name: "apple", codepoint: "\u{E34E}", font: LUCIDE, family: "lucide" },
    Icon { name: "archive", codepoint: "\u{E041}", font: LUCIDE, family: "lucide" },
    Icon { name: "archive-restore", codepoint: "\u{E2CD}", font: LUCIDE, family: "lucide" },
    Icon { name: "archive-x", codepoint: "\u{E50C}", font: LUCIDE, family: "lucide" },
    Icon { name: "area-chart", codepoint: "\u{E4D3}", font: LUCIDE, family: "lucide" },
    Icon { name: "armchair", codepoint: "\u{E2C0}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-big-down", codepoint: "\u{E1E1}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-big-down-dash", codepoint: "\u{E41D}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-big-left", codepoint: "\u{E1E2}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-big-left-dash", codepoint: "\u{E41E}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-big-right", codepoint: "\u{E1E3}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-big-right-dash", codepoint: "\u{E41F}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-big-up", codepoint: "\u{E1E4}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-big-up-dash", codepoint: "\u{E420}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down", codepoint: "\u{E042}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-0-1", codepoint: "\u{E413}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-01", codepoint: "\u{E413}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-1-0", codepoint: "\u{E414}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-10", codepoint: "\u{E414}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-a-z", codepoint: "\u{E415}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-az", codepoint: "\u{E415}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-circle", codepoint: "\u{E078}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-from-line", codepoint: "\u{E454}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-left", codepoint: "\u{E043}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-left-from-circle", codepoint: "\u{E3F7}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-left-from-square", codepoint: "\u{E5A1}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-left-square", codepoint: "\u{E4B5}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-narrow-wide", codepoint: "\u{E044}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-right", codepoint: "\u{E045}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-right-from-circle", codepoint: "\u{E3F8}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-right-from-square", codepoint: "\u{E5A2}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-right-square", codepoint: "\u{E4B6}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-square", codepoint: "\u{E427}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-to-dot", codepoint: "\u{E44D}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-to-line", codepoint: "\u{E455}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-up", codepoint: "\u{E046}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-wide-narrow", codepoint: "\u{E047}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-z-a", codepoint: "\u{E416}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-down-za", codepoint: "\u{E416}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-left", codepoint: "\u{E048}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-left-circle", codepoint: "\u{E079}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-left-from-line", codepoint: "\u{E456}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-left-right", codepoint: "\u{E24A}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-left-square", codepoint: "\u{E428}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-left-to-line", codepoint: "\u{E457}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-right", codepoint: "\u{E049}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-right-circle", codepoint: "\u{E07A}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-right-from-line", codepoint: "\u{E458}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-right-left", codepoint: "\u{E417}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-right-square", codepoint: "\u{E429}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-right-to-line", codepoint: "\u{E459}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up", codepoint: "\u{E04A}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-0-1", codepoint: "\u{E418}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-01", codepoint: "\u{E418}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-1-0", codepoint: "\u{E419}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-10", codepoint: "\u{E419}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-a-z", codepoint: "\u{E41A}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-az", codepoint: "\u{E41A}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-circle", codepoint: "\u{E07B}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-down", codepoint: "\u{E37D}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-from-dot", codepoint: "\u{E44E}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-from-line", codepoint: "\u{E45A}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-left", codepoint: "\u{E04B}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-left-from-circle", codepoint: "\u{E3F9}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-left-from-square", codepoint: "\u{E5A3}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-left-square", codepoint: "\u{E4B7}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-narrow-wide", codepoint: "\u{E04C}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-right", codepoint: "\u{E04D}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-right-from-circle", codepoint: "\u{E3FA}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-right-from-square", codepoint: "\u{E5A4}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-right-square", codepoint: "\u{E4B8}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-square", codepoint: "\u{E42A}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-to-line", codepoint: "\u{E45B}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-wide-narrow", codepoint: "\u{E41B}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-z-a", codepoint: "\u{E41C}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrow-up-za", codepoint: "\u{E41C}", font: LUCIDE, family: "lucide" },
    Icon { name: "arrows-up-from-line", codepoint: "\u{E4D4}", font: LUCIDE, family: "lucide" },
    Icon { name: "asterisk", codepoint: "\u{E1EF}", font: LUCIDE, family: "lucide" },
    Icon { name: "asterisk-square", codepoint: "\u{E168}", font: LUCIDE, family: "lucide" },
    Icon { name: "astroid", codepoint: "\u{E6F0}", font: LUCIDE, family: "lucide" },
    Icon { name: "at-sign", codepoint: "\u{E04E}", font: LUCIDE, family: "lucide" },
    Icon { name: "atom", codepoint: "\u{E3D7}", font: LUCIDE, family: "lucide" },
    Icon { name: "audio-lines", codepoint: "\u{E55A}", font: LUCIDE, family: "lucide" },
    Icon { name: "audio-lines-off", codepoint: "\u{E737}", font: LUCIDE, family: "lucide" },
    Icon { name: "audio-lines-x", codepoint: "\u{E72E}", font: LUCIDE, family: "lucide" },
    Icon { name: "audio-waveform", codepoint: "\u{E55B}", font: LUCIDE, family: "lucide" },
    Icon { name: "award", codepoint: "\u{E04F}", font: LUCIDE, family: "lucide" },
    Icon { name: "axe", codepoint: "\u{E050}", font: LUCIDE, family: "lucide" },
    Icon { name: "axis-3-d", codepoint: "\u{E2FE}", font: LUCIDE, family: "lucide" },
    Icon { name: "axis-3d", codepoint: "\u{E2FE}", font: LUCIDE, family: "lucide" },
    Icon { name: "baby", codepoint: "\u{E2CE}", font: LUCIDE, family: "lucide" },
    Icon { name: "backpack", codepoint: "\u{E2C8}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge", codepoint: "\u{E474}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-alert", codepoint: "\u{E475}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-cent", codepoint: "\u{E50F}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-check", codepoint: "\u{E241}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-dollar-sign", codepoint: "\u{E476}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-euro", codepoint: "\u{E510}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-help", codepoint: "\u{E47B}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-indian-rupee", codepoint: "\u{E511}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-info", codepoint: "\u{E477}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-japanese-yen", codepoint: "\u{E512}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-minus", codepoint: "\u{E478}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-percent", codepoint: "\u{E479}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-plus", codepoint: "\u{E47A}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-pound-sterling", codepoint: "\u{E513}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-question-mark", codepoint: "\u{E47B}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-russian-ruble", codepoint: "\u{E514}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-swiss-franc", codepoint: "\u{E515}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-turkish-lira", codepoint: "\u{E67E}", font: LUCIDE, family: "lucide" },
    Icon { name: "badge-x", codepoint: "\u{E47C}", font: LUCIDE, family: "lucide" },
    Icon { name: "baggage-claim", codepoint: "\u{E2C9}", font: LUCIDE, family: "lucide" },
    Icon { name: "balloon", codepoint: "\u{E6AF}", font: LUCIDE, family: "lucide" },
    Icon { name: "ban", codepoint: "\u{E051}", font: LUCIDE, family: "lucide" },
    Icon { name: "banana", codepoint: "\u{E34F}", font: LUCIDE, family: "lucide" },
    Icon { name: "bandage", codepoint: "\u{E61D}", font: LUCIDE, family: "lucide" },
    Icon { name: "banknote", codepoint: "\u{E052}", font: LUCIDE, family: "lucide" },
    Icon { name: "banknote-arrow-down", codepoint: "\u{E64C}", font: LUCIDE, family: "lucide" },
    Icon { name: "banknote-arrow-up", codepoint: "\u{E64D}", font: LUCIDE, family: "lucide" },
    Icon { name: "banknote-check", codepoint: "\u{E70C}", font: LUCIDE, family: "lucide" },
    Icon { name: "banknote-x", codepoint: "\u{E64E}", font: LUCIDE, family: "lucide" },
    Icon { name: "bar-chart", codepoint: "\u{E06A}", font: LUCIDE, family: "lucide" },
    Icon { name: "bar-chart-2", codepoint: "\u{E068}", font: LUCIDE, family: "lucide" },
    Icon { name: "bar-chart-3", codepoint: "\u{E2A3}", font: LUCIDE, family: "lucide" },
    Icon { name: "bar-chart-4", codepoint: "\u{E2A4}", font: LUCIDE, family: "lucide" },
    Icon { name: "bar-chart-big", codepoint: "\u{E4A9}", font: LUCIDE, family: "lucide" },
    Icon { name: "bar-chart-horizontal", codepoint: "\u{E2A2}", font: LUCIDE, family: "lucide" },
    Icon { name: "bar-chart-horizontal-big", codepoint: "\u{E4A7}", font: LUCIDE, family: "lucide" },
    Icon { name: "barcode", codepoint: "\u{E533}", font: LUCIDE, family: "lucide" },
    Icon { name: "barrel", codepoint: "\u{E675}", font: LUCIDE, family: "lucide" },
    Icon { name: "baseline", codepoint: "\u{E285}", font: LUCIDE, family: "lucide" },
    Icon { name: "bath", codepoint: "\u{E2AB}", font: LUCIDE, family: "lucide" },
    Icon { name: "battery", codepoint: "\u{E053}", font: LUCIDE, family: "lucide" },
    Icon { name: "battery-charging", codepoint: "\u{E054}", font: LUCIDE, family: "lucide" },
    Icon { name: "battery-full", codepoint: "\u{E055}", font: LUCIDE, family: "lucide" },
    Icon { name: "battery-low", codepoint: "\u{E056}", font: LUCIDE, family: "lucide" },
    Icon { name: "battery-medium", codepoint: "\u{E057}", font: LUCIDE, family: "lucide" },
    Icon { name: "battery-plus", codepoint: "\u{E63E}", font: LUCIDE, family: "lucide" },
    Icon { name: "battery-warning", codepoint: "\u{E3AC}", font: LUCIDE, family: "lucide" },
    Icon { name: "beaker", codepoint: "\u{E058}", font: LUCIDE, family: "lucide" },
    Icon { name: "bean", codepoint: "\u{E38F}", font: LUCIDE, family: "lucide" },
    Icon { name: "bean-off", codepoint: "\u{E390}", font: LUCIDE, family: "lucide" },
    Icon { name: "bed", codepoint: "\u{E2C1}", font: LUCIDE, family: "lucide" },
    Icon { name: "bed-double", codepoint: "\u{E2C2}", font: LUCIDE, family: "lucide" },
    Icon { name: "bed-single", codepoint: "\u{E2C3}", font: LUCIDE, family: "lucide" },
    Icon { name: "beef", codepoint: "\u{E3A5}", font: LUCIDE, family: "lucide" },
    Icon { name: "beef-off", codepoint: "\u{E6DC}", font: LUCIDE, family: "lucide" },
    Icon { name: "beer", codepoint: "\u{E2CF}", font: LUCIDE, family: "lucide" },
    Icon { name: "beer-off", codepoint: "\u{E5D9}", font: LUCIDE, family: "lucide" },
    Icon { name: "bell", codepoint: "\u{E059}", font: LUCIDE, family: "lucide" },
    Icon { name: "bell-check", codepoint: "\u{E6EF}", font: LUCIDE, family: "lucide" },
    Icon { name: "bell-dot", codepoint: "\u{E42B}", font: LUCIDE, family: "lucide" },
    Icon { name: "bell-electric", codepoint: "\u{E57C}", font: LUCIDE, family: "lucide" },
    Icon { name: "bell-minus", codepoint: "\u{E1F0}", font: LUCIDE, family: "lucide" },
    Icon { name: "bell-off", codepoint: "\u{E05A}", font: LUCIDE, family: "lucide" },
    Icon { name: "bell-plus", codepoint: "\u{E1F1}", font: LUCIDE, family: "lucide" },
    Icon { name: "bell-ring", codepoint: "\u{E224}", font: LUCIDE, family: "lucide" },
    Icon { name: "between-horizonal-end", codepoint: "\u{E591}", font: LUCIDE, family: "lucide" },
    Icon { name: "between-horizonal-start", codepoint: "\u{E592}", font: LUCIDE, family: "lucide" },
    Icon { name: "between-horizontal-end", codepoint: "\u{E591}", font: LUCIDE, family: "lucide" },
    Icon { name: "between-horizontal-start", codepoint: "\u{E592}", font: LUCIDE, family: "lucide" },
    Icon { name: "between-vertical-end", codepoint: "\u{E593}", font: LUCIDE, family: "lucide" },
    Icon { name: "between-vertical-start", codepoint: "\u{E594}", font: LUCIDE, family: "lucide" },
    Icon { name: "biceps-flexed", codepoint: "\u{E5EB}", font: LUCIDE, family: "lucide" },
    Icon { name: "bike", codepoint: "\u{E1D2}", font: LUCIDE, family: "lucide" },
    Icon { name: "binary", codepoint: "\u{E1F2}", font: LUCIDE, family: "lucide" },
    Icon { name: "binoculars", codepoint: "\u{E621}", font: LUCIDE, family: "lucide" },
    Icon { name: "biohazard", codepoint: "\u{E441}", font: LUCIDE, family: "lucide" },
    Icon { name: "bird", codepoint: "\u{E3C5}", font: LUCIDE, family: "lucide" },
    Icon { name: "birdhouse", codepoint: "\u{E69A}", font: LUCIDE, family: "lucide" },
    Icon { name: "bitcoin", codepoint: "\u{E05B}", font: LUCIDE, family: "lucide" },
    Icon { name: "blend", codepoint: "\u{E59C}", font: LUCIDE, family: "lucide" },
    Icon { name: "blender", codepoint: "\u{E6F4}", font: LUCIDE, family: "lucide" },
    Icon { name: "blinds", codepoint: "\u{E3C0}", font: LUCIDE, family: "lucide" },
    Icon { name: "blocks", codepoint: "\u{E4FA}", font: LUCIDE, family: "lucide" },
    Icon { name: "bluetooth", codepoint: "\u{E05C}", font: LUCIDE, family: "lucide" },
    Icon { name: "bluetooth-connected", codepoint: "\u{E1B8}", font: LUCIDE, family: "lucide" },
    Icon { name: "bluetooth-off", codepoint: "\u{E1B9}", font: LUCIDE, family: "lucide" },
    Icon { name: "bluetooth-searching", codepoint: "\u{E1BA}", font: LUCIDE, family: "lucide" },
    Icon { name: "bold", codepoint: "\u{E05D}", font: LUCIDE, family: "lucide" },
    Icon { name: "bolt", codepoint: "\u{E58C}", font: LUCIDE, family: "lucide" },
    Icon { name: "bomb", codepoint: "\u{E2FF}", font: LUCIDE, family: "lucide" },
    Icon { name: "bone", codepoint: "\u{E358}", font: LUCIDE, family: "lucide" },
    Icon { name: "bone-fracture", codepoint: "\u{E715}", font: LUCIDE, family: "lucide" },
    Icon { name: "book", codepoint: "\u{E05E}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-a", codepoint: "\u{E544}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-alert", codepoint: "\u{E672}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-audio", codepoint: "\u{E545}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-check", codepoint: "\u{E546}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-copy", codepoint: "\u{E3EC}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-dashed", codepoint: "\u{E3ED}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-down", codepoint: "\u{E3EE}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-headphones", codepoint: "\u{E547}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-heart", codepoint: "\u{E548}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-image", codepoint: "\u{E549}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-key", codepoint: "\u{E3EF}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-lock", codepoint: "\u{E3F0}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-marked", codepoint: "\u{E3F1}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-minus", codepoint: "\u{E3F2}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-open", codepoint: "\u{E05F}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-open-check", codepoint: "\u{E381}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-open-text", codepoint: "\u{E54A}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-plus", codepoint: "\u{E3F3}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-search", codepoint: "\u{E6AB}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-template", codepoint: "\u{E3ED}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-text", codepoint: "\u{E54B}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-type", codepoint: "\u{E54C}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-up", codepoint: "\u{E3F4}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-up-2", codepoint: "\u{E4A6}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-user", codepoint: "\u{E54D}", font: LUCIDE, family: "lucide" },
    Icon { name: "book-x", codepoint: "\u{E3F5}", font: LUCIDE, family: "lucide" },
    Icon { name: "bookmark", codepoint: "\u{E060}", font: LUCIDE, family: "lucide" },
    Icon { name: "bookmark-check", codepoint: "\u{E51F}", font: LUCIDE, family: "lucide" },
    Icon { name: "bookmark-minus", codepoint: "\u{E23C}", font: LUCIDE, family: "lucide" },
    Icon { name: "bookmark-off", codepoint: "\u{E6DF}", font: LUCIDE, family: "lucide" },
    Icon { name: "bookmark-plus", codepoint: "\u{E23D}", font: LUCIDE, family: "lucide" },
    Icon { name: "bookmark-x", codepoint: "\u{E520}", font: LUCIDE, family: "lucide" },
    Icon { name: "boom-box", codepoint: "\u{E4EE}", font: LUCIDE, family: "lucide" },
    Icon { name: "bot", codepoint: "\u{E1BB}", font: LUCIDE, family: "lucide" },
    Icon { name: "bot-message-square", codepoint: "\u{E5CE}", font: LUCIDE, family: "lucide" },
    Icon { name: "bot-off", codepoint: "\u{E5E0}", font: LUCIDE, family: "lucide" },
    Icon { name: "bottle-wine", codepoint: "\u{E67B}", font: LUCIDE, family: "lucide" },
    Icon { name: "bow-arrow", codepoint: "\u{E65E}", font: LUCIDE, family: "lucide" },
    Icon { name: "box", codepoint: "\u{E061}", font: LUCIDE, family: "lucide" },
    Icon { name: "box-select", codepoint: "\u{E1CB}", font: LUCIDE, family: "lucide" },
    Icon { name: "boxes", codepoint: "\u{E2D0}", font: LUCIDE, family: "lucide" },
    Icon { name: "braces", codepoint: "\u{E36A}", font: LUCIDE, family: "lucide" },
    Icon { name: "brackets", codepoint: "\u{E443}", font: LUCIDE, family: "lucide" },
    Icon { name: "brain", codepoint: "\u{E3C6}", font: LUCIDE, family: "lucide" },
    Icon { name: "brain-circuit", codepoint: "\u{E3C7}", font: LUCIDE, family: "lucide" },
    Icon { name: "brain-cog", codepoint: "\u{E3C8}", font: LUCIDE, family: "lucide" },
    Icon { name: "brick-wall", codepoint: "\u{E581}", font: LUCIDE, family: "lucide" },
    Icon { name: "brick-wall-fire", codepoint: "\u{E653}", font: LUCIDE, family: "lucide" },
    Icon { name: "brick-wall-shield", codepoint: "\u{E690}", font: LUCIDE, family: "lucide" },
    Icon { name: "briefcase", codepoint: "\u{E062}", font: LUCIDE, family: "lucide" },
    Icon { name: "briefcase-business", codepoint: "\u{E5D5}", font: LUCIDE, family: "lucide" },
    Icon { name: "briefcase-conveyor-belt", codepoint: "\u{E62B}", font: LUCIDE, family: "lucide" },
    Icon { name: "briefcase-medical", codepoint: "\u{E5D6}", font: LUCIDE, family: "lucide" },
    Icon { name: "bring-to-front", codepoint: "\u{E4EF}", font: LUCIDE, family: "lucide" },
    Icon { name: "broccoli", codepoint: "\u{E6F5}", font: LUCIDE, family: "lucide" },
    Icon { name: "broom", codepoint: "\u{E730}", font: LUCIDE, family: "lucide" },
    Icon { name: "broom-sparkles", codepoint: "\u{E72F}", font: LUCIDE, family: "lucide" },
    Icon { name: "brush", codepoint: "\u{E1D3}", font: LUCIDE, family: "lucide" },
    Icon { name: "brush-cleaning", codepoint: "\u{E666}", font: LUCIDE, family: "lucide" },
    Icon { name: "bubbles", codepoint: "\u{E654}", font: LUCIDE, family: "lucide" },
    Icon { name: "bug", codepoint: "\u{E20C}", font: LUCIDE, family: "lucide" },
    Icon { name: "bug-off", codepoint: "\u{E50D}", font: LUCIDE, family: "lucide" },
    Icon { name: "bug-play", codepoint: "\u{E50E}", font: LUCIDE, family: "lucide" },
    Icon { name: "building", codepoint: "\u{E1CC}", font: LUCIDE, family: "lucide" },
    Icon { name: "building-2", codepoint: "\u{E290}", font: LUCIDE, family: "lucide" },
    Icon { name: "bus", codepoint: "\u{E1D4}", font: LUCIDE, family: "lucide" },
    Icon { name: "bus-front", codepoint: "\u{E4FB}", font: LUCIDE, family: "lucide" },
    Icon { name: "cable", codepoint: "\u{E4E3}", font: LUCIDE, family: "lucide" },
    Icon { name: "cable-car", codepoint: "\u{E4FC}", font: LUCIDE, family: "lucide" },
    Icon { name: "cake", codepoint: "\u{E344}", font: LUCIDE, family: "lucide" },
    Icon { name: "cake-slice", codepoint: "\u{E4B9}", font: LUCIDE, family: "lucide" },
    Icon { name: "calculator", codepoint: "\u{E1BC}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar", codepoint: "\u{E063}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-1", codepoint: "\u{E630}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-arrow-down", codepoint: "\u{E5FE}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-arrow-up", codepoint: "\u{E5FF}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-check", codepoint: "\u{E2B7}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-check-2", codepoint: "\u{E2B8}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-clock", codepoint: "\u{E304}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-cog", codepoint: "\u{E5ED}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-days", codepoint: "\u{E2B9}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-fold", codepoint: "\u{E5B4}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-heart", codepoint: "\u{E305}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-minus", codepoint: "\u{E2BA}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-minus-2", codepoint: "\u{E5B5}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-off", codepoint: "\u{E2BB}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-plus", codepoint: "\u{E2BC}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-plus-2", codepoint: "\u{E5B6}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-range", codepoint: "\u{E2BD}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-search", codepoint: "\u{E306}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-sync", codepoint: "\u{E636}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-x", codepoint: "\u{E2BE}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendar-x-2", codepoint: "\u{E2BF}", font: LUCIDE, family: "lucide" },
    Icon { name: "calendars", codepoint: "\u{E6A7}", font: LUCIDE, family: "lucide" },
    Icon { name: "camera", codepoint: "\u{E064}", font: LUCIDE, family: "lucide" },
    Icon { name: "camera-off", codepoint: "\u{E065}", font: LUCIDE, family: "lucide" },
    Icon { name: "candlestick-chart", codepoint: "\u{E4A8}", font: LUCIDE, family: "lucide" },
    Icon { name: "candy", codepoint: "\u{E391}", font: LUCIDE, family: "lucide" },
    Icon { name: "candy-cane", codepoint: "\u{E4BA}", font: LUCIDE, family: "lucide" },
    Icon { name: "candy-off", codepoint: "\u{E392}", font: LUCIDE, family: "lucide" },
    Icon { name: "cannabis", codepoint: "\u{E5D4}", font: LUCIDE, family: "lucide" },
    Icon { name: "cannabis-off", codepoint: "\u{E6E0}", font: LUCIDE, family: "lucide" },
    Icon { name: "captions", codepoint: "\u{E3A4}", font: LUCIDE, family: "lucide" },
    Icon { name: "captions-off", codepoint: "\u{E5C1}", font: LUCIDE, family: "lucide" },
    Icon { name: "car", codepoint: "\u{E1D5}", font: LUCIDE, family: "lucide" },
    Icon { name: "car-battery", codepoint: "\u{E735}", font: LUCIDE, family: "lucide" },
    Icon { name: "car-front", codepoint: "\u{E4FD}", font: LUCIDE, family: "lucide" },
    Icon { name: "car-taxi-front", codepoint: "\u{E4FE}", font: LUCIDE, family: "lucide" },
    Icon { name: "caravan", codepoint: "\u{E539}", font: LUCIDE, family: "lucide" },
    Icon { name: "card-sim", codepoint: "\u{E671}", font: LUCIDE, family: "lucide" },
    Icon { name: "carrot", codepoint: "\u{E25A}", font: LUCIDE, family: "lucide" },
    Icon { name: "case-lower", codepoint: "\u{E3D8}", font: LUCIDE, family: "lucide" },
    Icon { name: "case-sensitive", codepoint: "\u{E3D9}", font: LUCIDE, family: "lucide" },
    Icon { name: "case-upper", codepoint: "\u{E3DA}", font: LUCIDE, family: "lucide" },
    Icon { name: "cassette-tape", codepoint: "\u{E4CA}", font: LUCIDE, family: "lucide" },
    Icon { name: "cast", codepoint: "\u{E066}", font: LUCIDE, family: "lucide" },
    Icon { name: "castle", codepoint: "\u{E3E0}", font: LUCIDE, family: "lucide" },
    Icon { name: "cat", codepoint: "\u{E38C}", font: LUCIDE, family: "lucide" },
    Icon { name: "cctv", codepoint: "\u{E57D}", font: LUCIDE, family: "lucide" },
    Icon { name: "cctv-off", codepoint: "\u{E6D8}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-area", codepoint: "\u{E4D3}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-bar", codepoint: "\u{E2A2}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-bar-big", codepoint: "\u{E4A7}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-bar-decreasing", codepoint: "\u{E607}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-bar-increasing", codepoint: "\u{E608}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-bar-stacked", codepoint: "\u{E609}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-candlestick", codepoint: "\u{E4A8}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-column", codepoint: "\u{E2A3}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-column-big", codepoint: "\u{E4A9}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-column-decreasing", codepoint: "\u{E067}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-column-increasing", codepoint: "\u{E2A4}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-column-stacked", codepoint: "\u{E60A}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-gantt", codepoint: "\u{E624}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-line", codepoint: "\u{E2A5}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-network", codepoint: "\u{E60B}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-no-axes-column", codepoint: "\u{E068}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-no-axes-column-decreasing", codepoint: "\u{E069}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-no-axes-column-increasing", codepoint: "\u{E06A}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-no-axes-combined", codepoint: "\u{E60C}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-no-axes-gantt", codepoint: "\u{E4C4}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-pie", codepoint: "\u{E06B}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-scatter", codepoint: "\u{E48A}", font: LUCIDE, family: "lucide" },
    Icon { name: "chart-spline", codepoint: "\u{E60D}", font: LUCIDE, family: "lucide" },
    Icon { name: "check", codepoint: "\u{E06C}", font: LUCIDE, family: "lucide" },
    Icon { name: "check-check", codepoint: "\u{E38E}", font: LUCIDE, family: "lucide" },
    Icon { name: "check-circle", codepoint: "\u{E07C}", font: LUCIDE, family: "lucide" },
    Icon { name: "check-circle-2", codepoint: "\u{E226}", font: LUCIDE, family: "lucide" },
    Icon { name: "check-line", codepoint: "\u{E66B}", font: LUCIDE, family: "lucide" },
    Icon { name: "check-square", codepoint: "\u{E16A}", font: LUCIDE, family: "lucide" },
    Icon { name: "check-square-2", codepoint: "\u{E559}", font: LUCIDE, family: "lucide" },
    Icon { name: "chef-hat", codepoint: "\u{E2AC}", font: LUCIDE, family: "lucide" },
    Icon { name: "cherry", codepoint: "\u{E350}", font: LUCIDE, family: "lucide" },
    Icon { name: "chess-bishop", codepoint: "\u{E6A0}", font: LUCIDE, family: "lucide" },
    Icon { name: "chess-king", codepoint: "\u{E6A1}", font: LUCIDE, family: "lucide" },
    Icon { name: "chess-knight", codepoint: "\u{E6A2}", font: LUCIDE, family: "lucide" },
    Icon { name: "chess-pawn", codepoint: "\u{E6A3}", font: LUCIDE, family: "lucide" },
    Icon { name: "chess-queen", codepoint: "\u{E6A4}", font: LUCIDE, family: "lucide" },
    Icon { name: "chess-rook", codepoint: "\u{E6A5}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-down", codepoint: "\u{E06D}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-down-circle", codepoint: "\u{E4DD}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-down-square", codepoint: "\u{E3CF}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-first", codepoint: "\u{E243}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-last", codepoint: "\u{E244}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-left", codepoint: "\u{E06E}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-left-circle", codepoint: "\u{E4DE}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-left-square", codepoint: "\u{E3D0}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-right", codepoint: "\u{E06F}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-right-circle", codepoint: "\u{E4DF}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-right-square", codepoint: "\u{E3D1}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-up", codepoint: "\u{E070}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-up-circle", codepoint: "\u{E4E0}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevron-up-square", codepoint: "\u{E3D2}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevrons-down", codepoint: "\u{E071}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevrons-down-up", codepoint: "\u{E228}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevrons-left", codepoint: "\u{E072}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevrons-left-right", codepoint: "\u{E293}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevrons-left-right-ellipsis", codepoint: "\u{E61F}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevrons-right", codepoint: "\u{E073}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevrons-right-left", codepoint: "\u{E294}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevrons-up", codepoint: "\u{E074}", font: LUCIDE, family: "lucide" },
    Icon { name: "chevrons-up-down", codepoint: "\u{E211}", font: LUCIDE, family: "lucide" },
    Icon { name: "church", codepoint: "\u{E3E1}", font: LUCIDE, family: "lucide" },
    Icon { name: "cigarette", codepoint: "\u{E2C6}", font: LUCIDE, family: "lucide" },
    Icon { name: "cigarette-off", codepoint: "\u{E2C7}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle", codepoint: "\u{E076}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-alert", codepoint: "\u{E077}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-arrow-down", codepoint: "\u{E078}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-arrow-left", codepoint: "\u{E079}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-arrow-out-down-left", codepoint: "\u{E3F7}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-arrow-out-down-right", codepoint: "\u{E3F8}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-arrow-out-up-left", codepoint: "\u{E3F9}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-arrow-out-up-right", codepoint: "\u{E3FA}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-arrow-right", codepoint: "\u{E07A}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-arrow-up", codepoint: "\u{E07B}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-check", codepoint: "\u{E226}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-check-big", codepoint: "\u{E07C}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-chevron-down", codepoint: "\u{E4DD}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-chevron-left", codepoint: "\u{E4DE}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-chevron-right", codepoint: "\u{E4DF}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-chevron-up", codepoint: "\u{E4E0}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-dashed", codepoint: "\u{E4B0}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-divide", codepoint: "\u{E07D}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-dollar-sign", codepoint: "\u{E47D}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-dot", codepoint: "\u{E345}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-dot-dashed", codepoint: "\u{E4B1}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-ellipsis", codepoint: "\u{E346}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-equal", codepoint: "\u{E400}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-euro", codepoint: "\u{E71F}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-fading-arrow-up", codepoint: "\u{E618}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-fading-plus", codepoint: "\u{E5BC}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-gauge", codepoint: "\u{E4E1}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-help", codepoint: "\u{E082}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-minus", codepoint: "\u{E07E}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-off", codepoint: "\u{E401}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-parking", codepoint: "\u{E3C9}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-parking-off", codepoint: "\u{E3CA}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-pause", codepoint: "\u{E07F}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-percent", codepoint: "\u{E51A}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-pile", codepoint: "\u{E6E1}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-play", codepoint: "\u{E080}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-plus", codepoint: "\u{E081}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-pound-sterling", codepoint: "\u{E66D}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-power", codepoint: "\u{E550}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-question-mark", codepoint: "\u{E082}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-slash", codepoint: "\u{E402}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-slash-2", codepoint: "\u{E213}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-slashed", codepoint: "\u{E213}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-small", codepoint: "\u{E640}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-star", codepoint: "\u{E68D}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-stop", codepoint: "\u{E083}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-user", codepoint: "\u{E461}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-user-round", codepoint: "\u{E462}", font: LUCIDE, family: "lucide" },
    Icon { name: "circle-x", codepoint: "\u{E084}", font: LUCIDE, family: "lucide" },
    Icon { name: "circuit-board", codepoint: "\u{E403}", font: LUCIDE, family: "lucide" },
    Icon { name: "citrus", codepoint: "\u{E375}", font: LUCIDE, family: "lucide" },
    Icon { name: "clapperboard", codepoint: "\u{E29B}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard", codepoint: "\u{E085}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-check", codepoint: "\u{E219}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-clock", codepoint: "\u{E688}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-copy", codepoint: "\u{E225}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-edit", codepoint: "\u{E307}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-list", codepoint: "\u{E086}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-minus", codepoint: "\u{E5BE}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-paste", codepoint: "\u{E3E8}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-pen", codepoint: "\u{E307}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-pen-line", codepoint: "\u{E308}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-plus", codepoint: "\u{E5BF}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-signature", codepoint: "\u{E308}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-type", codepoint: "\u{E309}", font: LUCIDE, family: "lucide" },
    Icon { name: "clipboard-x", codepoint: "\u{E222}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock", codepoint: "\u{E087}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-1", codepoint: "\u{E24B}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-10", codepoint: "\u{E24C}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-11", codepoint: "\u{E24D}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-12", codepoint: "\u{E24E}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-2", codepoint: "\u{E24F}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-3", codepoint: "\u{E250}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-4", codepoint: "\u{E251}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-5", codepoint: "\u{E252}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-6", codepoint: "\u{E253}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-7", codepoint: "\u{E254}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-8", codepoint: "\u{E255}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-9", codepoint: "\u{E256}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-alert", codepoint: "\u{E62A}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-arrow-down", codepoint: "\u{E600}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-arrow-left", codepoint: "\u{E70D}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-arrow-right", codepoint: "\u{E70E}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-arrow-up", codepoint: "\u{E601}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-check", codepoint: "\u{E69E}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-fading", codepoint: "\u{E64A}", font: LUCIDE, family: "lucide" },
    Icon { name: "clock-plus", codepoint: "\u{E667}", font: LUCIDE, family: "lucide" },
    Icon { name: "closed-caption", codepoint: "\u{E68A}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud", codepoint: "\u{E088}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-alert", codepoint: "\u{E633}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-backup", codepoint: "\u{E6E2}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-check", codepoint: "\u{E66E}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-cog", codepoint: "\u{E30A}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-download", codepoint: "\u{E089}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-drizzle", codepoint: "\u{E08A}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-fog", codepoint: "\u{E214}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-hail", codepoint: "\u{E08B}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-lightning", codepoint: "\u{E08C}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-moon", codepoint: "\u{E215}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-moon-rain", codepoint: "\u{E2FA}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-off", codepoint: "\u{E08D}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-rain", codepoint: "\u{E08E}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-rain-wind", codepoint: "\u{E08F}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-snow", codepoint: "\u{E090}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-sun", codepoint: "\u{E216}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-sun-rain", codepoint: "\u{E2FB}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-sync", codepoint: "\u{E6E3}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloud-upload", codepoint: "\u{E091}", font: LUCIDE, family: "lucide" },
    Icon { name: "cloudy", codepoint: "\u{E217}", font: LUCIDE, family: "lucide" },
    Icon { name: "clover", codepoint: "\u{E092}", font: LUCIDE, family: "lucide" },
    Icon { name: "club", codepoint: "\u{E496}", font: LUCIDE, family: "lucide" },
    Icon { name: "code", codepoint: "\u{E093}", font: LUCIDE, family: "lucide" },
    Icon { name: "code-2", codepoint: "\u{E206}", font: LUCIDE, family: "lucide" },
    Icon { name: "code-square", codepoint: "\u{E16B}", font: LUCIDE, family: "lucide" },
    Icon { name: "code-xml", codepoint: "\u{E206}", font: LUCIDE, family: "lucide" },
    Icon { name: "coffee", codepoint: "\u{E096}", font: LUCIDE, family: "lucide" },
    Icon { name: "cog", codepoint: "\u{E30B}", font: LUCIDE, family: "lucide" },
    Icon { name: "coins", codepoint: "\u{E097}", font: LUCIDE, family: "lucide" },
    Icon { name: "columns", codepoint: "\u{E098}", font: LUCIDE, family: "lucide" },
    Icon { name: "columns-2", codepoint: "\u{E098}", font: LUCIDE, family: "lucide" },
    Icon { name: "columns-3", codepoint: "\u{E099}", font: LUCIDE, family: "lucide" },
    Icon { name: "columns-3-cog", codepoint: "\u{E661}", font: LUCIDE, family: "lucide" },
    Icon { name: "columns-4", codepoint: "\u{E589}", font: LUCIDE, family: "lucide" },
    Icon { name: "columns-settings", codepoint: "\u{E661}", font: LUCIDE, family: "lucide" },
    Icon { name: "combine", codepoint: "\u{E44C}", font: LUCIDE, family: "lucide" },
    Icon { name: "command", codepoint: "\u{E09A}", font: LUCIDE, family: "lucide" },
    Icon { name: "compass", codepoint: "\u{E09B}", font: LUCIDE, family: "lucide" },
    Icon { name: "component", codepoint: "\u{E2AD}", font: LUCIDE, family: "lucide" },
    Icon { name: "computer", codepoint: "\u{E4E4}", font: LUCIDE, family: "lucide" },
    Icon { name: "concierge-bell", codepoint: "\u{E378}", font: LUCIDE, family: "lucide" },
    Icon { name: "cone", codepoint: "\u{E523}", font: LUCIDE, family: "lucide" },
    Icon { name: "construction", codepoint: "\u{E3B4}", font: LUCIDE, family: "lucide" },
    Icon { name: "contact", codepoint: "\u{E09C}", font: LUCIDE, family: "lucide" },
    Icon { name: "contact-2", codepoint: "\u{E463}", font: LUCIDE, family: "lucide" },
    Icon { name: "contact-round", codepoint: "\u{E463}", font: LUCIDE, family: "lucide" },
    Icon { name: "container", codepoint: "\u{E4D5}", font: LUCIDE, family: "lucide" },
    Icon { name: "contrast", codepoint: "\u{E09D}", font: LUCIDE, family: "lucide" },
    Icon { name: "cookie", codepoint: "\u{E26B}", font: LUCIDE, family: "lucide" },
    Icon { name: "cooking-pot", codepoint: "\u{E584}", font: LUCIDE, family: "lucide" },
    Icon { name: "copy", codepoint: "\u{E09E}", font: LUCIDE, family: "lucide" },
    Icon { name: "copy-check", codepoint: "\u{E3FB}", font: LUCIDE, family: "lucide" },
    Icon { name: "copy-minus", codepoint: "\u{E3FC}", font: LUCIDE, family: "lucide" },
    Icon { name: "copy-plus", codepoint: "\u{E3FD}", font: LUCIDE, family: "lucide" },
    Icon { name: "copy-slash", codepoint: "\u{E3FE}", font: LUCIDE, family: "lucide" },
    Icon { name: "copy-x", codepoint: "\u{E3FF}", font: LUCIDE, family: "lucide" },
    Icon { name: "copyleft", codepoint: "\u{E09F}", font: LUCIDE, family: "lucide" },
    Icon { name: "copyright", codepoint: "\u{E0A0}", font: LUCIDE, family: "lucide" },
    Icon { name: "corner-down-left", codepoint: "\u{E0A1}", font: LUCIDE, family: "lucide" },
    Icon { name: "corner-down-right", codepoint: "\u{E0A2}", font: LUCIDE, family: "lucide" },
    Icon { name: "corner-left-down", codepoint: "\u{E0A3}", font: LUCIDE, family: "lucide" },
    Icon { name: "corner-left-up", codepoint: "\u{E0A4}", font: LUCIDE, family: "lucide" },
    Icon { name: "corner-right-down", codepoint: "\u{E0A5}", font: LUCIDE, family: "lucide" },
    Icon { name: "corner-right-up", codepoint: "\u{E0A6}", font: LUCIDE, family: "lucide" },
    Icon { name: "corner-up-left", codepoint: "\u{E0A7}", font: LUCIDE, family: "lucide" },
    Icon { name: "corner-up-right", codepoint: "\u{E0A8}", font: LUCIDE, family: "lucide" },
    Icon { name: "cpu", codepoint: "\u{E0A9}", font: LUCIDE, family: "lucide" },
    Icon { name: "creative-commons", codepoint: "\u{E3B2}", font: LUCIDE, family: "lucide" },
    Icon { name: "credit-card", codepoint: "\u{E0AA}", font: LUCIDE, family: "lucide" },
    Icon { name: "credit-card-check", codepoint: "\u{E743}", font: LUCIDE, family: "lucide" },
    Icon { name: "credit-card-minus", codepoint: "\u{E744}", font: LUCIDE, family: "lucide" },
    Icon { name: "credit-card-plus", codepoint: "\u{E74B}", font: LUCIDE, family: "lucide" },
    Icon { name: "credit-card-x", codepoint: "\u{E745}", font: LUCIDE, family: "lucide" },
    Icon { name: "croissant", codepoint: "\u{E2AE}", font: LUCIDE, family: "lucide" },
    Icon { name: "crop", codepoint: "\u{E0AB}", font: LUCIDE, family: "lucide" },
    Icon { name: "cross", codepoint: "\u{E1E5}", font: LUCIDE, family: "lucide" },
    Icon { name: "crosshair", codepoint: "\u{E0AC}", font: LUCIDE, family: "lucide" },
    Icon { name: "crown", codepoint: "\u{E1D6}", font: LUCIDE, family: "lucide" },
    Icon { name: "cuboid", codepoint: "\u{E524}", font: LUCIDE, family: "lucide" },
    Icon { name: "cup-soda", codepoint: "\u{E2D1}", font: LUCIDE, family: "lucide" },
    Icon { name: "curly-braces", codepoint: "\u{E36A}", font: LUCIDE, family: "lucide" },
    Icon { name: "currency", codepoint: "\u{E230}", font: LUCIDE, family: "lucide" },
    Icon { name: "cylinder", codepoint: "\u{E525}", font: LUCIDE, family: "lucide" },
    Icon { name: "dam", codepoint: "\u{E606}", font: LUCIDE, family: "lucide" },
    Icon { name: "database", codepoint: "\u{E0AD}", font: LUCIDE, family: "lucide" },
    Icon { name: "database-arrow-down", codepoint: "\u{E716}", font: LUCIDE, family: "lucide" },
    Icon { name: "database-arrow-up", codepoint: "\u{E717}", font: LUCIDE, family: "lucide" },
    Icon { name: "database-backup", codepoint: "\u{E3AB}", font: LUCIDE, family: "lucide" },
    Icon { name: "database-check", codepoint: "\u{E718}", font: LUCIDE, family: "lucide" },
    Icon { name: "database-minus", codepoint: "\u{E719}", font: LUCIDE, family: "lucide" },
    Icon { name: "database-plus", codepoint: "\u{E71A}", font: LUCIDE, family: "lucide" },
    Icon { name: "database-search", codepoint: "\u{E6B1}", font: LUCIDE, family: "lucide" },
    Icon { name: "database-x", codepoint: "\u{E71B}", font: LUCIDE, family: "lucide" },
    Icon { name: "database-zap", codepoint: "\u{E50B}", font: LUCIDE, family: "lucide" },
    Icon { name: "decimals-arrow-left", codepoint: "\u{E65C}", font: LUCIDE, family: "lucide" },
    Icon { name: "decimals-arrow-right", codepoint: "\u{E65D}", font: LUCIDE, family: "lucide" },
    Icon { name: "delete", codepoint: "\u{E0AE}", font: LUCIDE, family: "lucide" },
    Icon { name: "dessert", codepoint: "\u{E4BB}", font: LUCIDE, family: "lucide" },
    Icon { name: "diameter", codepoint: "\u{E526}", font: LUCIDE, family: "lucide" },
    Icon { name: "diamond", codepoint: "\u{E2D2}", font: LUCIDE, family: "lucide" },
    Icon { name: "diamond-minus", codepoint: "\u{E5E1}", font: LUCIDE, family: "lucide" },
    Icon { name: "diamond-percent", codepoint: "\u{E51B}", font: LUCIDE, family: "lucide" },
    Icon { name: "diamond-plus", codepoint: "\u{E5E2}", font: LUCIDE, family: "lucide" },
    Icon { name: "dice-1", codepoint: "\u{E287}", font: LUCIDE, family: "lucide" },
    Icon { name: "dice-2", codepoint: "\u{E288}", font: LUCIDE, family: "lucide" },
    Icon { name: "dice-3", codepoint: "\u{E289}", font: LUCIDE, family: "lucide" },
    Icon { name: "dice-4", codepoint: "\u{E28A}", font: LUCIDE, family: "lucide" },
    Icon { name: "dice-5", codepoint: "\u{E28B}", font: LUCIDE, family: "lucide" },
    Icon { name: "dice-6", codepoint: "\u{E28C}", font: LUCIDE, family: "lucide" },
    Icon { name: "dices", codepoint: "\u{E2C5}", font: LUCIDE, family: "lucide" },
    Icon { name: "diff", codepoint: "\u{E30C}", font: LUCIDE, family: "lucide" },
    Icon { name: "disc", codepoint: "\u{E0AF}", font: LUCIDE, family: "lucide" },
    Icon { name: "disc-2", codepoint: "\u{E3F6}", font: LUCIDE, family: "lucide" },
    Icon { name: "disc-3", codepoint: "\u{E494}", font: LUCIDE, family: "lucide" },
    Icon { name: "disc-album", codepoint: "\u{E55C}", font: LUCIDE, family: "lucide" },
    Icon { name: "divide", codepoint: "\u{E0B0}", font: LUCIDE, family: "lucide" },
    Icon { name: "divide-circle", codepoint: "\u{E07D}", font: LUCIDE, family: "lucide" },
    Icon { name: "divide-square", codepoint: "\u{E16D}", font: LUCIDE, family: "lucide" },
    Icon { name: "dna", codepoint: "\u{E393}", font: LUCIDE, family: "lucide" },
    Icon { name: "dna-off", codepoint: "\u{E394}", font: LUCIDE, family: "lucide" },
    Icon { name: "dock", codepoint: "\u{E5D3}", font: LUCIDE, family: "lucide" },
    Icon { name: "dog", codepoint: "\u{E38D}", font: LUCIDE, family: "lucide" },
    Icon { name: "dollar-sign", codepoint: "\u{E0B1}", font: LUCIDE, family: "lucide" },
    Icon { name: "donut", codepoint: "\u{E4BC}", font: LUCIDE, family: "lucide" },
    Icon { name: "door-closed", codepoint: "\u{E3D5}", font: LUCIDE, family: "lucide" },
    Icon { name: "door-closed-locked", codepoint: "\u{E664}", font: LUCIDE, family: "lucide" },
    Icon { name: "door-open", codepoint: "\u{E3D6}", font: LUCIDE, family: "lucide" },
    Icon { name: "dot", codepoint: "\u{E44F}", font: LUCIDE, family: "lucide" },
    Icon { name: "dot-square", codepoint: "\u{E16E}", font: LUCIDE, family: "lucide" },
    Icon { name: "download", codepoint: "\u{E0B2}", font: LUCIDE, family: "lucide" },
    Icon { name: "download-cloud", codepoint: "\u{E089}", font: LUCIDE, family: "lucide" },
    Icon { name: "drafting-compass", codepoint: "\u{E527}", font: LUCIDE, family: "lucide" },
    Icon { name: "drama", codepoint: "\u{E521}", font: LUCIDE, family: "lucide" },
    Icon { name: "drill", codepoint: "\u{E58D}", font: LUCIDE, family: "lucide" },
    Icon { name: "drone", codepoint: "\u{E676}", font: LUCIDE, family: "lucide" },
    Icon { name: "droplet", codepoint: "\u{E0B4}", font: LUCIDE, family: "lucide" },
    Icon { name: "droplet-off", codepoint: "\u{E638}", font: LUCIDE, family: "lucide" },
    Icon { name: "droplets", codepoint: "\u{E0B5}", font: LUCIDE, family: "lucide" },
    Icon { name: "drum", codepoint: "\u{E55D}", font: LUCIDE, family: "lucide" },
    Icon { name: "drumstick", codepoint: "\u{E25B}", font: LUCIDE, family: "lucide" },
    Icon { name: "dumbbell", codepoint: "\u{E3A1}", font: LUCIDE, family: "lucide" },
    Icon { name: "ear", codepoint: "\u{E382}", font: LUCIDE, family: "lucide" },
    Icon { name: "ear-off", codepoint: "\u{E383}", font: LUCIDE, family: "lucide" },
    Icon { name: "earth", codepoint: "\u{E1F3}", font: LUCIDE, family: "lucide" },
    Icon { name: "earth-lock", codepoint: "\u{E5CC}", font: LUCIDE, family: "lucide" },
    Icon { name: "eclipse", codepoint: "\u{E59D}", font: LUCIDE, family: "lucide" },
    Icon { name: "edit", codepoint: "\u{E172}", font: LUCIDE, family: "lucide" },
    Icon { name: "edit-2", codepoint: "\u{E12F}", font: LUCIDE, family: "lucide" },
    Icon { name: "edit-3", codepoint: "\u{E130}", font: LUCIDE, family: "lucide" },
    Icon { name: "egg", codepoint: "\u{E25D}", font: LUCIDE, family: "lucide" },
    Icon { name: "egg-fried", codepoint: "\u{E351}", font: LUCIDE, family: "lucide" },
    Icon { name: "egg-off", codepoint: "\u{E395}", font: LUCIDE, family: "lucide" },
    Icon { name: "eject", codepoint: "\u{E733}", font: LUCIDE, family: "lucide" },
    Icon { name: "ellipse", codepoint: "\u{E6B2}", font: LUCIDE, family: "lucide" },
    Icon { name: "ellipsis", codepoint: "\u{E0B6}", font: LUCIDE, family: "lucide" },
    Icon { name: "ellipsis-vertical", codepoint: "\u{E0B7}", font: LUCIDE, family: "lucide" },
    Icon { name: "equal", codepoint: "\u{E1BD}", font: LUCIDE, family: "lucide" },
    Icon { name: "equal-approximately", codepoint: "\u{E634}", font: LUCIDE, family: "lucide" },
    Icon { name: "equal-not", codepoint: "\u{E1BE}", font: LUCIDE, family: "lucide" },
    Icon { name: "equal-square", codepoint: "\u{E16F}", font: LUCIDE, family: "lucide" },
    Icon { name: "eraser", codepoint: "\u{E28F}", font: LUCIDE, family: "lucide" },
    Icon { name: "ethernet-port", codepoint: "\u{E620}", font: LUCIDE, family: "lucide" },
    Icon { name: "euro", codepoint: "\u{E0B8}", font: LUCIDE, family: "lucide" },
    Icon { name: "ev-charger", codepoint: "\u{E697}", font: LUCIDE, family: "lucide" },
    Icon { name: "expand", codepoint: "\u{E21A}", font: LUCIDE, family: "lucide" },
    Icon { name: "external-link", codepoint: "\u{E0B9}", font: LUCIDE, family: "lucide" },
    Icon { name: "eye", codepoint: "\u{E0BA}", font: LUCIDE, family: "lucide" },
    Icon { name: "eye-closed", codepoint: "\u{E62E}", font: LUCIDE, family: "lucide" },
    Icon { name: "eye-dashed", codepoint: "\u{E701}", font: LUCIDE, family: "lucide" },
    Icon { name: "eye-off", codepoint: "\u{E0BB}", font: LUCIDE, family: "lucide" },
    Icon { name: "face-angry", codepoint: "\u{E2FC}", font: LUCIDE, family: "lucide" },
    Icon { name: "face-expressionless", codepoint: "\u{E2FD}", font: LUCIDE, family: "lucide" },
    Icon { name: "face-grinning", codepoint: "\u{E300}", font: LUCIDE, family: "lucide" },
    Icon { name: "face-neutral", codepoint: "\u{E114}", font: LUCIDE, family: "lucide" },
    Icon { name: "face-slightly-frowning", codepoint: "\u{E0DB}", font: LUCIDE, family: "lucide" },
    Icon { name: "face-slightly-smiling", codepoint: "\u{E164}", font: LUCIDE, family: "lucide" },
    Icon { name: "face-slightly-smiling-plus", codepoint: "\u{E301}", font: LUCIDE, family: "lucide" },
    Icon { name: "factory", codepoint: "\u{E29F}", font: LUCIDE, family: "lucide" },
    Icon { name: "fan", codepoint: "\u{E379}", font: LUCIDE, family: "lucide" },
    Icon { name: "fast-forward", codepoint: "\u{E0BD}", font: LUCIDE, family: "lucide" },
    Icon { name: "feather", codepoint: "\u{E0BE}", font: LUCIDE, family: "lucide" },
    Icon { name: "fence", codepoint: "\u{E582}", font: LUCIDE, family: "lucide" },
    Icon { name: "ferris-wheel", codepoint: "\u{E47F}", font: LUCIDE, family: "lucide" },
    Icon { name: "file", codepoint: "\u{E0C0}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-archive", codepoint: "\u{E30D}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-audio", codepoint: "\u{E31A}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-audio-2", codepoint: "\u{E31A}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-axis-3-d", codepoint: "\u{E30E}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-axis-3d", codepoint: "\u{E30E}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-badge", codepoint: "\u{E30F}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-badge-2", codepoint: "\u{E30F}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-bar-chart", codepoint: "\u{E312}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-bar-chart-2", codepoint: "\u{E311}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-box", codepoint: "\u{E310}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-braces", codepoint: "\u{E36B}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-braces-corner", codepoint: "\u{E36C}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-chart-column", codepoint: "\u{E311}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-chart-column-increasing", codepoint: "\u{E312}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-chart-line", codepoint: "\u{E313}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-chart-pie", codepoint: "\u{E314}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-check", codepoint: "\u{E0C1}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-check-2", codepoint: "\u{E0C2}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-check-corner", codepoint: "\u{E0C2}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-clock", codepoint: "\u{E315}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-code", codepoint: "\u{E0C3}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-code-2", codepoint: "\u{E45E}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-code-corner", codepoint: "\u{E45E}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-cog", codepoint: "\u{E316}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-cog-2", codepoint: "\u{E316}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-diff", codepoint: "\u{E317}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-digit", codepoint: "\u{E0C4}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-down", codepoint: "\u{E318}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-edit", codepoint: "\u{E31F}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-exclamation-point", codepoint: "\u{E319}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-headphone", codepoint: "\u{E31A}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-heart", codepoint: "\u{E31B}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-image", codepoint: "\u{E31C}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-input", codepoint: "\u{E0C5}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-json", codepoint: "\u{E36B}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-json-2", codepoint: "\u{E36C}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-key", codepoint: "\u{E31D}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-key-2", codepoint: "\u{E31D}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-line-chart", codepoint: "\u{E313}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-lock", codepoint: "\u{E31E}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-lock-2", codepoint: "\u{E31E}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-minus", codepoint: "\u{E0C6}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-minus-2", codepoint: "\u{E0C7}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-minus-corner", codepoint: "\u{E0C7}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-music", codepoint: "\u{E55E}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-output", codepoint: "\u{E0C8}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-pen", codepoint: "\u{E31F}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-pen-line", codepoint: "\u{E320}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-pie-chart", codepoint: "\u{E314}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-play", codepoint: "\u{E321}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-plus", codepoint: "\u{E0C9}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-plus-2", codepoint: "\u{E0CA}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-plus-corner", codepoint: "\u{E0CA}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-question", codepoint: "\u{E322}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-question-mark", codepoint: "\u{E322}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-scan", codepoint: "\u{E323}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-search", codepoint: "\u{E0CB}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-search-2", codepoint: "\u{E324}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-search-corner", codepoint: "\u{E324}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-signal", codepoint: "\u{E325}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-signature", codepoint: "\u{E320}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-sliders", codepoint: "\u{E5A0}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-spreadsheet", codepoint: "\u{E326}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-stack", codepoint: "\u{E4A1}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-symlink", codepoint: "\u{E327}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-terminal", codepoint: "\u{E328}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-text", codepoint: "\u{E0CC}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-type", codepoint: "\u{E329}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-type-2", codepoint: "\u{E36D}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-type-corner", codepoint: "\u{E36D}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-up", codepoint: "\u{E32A}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-user", codepoint: "\u{E62D}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-video", codepoint: "\u{E321}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-video-2", codepoint: "\u{E32B}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-video-camera", codepoint: "\u{E32B}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-volume", codepoint: "\u{E32C}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-volume-2", codepoint: "\u{E325}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-warning", codepoint: "\u{E319}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-x", codepoint: "\u{E0CD}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-x-2", codepoint: "\u{E0CE}", font: LUCIDE, family: "lucide" },
    Icon { name: "file-x-corner", codepoint: "\u{E0CE}", font: LUCIDE, family: "lucide" },
    Icon { name: "files", codepoint: "\u{E0CF}", font: LUCIDE, family: "lucide" },
    Icon { name: "film", codepoint: "\u{E0D0}", font: LUCIDE, family: "lucide" },
    Icon { name: "filter", codepoint: "\u{E0DC}", font: LUCIDE, family: "lucide" },
    Icon { name: "filter-x", codepoint: "\u{E3B5}", font: LUCIDE, family: "lucide" },
    Icon { name: "fingerprint", codepoint: "\u{E2CB}", font: LUCIDE, family: "lucide" },
    Icon { name: "fingerprint-pattern", codepoint: "\u{E2CB}", font: LUCIDE, family: "lucide" },
    Icon { name: "fire-extinguisher", codepoint: "\u{E57E}", font: LUCIDE, family: "lucide" },
    Icon { name: "fish", codepoint: "\u{E3A6}", font: LUCIDE, family: "lucide" },
    Icon { name: "fish-off", codepoint: "\u{E3B0}", font: LUCIDE, family: "lucide" },
    Icon { name: "fish-symbol", codepoint: "\u{E4F4}", font: LUCIDE, family: "lucide" },
    Icon { name: "fishing-hook", codepoint: "\u{E6E4}", font: LUCIDE, family: "lucide" },
    Icon { name: "fishing-rod", codepoint: "\u{E6B3}", font: LUCIDE, family: "lucide" },
    Icon { name: "flag", codepoint: "\u{E0D1}", font: LUCIDE, family: "lucide" },
    Icon { name: "flag-off", codepoint: "\u{E292}", font: LUCIDE, family: "lucide" },
    Icon { name: "flag-triangle-left", codepoint: "\u{E237}", font: LUCIDE, family: "lucide" },
    Icon { name: "flag-triangle-right", codepoint: "\u{E238}", font: LUCIDE, family: "lucide" },
    Icon { name: "flame", codepoint: "\u{E0D2}", font: LUCIDE, family: "lucide" },
    Icon { name: "flame-kindling", codepoint: "\u{E53A}", font: LUCIDE, family: "lucide" },
    Icon { name: "flashlight", codepoint: "\u{E0D3}", font: LUCIDE, family: "lucide" },
    Icon { name: "flashlight-off", codepoint: "\u{E0D4}", font: LUCIDE, family: "lucide" },
    Icon { name: "flask-conical", codepoint: "\u{E0D5}", font: LUCIDE, family: "lucide" },
    Icon { name: "flask-conical-off", codepoint: "\u{E396}", font: LUCIDE, family: "lucide" },
    Icon { name: "flask-round", codepoint: "\u{E0D6}", font: LUCIDE, family: "lucide" },
    Icon { name: "flip-horizontal", codepoint: "\u{E35D}", font: LUCIDE, family: "lucide" },
    Icon { name: "flip-horizontal-2", codepoint: "\u{E35E}", font: LUCIDE, family: "lucide" },
    Icon { name: "flip-vertical", codepoint: "\u{E35F}", font: LUCIDE, family: "lucide" },
    Icon { name: "flip-vertical-2", codepoint: "\u{E360}", font: LUCIDE, family: "lucide" },
    Icon { name: "flower", codepoint: "\u{E2D3}", font: LUCIDE, family: "lucide" },
    Icon { name: "flower-2", codepoint: "\u{E2D4}", font: LUCIDE, family: "lucide" },
    Icon { name: "focus", codepoint: "\u{E29E}", font: LUCIDE, family: "lucide" },
    Icon { name: "fold-horizontal", codepoint: "\u{E43B}", font: LUCIDE, family: "lucide" },
    Icon { name: "fold-vertical", codepoint: "\u{E43C}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder", codepoint: "\u{E0D7}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-archive", codepoint: "\u{E32D}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-bookmark", codepoint: "\u{E6F1}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-check", codepoint: "\u{E32E}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-clock", codepoint: "\u{E32F}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-closed", codepoint: "\u{E330}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-code", codepoint: "\u{E5FB}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-cog", codepoint: "\u{E331}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-cog-2", codepoint: "\u{E331}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-dot", codepoint: "\u{E4C5}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-down", codepoint: "\u{E332}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-edit", codepoint: "\u{E338}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-git", codepoint: "\u{E409}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-git-2", codepoint: "\u{E40A}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-heart", codepoint: "\u{E333}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-input", codepoint: "\u{E334}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-kanban", codepoint: "\u{E4C6}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-key", codepoint: "\u{E335}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-lock", codepoint: "\u{E336}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-minus", codepoint: "\u{E0D8}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-open", codepoint: "\u{E247}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-open-dot", codepoint: "\u{E4C7}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-output", codepoint: "\u{E337}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-pen", codepoint: "\u{E338}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-plus", codepoint: "\u{E0D9}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-root", codepoint: "\u{E4C8}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-search", codepoint: "\u{E339}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-search-2", codepoint: "\u{E33A}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-symlink", codepoint: "\u{E33B}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-sync", codepoint: "\u{E4C9}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-tree", codepoint: "\u{E33C}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-up", codepoint: "\u{E33D}", font: LUCIDE, family: "lucide" },
    Icon { name: "folder-x", codepoint: "\u{E33E}", font: LUCIDE, family: "lucide" },
    Icon { name: "folders", codepoint: "\u{E33F}", font: LUCIDE, family: "lucide" },
    Icon { name: "footprints", codepoint: "\u{E3B9}", font: LUCIDE, family: "lucide" },
    Icon { name: "fork-knife", codepoint: "\u{E2F6}", font: LUCIDE, family: "lucide" },
    Icon { name: "fork-knife-crossed", codepoint: "\u{E2F7}", font: LUCIDE, family: "lucide" },
    Icon { name: "forklift", codepoint: "\u{E3C1}", font: LUCIDE, family: "lucide" },
    Icon { name: "form", codepoint: "\u{E6A8}", font: LUCIDE, family: "lucide" },
    Icon { name: "form-input", codepoint: "\u{E21F}", font: LUCIDE, family: "lucide" },
    Icon { name: "forward", codepoint: "\u{E229}", font: LUCIDE, family: "lucide" },
    Icon { name: "frame", codepoint: "\u{E291}", font: LUCIDE, family: "lucide" },
    Icon { name: "frown", codepoint: "\u{E0DB}", font: LUCIDE, family: "lucide" },
    Icon { name: "fuel", codepoint: "\u{E2AF}", font: LUCIDE, family: "lucide" },
    Icon { name: "fullscreen", codepoint: "\u{E534}", font: LUCIDE, family: "lucide" },
    Icon { name: "function-square", codepoint: "\u{E22D}", font: LUCIDE, family: "lucide" },
    Icon { name: "funnel", codepoint: "\u{E0DC}", font: LUCIDE, family: "lucide" },
    Icon { name: "funnel-plus", codepoint: "\u{E0DD}", font: LUCIDE, family: "lucide" },
    Icon { name: "funnel-x", codepoint: "\u{E3B5}", font: LUCIDE, family: "lucide" },
    Icon { name: "galaxy", codepoint: "\u{E73F}", font: LUCIDE, family: "lucide" },
    Icon { name: "gallery-horizontal", codepoint: "\u{E4CE}", font: LUCIDE, family: "lucide" },
    Icon { name: "gallery-horizontal-end", codepoint: "\u{E4CF}", font: LUCIDE, family: "lucide" },
    Icon { name: "gallery-thumbnails", codepoint: "\u{E4D0}", font: LUCIDE, family: "lucide" },
    Icon { name: "gallery-vertical", codepoint: "\u{E4D1}", font: LUCIDE, family: "lucide" },
    Icon { name: "gallery-vertical-end", codepoint: "\u{E4D2}", font: LUCIDE, family: "lucide" },
    Icon { name: "gamepad", codepoint: "\u{E0DE}", font: LUCIDE, family: "lucide" },
    Icon { name: "gamepad-2", codepoint: "\u{E0DF}", font: LUCIDE, family: "lucide" },
    Icon { name: "gamepad-directional", codepoint: "\u{E69B}", font: LUCIDE, family: "lucide" },
    Icon { name: "gantt-chart", codepoint: "\u{E4C4}", font: LUCIDE, family: "lucide" },
    Icon { name: "gantt-chart-square", codepoint: "\u{E169}", font: LUCIDE, family: "lucide" },
    Icon { name: "gauge", codepoint: "\u{E1BF}", font: LUCIDE, family: "lucide" },
    Icon { name: "gauge-circle", codepoint: "\u{E4E1}", font: LUCIDE, family: "lucide" },
    Icon { name: "gavel", codepoint: "\u{E0E0}", font: LUCIDE, family: "lucide" },
    Icon { name: "gem", codepoint: "\u{E242}", font: LUCIDE, family: "lucide" },
    Icon { name: "georgian-lari", codepoint: "\u{E678}", font: LUCIDE, family: "lucide" },
    Icon { name: "ghost", codepoint: "\u{E20E}", font: LUCIDE, family: "lucide" },
    Icon { name: "gift", codepoint: "\u{E0E1}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-branch", codepoint: "\u{E0E2}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-branch-minus", codepoint: "\u{E69C}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-branch-plus", codepoint: "\u{E1F4}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-commit", codepoint: "\u{E0E3}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-commit-horizontal", codepoint: "\u{E0E3}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-commit-vertical", codepoint: "\u{E552}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-compare", codepoint: "\u{E359}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-compare-arrows", codepoint: "\u{E553}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-fork", codepoint: "\u{E28D}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-graph", codepoint: "\u{E554}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-merge", codepoint: "\u{E0E4}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-merge-conflict", codepoint: "\u{E6B4}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-pull-request", codepoint: "\u{E0E5}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-pull-request-arrow", codepoint: "\u{E555}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-pull-request-closed", codepoint: "\u{E35A}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-pull-request-create", codepoint: "\u{E556}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-pull-request-create-arrow", codepoint: "\u{E557}", font: LUCIDE, family: "lucide" },
    Icon { name: "git-pull-request-draft", codepoint: "\u{E35B}", font: LUCIDE, family: "lucide" },
    Icon { name: "glass-water", codepoint: "\u{E2D5}", font: LUCIDE, family: "lucide" },
    Icon { name: "glasses", codepoint: "\u{E20D}", font: LUCIDE, family: "lucide" },
    Icon { name: "globe", codepoint: "\u{E0E8}", font: LUCIDE, family: "lucide" },
    Icon { name: "globe-2", codepoint: "\u{E1F3}", font: LUCIDE, family: "lucide" },
    Icon { name: "globe-check", codepoint: "\u{E6FC}", font: LUCIDE, family: "lucide" },
    Icon { name: "globe-lock", codepoint: "\u{E5CD}", font: LUCIDE, family: "lucide" },
    Icon { name: "globe-off", codepoint: "\u{E6B5}", font: LUCIDE, family: "lucide" },
    Icon { name: "globe-x", codepoint: "\u{E6B6}", font: LUCIDE, family: "lucide" },
    Icon { name: "goal", codepoint: "\u{E4A5}", font: LUCIDE, family: "lucide" },
    Icon { name: "gpu", codepoint: "\u{E66A}", font: LUCIDE, family: "lucide" },
    Icon { name: "grab", codepoint: "\u{E1E6}", font: LUCIDE, family: "lucide" },
    Icon { name: "graduation-cap", codepoint: "\u{E234}", font: LUCIDE, family: "lucide" },
    Icon { name: "grape", codepoint: "\u{E352}", font: LUCIDE, family: "lucide" },
    Icon { name: "grid", codepoint: "\u{E0E9}", font: LUCIDE, family: "lucide" },
    Icon { name: "grid-2-x-2", codepoint: "\u{E4FF}", font: LUCIDE, family: "lucide" },
    Icon { name: "grid-2-x-2-check", codepoint: "\u{E5E4}", font: LUCIDE, family: "lucide" },
    Icon { name: "grid-2-x-2-plus", codepoint: "\u{E628}", font: LUCIDE, family: "lucide" },
    Icon { name: "grid-2-x-2-x", codepoint: "\u{E5E5}", font: LUCIDE, family: "lucide" },
    Icon { name: "grid-2x2", codepoint: "\u{E4FF}", font: LUCIDE, family: "lucide" },
    Icon { name: "grid-2x2-check", codepoint: "\u{E5E4}", font: LUCIDE, family: "lucide" },
    Icon { name: "grid-2x2-plus", codepoint: "\u{E628}", font: LUCIDE, family: "lucide" },
    Icon { name: "grid-2x2-x", codepoint: "\u{E5E5}", font: LUCIDE, family: "lucide" },
    Icon { name: "grid-3-x-3", codepoint: "\u{E0E9}", font: LUCIDE, family: "lucide" },
    Icon { name: "grid-3x2", codepoint: "\u{E66F}", font: LUCIDE, family: "lucide" },
    Icon { name: "grid-3x3", codepoint: "\u{E0E9}", font: LUCIDE, family: "lucide" },
    Icon { name: "grip", codepoint: "\u{E3B1}", font: LUCIDE, family: "lucide" },
    Icon { name: "grip-horizontal", codepoint: "\u{E0EA}", font: LUCIDE, family: "lucide" },
    Icon { name: "grip-vertical", codepoint: "\u{E0EB}", font: LUCIDE, family: "lucide" },
    Icon { name: "group", codepoint: "\u{E464}", font: LUCIDE, family: "lucide" },
    Icon { name: "guitar", codepoint: "\u{E55F}", font: LUCIDE, family: "lucide" },
    Icon { name: "ham", codepoint: "\u{E5D7}", font: LUCIDE, family: "lucide" },
    Icon { name: "hamburger", codepoint: "\u{E665}", font: LUCIDE, family: "lucide" },
    Icon { name: "hammer", codepoint: "\u{E0EC}", font: LUCIDE, family: "lucide" },
    Icon { name: "hand", codepoint: "\u{E1D7}", font: LUCIDE, family: "lucide" },
    Icon { name: "hand-coins", codepoint: "\u{E5B8}", font: LUCIDE, family: "lucide" },
    Icon { name: "hand-fist", codepoint: "\u{E68B}", font: LUCIDE, family: "lucide" },
    Icon { name: "hand-grab", codepoint: "\u{E1E6}", font: LUCIDE, family: "lucide" },
    Icon { name: "hand-heart", codepoint: "\u{E5B9}", font: LUCIDE, family: "lucide" },
    Icon { name: "hand-helping", codepoint: "\u{E3B8}", font: LUCIDE, family: "lucide" },
    Icon { name: "hand-metal", codepoint: "\u{E22C}", font: LUCIDE, family: "lucide" },
    Icon { name: "hand-platter", codepoint: "\u{E5BA}", font: LUCIDE, family: "lucide" },
    Icon { name: "handbag", codepoint: "\u{E689}", font: LUCIDE, family: "lucide" },
    Icon { name: "handshake", codepoint: "\u{E5C0}", font: LUCIDE, family: "lucide" },
    Icon { name: "hard-drive", codepoint: "\u{E0ED}", font: LUCIDE, family: "lucide" },
    Icon { name: "hard-drive-download", codepoint: "\u{E4E5}", font: LUCIDE, family: "lucide" },
    Icon { name: "hard-drive-upload", codepoint: "\u{E4E6}", font: LUCIDE, family: "lucide" },
    Icon { name: "hard-hat", codepoint: "\u{E0EE}", font: LUCIDE, family: "lucide" },
    Icon { name: "hash", codepoint: "\u{E0EF}", font: LUCIDE, family: "lucide" },
    Icon { name: "hat-glasses", codepoint: "\u{E683}", font: LUCIDE, family: "lucide" },
    Icon { name: "haze", codepoint: "\u{E0F0}", font: LUCIDE, family: "lucide" },
    Icon { name: "hd", codepoint: "\u{E6E5}", font: LUCIDE, family: "lucide" },
    Icon { name: "hdmi-port", codepoint: "\u{E4E7}", font: LUCIDE, family: "lucide" },
    Icon { name: "heading", codepoint: "\u{E384}", font: LUCIDE, family: "lucide" },
    Icon { name: "heading-1", codepoint: "\u{E385}", font: LUCIDE, family: "lucide" },
    Icon { name: "heading-2", codepoint: "\u{E386}", font: LUCIDE, family: "lucide" },
    Icon { name: "heading-3", codepoint: "\u{E387}", font: LUCIDE, family: "lucide" },
    Icon { name: "heading-4", codepoint: "\u{E388}", font: LUCIDE, family: "lucide" },
    Icon { name: "heading-5", codepoint: "\u{E389}", font: LUCIDE, family: "lucide" },
    Icon { name: "heading-6", codepoint: "\u{E38A}", font: LUCIDE, family: "lucide" },
    Icon { name: "headphone-off", codepoint: "\u{E629}", font: LUCIDE, family: "lucide" },
    Icon { name: "headphones", codepoint: "\u{E0F1}", font: LUCIDE, family: "lucide" },
    Icon { name: "headset", codepoint: "\u{E5BD}", font: LUCIDE, family: "lucide" },
    Icon { name: "heart", codepoint: "\u{E0F2}", font: LUCIDE, family: "lucide" },
    Icon { name: "heart-crack", codepoint: "\u{E2D6}", font: LUCIDE, family: "lucide" },
    Icon { name: "heart-handshake", codepoint: "\u{E2D7}", font: LUCIDE, family: "lucide" },
    Icon { name: "heart-minus", codepoint: "\u{E651}", font: LUCIDE, family: "lucide" },
    Icon { name: "heart-off", codepoint: "\u{E295}", font: LUCIDE, family: "lucide" },
    Icon { name: "heart-plus", codepoint: "\u{E652}", font: LUCIDE, family: "lucide" },
    Icon { name: "heart-pulse", codepoint: "\u{E36E}", font: LUCIDE, family: "lucide" },
    Icon { name: "heart-x", codepoint: "\u{E6ED}", font: LUCIDE, family: "lucide" },
    Icon { name: "heater", codepoint: "\u{E58E}", font: LUCIDE, family: "lucide" },
    Icon { name: "helicopter", codepoint: "\u{E69D}", font: LUCIDE, family: "lucide" },
    Icon { name: "help-circle", codepoint: "\u{E082}", font: LUCIDE, family: "lucide" },
    Icon { name: "helping-hand", codepoint: "\u{E3B8}", font: LUCIDE, family: "lucide" },
    Icon { name: "hexagon", codepoint: "\u{E0F3}", font: LUCIDE, family: "lucide" },
    Icon { name: "highlighter", codepoint: "\u{E0F4}", font: LUCIDE, family: "lucide" },
    Icon { name: "history", codepoint: "\u{E1F5}", font: LUCIDE, family: "lucide" },
    Icon { name: "home", codepoint: "\u{E0F5}", font: LUCIDE, family: "lucide" },
    Icon { name: "hop", codepoint: "\u{E397}", font: LUCIDE, family: "lucide" },
    Icon { name: "hop-off", codepoint: "\u{E398}", font: LUCIDE, family: "lucide" },
    Icon { name: "hospital", codepoint: "\u{E5D8}", font: LUCIDE, family: "lucide" },
    Icon { name: "hotel", codepoint: "\u{E3E2}", font: LUCIDE, family: "lucide" },
    Icon { name: "hourglass", codepoint: "\u{E296}", font: LUCIDE, family: "lucide" },
    Icon { name: "house", codepoint: "\u{E0F5}", font: LUCIDE, family: "lucide" },
    Icon { name: "house-heart", codepoint: "\u{E695}", font: LUCIDE, family: "lucide" },
    Icon { name: "house-plug", codepoint: "\u{E5F0}", font: LUCIDE, family: "lucide" },
    Icon { name: "house-plus", codepoint: "\u{E5F1}", font: LUCIDE, family: "lucide" },
    Icon { name: "house-wifi", codepoint: "\u{E63C}", font: LUCIDE, family: "lucide" },
    Icon { name: "ice-cream", codepoint: "\u{E353}", font: LUCIDE, family: "lucide" },
    Icon { name: "ice-cream-2", codepoint: "\u{E3A7}", font: LUCIDE, family: "lucide" },
    Icon { name: "ice-cream-bowl", codepoint: "\u{E3A7}", font: LUCIDE, family: "lucide" },
    Icon { name: "ice-cream-cone", codepoint: "\u{E353}", font: LUCIDE, family: "lucide" },
    Icon { name: "id-card", codepoint: "\u{E617}", font: LUCIDE, family: "lucide" },
    Icon { name: "id-card-lanyard", codepoint: "\u{E670}", font: LUCIDE, family: "lucide" },
    Icon { name: "image", codepoint: "\u{E0F6}", font: LUCIDE, family: "lucide" },
    Icon { name: "image-down", codepoint: "\u{E53C}", font: LUCIDE, family: "lucide" },
    Icon { name: "image-minus", codepoint: "\u{E1F6}", font: LUCIDE, family: "lucide" },
    Icon { name: "image-off", codepoint: "\u{E1C0}", font: LUCIDE, family: "lucide" },
    Icon { name: "image-play", codepoint: "\u{E5DF}", font: LUCIDE, family: "lucide" },
    Icon { name: "image-plus", codepoint: "\u{E1F7}", font: LUCIDE, family: "lucide" },
    Icon { name: "image-up", codepoint: "\u{E5CB}", font: LUCIDE, family: "lucide" },
    Icon { name: "image-upscale", codepoint: "\u{E637}", font: LUCIDE, family: "lucide" },
    Icon { name: "images", codepoint: "\u{E5C4}", font: LUCIDE, family: "lucide" },
    Icon { name: "import", codepoint: "\u{E22F}", font: LUCIDE, family: "lucide" },
    Icon { name: "inbox", codepoint: "\u{E0F7}", font: LUCIDE, family: "lucide" },
    Icon { name: "indent", codepoint: "\u{E108}", font: LUCIDE, family: "lucide" },
    Icon { name: "indent-decrease", codepoint: "\u{E107}", font: LUCIDE, family: "lucide" },
    Icon { name: "indent-increase", codepoint: "\u{E108}", font: LUCIDE, family: "lucide" },
    Icon { name: "indian-rupee", codepoint: "\u{E0F8}", font: LUCIDE, family: "lucide" },
    Icon { name: "infinity", codepoint: "\u{E1E7}", font: LUCIDE, family: "lucide" },
    Icon { name: "info", codepoint: "\u{E0F9}", font: LUCIDE, family: "lucide" },
    Icon { name: "inspect", codepoint: "\u{E202}", font: LUCIDE, family: "lucide" },
    Icon { name: "inspection-panel", codepoint: "\u{E583}", font: LUCIDE, family: "lucide" },
    Icon { name: "italic", codepoint: "\u{E0FB}", font: LUCIDE, family: "lucide" },
    Icon { name: "iteration-ccw", codepoint: "\u{E423}", font: LUCIDE, family: "lucide" },
    Icon { name: "iteration-cw", codepoint: "\u{E424}", font: LUCIDE, family: "lucide" },
    Icon { name: "japanese-yen", codepoint: "\u{E0FC}", font: LUCIDE, family: "lucide" },
    Icon { name: "joystick", codepoint: "\u{E355}", font: LUCIDE, family: "lucide" },
    Icon { name: "kanban", codepoint: "\u{E4DC}", font: LUCIDE, family: "lucide" },
    Icon { name: "kanban-square", codepoint: "\u{E170}", font: LUCIDE, family: "lucide" },
    Icon { name: "kanban-square-dashed", codepoint: "\u{E16C}", font: LUCIDE, family: "lucide" },
    Icon { name: "kayak", codepoint: "\u{E68F}", font: LUCIDE, family: "lucide" },
    Icon { name: "key", codepoint: "\u{E0FD}", font: LUCIDE, family: "lucide" },
    Icon { name: "key-round", codepoint: "\u{E4A3}", font: LUCIDE, family: "lucide" },
    Icon { name: "key-square", codepoint: "\u{E4A4}", font: LUCIDE, family: "lucide" },
    Icon { name: "keyboard", codepoint: "\u{E284}", font: LUCIDE, family: "lucide" },
    Icon { name: "keyboard-music", codepoint: "\u{E560}", font: LUCIDE, family: "lucide" },
    Icon { name: "keyboard-off", codepoint: "\u{E5DE}", font: LUCIDE, family: "lucide" },
    Icon { name: "lamp", codepoint: "\u{E2D8}", font: LUCIDE, family: "lucide" },
    Icon { name: "lamp-ceiling", codepoint: "\u{E2D9}", font: LUCIDE, family: "lucide" },
    Icon { name: "lamp-desk", codepoint: "\u{E2DA}", font: LUCIDE, family: "lucide" },
    Icon { name: "lamp-floor", codepoint: "\u{E2DB}", font: LUCIDE, family: "lucide" },
    Icon { name: "lamp-wall-down", codepoint: "\u{E2DC}", font: LUCIDE, family: "lucide" },
    Icon { name: "lamp-wall-up", codepoint: "\u{E2DD}", font: LUCIDE, family: "lucide" },
    Icon { name: "land-plot", codepoint: "\u{E528}", font: LUCIDE, family: "lucide" },
    Icon { name: "landmark", codepoint: "\u{E23A}", font: LUCIDE, family: "lucide" },
    Icon { name: "languages", codepoint: "\u{E0FE}", font: LUCIDE, family: "lucide" },
    Icon { name: "laptop", codepoint: "\u{E1CD}", font: LUCIDE, family: "lucide" },
    Icon { name: "laptop-2", codepoint: "\u{E1D8}", font: LUCIDE, family: "lucide" },
    Icon { name: "laptop-minimal", codepoint: "\u{E1D8}", font: LUCIDE, family: "lucide" },
    Icon { name: "laptop-minimal-check", codepoint: "\u{E632}", font: LUCIDE, family: "lucide" },
    Icon { name: "lasso", codepoint: "\u{E1CE}", font: LUCIDE, family: "lucide" },
    Icon { name: "lasso-select", codepoint: "\u{E1CF}", font: LUCIDE, family: "lucide" },
    Icon { name: "laugh", codepoint: "\u{E300}", font: LUCIDE, family: "lucide" },
    Icon { name: "layer-arrow-down", codepoint: "\u{E72A}", font: LUCIDE, family: "lucide" },
    Icon { name: "layer-arrow-up", codepoint: "\u{E72B}", font: LUCIDE, family: "lucide" },
    Icon { name: "layers", codepoint: "\u{E529}", font: LUCIDE, family: "lucide" },
    Icon { name: "layers-2", codepoint: "\u{E52A}", font: LUCIDE, family: "lucide" },
    Icon { name: "layers-3", codepoint: "\u{E529}", font: LUCIDE, family: "lucide" },
    Icon { name: "layers-arrow-down", codepoint: "\u{E72C}", font: LUCIDE, family: "lucide" },
    Icon { name: "layers-arrow-up", codepoint: "\u{E72D}", font: LUCIDE, family: "lucide" },
    Icon { name: "layers-minus", codepoint: "\u{E6EE}", font: LUCIDE, family: "lucide" },
    Icon { name: "layers-plus", codepoint: "\u{E6E6}", font: LUCIDE, family: "lucide" },
    Icon { name: "layout", codepoint: "\u{E12C}", font: LUCIDE, family: "lucide" },
    Icon { name: "layout-dashboard", codepoint: "\u{E1C1}", font: LUCIDE, family: "lucide" },
    Icon { name: "layout-freeform", codepoint: "\u{E723}", font: LUCIDE, family: "lucide" },
    Icon { name: "layout-grid", codepoint: "\u{E0FF}", font: LUCIDE, family: "lucide" },
    Icon { name: "layout-list", codepoint: "\u{E1D9}", font: LUCIDE, family: "lucide" },
    Icon { name: "layout-panel-left", codepoint: "\u{E470}", font: LUCIDE, family: "lucide" },
    Icon { name: "layout-panel-top", codepoint: "\u{E471}", font: LUCIDE, family: "lucide" },
    Icon { name: "layout-template", codepoint: "\u{E207}", font: LUCIDE, family: "lucide" },
    Icon { name: "leaf", codepoint: "\u{E2DE}", font: LUCIDE, family: "lucide" },
    Icon { name: "leafy-green", codepoint: "\u{E46F}", font: LUCIDE, family: "lucide" },
    Icon { name: "lectern", codepoint: "\u{E5E9}", font: LUCIDE, family: "lucide" },
    Icon { name: "lens-concave", codepoint: "\u{E6B7}", font: LUCIDE, family: "lucide" },
    Icon { name: "lens-convex", codepoint: "\u{E6B8}", font: LUCIDE, family: "lucide" },
    Icon { name: "letter-text", codepoint: "\u{E605}", font: LUCIDE, family: "lucide" },
    Icon { name: "library", codepoint: "\u{E100}", font: LUCIDE, family: "lucide" },
    Icon { name: "library-big", codepoint: "\u{E54E}", font: LUCIDE, family: "lucide" },
    Icon { name: "library-square", codepoint: "\u{E54F}", font: LUCIDE, family: "lucide" },
    Icon { name: "life-buoy", codepoint: "\u{E101}", font: LUCIDE, family: "lucide" },
    Icon { name: "ligature", codepoint: "\u{E43A}", font: LUCIDE, family: "lucide" },
    Icon { name: "lightbulb", codepoint: "\u{E1C2}", font: LUCIDE, family: "lucide" },
    Icon { name: "lightbulb-off", codepoint: "\u{E208}", font: LUCIDE, family: "lucide" },
    Icon { name: "line-chart", codepoint: "\u{E2A5}", font: LUCIDE, family: "lucide" },
    Icon { name: "line-dot-right-horizontal", codepoint: "\u{E6B9}", font: LUCIDE, family: "lucide" },
    Icon { name: "line-squiggle", codepoint: "\u{E67A}", font: LUCIDE, family: "lucide" },
    Icon { name: "line-style", codepoint: "\u{E6DA}", font: LUCIDE, family: "lucide" },
    Icon { name: "link", codepoint: "\u{E102}", font: LUCIDE, family: "lucide" },
    Icon { name: "link-2", codepoint: "\u{E103}", font: LUCIDE, family: "lucide" },
    Icon { name: "link-2-off", codepoint: "\u{E104}", font: LUCIDE, family: "lucide" },
    Icon { name: "list", codepoint: "\u{E106}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-check", codepoint: "\u{E5FA}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-checks", codepoint: "\u{E1D0}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-chevrons-down-up", codepoint: "\u{E694}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-chevrons-up-down", codepoint: "\u{E696}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-clock", codepoint: "\u{E738}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-collapse", codepoint: "\u{E59B}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-end", codepoint: "\u{E2DF}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-filter", codepoint: "\u{E460}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-filter-plus", codepoint: "\u{E639}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-indent-decrease", codepoint: "\u{E107}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-indent-increase", codepoint: "\u{E108}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-minus", codepoint: "\u{E23E}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-music", codepoint: "\u{E2E0}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-ordered", codepoint: "\u{E1D1}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-plus", codepoint: "\u{E23F}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-restart", codepoint: "\u{E452}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-sort-ascending", codepoint: "\u{E702}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-sort-descending", codepoint: "\u{E703}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-start", codepoint: "\u{E2E1}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-todo", codepoint: "\u{E4C3}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-tree", codepoint: "\u{E408}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-video", codepoint: "\u{E2E2}", font: LUCIDE, family: "lucide" },
    Icon { name: "list-x", codepoint: "\u{E240}", font: LUCIDE, family: "lucide" },
    Icon { name: "loader", codepoint: "\u{E109}", font: LUCIDE, family: "lucide" },
    Icon { name: "loader-2", codepoint: "\u{E10A}", font: LUCIDE, family: "lucide" },
    Icon { name: "loader-circle", codepoint: "\u{E10A}", font: LUCIDE, family: "lucide" },
    Icon { name: "loader-pinwheel", codepoint: "\u{E5E6}", font: LUCIDE, family: "lucide" },
    Icon { name: "locate", codepoint: "\u{E1DA}", font: LUCIDE, family: "lucide" },
    Icon { name: "locate-fixed", codepoint: "\u{E1DB}", font: LUCIDE, family: "lucide" },
    Icon { name: "locate-off", codepoint: "\u{E282}", font: LUCIDE, family: "lucide" },
    Icon { name: "location-edit", codepoint: "\u{E655}", font: LUCIDE, family: "lucide" },
    Icon { name: "lock", codepoint: "\u{E10B}", font: LUCIDE, family: "lucide" },
    Icon { name: "lock-keyhole", codepoint: "\u{E531}", font: LUCIDE, family: "lucide" },
    Icon { name: "lock-keyhole-open", codepoint: "\u{E532}", font: LUCIDE, family: "lucide" },
    Icon { name: "lock-open", codepoint: "\u{E10C}", font: LUCIDE, family: "lucide" },
    Icon { name: "log-in", codepoint: "\u{E10D}", font: LUCIDE, family: "lucide" },
    Icon { name: "log-out", codepoint: "\u{E10E}", font: LUCIDE, family: "lucide" },
    Icon { name: "logs", codepoint: "\u{E5F4}", font: LUCIDE, family: "lucide" },
    Icon { name: "lollipop", codepoint: "\u{E4BD}", font: LUCIDE, family: "lucide" },
    Icon { name: "luggage", codepoint: "\u{E2CA}", font: LUCIDE, family: "lucide" },
    Icon { name: "m-square", codepoint: "\u{E503}", font: LUCIDE, family: "lucide" },
    Icon { name: "magnet", codepoint: "\u{E2B5}", font: LUCIDE, family: "lucide" },
    Icon { name: "mail", codepoint: "\u{E10F}", font: LUCIDE, family: "lucide" },
    Icon { name: "mail-badge", codepoint: "\u{E734}", font: LUCIDE, family: "lucide" },
    Icon { name: "mail-check", codepoint: "\u{E361}", font: LUCIDE, family: "lucide" },
    Icon { name: "mail-clock", codepoint: "\u{E73E}", font: LUCIDE, family: "lucide" },
    Icon { name: "mail-minus", codepoint: "\u{E362}", font: LUCIDE, family: "lucide" },
    Icon { name: "mail-open", codepoint: "\u{E363}", font: LUCIDE, family: "lucide" },
    Icon { name: "mail-plus", codepoint: "\u{E364}", font: LUCIDE, family: "lucide" },
    Icon { name: "mail-question", codepoint: "\u{E365}", font: LUCIDE, family: "lucide" },
    Icon { name: "mail-question-mark", codepoint: "\u{E365}", font: LUCIDE, family: "lucide" },
    Icon { name: "mail-search", codepoint: "\u{E366}", font: LUCIDE, family: "lucide" },
    Icon { name: "mail-warning", codepoint: "\u{E367}", font: LUCIDE, family: "lucide" },
    Icon { name: "mail-x", codepoint: "\u{E368}", font: LUCIDE, family: "lucide" },
    Icon { name: "mailbox", codepoint: "\u{E3D4}", font: LUCIDE, family: "lucide" },
    Icon { name: "mails", codepoint: "\u{E369}", font: LUCIDE, family: "lucide" },
    Icon { name: "map", codepoint: "\u{E110}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-minus", codepoint: "\u{E686}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin", codepoint: "\u{E111}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin-check", codepoint: "\u{E60F}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin-check-inside", codepoint: "\u{E610}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin-house", codepoint: "\u{E61C}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin-minus", codepoint: "\u{E611}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin-minus-inside", codepoint: "\u{E612}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin-off", codepoint: "\u{E2A6}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin-pen", codepoint: "\u{E655}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin-plus", codepoint: "\u{E613}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin-plus-inside", codepoint: "\u{E614}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin-search", codepoint: "\u{E6DE}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin-x", codepoint: "\u{E615}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pin-x-inside", codepoint: "\u{E616}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-pinned", codepoint: "\u{E53D}", font: LUCIDE, family: "lucide" },
    Icon { name: "map-plus", codepoint: "\u{E63F}", font: LUCIDE, family: "lucide" },
    Icon { name: "mars", codepoint: "\u{E641}", font: LUCIDE, family: "lucide" },
    Icon { name: "mars-stroke", codepoint: "\u{E642}", font: LUCIDE, family: "lucide" },
    Icon { name: "martini", codepoint: "\u{E2E3}", font: LUCIDE, family: "lucide" },
    Icon { name: "maximize", codepoint: "\u{E112}", font: LUCIDE, family: "lucide" },
    Icon { name: "maximize-2", codepoint: "\u{E113}", font: LUCIDE, family: "lucide" },
    Icon { name: "medal", codepoint: "\u{E36F}", font: LUCIDE, family: "lucide" },
    Icon { name: "megaphone", codepoint: "\u{E235}", font: LUCIDE, family: "lucide" },
    Icon { name: "megaphone-off", codepoint: "\u{E370}", font: LUCIDE, family: "lucide" },
    Icon { name: "meh", codepoint: "\u{E114}", font: LUCIDE, family: "lucide" },
    Icon { name: "memory-stick", codepoint: "\u{E445}", font: LUCIDE, family: "lucide" },
    Icon { name: "menu", codepoint: "\u{E115}", font: LUCIDE, family: "lucide" },
    Icon { name: "menu-square", codepoint: "\u{E453}", font: LUCIDE, family: "lucide" },
    Icon { name: "merge", codepoint: "\u{E43F}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle", codepoint: "\u{E116}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-check", codepoint: "\u{E6BA}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-code", codepoint: "\u{E562}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-dashed", codepoint: "\u{E563}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-dashed-check", codepoint: "\u{E746}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-heart", codepoint: "\u{E564}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-more", codepoint: "\u{E565}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-off", codepoint: "\u{E566}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-plus", codepoint: "\u{E567}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-question", codepoint: "\u{E568}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-question-mark", codepoint: "\u{E568}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-reply", codepoint: "\u{E569}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-warning", codepoint: "\u{E56A}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-circle-x", codepoint: "\u{E56B}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square", codepoint: "\u{E117}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-check", codepoint: "\u{E6BB}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-code", codepoint: "\u{E56C}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-dashed", codepoint: "\u{E40B}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-diff", codepoint: "\u{E56D}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-dot", codepoint: "\u{E56E}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-heart", codepoint: "\u{E56F}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-lock", codepoint: "\u{E62C}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-more", codepoint: "\u{E570}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-off", codepoint: "\u{E571}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-plus", codepoint: "\u{E40C}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-quote", codepoint: "\u{E572}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-reply", codepoint: "\u{E573}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-share", codepoint: "\u{E574}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-text", codepoint: "\u{E575}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-warning", codepoint: "\u{E576}", font: LUCIDE, family: "lucide" },
    Icon { name: "message-square-x", codepoint: "\u{E577}", font: LUCIDE, family: "lucide" },
    Icon { name: "messages-square", codepoint: "\u{E40D}", font: LUCIDE, family: "lucide" },
    Icon { name: "metronome", codepoint: "\u{E6BC}", font: LUCIDE, family: "lucide" },
    Icon { name: "mic", codepoint: "\u{E118}", font: LUCIDE, family: "lucide" },
    Icon { name: "mic-2", codepoint: "\u{E349}", font: LUCIDE, family: "lucide" },
    Icon { name: "mic-audio-lines", codepoint: "\u{E724}", font: LUCIDE, family: "lucide" },
    Icon { name: "mic-off", codepoint: "\u{E119}", font: LUCIDE, family: "lucide" },
    Icon { name: "mic-signal", codepoint: "\u{E1FA}", font: LUCIDE, family: "lucide" },
    Icon { name: "mic-vocal", codepoint: "\u{E349}", font: LUCIDE, family: "lucide" },
    Icon { name: "microchip", codepoint: "\u{E61A}", font: LUCIDE, family: "lucide" },
    Icon { name: "microscope", codepoint: "\u{E2E4}", font: LUCIDE, family: "lucide" },
    Icon { name: "microwave", codepoint: "\u{E37A}", font: LUCIDE, family: "lucide" },
    Icon { name: "midi-port", codepoint: "\u{E739}", font: LUCIDE, family: "lucide" },
    Icon { name: "milestone", codepoint: "\u{E298}", font: LUCIDE, family: "lucide" },
    Icon { name: "milk", codepoint: "\u{E399}", font: LUCIDE, family: "lucide" },
    Icon { name: "milk-off", codepoint: "\u{E39A}", font: LUCIDE, family: "lucide" },
    Icon { name: "minimize", codepoint: "\u{E11A}", font: LUCIDE, family: "lucide" },
    Icon { name: "minimize-2", codepoint: "\u{E11B}", font: LUCIDE, family: "lucide" },
    Icon { name: "minus", codepoint: "\u{E11C}", font: LUCIDE, family: "lucide" },
    Icon { name: "minus-circle", codepoint: "\u{E07E}", font: LUCIDE, family: "lucide" },
    Icon { name: "minus-square", codepoint: "\u{E171}", font: LUCIDE, family: "lucide" },
    Icon { name: "mirror-rectangular", codepoint: "\u{E6BD}", font: LUCIDE, family: "lucide" },
    Icon { name: "mirror-round", codepoint: "\u{E6BE}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor", codepoint: "\u{E11D}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-check", codepoint: "\u{E482}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-cloud", codepoint: "\u{E699}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-cog", codepoint: "\u{E603}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-dot", codepoint: "\u{E483}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-down", codepoint: "\u{E421}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-off", codepoint: "\u{E1DC}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-pause", codepoint: "\u{E484}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-play", codepoint: "\u{E485}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-smartphone", codepoint: "\u{E3A2}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-speaker", codepoint: "\u{E210}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-stop", codepoint: "\u{E486}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-up", codepoint: "\u{E422}", font: LUCIDE, family: "lucide" },
    Icon { name: "monitor-x", codepoint: "\u{E487}", font: LUCIDE, family: "lucide" },
    Icon { name: "moon", codepoint: "\u{E11E}", font: LUCIDE, family: "lucide" },
    Icon { name: "moon-star", codepoint: "\u{E410}", font: LUCIDE, family: "lucide" },
    Icon { name: "mop", codepoint: "\u{E73B}", font: LUCIDE, family: "lucide" },
    Icon { name: "mop-sparkles", codepoint: "\u{E73A}", font: LUCIDE, family: "lucide" },
    Icon { name: "more-horizontal", codepoint: "\u{E0B6}", font: LUCIDE, family: "lucide" },
    Icon { name: "more-vertical", codepoint: "\u{E0B7}", font: LUCIDE, family: "lucide" },
    Icon { name: "mosque", codepoint: "\u{E725}", font: LUCIDE, family: "lucide" },
    Icon { name: "motorbike", codepoint: "\u{E698}", font: LUCIDE, family: "lucide" },
    Icon { name: "mountain", codepoint: "\u{E231}", font: LUCIDE, family: "lucide" },
    Icon { name: "mountain-snow", codepoint: "\u{E232}", font: LUCIDE, family: "lucide" },
    Icon { name: "mouse", codepoint: "\u{E28E}", font: LUCIDE, family: "lucide" },
    Icon { name: "mouse-left", codepoint: "\u{E6BF}", font: LUCIDE, family: "lucide" },
    Icon { name: "mouse-off", codepoint: "\u{E5DB}", font: LUCIDE, family: "lucide" },
    Icon { name: "mouse-pointer", codepoint: "\u{E11F}", font: LUCIDE, family: "lucide" },
    Icon { name: "mouse-pointer-2", codepoint: "\u{E1C3}", font: LUCIDE, family: "lucide" },
    Icon { name: "mouse-pointer-2-off", codepoint: "\u{E6A6}", font: LUCIDE, family: "lucide" },
    Icon { name: "mouse-pointer-ban", codepoint: "\u{E5E7}", font: LUCIDE, family: "lucide" },
    Icon { name: "mouse-pointer-click", codepoint: "\u{E120}", font: LUCIDE, family: "lucide" },
    Icon { name: "mouse-pointer-square-dashed", codepoint: "\u{E509}", font: LUCIDE, family: "lucide" },
    Icon { name: "mouse-right", codepoint: "\u{E6C0}", font: LUCIDE, family: "lucide" },
    Icon { name: "move", codepoint: "\u{E121}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-3-d", codepoint: "\u{E2E5}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-3d", codepoint: "\u{E2E5}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-diagonal", codepoint: "\u{E1C4}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-diagonal-2", codepoint: "\u{E1C5}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-down", codepoint: "\u{E48C}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-down-left", codepoint: "\u{E48D}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-down-right", codepoint: "\u{E48E}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-horizontal", codepoint: "\u{E1C6}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-left", codepoint: "\u{E48F}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-right", codepoint: "\u{E490}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-up", codepoint: "\u{E491}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-up-left", codepoint: "\u{E492}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-up-right", codepoint: "\u{E493}", font: LUCIDE, family: "lucide" },
    Icon { name: "move-vertical", codepoint: "\u{E1C7}", font: LUCIDE, family: "lucide" },
    Icon { name: "music", codepoint: "\u{E122}", font: LUCIDE, family: "lucide" },
    Icon { name: "music-2", codepoint: "\u{E34A}", font: LUCIDE, family: "lucide" },
    Icon { name: "music-3", codepoint: "\u{E34B}", font: LUCIDE, family: "lucide" },
    Icon { name: "music-4", codepoint: "\u{E34C}", font: LUCIDE, family: "lucide" },
    Icon { name: "navigation", codepoint: "\u{E123}", font: LUCIDE, family: "lucide" },
    Icon { name: "navigation-2", codepoint: "\u{E124}", font: LUCIDE, family: "lucide" },
    Icon { name: "navigation-2-off", codepoint: "\u{E2A7}", font: LUCIDE, family: "lucide" },
    Icon { name: "navigation-off", codepoint: "\u{E2A8}", font: LUCIDE, family: "lucide" },
    Icon { name: "network", codepoint: "\u{E125}", font: LUCIDE, family: "lucide" },
    Icon { name: "newspaper", codepoint: "\u{E348}", font: LUCIDE, family: "lucide" },
    Icon { name: "nfc", codepoint: "\u{E3C3}", font: LUCIDE, family: "lucide" },
    Icon { name: "non-binary", codepoint: "\u{E643}", font: LUCIDE, family: "lucide" },
    Icon { name: "notebook", codepoint: "\u{E595}", font: LUCIDE, family: "lucide" },
    Icon { name: "notebook-pen", codepoint: "\u{E596}", font: LUCIDE, family: "lucide" },
    Icon { name: "notebook-tabs", codepoint: "\u{E597}", font: LUCIDE, family: "lucide" },
    Icon { name: "notebook-text", codepoint: "\u{E598}", font: LUCIDE, family: "lucide" },
    Icon { name: "notepad-text", codepoint: "\u{E599}", font: LUCIDE, family: "lucide" },
    Icon { name: "notepad-text-dashed", codepoint: "\u{E59A}", font: LUCIDE, family: "lucide" },
    Icon { name: "nut", codepoint: "\u{E39B}", font: LUCIDE, family: "lucide" },
    Icon { name: "nut-off", codepoint: "\u{E39C}", font: LUCIDE, family: "lucide" },
    Icon { name: "octagon", codepoint: "\u{E126}", font: LUCIDE, family: "lucide" },
    Icon { name: "octagon-alert", codepoint: "\u{E127}", font: LUCIDE, family: "lucide" },
    Icon { name: "octagon-minus", codepoint: "\u{E627}", font: LUCIDE, family: "lucide" },
    Icon { name: "octagon-pause", codepoint: "\u{E21B}", font: LUCIDE, family: "lucide" },
    Icon { name: "octagon-x", codepoint: "\u{E128}", font: LUCIDE, family: "lucide" },
    Icon { name: "omega", codepoint: "\u{E619}", font: LUCIDE, family: "lucide" },
    Icon { name: "option", codepoint: "\u{E1F8}", font: LUCIDE, family: "lucide" },
    Icon { name: "orbit", codepoint: "\u{E3E7}", font: LUCIDE, family: "lucide" },
    Icon { name: "origami", codepoint: "\u{E5E3}", font: LUCIDE, family: "lucide" },
    Icon { name: "outdent", codepoint: "\u{E107}", font: LUCIDE, family: "lucide" },
    Icon { name: "package", codepoint: "\u{E129}", font: LUCIDE, family: "lucide" },
    Icon { name: "package-2", codepoint: "\u{E340}", font: LUCIDE, family: "lucide" },
    Icon { name: "package-check", codepoint: "\u{E266}", font: LUCIDE, family: "lucide" },
    Icon { name: "package-minus", codepoint: "\u{E267}", font: LUCIDE, family: "lucide" },
    Icon { name: "package-open", codepoint: "\u{E2CC}", font: LUCIDE, family: "lucide" },
    Icon { name: "package-plus", codepoint: "\u{E268}", font: LUCIDE, family: "lucide" },
    Icon { name: "package-search", codepoint: "\u{E269}", font: LUCIDE, family: "lucide" },
    Icon { name: "package-x", codepoint: "\u{E26A}", font: LUCIDE, family: "lucide" },
    Icon { name: "paint-bucket", codepoint: "\u{E2E6}", font: LUCIDE, family: "lucide" },
    Icon { name: "paint-roller", codepoint: "\u{E59E}", font: LUCIDE, family: "lucide" },
    Icon { name: "paintbrush", codepoint: "\u{E2E7}", font: LUCIDE, family: "lucide" },
    Icon { name: "paintbrush-2", codepoint: "\u{E2E8}", font: LUCIDE, family: "lucide" },
    Icon { name: "paintbrush-vertical", codepoint: "\u{E2E8}", font: LUCIDE, family: "lucide" },
    Icon { name: "palette", codepoint: "\u{E1DD}", font: LUCIDE, family: "lucide" },
    Icon { name: "palmtree", codepoint: "\u{E281}", font: LUCIDE, family: "lucide" },
    Icon { name: "panda", codepoint: "\u{E668}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-bottom", codepoint: "\u{E42C}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-bottom-close", codepoint: "\u{E42D}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-bottom-dashed", codepoint: "\u{E42E}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-bottom-inactive", codepoint: "\u{E42E}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-bottom-open", codepoint: "\u{E42F}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-left", codepoint: "\u{E12A}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-left-close", codepoint: "\u{E21C}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-left-dashed", codepoint: "\u{E430}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-left-inactive", codepoint: "\u{E430}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-left-open", codepoint: "\u{E21D}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-left-right-dashed", codepoint: "\u{E692}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-right", codepoint: "\u{E431}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-right-close", codepoint: "\u{E432}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-right-dashed", codepoint: "\u{E433}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-right-inactive", codepoint: "\u{E433}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-right-open", codepoint: "\u{E434}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-top", codepoint: "\u{E435}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-top-bottom-dashed", codepoint: "\u{E693}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-top-close", codepoint: "\u{E436}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-top-dashed", codepoint: "\u{E437}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-top-inactive", codepoint: "\u{E437}", font: LUCIDE, family: "lucide" },
    Icon { name: "panel-top-open", codepoint: "\u{E438}", font: LUCIDE, family: "lucide" },
    Icon { name: "panels-left-bottom", codepoint: "\u{E12B}", font: LUCIDE, family: "lucide" },
    Icon { name: "panels-left-right", codepoint: "\u{E099}", font: LUCIDE, family: "lucide" },
    Icon { name: "panels-right-bottom", codepoint: "\u{E588}", font: LUCIDE, family: "lucide" },
    Icon { name: "panels-top-bottom", codepoint: "\u{E58A}", font: LUCIDE, family: "lucide" },
    Icon { name: "panels-top-left", codepoint: "\u{E12C}", font: LUCIDE, family: "lucide" },
    Icon { name: "paper-bag", codepoint: "\u{E71D}", font: LUCIDE, family: "lucide" },
    Icon { name: "paperclip", codepoint: "\u{E12D}", font: LUCIDE, family: "lucide" },
    Icon { name: "parasol", codepoint: "\u{E6FD}", font: LUCIDE, family: "lucide" },
    Icon { name: "parentheses", codepoint: "\u{E444}", font: LUCIDE, family: "lucide" },
    Icon { name: "parking-circle", codepoint: "\u{E3C9}", font: LUCIDE, family: "lucide" },
    Icon { name: "parking-circle-off", codepoint: "\u{E3CA}", font: LUCIDE, family: "lucide" },
    Icon { name: "parking-meter", codepoint: "\u{E500}", font: LUCIDE, family: "lucide" },
    Icon { name: "parking-square", codepoint: "\u{E3CB}", font: LUCIDE, family: "lucide" },
    Icon { name: "parking-square-off", codepoint: "\u{E3CC}", font: LUCIDE, family: "lucide" },
    Icon { name: "party-popper", codepoint: "\u{E343}", font: LUCIDE, family: "lucide" },
    Icon { name: "pause", codepoint: "\u{E12E}", font: LUCIDE, family: "lucide" },
    Icon { name: "pause-circle", codepoint: "\u{E07F}", font: LUCIDE, family: "lucide" },
    Icon { name: "pause-octagon", codepoint: "\u{E21B}", font: LUCIDE, family: "lucide" },
    Icon { name: "paw-print", codepoint: "\u{E4F5}", font: LUCIDE, family: "lucide" },
    Icon { name: "pc-case", codepoint: "\u{E446}", font: LUCIDE, family: "lucide" },
    Icon { name: "pen", codepoint: "\u{E12F}", font: LUCIDE, family: "lucide" },
    Icon { name: "pen-box", codepoint: "\u{E172}", font: LUCIDE, family: "lucide" },
    Icon { name: "pen-line", codepoint: "\u{E130}", font: LUCIDE, family: "lucide" },
    Icon { name: "pen-off", codepoint: "\u{E5EE}", font: LUCIDE, family: "lucide" },
    Icon { name: "pen-square", codepoint: "\u{E172}", font: LUCIDE, family: "lucide" },
    Icon { name: "pen-tool", codepoint: "\u{E131}", font: LUCIDE, family: "lucide" },
    Icon { name: "pencil", codepoint: "\u{E1F9}", font: LUCIDE, family: "lucide" },
    Icon { name: "pencil-line", codepoint: "\u{E4F0}", font: LUCIDE, family: "lucide" },
    Icon { name: "pencil-off", codepoint: "\u{E5EF}", font: LUCIDE, family: "lucide" },
    Icon { name: "pencil-ruler", codepoint: "\u{E4F1}", font: LUCIDE, family: "lucide" },
    Icon { name: "pencil-sparkles", codepoint: "\u{E70F}", font: LUCIDE, family: "lucide" },
    Icon { name: "pentagon", codepoint: "\u{E52B}", font: LUCIDE, family: "lucide" },
    Icon { name: "percent", codepoint: "\u{E132}", font: LUCIDE, family: "lucide" },
    Icon { name: "percent-circle", codepoint: "\u{E51A}", font: LUCIDE, family: "lucide" },
    Icon { name: "percent-diamond", codepoint: "\u{E51B}", font: LUCIDE, family: "lucide" },
    Icon { name: "percent-square", codepoint: "\u{E51C}", font: LUCIDE, family: "lucide" },
    Icon { name: "person-standing", codepoint: "\u{E21E}", font: LUCIDE, family: "lucide" },
    Icon { name: "phi", codepoint: "\u{E71C}", font: LUCIDE, family: "lucide" },
    Icon { name: "philippine-peso", codepoint: "\u{E604}", font: LUCIDE, family: "lucide" },
    Icon { name: "phone", codepoint: "\u{E133}", font: LUCIDE, family: "lucide" },
    Icon { name: "phone-call", codepoint: "\u{E134}", font: LUCIDE, family: "lucide" },
    Icon { name: "phone-forwarded", codepoint: "\u{E135}", font: LUCIDE, family: "lucide" },
    Icon { name: "phone-incoming", codepoint: "\u{E136}", font: LUCIDE, family: "lucide" },
    Icon { name: "phone-missed", codepoint: "\u{E137}", font: LUCIDE, family: "lucide" },
    Icon { name: "phone-off", codepoint: "\u{E138}", font: LUCIDE, family: "lucide" },
    Icon { name: "phone-outgoing", codepoint: "\u{E139}", font: LUCIDE, family: "lucide" },
    Icon { name: "pi", codepoint: "\u{E472}", font: LUCIDE, family: "lucide" },
    Icon { name: "pi-square", codepoint: "\u{E488}", font: LUCIDE, family: "lucide" },
    Icon { name: "piano", codepoint: "\u{E561}", font: LUCIDE, family: "lucide" },
    Icon { name: "pickaxe", codepoint: "\u{E5C6}", font: LUCIDE, family: "lucide" },
    Icon { name: "picture-in-picture", codepoint: "\u{E3AE}", font: LUCIDE, family: "lucide" },
    Icon { name: "picture-in-picture-2", codepoint: "\u{E3AF}", font: LUCIDE, family: "lucide" },
    Icon { name: "pie-chart", codepoint: "\u{E06B}", font: LUCIDE, family: "lucide" },
    Icon { name: "piggy-bank", codepoint: "\u{E13A}", font: LUCIDE, family: "lucide" },
    Icon { name: "pilcrow", codepoint: "\u{E3A3}", font: LUCIDE, family: "lucide" },
    Icon { name: "pilcrow-left", codepoint: "\u{E5DC}", font: LUCIDE, family: "lucide" },
    Icon { name: "pilcrow-right", codepoint: "\u{E5DD}", font: LUCIDE, family: "lucide" },
    Icon { name: "pilcrow-square", codepoint: "\u{E48B}", font: LUCIDE, family: "lucide" },
    Icon { name: "pill", codepoint: "\u{E3BD}", font: LUCIDE, family: "lucide" },
    Icon { name: "pill-bottle", codepoint: "\u{E5EA}", font: LUCIDE, family: "lucide" },
    Icon { name: "pin", codepoint: "\u{E259}", font: LUCIDE, family: "lucide" },
    Icon { name: "pin-off", codepoint: "\u{E2B6}", font: LUCIDE, family: "lucide" },
    Icon { name: "pipette", codepoint: "\u{E13B}", font: LUCIDE, family: "lucide" },
    Icon { name: "pizza", codepoint: "\u{E354}", font: LUCIDE, family: "lucide" },
    Icon { name: "plane", codepoint: "\u{E1DE}", font: LUCIDE, family: "lucide" },
    Icon { name: "plane-landing", codepoint: "\u{E3CD}", font: LUCIDE, family: "lucide" },
    Icon { name: "plane-takeoff", codepoint: "\u{E3CE}", font: LUCIDE, family: "lucide" },
    Icon { name: "play", codepoint: "\u{E13C}", font: LUCIDE, family: "lucide" },
    Icon { name: "play-circle", codepoint: "\u{E080}", font: LUCIDE, family: "lucide" },
    Icon { name: "play-off", codepoint: "\u{E6FE}", font: LUCIDE, family: "lucide" },
    Icon { name: "play-square", codepoint: "\u{E481}", font: LUCIDE, family: "lucide" },
    Icon { name: "playing-card", codepoint: "\u{E747}", font: LUCIDE, family: "lucide" },
    Icon { name: "playing-cards", codepoint: "\u{E749}", font: LUCIDE, family: "lucide" },
    Icon { name: "playing-cards-fan", codepoint: "\u{E748}", font: LUCIDE, family: "lucide" },
    Icon { name: "plug", codepoint: "\u{E37F}", font: LUCIDE, family: "lucide" },
    Icon { name: "plug-2", codepoint: "\u{E380}", font: LUCIDE, family: "lucide" },
    Icon { name: "plug-zap", codepoint: "\u{E45C}", font: LUCIDE, family: "lucide" },
    Icon { name: "plug-zap-2", codepoint: "\u{E45C}", font: LUCIDE, family: "lucide" },
    Icon { name: "plus", codepoint: "\u{E13D}", font: LUCIDE, family: "lucide" },
    Icon { name: "plus-circle", codepoint: "\u{E081}", font: LUCIDE, family: "lucide" },
    Icon { name: "plus-square", codepoint: "\u{E173}", font: LUCIDE, family: "lucide" },
    Icon { name: "pocket-knife", codepoint: "\u{E4A0}", font: LUCIDE, family: "lucide" },
    Icon { name: "podcast", codepoint: "\u{E1FA}", font: LUCIDE, family: "lucide" },
    Icon { name: "podium", codepoint: "\u{E704}", font: LUCIDE, family: "lucide" },
    Icon { name: "pointer", codepoint: "\u{E1E8}", font: LUCIDE, family: "lucide" },
    Icon { name: "pointer-off", codepoint: "\u{E57F}", font: LUCIDE, family: "lucide" },
    Icon { name: "popcorn", codepoint: "\u{E4BE}", font: LUCIDE, family: "lucide" },
    Icon { name: "popsicle", codepoint: "\u{E4BF}", font: LUCIDE, family: "lucide" },
    Icon { name: "pound-sterling", codepoint: "\u{E13F}", font: LUCIDE, family: "lucide" },
    Icon { name: "power", codepoint: "\u{E140}", font: LUCIDE, family: "lucide" },
    Icon { name: "power-circle", codepoint: "\u{E550}", font: LUCIDE, family: "lucide" },
    Icon { name: "power-off", codepoint: "\u{E209}", font: LUCIDE, family: "lucide" },
    Icon { name: "power-square", codepoint: "\u{E551}", font: LUCIDE, family: "lucide" },
    Icon { name: "presentation", codepoint: "\u{E4AE}", font: LUCIDE, family: "lucide" },
    Icon { name: "printer", codepoint: "\u{E141}", font: LUCIDE, family: "lucide" },
    Icon { name: "printer-check", codepoint: "\u{E5F5}", font: LUCIDE, family: "lucide" },
    Icon { name: "printer-x", codepoint: "\u{E6C1}", font: LUCIDE, family: "lucide" },
    Icon { name: "projector", codepoint: "\u{E4AF}", font: LUCIDE, family: "lucide" },
    Icon { name: "proportions", codepoint: "\u{E5CF}", font: LUCIDE, family: "lucide" },
    Icon { name: "puzzle", codepoint: "\u{E29C}", font: LUCIDE, family: "lucide" },
    Icon { name: "pyramid", codepoint: "\u{E52C}", font: LUCIDE, family: "lucide" },
    Icon { name: "qr-code", codepoint: "\u{E1DF}", font: LUCIDE, family: "lucide" },
    Icon { name: "quote", codepoint: "\u{E239}", font: LUCIDE, family: "lucide" },
    Icon { name: "rabbit", codepoint: "\u{E4F6}", font: LUCIDE, family: "lucide" },
    Icon { name: "radar", codepoint: "\u{E497}", font: LUCIDE, family: "lucide" },
    Icon { name: "radiation", codepoint: "\u{E442}", font: LUCIDE, family: "lucide" },
    Icon { name: "radical", codepoint: "\u{E5C2}", font: LUCIDE, family: "lucide" },
    Icon { name: "radio", codepoint: "\u{E142}", font: LUCIDE, family: "lucide" },
    Icon { name: "radio-off", codepoint: "\u{E6DD}", font: LUCIDE, family: "lucide" },
    Icon { name: "radio-receiver", codepoint: "\u{E1FB}", font: LUCIDE, family: "lucide" },
    Icon { name: "radio-tower", codepoint: "\u{E404}", font: LUCIDE, family: "lucide" },
    Icon { name: "radius", codepoint: "\u{E52D}", font: LUCIDE, family: "lucide" },
    Icon { name: "rainbow", codepoint: "\u{E4C2}", font: LUCIDE, family: "lucide" },
    Icon { name: "rat", codepoint: "\u{E3EB}", font: LUCIDE, family: "lucide" },
    Icon { name: "ratio", codepoint: "\u{E4E8}", font: LUCIDE, family: "lucide" },
    Icon { name: "receipt", codepoint: "\u{E3D3}", font: LUCIDE, family: "lucide" },
    Icon { name: "receipt-cent", codepoint: "\u{E5A5}", font: LUCIDE, family: "lucide" },
    Icon { name: "receipt-euro", codepoint: "\u{E5A6}", font: LUCIDE, family: "lucide" },
    Icon { name: "receipt-indian-rupee", codepoint: "\u{E5A7}", font: LUCIDE, family: "lucide" },
    Icon { name: "receipt-japanese-yen", codepoint: "\u{E5A8}", font: LUCIDE, family: "lucide" },
    Icon { name: "receipt-pound-sterling", codepoint: "\u{E5A9}", font: LUCIDE, family: "lucide" },
    Icon { name: "receipt-russian-ruble", codepoint: "\u{E5AA}", font: LUCIDE, family: "lucide" },
    Icon { name: "receipt-swiss-franc", codepoint: "\u{E5AB}", font: LUCIDE, family: "lucide" },
    Icon { name: "receipt-text", codepoint: "\u{E5AC}", font: LUCIDE, family: "lucide" },
    Icon { name: "receipt-turkish-lira", codepoint: "\u{E67F}", font: LUCIDE, family: "lucide" },
    Icon { name: "rectangle-circle", codepoint: "\u{E673}", font: LUCIDE, family: "lucide" },
    Icon { name: "rectangle-ellipsis", codepoint: "\u{E21F}", font: LUCIDE, family: "lucide" },
    Icon { name: "rectangle-goggles", codepoint: "\u{E656}", font: LUCIDE, family: "lucide" },
    Icon { name: "rectangle-horizontal", codepoint: "\u{E376}", font: LUCIDE, family: "lucide" },
    Icon { name: "rectangle-vertical", codepoint: "\u{E377}", font: LUCIDE, family: "lucide" },
    Icon { name: "recycle", codepoint: "\u{E2E9}", font: LUCIDE, family: "lucide" },
    Icon { name: "redo", codepoint: "\u{E143}", font: LUCIDE, family: "lucide" },
    Icon { name: "redo-2", codepoint: "\u{E2A0}", font: LUCIDE, family: "lucide" },
    Icon { name: "redo-dot", codepoint: "\u{E450}", font: LUCIDE, family: "lucide" },
    Icon { name: "refresh-ccw", codepoint: "\u{E144}", font: LUCIDE, family: "lucide" },
    Icon { name: "refresh-ccw-dot", codepoint: "\u{E4B2}", font: LUCIDE, family: "lucide" },
    Icon { name: "refresh-cw", codepoint: "\u{E145}", font: LUCIDE, family: "lucide" },
    Icon { name: "refresh-cw-off", codepoint: "\u{E498}", font: LUCIDE, family: "lucide" },
    Icon { name: "refrigerator", codepoint: "\u{E37B}", font: LUCIDE, family: "lucide" },
    Icon { name: "regex", codepoint: "\u{E1FC}", font: LUCIDE, family: "lucide" },
    Icon { name: "remove-formatting", codepoint: "\u{E3B3}", font: LUCIDE, family: "lucide" },
    Icon { name: "repeat", codepoint: "\u{E146}", font: LUCIDE, family: "lucide" },
    Icon { name: "repeat-1", codepoint: "\u{E1FD}", font: LUCIDE, family: "lucide" },
    Icon { name: "repeat-2", codepoint: "\u{E411}", font: LUCIDE, family: "lucide" },
    Icon { name: "repeat-off", codepoint: "\u{E6F3}", font: LUCIDE, family: "lucide" },
    Icon { name: "replace", codepoint: "\u{E3DB}", font: LUCIDE, family: "lucide" },
    Icon { name: "replace-all", codepoint: "\u{E3DC}", font: LUCIDE, family: "lucide" },
    Icon { name: "reply", codepoint: "\u{E22A}", font: LUCIDE, family: "lucide" },
    Icon { name: "reply-all", codepoint: "\u{E22B}", font: LUCIDE, family: "lucide" },
    Icon { name: "rewind", codepoint: "\u{E147}", font: LUCIDE, family: "lucide" },
    Icon { name: "ribbon", codepoint: "\u{E558}", font: LUCIDE, family: "lucide" },
    Icon { name: "road", codepoint: "\u{E6D9}", font: LUCIDE, family: "lucide" },
    Icon { name: "robot-arm", codepoint: "\u{E740}", font: LUCIDE, family: "lucide" },
    Icon { name: "robot-vacuum", codepoint: "\u{E74A}", font: LUCIDE, family: "lucide" },
    Icon { name: "rocket", codepoint: "\u{E286}", font: LUCIDE, family: "lucide" },
    Icon { name: "rocking-chair", codepoint: "\u{E233}", font: LUCIDE, family: "lucide" },
    Icon { name: "roller-coaster", codepoint: "\u{E480}", font: LUCIDE, family: "lucide" },
    Icon { name: "rose", codepoint: "\u{E691}", font: LUCIDE, family: "lucide" },
    Icon { name: "rotate-3-d", codepoint: "\u{E2EA}", font: LUCIDE, family: "lucide" },
    Icon { name: "rotate-3d", codepoint: "\u{E2EA}", font: LUCIDE, family: "lucide" },
    Icon { name: "rotate-ccw", codepoint: "\u{E148}", font: LUCIDE, family: "lucide" },
    Icon { name: "rotate-ccw-clock", codepoint: "\u{E1F5}", font: LUCIDE, family: "lucide" },
    Icon { name: "rotate-ccw-key", codepoint: "\u{E650}", font: LUCIDE, family: "lucide" },
    Icon { name: "rotate-ccw-square", codepoint: "\u{E5D0}", font: LUCIDE, family: "lucide" },
    Icon { name: "rotate-cw", codepoint: "\u{E149}", font: LUCIDE, family: "lucide" },
    Icon { name: "rotate-cw-fading-clock", codepoint: "\u{E726}", font: LUCIDE, family: "lucide" },
    Icon { name: "rotate-cw-square", codepoint: "\u{E5D1}", font: LUCIDE, family: "lucide" },
    Icon { name: "route", codepoint: "\u{E53E}", font: LUCIDE, family: "lucide" },
    Icon { name: "route-off", codepoint: "\u{E53F}", font: LUCIDE, family: "lucide" },
    Icon { name: "router", codepoint: "\u{E3BF}", font: LUCIDE, family: "lucide" },
    Icon { name: "rows", codepoint: "\u{E439}", font: LUCIDE, family: "lucide" },
    Icon { name: "rows-2", codepoint: "\u{E439}", font: LUCIDE, family: "lucide" },
    Icon { name: "rows-3", codepoint: "\u{E58A}", font: LUCIDE, family: "lucide" },
    Icon { name: "rows-4", codepoint: "\u{E58B}", font: LUCIDE, family: "lucide" },
    Icon { name: "rss", codepoint: "\u{E14A}", font: LUCIDE, family: "lucide" },
    Icon { name: "ruler", codepoint: "\u{E14B}", font: LUCIDE, family: "lucide" },
    Icon { name: "ruler-dimension-line", codepoint: "\u{E662}", font: LUCIDE, family: "lucide" },
    Icon { name: "russian-ruble", codepoint: "\u{E14C}", font: LUCIDE, family: "lucide" },
    Icon { name: "sailboat", codepoint: "\u{E37E}", font: LUCIDE, family: "lucide" },
    Icon { name: "salad", codepoint: "\u{E3A8}", font: LUCIDE, family: "lucide" },
    Icon { name: "sandwich", codepoint: "\u{E3A9}", font: LUCIDE, family: "lucide" },
    Icon { name: "satellite", codepoint: "\u{E447}", font: LUCIDE, family: "lucide" },
    Icon { name: "satellite-dish", codepoint: "\u{E448}", font: LUCIDE, family: "lucide" },
    Icon { name: "saudi-riyal", codepoint: "\u{E64B}", font: LUCIDE, family: "lucide" },
    Icon { name: "save", codepoint: "\u{E14D}", font: LUCIDE, family: "lucide" },
    Icon { name: "save-all", codepoint: "\u{E40F}", font: LUCIDE, family: "lucide" },
    Icon { name: "save-check", codepoint: "\u{E710}", font: LUCIDE, family: "lucide" },
    Icon { name: "save-off", codepoint: "\u{E5F3}", font: LUCIDE, family: "lucide" },
    Icon { name: "save-pen", codepoint: "\u{E705}", font: LUCIDE, family: "lucide" },
    Icon { name: "save-plus", codepoint: "\u{E706}", font: LUCIDE, family: "lucide" },
    Icon { name: "scale", codepoint: "\u{E212}", font: LUCIDE, family: "lucide" },
    Icon { name: "scale-3-d", codepoint: "\u{E2EB}", font: LUCIDE, family: "lucide" },
    Icon { name: "scale-3d", codepoint: "\u{E2EB}", font: LUCIDE, family: "lucide" },
    Icon { name: "scaling", codepoint: "\u{E2EC}", font: LUCIDE, family: "lucide" },
    Icon { name: "scan", codepoint: "\u{E257}", font: LUCIDE, family: "lucide" },
    Icon { name: "scan-barcode", codepoint: "\u{E535}", font: LUCIDE, family: "lucide" },
    Icon { name: "scan-box", codepoint: "\u{E720}", font: LUCIDE, family: "lucide" },
    Icon { name: "scan-eye", codepoint: "\u{E536}", font: LUCIDE, family: "lucide" },
    Icon { name: "scan-face", codepoint: "\u{E371}", font: LUCIDE, family: "lucide" },
    Icon { name: "scan-heart", codepoint: "\u{E63A}", font: LUCIDE, family: "lucide" },
    Icon { name: "scan-line", codepoint: "\u{E258}", font: LUCIDE, family: "lucide" },
    Icon { name: "scan-qr-code", codepoint: "\u{E5F6}", font: LUCIDE, family: "lucide" },
    Icon { name: "scan-search", codepoint: "\u{E537}", font: LUCIDE, family: "lucide" },
    Icon { name: "scan-square", codepoint: "\u{E727}", font: LUCIDE, family: "lucide" },
    Icon { name: "scan-text", codepoint: "\u{E538}", font: LUCIDE, family: "lucide" },
    Icon { name: "scatter-chart", codepoint: "\u{E48A}", font: LUCIDE, family: "lucide" },
    Icon { name: "school", codepoint: "\u{E3E3}", font: LUCIDE, family: "lucide" },
    Icon { name: "school-2", codepoint: "\u{E3E5}", font: LUCIDE, family: "lucide" },
    Icon { name: "scissors", codepoint: "\u{E14E}", font: LUCIDE, family: "lucide" },
    Icon { name: "scissors-line-dashed", codepoint: "\u{E4E9}", font: LUCIDE, family: "lucide" },
    Icon { name: "scissors-square", codepoint: "\u{E4EC}", font: LUCIDE, family: "lucide" },
    Icon { name: "scissors-square-dashed-bottom", codepoint: "\u{E4EB}", font: LUCIDE, family: "lucide" },
    Icon { name: "scooter", codepoint: "\u{E6AC}", font: LUCIDE, family: "lucide" },
    Icon { name: "screen-share", codepoint: "\u{E14F}", font: LUCIDE, family: "lucide" },
    Icon { name: "screen-share-off", codepoint: "\u{E150}", font: LUCIDE, family: "lucide" },
    Icon { name: "scroll", codepoint: "\u{E2ED}", font: LUCIDE, family: "lucide" },
    Icon { name: "scroll-text", codepoint: "\u{E45F}", font: LUCIDE, family: "lucide" },
    Icon { name: "search", codepoint: "\u{E151}", font: LUCIDE, family: "lucide" },
    Icon { name: "search-alert", codepoint: "\u{E6E7}", font: LUCIDE, family: "lucide" },
    Icon { name: "search-check", codepoint: "\u{E4AA}", font: LUCIDE, family: "lucide" },
    Icon { name: "search-code", codepoint: "\u{E4AB}", font: LUCIDE, family: "lucide" },
    Icon { name: "search-slash", codepoint: "\u{E4AC}", font: LUCIDE, family: "lucide" },
    Icon { name: "search-x", codepoint: "\u{E4AD}", font: LUCIDE, family: "lucide" },
    Icon { name: "section", codepoint: "\u{E5E8}", font: LUCIDE, family: "lucide" },
    Icon { name: "send", codepoint: "\u{E152}", font: LUCIDE, family: "lucide" },
    Icon { name: "send-horizonal", codepoint: "\u{E4F2}", font: LUCIDE, family: "lucide" },
    Icon { name: "send-horizontal", codepoint: "\u{E4F2}", font: LUCIDE, family: "lucide" },
    Icon { name: "send-to-back", codepoint: "\u{E4F3}", font: LUCIDE, family: "lucide" },
    Icon { name: "separator-horizontal", codepoint: "\u{E1C8}", font: LUCIDE, family: "lucide" },
    Icon { name: "separator-vertical", codepoint: "\u{E1C9}", font: LUCIDE, family: "lucide" },
    Icon { name: "server", codepoint: "\u{E153}", font: LUCIDE, family: "lucide" },
    Icon { name: "server-cog", codepoint: "\u{E341}", font: LUCIDE, family: "lucide" },
    Icon { name: "server-crash", codepoint: "\u{E1E9}", font: LUCIDE, family: "lucide" },
    Icon { name: "server-off", codepoint: "\u{E1EA}", font: LUCIDE, family: "lucide" },
    Icon { name: "server-plus", codepoint: "\u{E721}", font: LUCIDE, family: "lucide" },
    Icon { name: "settings", codepoint: "\u{E154}", font: LUCIDE, family: "lucide" },
    Icon { name: "settings-2", codepoint: "\u{E245}", font: LUCIDE, family: "lucide" },
    Icon { name: "shapes", codepoint: "\u{E4B3}", font: LUCIDE, family: "lucide" },
    Icon { name: "share", codepoint: "\u{E155}", font: LUCIDE, family: "lucide" },
    Icon { name: "share-2", codepoint: "\u{E156}", font: LUCIDE, family: "lucide" },
    Icon { name: "sheet", codepoint: "\u{E157}", font: LUCIDE, family: "lucide" },
    Icon { name: "shell", codepoint: "\u{E4F7}", font: LUCIDE, family: "lucide" },
    Icon { name: "shelving-unit", codepoint: "\u{E6C2}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield", codepoint: "\u{E158}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-alert", codepoint: "\u{E1FE}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-ban", codepoint: "\u{E159}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-check", codepoint: "\u{E1FF}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-close", codepoint: "\u{E200}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-cog", codepoint: "\u{E6E9}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-cog-corner", codepoint: "\u{E6E8}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-ellipsis", codepoint: "\u{E516}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-half", codepoint: "\u{E517}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-keyhole", codepoint: "\u{E722}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-lock", codepoint: "\u{E731}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-minus", codepoint: "\u{E518}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-off", codepoint: "\u{E15A}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-plus", codepoint: "\u{E519}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-question", codepoint: "\u{E40E}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-question-mark", codepoint: "\u{E40E}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-user", codepoint: "\u{E647}", font: LUCIDE, family: "lucide" },
    Icon { name: "shield-x", codepoint: "\u{E200}", font: LUCIDE, family: "lucide" },
    Icon { name: "ship", codepoint: "\u{E3BA}", font: LUCIDE, family: "lucide" },
    Icon { name: "ship-cargo", codepoint: "\u{E741}", font: LUCIDE, family: "lucide" },
    Icon { name: "ship-wheel", codepoint: "\u{E502}", font: LUCIDE, family: "lucide" },
    Icon { name: "shirt", codepoint: "\u{E1CA}", font: LUCIDE, family: "lucide" },
    Icon { name: "shopping-bag", codepoint: "\u{E15B}", font: LUCIDE, family: "lucide" },
    Icon { name: "shopping-basket", codepoint: "\u{E4EA}", font: LUCIDE, family: "lucide" },
    Icon { name: "shopping-cart", codepoint: "\u{E15C}", font: LUCIDE, family: "lucide" },
    Icon { name: "shovel", codepoint: "\u{E15D}", font: LUCIDE, family: "lucide" },
    Icon { name: "shower-head", codepoint: "\u{E37C}", font: LUCIDE, family: "lucide" },
    Icon { name: "shredder", codepoint: "\u{E65B}", font: LUCIDE, family: "lucide" },
    Icon { name: "shrimp", codepoint: "\u{E649}", font: LUCIDE, family: "lucide" },
    Icon { name: "shrink", codepoint: "\u{E220}", font: LUCIDE, family: "lucide" },
    Icon { name: "shrub", codepoint: "\u{E2EE}", font: LUCIDE, family: "lucide" },
    Icon { name: "shuffle", codepoint: "\u{E15E}", font: LUCIDE, family: "lucide" },
    Icon { name: "sidebar", codepoint: "\u{E12A}", font: LUCIDE, family: "lucide" },
    Icon { name: "sidebar-close", codepoint: "\u{E21C}", font: LUCIDE, family: "lucide" },
    Icon { name: "sidebar-open", codepoint: "\u{E21D}", font: LUCIDE, family: "lucide" },
    Icon { name: "sigma", codepoint: "\u{E201}", font: LUCIDE, family: "lucide" },
    Icon { name: "sigma-square", codepoint: "\u{E489}", font: LUCIDE, family: "lucide" },
    Icon { name: "signal", codepoint: "\u{E25F}", font: LUCIDE, family: "lucide" },
    Icon { name: "signal-high", codepoint: "\u{E260}", font: LUCIDE, family: "lucide" },
    Icon { name: "signal-low", codepoint: "\u{E261}", font: LUCIDE, family: "lucide" },
    Icon { name: "signal-medium", codepoint: "\u{E262}", font: LUCIDE, family: "lucide" },
    Icon { name: "signal-zero", codepoint: "\u{E263}", font: LUCIDE, family: "lucide" },
    Icon { name: "signature", codepoint: "\u{E5F2}", font: LUCIDE, family: "lucide" },
    Icon { name: "signpost", codepoint: "\u{E540}", font: LUCIDE, family: "lucide" },
    Icon { name: "signpost-big", codepoint: "\u{E541}", font: LUCIDE, family: "lucide" },
    Icon { name: "siren", codepoint: "\u{E2EF}", font: LUCIDE, family: "lucide" },
    Icon { name: "skip-back", codepoint: "\u{E15F}", font: LUCIDE, family: "lucide" },
    Icon { name: "skip-forward", codepoint: "\u{E160}", font: LUCIDE, family: "lucide" },
    Icon { name: "skull", codepoint: "\u{E221}", font: LUCIDE, family: "lucide" },
    Icon { name: "slash", codepoint: "\u{E51D}", font: LUCIDE, family: "lucide" },
    Icon { name: "slash-square", codepoint: "\u{E174}", font: LUCIDE, family: "lucide" },
    Icon { name: "slice", codepoint: "\u{E2F0}", font: LUCIDE, family: "lucide" },
    Icon { name: "sliders", codepoint: "\u{E162}", font: LUCIDE, family: "lucide" },
    Icon { name: "sliders-horizontal", codepoint: "\u{E29A}", font: LUCIDE, family: "lucide" },
    Icon { name: "sliders-vertical", codepoint: "\u{E162}", font: LUCIDE, family: "lucide" },
    Icon { name: "smartphone", codepoint: "\u{E163}", font: LUCIDE, family: "lucide" },
    Icon { name: "smartphone-charging", codepoint: "\u{E22E}", font: LUCIDE, family: "lucide" },
    Icon { name: "smartphone-nfc", codepoint: "\u{E3C4}", font: LUCIDE, family: "lucide" },
    Icon { name: "smile", codepoint: "\u{E164}", font: LUCIDE, family: "lucide" },
    Icon { name: "smile-plus", codepoint: "\u{E301}", font: LUCIDE, family: "lucide" },
    Icon { name: "snail", codepoint: "\u{E4F8}", font: LUCIDE, family: "lucide" },
    Icon { name: "snowflake", codepoint: "\u{E165}", font: LUCIDE, family: "lucide" },
    Icon { name: "soap-dispenser-droplet", codepoint: "\u{E669}", font: LUCIDE, family: "lucide" },
    Icon { name: "sofa", codepoint: "\u{E2C4}", font: LUCIDE, family: "lucide" },
    Icon { name: "solar-panel", codepoint: "\u{E69F}", font: LUCIDE, family: "lucide" },
    Icon { name: "sort-asc", codepoint: "\u{E04C}", font: LUCIDE, family: "lucide" },
    Icon { name: "sort-desc", codepoint: "\u{E047}", font: LUCIDE, family: "lucide" },
    Icon { name: "soup", codepoint: "\u{E3AA}", font: LUCIDE, family: "lucide" },
    Icon { name: "space", codepoint: "\u{E3DD}", font: LUCIDE, family: "lucide" },
    Icon { name: "spade", codepoint: "\u{E499}", font: LUCIDE, family: "lucide" },
    Icon { name: "sparkle", codepoint: "\u{E47E}", font: LUCIDE, family: "lucide" },
    Icon { name: "sparkles", codepoint: "\u{E412}", font: LUCIDE, family: "lucide" },
    Icon { name: "speaker", codepoint: "\u{E166}", font: LUCIDE, family: "lucide" },
    Icon { name: "speech", codepoint: "\u{E51E}", font: LUCIDE, family: "lucide" },
    Icon { name: "spell-check", codepoint: "\u{E49A}", font: LUCIDE, family: "lucide" },
    Icon { name: "spell-check-2", codepoint: "\u{E49B}", font: LUCIDE, family: "lucide" },
    Icon { name: "spline", codepoint: "\u{E38B}", font: LUCIDE, family: "lucide" },
    Icon { name: "spline-pointer", codepoint: "\u{E64F}", font: LUCIDE, family: "lucide" },
    Icon { name: "split", codepoint: "\u{E440}", font: LUCIDE, family: "lucide" },
    Icon { name: "split-square-horizontal", codepoint: "\u{E3B6}", font: LUCIDE, family: "lucide" },
    Icon { name: "split-square-vertical", codepoint: "\u{E3B7}", font: LUCIDE, family: "lucide" },
    Icon { name: "spool", codepoint: "\u{E677}", font: LUCIDE, family: "lucide" },
    Icon { name: "sport-shoe", codepoint: "\u{E6DB}", font: LUCIDE, family: "lucide" },
    Icon { name: "spotlight", codepoint: "\u{E682}", font: LUCIDE, family: "lucide" },
    Icon { name: "spray-can", codepoint: "\u{E495}", font: LUCIDE, family: "lucide" },
    Icon { name: "sprout", codepoint: "\u{E1EB}", font: LUCIDE, family: "lucide" },
    Icon { name: "square", codepoint: "\u{E167}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-activity", codepoint: "\u{E4B4}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-down", codepoint: "\u{E427}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-down-left", codepoint: "\u{E4B5}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-down-right", codepoint: "\u{E4B6}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-left", codepoint: "\u{E428}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-out-down-left", codepoint: "\u{E5A1}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-out-down-right", codepoint: "\u{E5A2}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-out-up-left", codepoint: "\u{E5A3}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-out-up-right", codepoint: "\u{E5A4}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-right", codepoint: "\u{E429}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-right-enter", codepoint: "\u{E6C3}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-right-exit", codepoint: "\u{E6C4}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-up", codepoint: "\u{E42A}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-up-left", codepoint: "\u{E4B7}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-arrow-up-right", codepoint: "\u{E4B8}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-asterisk", codepoint: "\u{E168}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-bottom-dashed-scissors", codepoint: "\u{E4EB}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-centerline-dashed-horizontal", codepoint: "\u{E6C5}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-centerline-dashed-vertical", codepoint: "\u{E6C6}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-chart-gantt", codepoint: "\u{E169}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-check", codepoint: "\u{E559}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-check-big", codepoint: "\u{E16A}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-chevron-down", codepoint: "\u{E3CF}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-chevron-left", codepoint: "\u{E3D0}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-chevron-right", codepoint: "\u{E3D1}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-chevron-up", codepoint: "\u{E3D2}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-code", codepoint: "\u{E16B}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-dashed", codepoint: "\u{E1CB}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-dashed-bottom", codepoint: "\u{E4C0}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-dashed-bottom-code", codepoint: "\u{E4C1}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-dashed-kanban", codepoint: "\u{E16C}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-dashed-mouse-pointer", codepoint: "\u{E509}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-dashed-text", codepoint: "\u{E6EA}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-dashed-top-solid", codepoint: "\u{E66C}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-dimensions", codepoint: "\u{E73C}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-divide", codepoint: "\u{E16D}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-dot", codepoint: "\u{E16E}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-equal", codepoint: "\u{E16F}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-function", codepoint: "\u{E22D}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-gantt-chart", codepoint: "\u{E169}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-kanban", codepoint: "\u{E170}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-library", codepoint: "\u{E54F}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-m", codepoint: "\u{E503}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-menu", codepoint: "\u{E453}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-minus", codepoint: "\u{E171}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-mouse-pointer", codepoint: "\u{E202}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-off", codepoint: "\u{E728}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-parking", codepoint: "\u{E3CB}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-parking-off", codepoint: "\u{E3CC}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-pause", codepoint: "\u{E684}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-pen", codepoint: "\u{E172}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-percent", codepoint: "\u{E51C}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-pi", codepoint: "\u{E488}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-pilcrow", codepoint: "\u{E48B}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-play", codepoint: "\u{E481}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-plus", codepoint: "\u{E173}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-power", codepoint: "\u{E551}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-radical", codepoint: "\u{E5C3}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-round-corner", codepoint: "\u{E648}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-scissors", codepoint: "\u{E4EC}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-sigma", codepoint: "\u{E489}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-slash", codepoint: "\u{E174}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-split-horizontal", codepoint: "\u{E3B6}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-split-vertical", codepoint: "\u{E3B7}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-square", codepoint: "\u{E60E}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-stack", codepoint: "\u{E4A2}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-star", codepoint: "\u{E68E}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-stop", codepoint: "\u{E685}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-terminal", codepoint: "\u{E20A}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-text", codepoint: "\u{E736}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-user", codepoint: "\u{E465}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-user-round", codepoint: "\u{E466}", font: LUCIDE, family: "lucide" },
    Icon { name: "square-x", codepoint: "\u{E175}", font: LUCIDE, family: "lucide" },
    Icon { name: "squares-exclude", codepoint: "\u{E657}", font: LUCIDE, family: "lucide" },
    Icon { name: "squares-intersect", codepoint: "\u{E658}", font: LUCIDE, family: "lucide" },
    Icon { name: "squares-subtract", codepoint: "\u{E659}", font: LUCIDE, family: "lucide" },
    Icon { name: "squares-unite", codepoint: "\u{E65A}", font: LUCIDE, family: "lucide" },
    Icon { name: "squircle", codepoint: "\u{E57A}", font: LUCIDE, family: "lucide" },
    Icon { name: "squircle-dashed", codepoint: "\u{E679}", font: LUCIDE, family: "lucide" },
    Icon { name: "squirrel", codepoint: "\u{E49F}", font: LUCIDE, family: "lucide" },
    Icon { name: "stamp", codepoint: "\u{E3BB}", font: LUCIDE, family: "lucide" },
    Icon { name: "star", codepoint: "\u{E176}", font: LUCIDE, family: "lucide" },
    Icon { name: "star-check", codepoint: "\u{E707}", font: LUCIDE, family: "lucide" },
    Icon { name: "star-half", codepoint: "\u{E20B}", font: LUCIDE, family: "lucide" },
    Icon { name: "star-minus", codepoint: "\u{E708}", font: LUCIDE, family: "lucide" },
    Icon { name: "star-off", codepoint: "\u{E2B0}", font: LUCIDE, family: "lucide" },
    Icon { name: "star-plus", codepoint: "\u{E709}", font: LUCIDE, family: "lucide" },
    Icon { name: "star-x", codepoint: "\u{E70A}", font: LUCIDE, family: "lucide" },
    Icon { name: "stars", codepoint: "\u{E412}", font: LUCIDE, family: "lucide" },
    Icon { name: "step-back", codepoint: "\u{E3E9}", font: LUCIDE, family: "lucide" },
    Icon { name: "step-forward", codepoint: "\u{E3EA}", font: LUCIDE, family: "lucide" },
    Icon { name: "stethoscope", codepoint: "\u{E2F1}", font: LUCIDE, family: "lucide" },
    Icon { name: "sticker", codepoint: "\u{E302}", font: LUCIDE, family: "lucide" },
    Icon { name: "sticky-note", codepoint: "\u{E303}", font: LUCIDE, family: "lucide" },
    Icon { name: "sticky-note-check", codepoint: "\u{E6F6}", font: LUCIDE, family: "lucide" },
    Icon { name: "sticky-note-minus", codepoint: "\u{E6F7}", font: LUCIDE, family: "lucide" },
    Icon { name: "sticky-note-off", codepoint: "\u{E6F8}", font: LUCIDE, family: "lucide" },
    Icon { name: "sticky-note-plus", codepoint: "\u{E6F9}", font: LUCIDE, family: "lucide" },
    Icon { name: "sticky-note-x", codepoint: "\u{E6FA}", font: LUCIDE, family: "lucide" },
    Icon { name: "sticky-notes", codepoint: "\u{E6FB}", font: LUCIDE, family: "lucide" },
    Icon { name: "stone", codepoint: "\u{E6EB}", font: LUCIDE, family: "lucide" },
    Icon { name: "stop-circle", codepoint: "\u{E083}", font: LUCIDE, family: "lucide" },
    Icon { name: "store", codepoint: "\u{E3E4}", font: LUCIDE, family: "lucide" },
    Icon { name: "stretch-horizontal", codepoint: "\u{E27C}", font: LUCIDE, family: "lucide" },
    Icon { name: "stretch-vertical", codepoint: "\u{E27D}", font: LUCIDE, family: "lucide" },
    Icon { name: "strikethrough", codepoint: "\u{E177}", font: LUCIDE, family: "lucide" },
    Icon { name: "subscript", codepoint: "\u{E25C}", font: LUCIDE, family: "lucide" },
    Icon { name: "subtitles", codepoint: "\u{E3A4}", font: LUCIDE, family: "lucide" },
    Icon { name: "summary", codepoint: "\u{E711}", font: LUCIDE, family: "lucide" },
    Icon { name: "sun", codepoint: "\u{E178}", font: LUCIDE, family: "lucide" },
    Icon { name: "sun-dim", codepoint: "\u{E299}", font: LUCIDE, family: "lucide" },
    Icon { name: "sun-medium", codepoint: "\u{E2B1}", font: LUCIDE, family: "lucide" },
    Icon { name: "sun-moon", codepoint: "\u{E2B2}", font: LUCIDE, family: "lucide" },
    Icon { name: "sun-snow", codepoint: "\u{E372}", font: LUCIDE, family: "lucide" },
    Icon { name: "sunrise", codepoint: "\u{E179}", font: LUCIDE, family: "lucide" },
    Icon { name: "sunset", codepoint: "\u{E17A}", font: LUCIDE, family: "lucide" },
    Icon { name: "superscript", codepoint: "\u{E25E}", font: LUCIDE, family: "lucide" },
    Icon { name: "swatch-book", codepoint: "\u{E59F}", font: LUCIDE, family: "lucide" },
    Icon { name: "swiss-franc", codepoint: "\u{E17B}", font: LUCIDE, family: "lucide" },
    Icon { name: "switch-camera", codepoint: "\u{E17C}", font: LUCIDE, family: "lucide" },
    Icon { name: "sword", codepoint: "\u{E2B3}", font: LUCIDE, family: "lucide" },
    Icon { name: "swords", codepoint: "\u{E2B4}", font: LUCIDE, family: "lucide" },
    Icon { name: "syringe", codepoint: "\u{E2F2}", font: LUCIDE, family: "lucide" },
    Icon { name: "table", codepoint: "\u{E17D}", font: LUCIDE, family: "lucide" },
    Icon { name: "table-2", codepoint: "\u{E2F9}", font: LUCIDE, family: "lucide" },
    Icon { name: "table-cells-merge", codepoint: "\u{E5C7}", font: LUCIDE, family: "lucide" },
    Icon { name: "table-cells-split", codepoint: "\u{E5C8}", font: LUCIDE, family: "lucide" },
    Icon { name: "table-columns-split", codepoint: "\u{E5C9}", font: LUCIDE, family: "lucide" },
    Icon { name: "table-config", codepoint: "\u{E661}", font: LUCIDE, family: "lucide" },
    Icon { name: "table-of-contents", codepoint: "\u{E61E}", font: LUCIDE, family: "lucide" },
    Icon { name: "table-properties", codepoint: "\u{E4DB}", font: LUCIDE, family: "lucide" },
    Icon { name: "table-rows-split", codepoint: "\u{E5CA}", font: LUCIDE, family: "lucide" },
    Icon { name: "tablet", codepoint: "\u{E17E}", font: LUCIDE, family: "lucide" },
    Icon { name: "tablet-smartphone", codepoint: "\u{E50A}", font: LUCIDE, family: "lucide" },
    Icon { name: "tablets", codepoint: "\u{E3BE}", font: LUCIDE, family: "lucide" },
    Icon { name: "tag", codepoint: "\u{E17F}", font: LUCIDE, family: "lucide" },
    Icon { name: "tag-plus", codepoint: "\u{E712}", font: LUCIDE, family: "lucide" },
    Icon { name: "tag-x", codepoint: "\u{E713}", font: LUCIDE, family: "lucide" },
    Icon { name: "tags", codepoint: "\u{E35C}", font: LUCIDE, family: "lucide" },
    Icon { name: "tally-1", codepoint: "\u{E4D6}", font: LUCIDE, family: "lucide" },
    Icon { name: "tally-2", codepoint: "\u{E4D7}", font: LUCIDE, family: "lucide" },
    Icon { name: "tally-3", codepoint: "\u{E4D8}", font: LUCIDE, family: "lucide" },
    Icon { name: "tally-4", codepoint: "\u{E4D9}", font: LUCIDE, family: "lucide" },
    Icon { name: "tally-5", codepoint: "\u{E4DA}", font: LUCIDE, family: "lucide" },
    Icon { name: "tangent", codepoint: "\u{E52E}", font: LUCIDE, family: "lucide" },
    Icon { name: "target", codepoint: "\u{E180}", font: LUCIDE, family: "lucide" },
    Icon { name: "telescope", codepoint: "\u{E5C5}", font: LUCIDE, family: "lucide" },
    Icon { name: "tent", codepoint: "\u{E227}", font: LUCIDE, family: "lucide" },
    Icon { name: "tent-tree", codepoint: "\u{E53B}", font: LUCIDE, family: "lucide" },
    Icon { name: "terminal", codepoint: "\u{E181}", font: LUCIDE, family: "lucide" },
    Icon { name: "terminal-square", codepoint: "\u{E20A}", font: LUCIDE, family: "lucide" },
    Icon { name: "test-tube", codepoint: "\u{E405}", font: LUCIDE, family: "lucide" },
    Icon { name: "test-tube-2", codepoint: "\u{E406}", font: LUCIDE, family: "lucide" },
    Icon { name: "test-tube-diagonal", codepoint: "\u{E406}", font: LUCIDE, family: "lucide" },
    Icon { name: "test-tubes", codepoint: "\u{E407}", font: LUCIDE, family: "lucide" },
    Icon { name: "text", codepoint: "\u{E185}", font: LUCIDE, family: "lucide" },
    Icon { name: "text-align-center", codepoint: "\u{E182}", font: LUCIDE, family: "lucide" },
    Icon { name: "text-align-end", codepoint: "\u{E183}", font: LUCIDE, family: "lucide" },
    Icon { name: "text-align-justify", codepoint: "\u{E184}", font: LUCIDE, family: "lucide" },
    Icon { name: "text-align-start", codepoint: "\u{E185}", font: LUCIDE, family: "lucide" },
    Icon { name: "text-cursor", codepoint: "\u{E264}", font: LUCIDE, family: "lucide" },
    Icon { name: "text-cursor-input", codepoint: "\u{E265}", font: LUCIDE, family: "lucide" },
    Icon { name: "text-initial", codepoint: "\u{E605}", font: LUCIDE, family: "lucide" },
    Icon { name: "text-quote", codepoint: "\u{E49E}", font: LUCIDE, family: "lucide" },
    Icon { name: "text-search", codepoint: "\u{E5AD}", font: LUCIDE, family: "lucide" },
    Icon { name: "text-select", codepoint: "\u{E6EA}", font: LUCIDE, family: "lucide" },
    Icon { name: "text-selection", codepoint: "\u{E3DE}", font: LUCIDE, family: "lucide" },
    Icon { name: "text-wrap", codepoint: "\u{E248}", font: LUCIDE, family: "lucide" },
    Icon { name: "theater", codepoint: "\u{E522}", font: LUCIDE, family: "lucide" },
    Icon { name: "thermometer", codepoint: "\u{E186}", font: LUCIDE, family: "lucide" },
    Icon { name: "thermometer-snowflake", codepoint: "\u{E187}", font: LUCIDE, family: "lucide" },
    Icon { name: "thermometer-sun", codepoint: "\u{E188}", font: LUCIDE, family: "lucide" },
    Icon { name: "thumbs-down", codepoint: "\u{E189}", font: LUCIDE, family: "lucide" },
    Icon { name: "thumbs-up", codepoint: "\u{E18A}", font: LUCIDE, family: "lucide" },
    Icon { name: "ticket", codepoint: "\u{E20F}", font: LUCIDE, family: "lucide" },
    Icon { name: "ticket-check", codepoint: "\u{E5AE}", font: LUCIDE, family: "lucide" },
    Icon { name: "ticket-minus", codepoint: "\u{E5AF}", font: LUCIDE, family: "lucide" },
    Icon { name: "ticket-percent", codepoint: "\u{E5B0}", font: LUCIDE, family: "lucide" },
    Icon { name: "ticket-plus", codepoint: "\u{E5B1}", font: LUCIDE, family: "lucide" },
    Icon { name: "ticket-slash", codepoint: "\u{E5B2}", font: LUCIDE, family: "lucide" },
    Icon { name: "ticket-x", codepoint: "\u{E5B3}", font: LUCIDE, family: "lucide" },
    Icon { name: "tickets", codepoint: "\u{E622}", font: LUCIDE, family: "lucide" },
    Icon { name: "tickets-plane", codepoint: "\u{E623}", font: LUCIDE, family: "lucide" },
    Icon { name: "timeline", codepoint: "\u{E6EC}", font: LUCIDE, family: "lucide" },
    Icon { name: "timer", codepoint: "\u{E1E0}", font: LUCIDE, family: "lucide" },
    Icon { name: "timer-off", codepoint: "\u{E249}", font: LUCIDE, family: "lucide" },
    Icon { name: "timer-reset", codepoint: "\u{E236}", font: LUCIDE, family: "lucide" },
    Icon { name: "toggle-left", codepoint: "\u{E18B}", font: LUCIDE, family: "lucide" },
    Icon { name: "toggle-right", codepoint: "\u{E18C}", font: LUCIDE, family: "lucide" },
    Icon { name: "toilet", codepoint: "\u{E635}", font: LUCIDE, family: "lucide" },
    Icon { name: "tool-case", codepoint: "\u{E67D}", font: LUCIDE, family: "lucide" },
    Icon { name: "toolbox", codepoint: "\u{E6B0}", font: LUCIDE, family: "lucide" },
    Icon { name: "tornado", codepoint: "\u{E218}", font: LUCIDE, family: "lucide" },
    Icon { name: "torus", codepoint: "\u{E52F}", font: LUCIDE, family: "lucide" },
    Icon { name: "touchpad", codepoint: "\u{E449}", font: LUCIDE, family: "lucide" },
    Icon { name: "touchpad-off", codepoint: "\u{E44A}", font: LUCIDE, family: "lucide" },
    Icon { name: "towel-rack", codepoint: "\u{E6C7}", font: LUCIDE, family: "lucide" },
    Icon { name: "tower-control", codepoint: "\u{E3BC}", font: LUCIDE, family: "lucide" },
    Icon { name: "toy-brick", codepoint: "\u{E347}", font: LUCIDE, family: "lucide" },
    Icon { name: "tractor", codepoint: "\u{E504}", font: LUCIDE, family: "lucide" },
    Icon { name: "traffic-cone", codepoint: "\u{E505}", font: LUCIDE, family: "lucide" },
    Icon { name: "trailer", codepoint: "\u{E742}", font: LUCIDE, family: "lucide" },
    Icon { name: "train", codepoint: "\u{E2A9}", font: LUCIDE, family: "lucide" },
    Icon { name: "train-front", codepoint: "\u{E506}", font: LUCIDE, family: "lucide" },
    Icon { name: "train-front-tunnel", codepoint: "\u{E507}", font: LUCIDE, family: "lucide" },
    Icon { name: "train-track", codepoint: "\u{E508}", font: LUCIDE, family: "lucide" },
    Icon { name: "tram-front", codepoint: "\u{E2A9}", font: LUCIDE, family: "lucide" },
    Icon { name: "transgender", codepoint: "\u{E644}", font: LUCIDE, family: "lucide" },
    Icon { name: "trash", codepoint: "\u{E18D}", font: LUCIDE, family: "lucide" },
    Icon { name: "trash-2", codepoint: "\u{E18E}", font: LUCIDE, family: "lucide" },
    Icon { name: "tree-deciduous", codepoint: "\u{E2F3}", font: LUCIDE, family: "lucide" },
    Icon { name: "tree-palm", codepoint: "\u{E281}", font: LUCIDE, family: "lucide" },
    Icon { name: "tree-pine", codepoint: "\u{E2F4}", font: LUCIDE, family: "lucide" },
    Icon { name: "trees", codepoint: "\u{E2F5}", font: LUCIDE, family: "lucide" },
    Icon { name: "trending-down", codepoint: "\u{E190}", font: LUCIDE, family: "lucide" },
    Icon { name: "trending-up", codepoint: "\u{E191}", font: LUCIDE, family: "lucide" },
    Icon { name: "trending-up-down", codepoint: "\u{E625}", font: LUCIDE, family: "lucide" },
    Icon { name: "triangle", codepoint: "\u{E192}", font: LUCIDE, family: "lucide" },
    Icon { name: "triangle-alert", codepoint: "\u{E193}", font: LUCIDE, family: "lucide" },
    Icon { name: "triangle-dashed", codepoint: "\u{E63D}", font: LUCIDE, family: "lucide" },
    Icon { name: "triangle-right", codepoint: "\u{E4ED}", font: LUCIDE, family: "lucide" },
    Icon { name: "trophy", codepoint: "\u{E373}", font: LUCIDE, family: "lucide" },
    Icon { name: "truck", codepoint: "\u{E194}", font: LUCIDE, family: "lucide" },
    Icon { name: "truck-electric", codepoint: "\u{E65F}", font: LUCIDE, family: "lucide" },
    Icon { name: "turkish-lira", codepoint: "\u{E680}", font: LUCIDE, family: "lucide" },
    Icon { name: "turntable", codepoint: "\u{E68C}", font: LUCIDE, family: "lucide" },
    Icon { name: "turtle", codepoint: "\u{E4F9}", font: LUCIDE, family: "lucide" },
    Icon { name: "tv", codepoint: "\u{E195}", font: LUCIDE, family: "lucide" },
    Icon { name: "tv-2", codepoint: "\u{E203}", font: LUCIDE, family: "lucide" },
    Icon { name: "tv-minimal", codepoint: "\u{E203}", font: LUCIDE, family: "lucide" },
    Icon { name: "tv-minimal-play", codepoint: "\u{E5EC}", font: LUCIDE, family: "lucide" },
    Icon { name: "type", codepoint: "\u{E198}", font: LUCIDE, family: "lucide" },
    Icon { name: "type-outline", codepoint: "\u{E602}", font: LUCIDE, family: "lucide" },
    Icon { name: "umbrella", codepoint: "\u{E199}", font: LUCIDE, family: "lucide" },
    Icon { name: "umbrella-off", codepoint: "\u{E543}", font: LUCIDE, family: "lucide" },
    Icon { name: "underline", codepoint: "\u{E19A}", font: LUCIDE, family: "lucide" },
    Icon { name: "undo", codepoint: "\u{E19B}", font: LUCIDE, family: "lucide" },
    Icon { name: "undo-2", codepoint: "\u{E2A1}", font: LUCIDE, family: "lucide" },
    Icon { name: "undo-dot", codepoint: "\u{E451}", font: LUCIDE, family: "lucide" },
    Icon { name: "unfold-horizontal", codepoint: "\u{E43D}", font: LUCIDE, family: "lucide" },
    Icon { name: "unfold-vertical", codepoint: "\u{E43E}", font: LUCIDE, family: "lucide" },
    Icon { name: "ungroup", codepoint: "\u{E467}", font: LUCIDE, family: "lucide" },
    Icon { name: "university", codepoint: "\u{E3E5}", font: LUCIDE, family: "lucide" },
    Icon { name: "unlink", codepoint: "\u{E19C}", font: LUCIDE, family: "lucide" },
    Icon { name: "unlink-2", codepoint: "\u{E19D}", font: LUCIDE, family: "lucide" },
    Icon { name: "unlock", codepoint: "\u{E10C}", font: LUCIDE, family: "lucide" },
    Icon { name: "unlock-keyhole", codepoint: "\u{E532}", font: LUCIDE, family: "lucide" },
    Icon { name: "unplug", codepoint: "\u{E45D}", font: LUCIDE, family: "lucide" },
    Icon { name: "upload", codepoint: "\u{E19E}", font: LUCIDE, family: "lucide" },
    Icon { name: "upload-cloud", codepoint: "\u{E091}", font: LUCIDE, family: "lucide" },
    Icon { name: "usb", codepoint: "\u{E356}", font: LUCIDE, family: "lucide" },
    Icon { name: "usb-c-port", codepoint: "\u{E73D}", font: LUCIDE, family: "lucide" },
    Icon { name: "user", codepoint: "\u{E19F}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-2", codepoint: "\u{E468}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-check", codepoint: "\u{E1A0}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-check-2", codepoint: "\u{E469}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-circle", codepoint: "\u{E461}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-circle-2", codepoint: "\u{E462}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-cog", codepoint: "\u{E342}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-cog-2", codepoint: "\u{E46A}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-key", codepoint: "\u{E6C8}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-lock", codepoint: "\u{E660}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-minus", codepoint: "\u{E1A1}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-minus-2", codepoint: "\u{E46B}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-pen", codepoint: "\u{E5FC}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-plus", codepoint: "\u{E1A2}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-plus-2", codepoint: "\u{E46C}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-round", codepoint: "\u{E468}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-round-arrow-left", codepoint: "\u{E714}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-round-check", codepoint: "\u{E469}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-round-cog", codepoint: "\u{E46A}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-round-key", codepoint: "\u{E6C9}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-round-minus", codepoint: "\u{E46B}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-round-pen", codepoint: "\u{E5FD}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-round-plus", codepoint: "\u{E46C}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-round-search", codepoint: "\u{E578}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-round-x", codepoint: "\u{E46D}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-search", codepoint: "\u{E579}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-shield", codepoint: "\u{E729}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-square", codepoint: "\u{E465}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-square-2", codepoint: "\u{E466}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-star", codepoint: "\u{E687}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-x", codepoint: "\u{E1A3}", font: LUCIDE, family: "lucide" },
    Icon { name: "user-x-2", codepoint: "\u{E46D}", font: LUCIDE, family: "lucide" },
    Icon { name: "users", codepoint: "\u{E1A4}", font: LUCIDE, family: "lucide" },
    Icon { name: "users-2", codepoint: "\u{E46E}", font: LUCIDE, family: "lucide" },
    Icon { name: "users-round", codepoint: "\u{E46E}", font: LUCIDE, family: "lucide" },
    Icon { name: "utensils", codepoint: "\u{E2F6}", font: LUCIDE, family: "lucide" },
    Icon { name: "utensils-crossed", codepoint: "\u{E2F7}", font: LUCIDE, family: "lucide" },
    Icon { name: "utility-pole", codepoint: "\u{E3C2}", font: LUCIDE, family: "lucide" },
    Icon { name: "van", codepoint: "\u{E6AD}", font: LUCIDE, family: "lucide" },
    Icon { name: "variable", codepoint: "\u{E473}", font: LUCIDE, family: "lucide" },
    Icon { name: "vault", codepoint: "\u{E58F}", font: LUCIDE, family: "lucide" },
    Icon { name: "vector-square", codepoint: "\u{E67C}", font: LUCIDE, family: "lucide" },
    Icon { name: "vegan", codepoint: "\u{E39D}", font: LUCIDE, family: "lucide" },
    Icon { name: "venetian-mask", codepoint: "\u{E2AA}", font: LUCIDE, family: "lucide" },
    Icon { name: "venus", codepoint: "\u{E645}", font: LUCIDE, family: "lucide" },
    Icon { name: "venus-and-mars", codepoint: "\u{E646}", font: LUCIDE, family: "lucide" },
    Icon { name: "verified", codepoint: "\u{E241}", font: LUCIDE, family: "lucide" },
    Icon { name: "vibrate", codepoint: "\u{E223}", font: LUCIDE, family: "lucide" },
    Icon { name: "vibrate-off", codepoint: "\u{E29D}", font: LUCIDE, family: "lucide" },
    Icon { name: "video", codepoint: "\u{E1A5}", font: LUCIDE, family: "lucide" },
    Icon { name: "video-off", codepoint: "\u{E1A6}", font: LUCIDE, family: "lucide" },
    Icon { name: "videotape", codepoint: "\u{E4CB}", font: LUCIDE, family: "lucide" },
    Icon { name: "view", codepoint: "\u{E1A7}", font: LUCIDE, family: "lucide" },
    Icon { name: "voicemail", codepoint: "\u{E1A8}", font: LUCIDE, family: "lucide" },
    Icon { name: "volleyball", codepoint: "\u{E62F}", font: LUCIDE, family: "lucide" },
    Icon { name: "volume", codepoint: "\u{E1A9}", font: LUCIDE, family: "lucide" },
    Icon { name: "volume-1", codepoint: "\u{E1AA}", font: LUCIDE, family: "lucide" },
    Icon { name: "volume-2", codepoint: "\u{E1AB}", font: LUCIDE, family: "lucide" },
    Icon { name: "volume-off", codepoint: "\u{E626}", font: LUCIDE, family: "lucide" },
    Icon { name: "volume-x", codepoint: "\u{E1AC}", font: LUCIDE, family: "lucide" },
    Icon { name: "vote", codepoint: "\u{E3AD}", font: LUCIDE, family: "lucide" },
    Icon { name: "wallet", codepoint: "\u{E204}", font: LUCIDE, family: "lucide" },
    Icon { name: "wallet-2", codepoint: "\u{E4CD}", font: LUCIDE, family: "lucide" },
    Icon { name: "wallet-cards", codepoint: "\u{E4CC}", font: LUCIDE, family: "lucide" },
    Icon { name: "wallet-minimal", codepoint: "\u{E4CD}", font: LUCIDE, family: "lucide" },
    Icon { name: "wallpaper", codepoint: "\u{E44B}", font: LUCIDE, family: "lucide" },
    Icon { name: "wand", codepoint: "\u{E246}", font: LUCIDE, family: "lucide" },
    Icon { name: "wand-2", codepoint: "\u{E357}", font: LUCIDE, family: "lucide" },
    Icon { name: "wand-sparkles", codepoint: "\u{E357}", font: LUCIDE, family: "lucide" },
    Icon { name: "warehouse", codepoint: "\u{E3E6}", font: LUCIDE, family: "lucide" },
    Icon { name: "washing-machine", codepoint: "\u{E590}", font: LUCIDE, family: "lucide" },
    Icon { name: "watch", codepoint: "\u{E1AD}", font: LUCIDE, family: "lucide" },
    Icon { name: "waves", codepoint: "\u{E283}", font: LUCIDE, family: "lucide" },
    Icon { name: "waves-arrow-down", codepoint: "\u{E6A9}", font: LUCIDE, family: "lucide" },
    Icon { name: "waves-arrow-up", codepoint: "\u{E6AA}", font: LUCIDE, family: "lucide" },
    Icon { name: "waves-horizontal", codepoint: "\u{E283}", font: LUCIDE, family: "lucide" },
    Icon { name: "waves-ladder", codepoint: "\u{E63B}", font: LUCIDE, family: "lucide" },
    Icon { name: "waves-vertical", codepoint: "\u{E6F2}", font: LUCIDE, family: "lucide" },
    Icon { name: "waypoints", codepoint: "\u{E542}", font: LUCIDE, family: "lucide" },
    Icon { name: "webcam", codepoint: "\u{E205}", font: LUCIDE, family: "lucide" },
    Icon { name: "webcam-off", codepoint: "\u{E6FF}", font: LUCIDE, family: "lucide" },
    Icon { name: "webhook", codepoint: "\u{E374}", font: LUCIDE, family: "lucide" },
    Icon { name: "webhook-off", codepoint: "\u{E5B7}", font: LUCIDE, family: "lucide" },
    Icon { name: "weight", codepoint: "\u{E530}", font: LUCIDE, family: "lucide" },
    Icon { name: "weight-tilde", codepoint: "\u{E6AE}", font: LUCIDE, family: "lucide" },
    Icon { name: "wheat", codepoint: "\u{E39E}", font: LUCIDE, family: "lucide" },
    Icon { name: "wheat-off", codepoint: "\u{E39F}", font: LUCIDE, family: "lucide" },
    Icon { name: "whole-word", codepoint: "\u{E3DF}", font: LUCIDE, family: "lucide" },
    Icon { name: "wifi", codepoint: "\u{E1AE}", font: LUCIDE, family: "lucide" },
    Icon { name: "wifi-cog", codepoint: "\u{E674}", font: LUCIDE, family: "lucide" },
    Icon { name: "wifi-high", codepoint: "\u{E5F7}", font: LUCIDE, family: "lucide" },
    Icon { name: "wifi-low", codepoint: "\u{E5F8}", font: LUCIDE, family: "lucide" },
    Icon { name: "wifi-off", codepoint: "\u{E1AF}", font: LUCIDE, family: "lucide" },
    Icon { name: "wifi-pen", codepoint: "\u{E663}", font: LUCIDE, family: "lucide" },
    Icon { name: "wifi-sync", codepoint: "\u{E681}", font: LUCIDE, family: "lucide" },
    Icon { name: "wifi-zero", codepoint: "\u{E5F9}", font: LUCIDE, family: "lucide" },
    Icon { name: "wind", codepoint: "\u{E1B0}", font: LUCIDE, family: "lucide" },
    Icon { name: "wind-arrow-down", codepoint: "\u{E631}", font: LUCIDE, family: "lucide" },
    Icon { name: "wine", codepoint: "\u{E2F8}", font: LUCIDE, family: "lucide" },
    Icon { name: "wine-off", codepoint: "\u{E3A0}", font: LUCIDE, family: "lucide" },
    Icon { name: "workflow", codepoint: "\u{E425}", font: LUCIDE, family: "lucide" },
    Icon { name: "worm", codepoint: "\u{E5DA}", font: LUCIDE, family: "lucide" },
    Icon { name: "wrap-text", codepoint: "\u{E248}", font: LUCIDE, family: "lucide" },
    Icon { name: "wrench", codepoint: "\u{E1B1}", font: LUCIDE, family: "lucide" },
    Icon { name: "wrench-off", codepoint: "\u{E70B}", font: LUCIDE, family: "lucide" },
    Icon { name: "x", codepoint: "\u{E1B2}", font: LUCIDE, family: "lucide" },
    Icon { name: "x-circle", codepoint: "\u{E084}", font: LUCIDE, family: "lucide" },
    Icon { name: "x-line-top", codepoint: "\u{E6CA}", font: LUCIDE, family: "lucide" },
    Icon { name: "x-octagon", codepoint: "\u{E128}", font: LUCIDE, family: "lucide" },
    Icon { name: "x-square", codepoint: "\u{E175}", font: LUCIDE, family: "lucide" },
    Icon { name: "zap", codepoint: "\u{E1B4}", font: LUCIDE, family: "lucide" },
    Icon { name: "zap-off", codepoint: "\u{E1B5}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-aquarius", codepoint: "\u{E6CB}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-aries", codepoint: "\u{E6CC}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-cancer", codepoint: "\u{E6CD}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-capricorn", codepoint: "\u{E6CE}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-gemini", codepoint: "\u{E6CF}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-leo", codepoint: "\u{E6D0}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-libra", codepoint: "\u{E6D1}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-ophiuchus", codepoint: "\u{E6D2}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-pisces", codepoint: "\u{E6D3}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-sagittarius", codepoint: "\u{E6D4}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-scorpio", codepoint: "\u{E6D5}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-taurus", codepoint: "\u{E6D6}", font: LUCIDE, family: "lucide" },
    Icon { name: "zodiac-virgo", codepoint: "\u{E6D7}", font: LUCIDE, family: "lucide" },
    Icon { name: "zoom-in", codepoint: "\u{E1B6}", font: LUCIDE, family: "lucide" },
    Icon { name: "zoom-out", codepoint: "\u{E1B7}", font: LUCIDE, family: "lucide" },
];

/// `a-arrow-down` from Lucide.
pub fn a_arrow_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E585}").font(LUCIDE)
}

/// `a-arrow-up` from Lucide.
pub fn a_arrow_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E586}").font(LUCIDE)
}

/// `a-large-small` from Lucide.
pub fn a_large_small<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E587}").font(LUCIDE)
}

/// `accessibility` from Lucide.
pub fn accessibility<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E297}").font(LUCIDE)
}

/// `activity` from Lucide.
pub fn activity<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E038}").font(LUCIDE)
}

/// `activity-square` from Lucide.
pub fn activity_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B4}").font(LUCIDE)
}

/// `ad` from Lucide.
pub fn ad<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E700}").font(LUCIDE)
}

/// `air-vent` from Lucide.
pub fn air_vent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E34D}").font(LUCIDE)
}

/// `airplay` from Lucide.
pub fn airplay<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E039}").font(LUCIDE)
}

/// `alarm-check` from Lucide.
pub fn alarm_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1EC}").font(LUCIDE)
}

/// `alarm-clock` from Lucide.
pub fn alarm_clock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E03A}").font(LUCIDE)
}

/// `alarm-clock-check` from Lucide.
pub fn alarm_clock_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1EC}").font(LUCIDE)
}

/// `alarm-clock-minus` from Lucide.
pub fn alarm_clock_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1ED}").font(LUCIDE)
}

/// `alarm-clock-off` from Lucide.
pub fn alarm_clock_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E23B}").font(LUCIDE)
}

/// `alarm-clock-plus` from Lucide.
pub fn alarm_clock_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1EE}").font(LUCIDE)
}

/// `alarm-minus` from Lucide.
pub fn alarm_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1ED}").font(LUCIDE)
}

/// `alarm-plus` from Lucide.
pub fn alarm_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1EE}").font(LUCIDE)
}

/// `alarm-smoke` from Lucide.
pub fn alarm_smoke<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E57B}").font(LUCIDE)
}

/// `album` from Lucide.
pub fn album<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E03B}").font(LUCIDE)
}

/// `alert-circle` from Lucide.
pub fn alert_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E077}").font(LUCIDE)
}

/// `alert-octagon` from Lucide.
pub fn alert_octagon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E127}").font(LUCIDE)
}

/// `alert-triangle` from Lucide.
pub fn alert_triangle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E193}").font(LUCIDE)
}

/// `align-center` from Lucide.
pub fn align_center<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E182}").font(LUCIDE)
}

/// `align-center-horizontal` from Lucide.
pub fn align_center_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E26C}").font(LUCIDE)
}

/// `align-center-vertical` from Lucide.
pub fn align_center_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E26D}").font(LUCIDE)
}

/// `align-end-horizontal` from Lucide.
pub fn align_end_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E26E}").font(LUCIDE)
}

/// `align-end-vertical` from Lucide.
pub fn align_end_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E26F}").font(LUCIDE)
}

/// `align-horizontal-distribute-center` from Lucide.
pub fn align_horizontal_distribute_center<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E03C}").font(LUCIDE)
}

/// `align-horizontal-distribute-end` from Lucide.
pub fn align_horizontal_distribute_end<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E03D}").font(LUCIDE)
}

/// `align-horizontal-distribute-start` from Lucide.
pub fn align_horizontal_distribute_start<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E03E}").font(LUCIDE)
}

/// `align-horizontal-justify-center` from Lucide.
pub fn align_horizontal_justify_center<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E272}").font(LUCIDE)
}

/// `align-horizontal-justify-end` from Lucide.
pub fn align_horizontal_justify_end<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E273}").font(LUCIDE)
}

/// `align-horizontal-justify-start` from Lucide.
pub fn align_horizontal_justify_start<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E274}").font(LUCIDE)
}

/// `align-horizontal-space-around` from Lucide.
pub fn align_horizontal_space_around<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E275}").font(LUCIDE)
}

/// `align-horizontal-space-between` from Lucide.
pub fn align_horizontal_space_between<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E276}").font(LUCIDE)
}

/// `align-justify` from Lucide.
pub fn align_justify<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E184}").font(LUCIDE)
}

/// `align-left` from Lucide.
pub fn align_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E185}").font(LUCIDE)
}

/// `align-right` from Lucide.
pub fn align_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E183}").font(LUCIDE)
}

/// `align-start-horizontal` from Lucide.
pub fn align_start_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E270}").font(LUCIDE)
}

/// `align-start-vertical` from Lucide.
pub fn align_start_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E271}").font(LUCIDE)
}

/// `align-vertical-distribute-center` from Lucide.
pub fn align_vertical_distribute_center<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E27E}").font(LUCIDE)
}

/// `align-vertical-distribute-end` from Lucide.
pub fn align_vertical_distribute_end<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E27F}").font(LUCIDE)
}

/// `align-vertical-distribute-start` from Lucide.
pub fn align_vertical_distribute_start<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E280}").font(LUCIDE)
}

/// `align-vertical-justify-center` from Lucide.
pub fn align_vertical_justify_center<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E277}").font(LUCIDE)
}

/// `align-vertical-justify-end` from Lucide.
pub fn align_vertical_justify_end<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E278}").font(LUCIDE)
}

/// `align-vertical-justify-start` from Lucide.
pub fn align_vertical_justify_start<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E279}").font(LUCIDE)
}

/// `align-vertical-space-around` from Lucide.
pub fn align_vertical_space_around<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E27A}").font(LUCIDE)
}

/// `align-vertical-space-between` from Lucide.
pub fn align_vertical_space_between<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E27B}").font(LUCIDE)
}

/// `ambulance` from Lucide.
pub fn ambulance<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5BB}").font(LUCIDE)
}

/// `ampersand` from Lucide.
pub fn ampersand<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E49C}").font(LUCIDE)
}

/// `ampersands` from Lucide.
pub fn ampersands<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E49D}").font(LUCIDE)
}

/// `amphora` from Lucide.
pub fn amphora<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E61B}").font(LUCIDE)
}

/// `anchor` from Lucide.
pub fn anchor<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E03F}").font(LUCIDE)
}

/// `angle` from Lucide.
pub fn angle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E732}").font(LUCIDE)
}

/// `angry` from Lucide.
pub fn angry<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2FC}").font(LUCIDE)
}

/// `annoyed` from Lucide.
pub fn annoyed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2FD}").font(LUCIDE)
}

/// `antenna` from Lucide.
pub fn antenna<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4E2}").font(LUCIDE)
}

/// `anvil` from Lucide.
pub fn anvil<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E580}").font(LUCIDE)
}

/// `aperture` from Lucide.
pub fn aperture<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E040}").font(LUCIDE)
}

/// `app-window` from Lucide.
pub fn app_window<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E426}").font(LUCIDE)
}

/// `app-window-mac` from Lucide.
pub fn app_window_mac<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5D2}").font(LUCIDE)
}

/// `apple` from Lucide.
pub fn apple<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E34E}").font(LUCIDE)
}

/// `archive` from Lucide.
pub fn archive<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E041}").font(LUCIDE)
}

/// `archive-restore` from Lucide.
pub fn archive_restore<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2CD}").font(LUCIDE)
}

/// `archive-x` from Lucide.
pub fn archive_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E50C}").font(LUCIDE)
}

/// `area-chart` from Lucide.
pub fn area_chart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4D3}").font(LUCIDE)
}

/// `armchair` from Lucide.
pub fn armchair<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2C0}").font(LUCIDE)
}

/// `arrow-big-down` from Lucide.
pub fn arrow_big_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1E1}").font(LUCIDE)
}

/// `arrow-big-down-dash` from Lucide.
pub fn arrow_big_down_dash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E41D}").font(LUCIDE)
}

/// `arrow-big-left` from Lucide.
pub fn arrow_big_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1E2}").font(LUCIDE)
}

/// `arrow-big-left-dash` from Lucide.
pub fn arrow_big_left_dash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E41E}").font(LUCIDE)
}

/// `arrow-big-right` from Lucide.
pub fn arrow_big_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1E3}").font(LUCIDE)
}

/// `arrow-big-right-dash` from Lucide.
pub fn arrow_big_right_dash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E41F}").font(LUCIDE)
}

/// `arrow-big-up` from Lucide.
pub fn arrow_big_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1E4}").font(LUCIDE)
}

/// `arrow-big-up-dash` from Lucide.
pub fn arrow_big_up_dash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E420}").font(LUCIDE)
}

/// `arrow-down` from Lucide.
pub fn arrow_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E042}").font(LUCIDE)
}

/// `arrow-down-0-1` from Lucide.
pub fn arrow_down_0_1<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E413}").font(LUCIDE)
}

/// `arrow-down-01` from Lucide.
pub fn arrow_down_01<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E413}").font(LUCIDE)
}

/// `arrow-down-1-0` from Lucide.
pub fn arrow_down_1_0<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E414}").font(LUCIDE)
}

/// `arrow-down-10` from Lucide.
pub fn arrow_down_10<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E414}").font(LUCIDE)
}

/// `arrow-down-a-z` from Lucide.
pub fn arrow_down_a_z<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E415}").font(LUCIDE)
}

/// `arrow-down-az` from Lucide.
pub fn arrow_down_az<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E415}").font(LUCIDE)
}

/// `arrow-down-circle` from Lucide.
pub fn arrow_down_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E078}").font(LUCIDE)
}

/// `arrow-down-from-line` from Lucide.
pub fn arrow_down_from_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E454}").font(LUCIDE)
}

/// `arrow-down-left` from Lucide.
pub fn arrow_down_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E043}").font(LUCIDE)
}

/// `arrow-down-left-from-circle` from Lucide.
pub fn arrow_down_left_from_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F7}").font(LUCIDE)
}

/// `arrow-down-left-from-square` from Lucide.
pub fn arrow_down_left_from_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A1}").font(LUCIDE)
}

/// `arrow-down-left-square` from Lucide.
pub fn arrow_down_left_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B5}").font(LUCIDE)
}

/// `arrow-down-narrow-wide` from Lucide.
pub fn arrow_down_narrow_wide<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E044}").font(LUCIDE)
}

/// `arrow-down-right` from Lucide.
pub fn arrow_down_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E045}").font(LUCIDE)
}

/// `arrow-down-right-from-circle` from Lucide.
pub fn arrow_down_right_from_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F8}").font(LUCIDE)
}

/// `arrow-down-right-from-square` from Lucide.
pub fn arrow_down_right_from_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A2}").font(LUCIDE)
}

/// `arrow-down-right-square` from Lucide.
pub fn arrow_down_right_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B6}").font(LUCIDE)
}

/// `arrow-down-square` from Lucide.
pub fn arrow_down_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E427}").font(LUCIDE)
}

/// `arrow-down-to-dot` from Lucide.
pub fn arrow_down_to_dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E44D}").font(LUCIDE)
}

/// `arrow-down-to-line` from Lucide.
pub fn arrow_down_to_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E455}").font(LUCIDE)
}

/// `arrow-down-up` from Lucide.
pub fn arrow_down_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E046}").font(LUCIDE)
}

/// `arrow-down-wide-narrow` from Lucide.
pub fn arrow_down_wide_narrow<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E047}").font(LUCIDE)
}

/// `arrow-down-z-a` from Lucide.
pub fn arrow_down_z_a<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E416}").font(LUCIDE)
}

/// `arrow-down-za` from Lucide.
pub fn arrow_down_za<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E416}").font(LUCIDE)
}

/// `arrow-left` from Lucide.
pub fn arrow_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E048}").font(LUCIDE)
}

/// `arrow-left-circle` from Lucide.
pub fn arrow_left_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E079}").font(LUCIDE)
}

/// `arrow-left-from-line` from Lucide.
pub fn arrow_left_from_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E456}").font(LUCIDE)
}

/// `arrow-left-right` from Lucide.
pub fn arrow_left_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E24A}").font(LUCIDE)
}

/// `arrow-left-square` from Lucide.
pub fn arrow_left_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E428}").font(LUCIDE)
}

/// `arrow-left-to-line` from Lucide.
pub fn arrow_left_to_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E457}").font(LUCIDE)
}

/// `arrow-right` from Lucide.
pub fn arrow_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E049}").font(LUCIDE)
}

/// `arrow-right-circle` from Lucide.
pub fn arrow_right_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E07A}").font(LUCIDE)
}

/// `arrow-right-from-line` from Lucide.
pub fn arrow_right_from_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E458}").font(LUCIDE)
}

/// `arrow-right-left` from Lucide.
pub fn arrow_right_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E417}").font(LUCIDE)
}

/// `arrow-right-square` from Lucide.
pub fn arrow_right_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E429}").font(LUCIDE)
}

/// `arrow-right-to-line` from Lucide.
pub fn arrow_right_to_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E459}").font(LUCIDE)
}

/// `arrow-up` from Lucide.
pub fn arrow_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E04A}").font(LUCIDE)
}

/// `arrow-up-0-1` from Lucide.
pub fn arrow_up_0_1<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E418}").font(LUCIDE)
}

/// `arrow-up-01` from Lucide.
pub fn arrow_up_01<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E418}").font(LUCIDE)
}

/// `arrow-up-1-0` from Lucide.
pub fn arrow_up_1_0<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E419}").font(LUCIDE)
}

/// `arrow-up-10` from Lucide.
pub fn arrow_up_10<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E419}").font(LUCIDE)
}

/// `arrow-up-a-z` from Lucide.
pub fn arrow_up_a_z<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E41A}").font(LUCIDE)
}

/// `arrow-up-az` from Lucide.
pub fn arrow_up_az<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E41A}").font(LUCIDE)
}

/// `arrow-up-circle` from Lucide.
pub fn arrow_up_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E07B}").font(LUCIDE)
}

/// `arrow-up-down` from Lucide.
pub fn arrow_up_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E37D}").font(LUCIDE)
}

/// `arrow-up-from-dot` from Lucide.
pub fn arrow_up_from_dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E44E}").font(LUCIDE)
}

/// `arrow-up-from-line` from Lucide.
pub fn arrow_up_from_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E45A}").font(LUCIDE)
}

/// `arrow-up-left` from Lucide.
pub fn arrow_up_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E04B}").font(LUCIDE)
}

/// `arrow-up-left-from-circle` from Lucide.
pub fn arrow_up_left_from_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F9}").font(LUCIDE)
}

/// `arrow-up-left-from-square` from Lucide.
pub fn arrow_up_left_from_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A3}").font(LUCIDE)
}

/// `arrow-up-left-square` from Lucide.
pub fn arrow_up_left_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B7}").font(LUCIDE)
}

/// `arrow-up-narrow-wide` from Lucide.
pub fn arrow_up_narrow_wide<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E04C}").font(LUCIDE)
}

/// `arrow-up-right` from Lucide.
pub fn arrow_up_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E04D}").font(LUCIDE)
}

/// `arrow-up-right-from-circle` from Lucide.
pub fn arrow_up_right_from_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3FA}").font(LUCIDE)
}

/// `arrow-up-right-from-square` from Lucide.
pub fn arrow_up_right_from_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A4}").font(LUCIDE)
}

/// `arrow-up-right-square` from Lucide.
pub fn arrow_up_right_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B8}").font(LUCIDE)
}

/// `arrow-up-square` from Lucide.
pub fn arrow_up_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E42A}").font(LUCIDE)
}

/// `arrow-up-to-line` from Lucide.
pub fn arrow_up_to_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E45B}").font(LUCIDE)
}

/// `arrow-up-wide-narrow` from Lucide.
pub fn arrow_up_wide_narrow<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E41B}").font(LUCIDE)
}

/// `arrow-up-z-a` from Lucide.
pub fn arrow_up_z_a<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E41C}").font(LUCIDE)
}

/// `arrow-up-za` from Lucide.
pub fn arrow_up_za<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E41C}").font(LUCIDE)
}

/// `arrows-up-from-line` from Lucide.
pub fn arrows_up_from_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4D4}").font(LUCIDE)
}

/// `asterisk` from Lucide.
pub fn asterisk<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1EF}").font(LUCIDE)
}

/// `asterisk-square` from Lucide.
pub fn asterisk_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E168}").font(LUCIDE)
}

/// `astroid` from Lucide.
pub fn astroid<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6F0}").font(LUCIDE)
}

/// `at-sign` from Lucide.
pub fn at_sign<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E04E}").font(LUCIDE)
}

/// `atom` from Lucide.
pub fn atom<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D7}").font(LUCIDE)
}

/// `audio-lines` from Lucide.
pub fn audio_lines<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E55A}").font(LUCIDE)
}

/// `audio-lines-off` from Lucide.
pub fn audio_lines_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E737}").font(LUCIDE)
}

/// `audio-lines-x` from Lucide.
pub fn audio_lines_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E72E}").font(LUCIDE)
}

/// `audio-waveform` from Lucide.
pub fn audio_waveform<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E55B}").font(LUCIDE)
}

/// `award` from Lucide.
pub fn award<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E04F}").font(LUCIDE)
}

/// `axe` from Lucide.
pub fn axe<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E050}").font(LUCIDE)
}

/// `axis-3-d` from Lucide.
pub fn axis_3_d<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2FE}").font(LUCIDE)
}

/// `axis-3d` from Lucide.
pub fn axis_3d<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2FE}").font(LUCIDE)
}

/// `baby` from Lucide.
pub fn baby<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2CE}").font(LUCIDE)
}

/// `backpack` from Lucide.
pub fn backpack<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2C8}").font(LUCIDE)
}

/// `badge` from Lucide.
pub fn badge<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E474}").font(LUCIDE)
}

/// `badge-alert` from Lucide.
pub fn badge_alert<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E475}").font(LUCIDE)
}

/// `badge-cent` from Lucide.
pub fn badge_cent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E50F}").font(LUCIDE)
}

/// `badge-check` from Lucide.
pub fn badge_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E241}").font(LUCIDE)
}

/// `badge-dollar-sign` from Lucide.
pub fn badge_dollar_sign<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E476}").font(LUCIDE)
}

/// `badge-euro` from Lucide.
pub fn badge_euro<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E510}").font(LUCIDE)
}

/// `badge-help` from Lucide.
pub fn badge_help<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E47B}").font(LUCIDE)
}

/// `badge-indian-rupee` from Lucide.
pub fn badge_indian_rupee<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E511}").font(LUCIDE)
}

/// `badge-info` from Lucide.
pub fn badge_info<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E477}").font(LUCIDE)
}

/// `badge-japanese-yen` from Lucide.
pub fn badge_japanese_yen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E512}").font(LUCIDE)
}

/// `badge-minus` from Lucide.
pub fn badge_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E478}").font(LUCIDE)
}

/// `badge-percent` from Lucide.
pub fn badge_percent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E479}").font(LUCIDE)
}

/// `badge-plus` from Lucide.
pub fn badge_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E47A}").font(LUCIDE)
}

/// `badge-pound-sterling` from Lucide.
pub fn badge_pound_sterling<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E513}").font(LUCIDE)
}

/// `badge-question-mark` from Lucide.
pub fn badge_question_mark<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E47B}").font(LUCIDE)
}

/// `badge-russian-ruble` from Lucide.
pub fn badge_russian_ruble<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E514}").font(LUCIDE)
}

/// `badge-swiss-franc` from Lucide.
pub fn badge_swiss_franc<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E515}").font(LUCIDE)
}

/// `badge-turkish-lira` from Lucide.
pub fn badge_turkish_lira<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E67E}").font(LUCIDE)
}

/// `badge-x` from Lucide.
pub fn badge_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E47C}").font(LUCIDE)
}

/// `baggage-claim` from Lucide.
pub fn baggage_claim<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2C9}").font(LUCIDE)
}

/// `balloon` from Lucide.
pub fn balloon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6AF}").font(LUCIDE)
}

/// `ban` from Lucide.
pub fn ban<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E051}").font(LUCIDE)
}

/// `banana` from Lucide.
pub fn banana<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E34F}").font(LUCIDE)
}

/// `bandage` from Lucide.
pub fn bandage<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E61D}").font(LUCIDE)
}

/// `banknote` from Lucide.
pub fn banknote<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E052}").font(LUCIDE)
}

/// `banknote-arrow-down` from Lucide.
pub fn banknote_arrow_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E64C}").font(LUCIDE)
}

/// `banknote-arrow-up` from Lucide.
pub fn banknote_arrow_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E64D}").font(LUCIDE)
}

/// `banknote-check` from Lucide.
pub fn banknote_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E70C}").font(LUCIDE)
}

/// `banknote-x` from Lucide.
pub fn banknote_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E64E}").font(LUCIDE)
}

/// `bar-chart` from Lucide.
pub fn bar_chart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E06A}").font(LUCIDE)
}

/// `bar-chart-2` from Lucide.
pub fn bar_chart_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E068}").font(LUCIDE)
}

/// `bar-chart-3` from Lucide.
pub fn bar_chart_3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A3}").font(LUCIDE)
}

/// `bar-chart-4` from Lucide.
pub fn bar_chart_4<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A4}").font(LUCIDE)
}

/// `bar-chart-big` from Lucide.
pub fn bar_chart_big<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A9}").font(LUCIDE)
}

/// `bar-chart-horizontal` from Lucide.
pub fn bar_chart_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A2}").font(LUCIDE)
}

/// `bar-chart-horizontal-big` from Lucide.
pub fn bar_chart_horizontal_big<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A7}").font(LUCIDE)
}

/// `barcode` from Lucide.
pub fn barcode<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E533}").font(LUCIDE)
}

/// `barrel` from Lucide.
pub fn barrel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E675}").font(LUCIDE)
}

/// `baseline` from Lucide.
pub fn baseline<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E285}").font(LUCIDE)
}

/// `bath` from Lucide.
pub fn bath<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2AB}").font(LUCIDE)
}

/// `battery` from Lucide.
pub fn battery<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E053}").font(LUCIDE)
}

/// `battery-charging` from Lucide.
pub fn battery_charging<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E054}").font(LUCIDE)
}

/// `battery-full` from Lucide.
pub fn battery_full<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E055}").font(LUCIDE)
}

/// `battery-low` from Lucide.
pub fn battery_low<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E056}").font(LUCIDE)
}

/// `battery-medium` from Lucide.
pub fn battery_medium<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E057}").font(LUCIDE)
}

/// `battery-plus` from Lucide.
pub fn battery_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E63E}").font(LUCIDE)
}

/// `battery-warning` from Lucide.
pub fn battery_warning<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3AC}").font(LUCIDE)
}

/// `beaker` from Lucide.
pub fn beaker<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E058}").font(LUCIDE)
}

/// `bean` from Lucide.
pub fn bean<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E38F}").font(LUCIDE)
}

/// `bean-off` from Lucide.
pub fn bean_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E390}").font(LUCIDE)
}

/// `bed` from Lucide.
pub fn bed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2C1}").font(LUCIDE)
}

/// `bed-double` from Lucide.
pub fn bed_double<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2C2}").font(LUCIDE)
}

/// `bed-single` from Lucide.
pub fn bed_single<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2C3}").font(LUCIDE)
}

/// `beef` from Lucide.
pub fn beef<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3A5}").font(LUCIDE)
}

/// `beef-off` from Lucide.
pub fn beef_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6DC}").font(LUCIDE)
}

/// `beer` from Lucide.
pub fn beer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2CF}").font(LUCIDE)
}

/// `beer-off` from Lucide.
pub fn beer_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5D9}").font(LUCIDE)
}

/// `bell` from Lucide.
pub fn bell<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E059}").font(LUCIDE)
}

/// `bell-check` from Lucide.
pub fn bell_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6EF}").font(LUCIDE)
}

/// `bell-dot` from Lucide.
pub fn bell_dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E42B}").font(LUCIDE)
}

/// `bell-electric` from Lucide.
pub fn bell_electric<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E57C}").font(LUCIDE)
}

/// `bell-minus` from Lucide.
pub fn bell_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1F0}").font(LUCIDE)
}

/// `bell-off` from Lucide.
pub fn bell_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E05A}").font(LUCIDE)
}

/// `bell-plus` from Lucide.
pub fn bell_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1F1}").font(LUCIDE)
}

/// `bell-ring` from Lucide.
pub fn bell_ring<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E224}").font(LUCIDE)
}

/// `between-horizonal-end` from Lucide.
pub fn between_horizonal_end<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E591}").font(LUCIDE)
}

/// `between-horizonal-start` from Lucide.
pub fn between_horizonal_start<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E592}").font(LUCIDE)
}

/// `between-horizontal-end` from Lucide.
pub fn between_horizontal_end<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E591}").font(LUCIDE)
}

/// `between-horizontal-start` from Lucide.
pub fn between_horizontal_start<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E592}").font(LUCIDE)
}

/// `between-vertical-end` from Lucide.
pub fn between_vertical_end<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E593}").font(LUCIDE)
}

/// `between-vertical-start` from Lucide.
pub fn between_vertical_start<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E594}").font(LUCIDE)
}

/// `biceps-flexed` from Lucide.
pub fn biceps_flexed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5EB}").font(LUCIDE)
}

/// `bike` from Lucide.
pub fn bike<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1D2}").font(LUCIDE)
}

/// `binary` from Lucide.
pub fn binary<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1F2}").font(LUCIDE)
}

/// `binoculars` from Lucide.
pub fn binoculars<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E621}").font(LUCIDE)
}

/// `biohazard` from Lucide.
pub fn biohazard<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E441}").font(LUCIDE)
}

/// `bird` from Lucide.
pub fn bird<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3C5}").font(LUCIDE)
}

/// `birdhouse` from Lucide.
pub fn birdhouse<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E69A}").font(LUCIDE)
}

/// `bitcoin` from Lucide.
pub fn bitcoin<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E05B}").font(LUCIDE)
}

/// `blend` from Lucide.
pub fn blend<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E59C}").font(LUCIDE)
}

/// `blender` from Lucide.
pub fn blender<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6F4}").font(LUCIDE)
}

/// `blinds` from Lucide.
pub fn blinds<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3C0}").font(LUCIDE)
}

/// `blocks` from Lucide.
pub fn blocks<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4FA}").font(LUCIDE)
}

/// `bluetooth` from Lucide.
pub fn bluetooth<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E05C}").font(LUCIDE)
}

/// `bluetooth-connected` from Lucide.
pub fn bluetooth_connected<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1B8}").font(LUCIDE)
}

/// `bluetooth-off` from Lucide.
pub fn bluetooth_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1B9}").font(LUCIDE)
}

/// `bluetooth-searching` from Lucide.
pub fn bluetooth_searching<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1BA}").font(LUCIDE)
}

/// `bold` from Lucide.
pub fn bold<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E05D}").font(LUCIDE)
}

/// `bolt` from Lucide.
pub fn bolt<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E58C}").font(LUCIDE)
}

/// `bomb` from Lucide.
pub fn bomb<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2FF}").font(LUCIDE)
}

/// `bone` from Lucide.
pub fn bone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E358}").font(LUCIDE)
}

/// `bone-fracture` from Lucide.
pub fn bone_fracture<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E715}").font(LUCIDE)
}

/// `book` from Lucide.
pub fn book<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E05E}").font(LUCIDE)
}

/// `book-a` from Lucide.
pub fn book_a<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E544}").font(LUCIDE)
}

/// `book-alert` from Lucide.
pub fn book_alert<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E672}").font(LUCIDE)
}

/// `book-audio` from Lucide.
pub fn book_audio<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E545}").font(LUCIDE)
}

/// `book-check` from Lucide.
pub fn book_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E546}").font(LUCIDE)
}

/// `book-copy` from Lucide.
pub fn book_copy<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3EC}").font(LUCIDE)
}

/// `book-dashed` from Lucide.
pub fn book_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3ED}").font(LUCIDE)
}

/// `book-down` from Lucide.
pub fn book_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3EE}").font(LUCIDE)
}

/// `book-headphones` from Lucide.
pub fn book_headphones<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E547}").font(LUCIDE)
}

/// `book-heart` from Lucide.
pub fn book_heart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E548}").font(LUCIDE)
}

/// `book-image` from Lucide.
pub fn book_image<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E549}").font(LUCIDE)
}

/// `book-key` from Lucide.
pub fn book_key<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3EF}").font(LUCIDE)
}

/// `book-lock` from Lucide.
pub fn book_lock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F0}").font(LUCIDE)
}

/// `book-marked` from Lucide.
pub fn book_marked<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F1}").font(LUCIDE)
}

/// `book-minus` from Lucide.
pub fn book_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F2}").font(LUCIDE)
}

/// `book-open` from Lucide.
pub fn book_open<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E05F}").font(LUCIDE)
}

/// `book-open-check` from Lucide.
pub fn book_open_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E381}").font(LUCIDE)
}

/// `book-open-text` from Lucide.
pub fn book_open_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E54A}").font(LUCIDE)
}

/// `book-plus` from Lucide.
pub fn book_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F3}").font(LUCIDE)
}

/// `book-search` from Lucide.
pub fn book_search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6AB}").font(LUCIDE)
}

/// `book-template` from Lucide.
pub fn book_template<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3ED}").font(LUCIDE)
}

/// `book-text` from Lucide.
pub fn book_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E54B}").font(LUCIDE)
}

/// `book-type` from Lucide.
pub fn book_type<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E54C}").font(LUCIDE)
}

/// `book-up` from Lucide.
pub fn book_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F4}").font(LUCIDE)
}

/// `book-up-2` from Lucide.
pub fn book_up_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A6}").font(LUCIDE)
}

/// `book-user` from Lucide.
pub fn book_user<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E54D}").font(LUCIDE)
}

/// `book-x` from Lucide.
pub fn book_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F5}").font(LUCIDE)
}

/// `bookmark` from Lucide.
pub fn bookmark<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E060}").font(LUCIDE)
}

/// `bookmark-check` from Lucide.
pub fn bookmark_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E51F}").font(LUCIDE)
}

/// `bookmark-minus` from Lucide.
pub fn bookmark_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E23C}").font(LUCIDE)
}

/// `bookmark-off` from Lucide.
pub fn bookmark_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6DF}").font(LUCIDE)
}

/// `bookmark-plus` from Lucide.
pub fn bookmark_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E23D}").font(LUCIDE)
}

/// `bookmark-x` from Lucide.
pub fn bookmark_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E520}").font(LUCIDE)
}

/// `boom-box` from Lucide.
pub fn boom_box<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4EE}").font(LUCIDE)
}

/// `bot` from Lucide.
pub fn bot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1BB}").font(LUCIDE)
}

/// `bot-message-square` from Lucide.
pub fn bot_message_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5CE}").font(LUCIDE)
}

/// `bot-off` from Lucide.
pub fn bot_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5E0}").font(LUCIDE)
}

/// `bottle-wine` from Lucide.
pub fn bottle_wine<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E67B}").font(LUCIDE)
}

/// `bow-arrow` from Lucide.
pub fn bow_arrow<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E65E}").font(LUCIDE)
}

/// `box` from Lucide.
pub fn box_icon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E061}").font(LUCIDE)
}

/// `box-select` from Lucide.
pub fn box_select<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1CB}").font(LUCIDE)
}

/// `boxes` from Lucide.
pub fn boxes<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2D0}").font(LUCIDE)
}

/// `braces` from Lucide.
pub fn braces<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E36A}").font(LUCIDE)
}

/// `brackets` from Lucide.
pub fn brackets<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E443}").font(LUCIDE)
}

/// `brain` from Lucide.
pub fn brain<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3C6}").font(LUCIDE)
}

/// `brain-circuit` from Lucide.
pub fn brain_circuit<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3C7}").font(LUCIDE)
}

/// `brain-cog` from Lucide.
pub fn brain_cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3C8}").font(LUCIDE)
}

/// `brick-wall` from Lucide.
pub fn brick_wall<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E581}").font(LUCIDE)
}

/// `brick-wall-fire` from Lucide.
pub fn brick_wall_fire<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E653}").font(LUCIDE)
}

/// `brick-wall-shield` from Lucide.
pub fn brick_wall_shield<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E690}").font(LUCIDE)
}

/// `briefcase` from Lucide.
pub fn briefcase<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E062}").font(LUCIDE)
}

/// `briefcase-business` from Lucide.
pub fn briefcase_business<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5D5}").font(LUCIDE)
}

/// `briefcase-conveyor-belt` from Lucide.
pub fn briefcase_conveyor_belt<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E62B}").font(LUCIDE)
}

/// `briefcase-medical` from Lucide.
pub fn briefcase_medical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5D6}").font(LUCIDE)
}

/// `bring-to-front` from Lucide.
pub fn bring_to_front<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4EF}").font(LUCIDE)
}

/// `broccoli` from Lucide.
pub fn broccoli<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6F5}").font(LUCIDE)
}

/// `broom` from Lucide.
pub fn broom<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E730}").font(LUCIDE)
}

/// `broom-sparkles` from Lucide.
pub fn broom_sparkles<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E72F}").font(LUCIDE)
}

/// `brush` from Lucide.
pub fn brush<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1D3}").font(LUCIDE)
}

/// `brush-cleaning` from Lucide.
pub fn brush_cleaning<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E666}").font(LUCIDE)
}

/// `bubbles` from Lucide.
pub fn bubbles<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E654}").font(LUCIDE)
}

/// `bug` from Lucide.
pub fn bug<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E20C}").font(LUCIDE)
}

/// `bug-off` from Lucide.
pub fn bug_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E50D}").font(LUCIDE)
}

/// `bug-play` from Lucide.
pub fn bug_play<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E50E}").font(LUCIDE)
}

/// `building` from Lucide.
pub fn building<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1CC}").font(LUCIDE)
}

/// `building-2` from Lucide.
pub fn building_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E290}").font(LUCIDE)
}

/// `bus` from Lucide.
pub fn bus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1D4}").font(LUCIDE)
}

/// `bus-front` from Lucide.
pub fn bus_front<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4FB}").font(LUCIDE)
}

/// `cable` from Lucide.
pub fn cable<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4E3}").font(LUCIDE)
}

/// `cable-car` from Lucide.
pub fn cable_car<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4FC}").font(LUCIDE)
}

/// `cake` from Lucide.
pub fn cake<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E344}").font(LUCIDE)
}

/// `cake-slice` from Lucide.
pub fn cake_slice<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B9}").font(LUCIDE)
}

/// `calculator` from Lucide.
pub fn calculator<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1BC}").font(LUCIDE)
}

/// `calendar` from Lucide.
pub fn calendar<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E063}").font(LUCIDE)
}

/// `calendar-1` from Lucide.
pub fn calendar_1<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E630}").font(LUCIDE)
}

/// `calendar-arrow-down` from Lucide.
pub fn calendar_arrow_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5FE}").font(LUCIDE)
}

/// `calendar-arrow-up` from Lucide.
pub fn calendar_arrow_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5FF}").font(LUCIDE)
}

/// `calendar-check` from Lucide.
pub fn calendar_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2B7}").font(LUCIDE)
}

/// `calendar-check-2` from Lucide.
pub fn calendar_check_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2B8}").font(LUCIDE)
}

/// `calendar-clock` from Lucide.
pub fn calendar_clock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E304}").font(LUCIDE)
}

/// `calendar-cog` from Lucide.
pub fn calendar_cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5ED}").font(LUCIDE)
}

/// `calendar-days` from Lucide.
pub fn calendar_days<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2B9}").font(LUCIDE)
}

/// `calendar-fold` from Lucide.
pub fn calendar_fold<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5B4}").font(LUCIDE)
}

/// `calendar-heart` from Lucide.
pub fn calendar_heart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E305}").font(LUCIDE)
}

/// `calendar-minus` from Lucide.
pub fn calendar_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2BA}").font(LUCIDE)
}

/// `calendar-minus-2` from Lucide.
pub fn calendar_minus_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5B5}").font(LUCIDE)
}

/// `calendar-off` from Lucide.
pub fn calendar_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2BB}").font(LUCIDE)
}

/// `calendar-plus` from Lucide.
pub fn calendar_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2BC}").font(LUCIDE)
}

/// `calendar-plus-2` from Lucide.
pub fn calendar_plus_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5B6}").font(LUCIDE)
}

/// `calendar-range` from Lucide.
pub fn calendar_range<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2BD}").font(LUCIDE)
}

/// `calendar-search` from Lucide.
pub fn calendar_search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E306}").font(LUCIDE)
}

/// `calendar-sync` from Lucide.
pub fn calendar_sync<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E636}").font(LUCIDE)
}

/// `calendar-x` from Lucide.
pub fn calendar_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2BE}").font(LUCIDE)
}

/// `calendar-x-2` from Lucide.
pub fn calendar_x_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2BF}").font(LUCIDE)
}

/// `calendars` from Lucide.
pub fn calendars<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6A7}").font(LUCIDE)
}

/// `camera` from Lucide.
pub fn camera<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E064}").font(LUCIDE)
}

/// `camera-off` from Lucide.
pub fn camera_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E065}").font(LUCIDE)
}

/// `candlestick-chart` from Lucide.
pub fn candlestick_chart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A8}").font(LUCIDE)
}

/// `candy` from Lucide.
pub fn candy<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E391}").font(LUCIDE)
}

/// `candy-cane` from Lucide.
pub fn candy_cane<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4BA}").font(LUCIDE)
}

/// `candy-off` from Lucide.
pub fn candy_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E392}").font(LUCIDE)
}

/// `cannabis` from Lucide.
pub fn cannabis<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5D4}").font(LUCIDE)
}

/// `cannabis-off` from Lucide.
pub fn cannabis_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6E0}").font(LUCIDE)
}

/// `captions` from Lucide.
pub fn captions<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3A4}").font(LUCIDE)
}

/// `captions-off` from Lucide.
pub fn captions_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5C1}").font(LUCIDE)
}

/// `car` from Lucide.
pub fn car<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1D5}").font(LUCIDE)
}

/// `car-battery` from Lucide.
pub fn car_battery<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E735}").font(LUCIDE)
}

/// `car-front` from Lucide.
pub fn car_front<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4FD}").font(LUCIDE)
}

/// `car-taxi-front` from Lucide.
pub fn car_taxi_front<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4FE}").font(LUCIDE)
}

/// `caravan` from Lucide.
pub fn caravan<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E539}").font(LUCIDE)
}

/// `card-sim` from Lucide.
pub fn card_sim<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E671}").font(LUCIDE)
}

/// `carrot` from Lucide.
pub fn carrot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E25A}").font(LUCIDE)
}

/// `case-lower` from Lucide.
pub fn case_lower<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D8}").font(LUCIDE)
}

/// `case-sensitive` from Lucide.
pub fn case_sensitive<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D9}").font(LUCIDE)
}

/// `case-upper` from Lucide.
pub fn case_upper<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3DA}").font(LUCIDE)
}

/// `cassette-tape` from Lucide.
pub fn cassette_tape<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4CA}").font(LUCIDE)
}

/// `cast` from Lucide.
pub fn cast<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E066}").font(LUCIDE)
}

/// `castle` from Lucide.
pub fn castle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3E0}").font(LUCIDE)
}

/// `cat` from Lucide.
pub fn cat<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E38C}").font(LUCIDE)
}

/// `cctv` from Lucide.
pub fn cctv<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E57D}").font(LUCIDE)
}

/// `cctv-off` from Lucide.
pub fn cctv_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6D8}").font(LUCIDE)
}

/// `chart-area` from Lucide.
pub fn chart_area<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4D3}").font(LUCIDE)
}

/// `chart-bar` from Lucide.
pub fn chart_bar<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A2}").font(LUCIDE)
}

/// `chart-bar-big` from Lucide.
pub fn chart_bar_big<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A7}").font(LUCIDE)
}

/// `chart-bar-decreasing` from Lucide.
pub fn chart_bar_decreasing<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E607}").font(LUCIDE)
}

/// `chart-bar-increasing` from Lucide.
pub fn chart_bar_increasing<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E608}").font(LUCIDE)
}

/// `chart-bar-stacked` from Lucide.
pub fn chart_bar_stacked<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E609}").font(LUCIDE)
}

/// `chart-candlestick` from Lucide.
pub fn chart_candlestick<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A8}").font(LUCIDE)
}

/// `chart-column` from Lucide.
pub fn chart_column<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A3}").font(LUCIDE)
}

/// `chart-column-big` from Lucide.
pub fn chart_column_big<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A9}").font(LUCIDE)
}

/// `chart-column-decreasing` from Lucide.
pub fn chart_column_decreasing<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E067}").font(LUCIDE)
}

/// `chart-column-increasing` from Lucide.
pub fn chart_column_increasing<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A4}").font(LUCIDE)
}

/// `chart-column-stacked` from Lucide.
pub fn chart_column_stacked<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E60A}").font(LUCIDE)
}

/// `chart-gantt` from Lucide.
pub fn chart_gantt<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E624}").font(LUCIDE)
}

/// `chart-line` from Lucide.
pub fn chart_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A5}").font(LUCIDE)
}

/// `chart-network` from Lucide.
pub fn chart_network<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E60B}").font(LUCIDE)
}

/// `chart-no-axes-column` from Lucide.
pub fn chart_no_axes_column<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E068}").font(LUCIDE)
}

/// `chart-no-axes-column-decreasing` from Lucide.
pub fn chart_no_axes_column_decreasing<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E069}").font(LUCIDE)
}

/// `chart-no-axes-column-increasing` from Lucide.
pub fn chart_no_axes_column_increasing<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E06A}").font(LUCIDE)
}

/// `chart-no-axes-combined` from Lucide.
pub fn chart_no_axes_combined<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E60C}").font(LUCIDE)
}

/// `chart-no-axes-gantt` from Lucide.
pub fn chart_no_axes_gantt<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4C4}").font(LUCIDE)
}

/// `chart-pie` from Lucide.
pub fn chart_pie<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E06B}").font(LUCIDE)
}

/// `chart-scatter` from Lucide.
pub fn chart_scatter<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E48A}").font(LUCIDE)
}

/// `chart-spline` from Lucide.
pub fn chart_spline<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E60D}").font(LUCIDE)
}

/// `check` from Lucide.
pub fn check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E06C}").font(LUCIDE)
}

/// `check-check` from Lucide.
pub fn check_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E38E}").font(LUCIDE)
}

/// `check-circle` from Lucide.
pub fn check_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E07C}").font(LUCIDE)
}

/// `check-circle-2` from Lucide.
pub fn check_circle_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E226}").font(LUCIDE)
}

/// `check-line` from Lucide.
pub fn check_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E66B}").font(LUCIDE)
}

/// `check-square` from Lucide.
pub fn check_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E16A}").font(LUCIDE)
}

/// `check-square-2` from Lucide.
pub fn check_square_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E559}").font(LUCIDE)
}

/// `chef-hat` from Lucide.
pub fn chef_hat<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2AC}").font(LUCIDE)
}

/// `cherry` from Lucide.
pub fn cherry<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E350}").font(LUCIDE)
}

/// `chess-bishop` from Lucide.
pub fn chess_bishop<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6A0}").font(LUCIDE)
}

/// `chess-king` from Lucide.
pub fn chess_king<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6A1}").font(LUCIDE)
}

/// `chess-knight` from Lucide.
pub fn chess_knight<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6A2}").font(LUCIDE)
}

/// `chess-pawn` from Lucide.
pub fn chess_pawn<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6A3}").font(LUCIDE)
}

/// `chess-queen` from Lucide.
pub fn chess_queen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6A4}").font(LUCIDE)
}

/// `chess-rook` from Lucide.
pub fn chess_rook<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6A5}").font(LUCIDE)
}

/// `chevron-down` from Lucide.
pub fn chevron_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E06D}").font(LUCIDE)
}

/// `chevron-down-circle` from Lucide.
pub fn chevron_down_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4DD}").font(LUCIDE)
}

/// `chevron-down-square` from Lucide.
pub fn chevron_down_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3CF}").font(LUCIDE)
}

/// `chevron-first` from Lucide.
pub fn chevron_first<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E243}").font(LUCIDE)
}

/// `chevron-last` from Lucide.
pub fn chevron_last<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E244}").font(LUCIDE)
}

/// `chevron-left` from Lucide.
pub fn chevron_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E06E}").font(LUCIDE)
}

/// `chevron-left-circle` from Lucide.
pub fn chevron_left_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4DE}").font(LUCIDE)
}

/// `chevron-left-square` from Lucide.
pub fn chevron_left_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D0}").font(LUCIDE)
}

/// `chevron-right` from Lucide.
pub fn chevron_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E06F}").font(LUCIDE)
}

/// `chevron-right-circle` from Lucide.
pub fn chevron_right_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4DF}").font(LUCIDE)
}

/// `chevron-right-square` from Lucide.
pub fn chevron_right_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D1}").font(LUCIDE)
}

/// `chevron-up` from Lucide.
pub fn chevron_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E070}").font(LUCIDE)
}

/// `chevron-up-circle` from Lucide.
pub fn chevron_up_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4E0}").font(LUCIDE)
}

/// `chevron-up-square` from Lucide.
pub fn chevron_up_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D2}").font(LUCIDE)
}

/// `chevrons-down` from Lucide.
pub fn chevrons_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E071}").font(LUCIDE)
}

/// `chevrons-down-up` from Lucide.
pub fn chevrons_down_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E228}").font(LUCIDE)
}

/// `chevrons-left` from Lucide.
pub fn chevrons_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E072}").font(LUCIDE)
}

/// `chevrons-left-right` from Lucide.
pub fn chevrons_left_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E293}").font(LUCIDE)
}

/// `chevrons-left-right-ellipsis` from Lucide.
pub fn chevrons_left_right_ellipsis<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E61F}").font(LUCIDE)
}

/// `chevrons-right` from Lucide.
pub fn chevrons_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E073}").font(LUCIDE)
}

/// `chevrons-right-left` from Lucide.
pub fn chevrons_right_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E294}").font(LUCIDE)
}

/// `chevrons-up` from Lucide.
pub fn chevrons_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E074}").font(LUCIDE)
}

/// `chevrons-up-down` from Lucide.
pub fn chevrons_up_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E211}").font(LUCIDE)
}

/// `church` from Lucide.
pub fn church<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3E1}").font(LUCIDE)
}

/// `cigarette` from Lucide.
pub fn cigarette<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2C6}").font(LUCIDE)
}

/// `cigarette-off` from Lucide.
pub fn cigarette_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2C7}").font(LUCIDE)
}

/// `circle` from Lucide.
pub fn circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E076}").font(LUCIDE)
}

/// `circle-alert` from Lucide.
pub fn circle_alert<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E077}").font(LUCIDE)
}

/// `circle-arrow-down` from Lucide.
pub fn circle_arrow_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E078}").font(LUCIDE)
}

/// `circle-arrow-left` from Lucide.
pub fn circle_arrow_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E079}").font(LUCIDE)
}

/// `circle-arrow-out-down-left` from Lucide.
pub fn circle_arrow_out_down_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F7}").font(LUCIDE)
}

/// `circle-arrow-out-down-right` from Lucide.
pub fn circle_arrow_out_down_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F8}").font(LUCIDE)
}

/// `circle-arrow-out-up-left` from Lucide.
pub fn circle_arrow_out_up_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F9}").font(LUCIDE)
}

/// `circle-arrow-out-up-right` from Lucide.
pub fn circle_arrow_out_up_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3FA}").font(LUCIDE)
}

/// `circle-arrow-right` from Lucide.
pub fn circle_arrow_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E07A}").font(LUCIDE)
}

/// `circle-arrow-up` from Lucide.
pub fn circle_arrow_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E07B}").font(LUCIDE)
}

/// `circle-check` from Lucide.
pub fn circle_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E226}").font(LUCIDE)
}

/// `circle-check-big` from Lucide.
pub fn circle_check_big<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E07C}").font(LUCIDE)
}

/// `circle-chevron-down` from Lucide.
pub fn circle_chevron_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4DD}").font(LUCIDE)
}

/// `circle-chevron-left` from Lucide.
pub fn circle_chevron_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4DE}").font(LUCIDE)
}

/// `circle-chevron-right` from Lucide.
pub fn circle_chevron_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4DF}").font(LUCIDE)
}

/// `circle-chevron-up` from Lucide.
pub fn circle_chevron_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4E0}").font(LUCIDE)
}

/// `circle-dashed` from Lucide.
pub fn circle_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B0}").font(LUCIDE)
}

/// `circle-divide` from Lucide.
pub fn circle_divide<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E07D}").font(LUCIDE)
}

/// `circle-dollar-sign` from Lucide.
pub fn circle_dollar_sign<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E47D}").font(LUCIDE)
}

/// `circle-dot` from Lucide.
pub fn circle_dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E345}").font(LUCIDE)
}

/// `circle-dot-dashed` from Lucide.
pub fn circle_dot_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B1}").font(LUCIDE)
}

/// `circle-ellipsis` from Lucide.
pub fn circle_ellipsis<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E346}").font(LUCIDE)
}

/// `circle-equal` from Lucide.
pub fn circle_equal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E400}").font(LUCIDE)
}

/// `circle-euro` from Lucide.
pub fn circle_euro<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E71F}").font(LUCIDE)
}

/// `circle-fading-arrow-up` from Lucide.
pub fn circle_fading_arrow_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E618}").font(LUCIDE)
}

/// `circle-fading-plus` from Lucide.
pub fn circle_fading_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5BC}").font(LUCIDE)
}

/// `circle-gauge` from Lucide.
pub fn circle_gauge<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4E1}").font(LUCIDE)
}

/// `circle-help` from Lucide.
pub fn circle_help<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E082}").font(LUCIDE)
}

/// `circle-minus` from Lucide.
pub fn circle_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E07E}").font(LUCIDE)
}

/// `circle-off` from Lucide.
pub fn circle_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E401}").font(LUCIDE)
}

/// `circle-parking` from Lucide.
pub fn circle_parking<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3C9}").font(LUCIDE)
}

/// `circle-parking-off` from Lucide.
pub fn circle_parking_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3CA}").font(LUCIDE)
}

/// `circle-pause` from Lucide.
pub fn circle_pause<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E07F}").font(LUCIDE)
}

/// `circle-percent` from Lucide.
pub fn circle_percent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E51A}").font(LUCIDE)
}

/// `circle-pile` from Lucide.
pub fn circle_pile<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6E1}").font(LUCIDE)
}

/// `circle-play` from Lucide.
pub fn circle_play<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E080}").font(LUCIDE)
}

/// `circle-plus` from Lucide.
pub fn circle_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E081}").font(LUCIDE)
}

/// `circle-pound-sterling` from Lucide.
pub fn circle_pound_sterling<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E66D}").font(LUCIDE)
}

/// `circle-power` from Lucide.
pub fn circle_power<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E550}").font(LUCIDE)
}

/// `circle-question-mark` from Lucide.
pub fn circle_question_mark<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E082}").font(LUCIDE)
}

/// `circle-slash` from Lucide.
pub fn circle_slash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E402}").font(LUCIDE)
}

/// `circle-slash-2` from Lucide.
pub fn circle_slash_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E213}").font(LUCIDE)
}

/// `circle-slashed` from Lucide.
pub fn circle_slashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E213}").font(LUCIDE)
}

/// `circle-small` from Lucide.
pub fn circle_small<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E640}").font(LUCIDE)
}

/// `circle-star` from Lucide.
pub fn circle_star<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E68D}").font(LUCIDE)
}

/// `circle-stop` from Lucide.
pub fn circle_stop<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E083}").font(LUCIDE)
}

/// `circle-user` from Lucide.
pub fn circle_user<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E461}").font(LUCIDE)
}

/// `circle-user-round` from Lucide.
pub fn circle_user_round<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E462}").font(LUCIDE)
}

/// `circle-x` from Lucide.
pub fn circle_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E084}").font(LUCIDE)
}

/// `circuit-board` from Lucide.
pub fn circuit_board<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E403}").font(LUCIDE)
}

/// `citrus` from Lucide.
pub fn citrus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E375}").font(LUCIDE)
}

/// `clapperboard` from Lucide.
pub fn clapperboard<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E29B}").font(LUCIDE)
}

/// `clipboard` from Lucide.
pub fn clipboard<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E085}").font(LUCIDE)
}

/// `clipboard-check` from Lucide.
pub fn clipboard_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E219}").font(LUCIDE)
}

/// `clipboard-clock` from Lucide.
pub fn clipboard_clock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E688}").font(LUCIDE)
}

/// `clipboard-copy` from Lucide.
pub fn clipboard_copy<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E225}").font(LUCIDE)
}

/// `clipboard-edit` from Lucide.
pub fn clipboard_edit<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E307}").font(LUCIDE)
}

/// `clipboard-list` from Lucide.
pub fn clipboard_list<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E086}").font(LUCIDE)
}

/// `clipboard-minus` from Lucide.
pub fn clipboard_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5BE}").font(LUCIDE)
}

/// `clipboard-paste` from Lucide.
pub fn clipboard_paste<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3E8}").font(LUCIDE)
}

/// `clipboard-pen` from Lucide.
pub fn clipboard_pen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E307}").font(LUCIDE)
}

/// `clipboard-pen-line` from Lucide.
pub fn clipboard_pen_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E308}").font(LUCIDE)
}

/// `clipboard-plus` from Lucide.
pub fn clipboard_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5BF}").font(LUCIDE)
}

/// `clipboard-signature` from Lucide.
pub fn clipboard_signature<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E308}").font(LUCIDE)
}

/// `clipboard-type` from Lucide.
pub fn clipboard_type<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E309}").font(LUCIDE)
}

/// `clipboard-x` from Lucide.
pub fn clipboard_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E222}").font(LUCIDE)
}

/// `clock` from Lucide.
pub fn clock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E087}").font(LUCIDE)
}

/// `clock-1` from Lucide.
pub fn clock_1<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E24B}").font(LUCIDE)
}

/// `clock-10` from Lucide.
pub fn clock_10<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E24C}").font(LUCIDE)
}

/// `clock-11` from Lucide.
pub fn clock_11<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E24D}").font(LUCIDE)
}

/// `clock-12` from Lucide.
pub fn clock_12<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E24E}").font(LUCIDE)
}

/// `clock-2` from Lucide.
pub fn clock_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E24F}").font(LUCIDE)
}

/// `clock-3` from Lucide.
pub fn clock_3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E250}").font(LUCIDE)
}

/// `clock-4` from Lucide.
pub fn clock_4<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E251}").font(LUCIDE)
}

/// `clock-5` from Lucide.
pub fn clock_5<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E252}").font(LUCIDE)
}

/// `clock-6` from Lucide.
pub fn clock_6<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E253}").font(LUCIDE)
}

/// `clock-7` from Lucide.
pub fn clock_7<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E254}").font(LUCIDE)
}

/// `clock-8` from Lucide.
pub fn clock_8<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E255}").font(LUCIDE)
}

/// `clock-9` from Lucide.
pub fn clock_9<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E256}").font(LUCIDE)
}

/// `clock-alert` from Lucide.
pub fn clock_alert<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E62A}").font(LUCIDE)
}

/// `clock-arrow-down` from Lucide.
pub fn clock_arrow_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E600}").font(LUCIDE)
}

/// `clock-arrow-left` from Lucide.
pub fn clock_arrow_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E70D}").font(LUCIDE)
}

/// `clock-arrow-right` from Lucide.
pub fn clock_arrow_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E70E}").font(LUCIDE)
}

/// `clock-arrow-up` from Lucide.
pub fn clock_arrow_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E601}").font(LUCIDE)
}

/// `clock-check` from Lucide.
pub fn clock_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E69E}").font(LUCIDE)
}

/// `clock-fading` from Lucide.
pub fn clock_fading<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E64A}").font(LUCIDE)
}

/// `clock-plus` from Lucide.
pub fn clock_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E667}").font(LUCIDE)
}

/// `closed-caption` from Lucide.
pub fn closed_caption<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E68A}").font(LUCIDE)
}

/// `cloud` from Lucide.
pub fn cloud<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E088}").font(LUCIDE)
}

/// `cloud-alert` from Lucide.
pub fn cloud_alert<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E633}").font(LUCIDE)
}

/// `cloud-backup` from Lucide.
pub fn cloud_backup<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6E2}").font(LUCIDE)
}

/// `cloud-check` from Lucide.
pub fn cloud_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E66E}").font(LUCIDE)
}

/// `cloud-cog` from Lucide.
pub fn cloud_cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E30A}").font(LUCIDE)
}

/// `cloud-download` from Lucide.
pub fn cloud_download<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E089}").font(LUCIDE)
}

/// `cloud-drizzle` from Lucide.
pub fn cloud_drizzle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E08A}").font(LUCIDE)
}

/// `cloud-fog` from Lucide.
pub fn cloud_fog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E214}").font(LUCIDE)
}

/// `cloud-hail` from Lucide.
pub fn cloud_hail<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E08B}").font(LUCIDE)
}

/// `cloud-lightning` from Lucide.
pub fn cloud_lightning<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E08C}").font(LUCIDE)
}

/// `cloud-moon` from Lucide.
pub fn cloud_moon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E215}").font(LUCIDE)
}

/// `cloud-moon-rain` from Lucide.
pub fn cloud_moon_rain<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2FA}").font(LUCIDE)
}

/// `cloud-off` from Lucide.
pub fn cloud_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E08D}").font(LUCIDE)
}

/// `cloud-rain` from Lucide.
pub fn cloud_rain<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E08E}").font(LUCIDE)
}

/// `cloud-rain-wind` from Lucide.
pub fn cloud_rain_wind<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E08F}").font(LUCIDE)
}

/// `cloud-snow` from Lucide.
pub fn cloud_snow<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E090}").font(LUCIDE)
}

/// `cloud-sun` from Lucide.
pub fn cloud_sun<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E216}").font(LUCIDE)
}

/// `cloud-sun-rain` from Lucide.
pub fn cloud_sun_rain<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2FB}").font(LUCIDE)
}

/// `cloud-sync` from Lucide.
pub fn cloud_sync<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6E3}").font(LUCIDE)
}

/// `cloud-upload` from Lucide.
pub fn cloud_upload<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E091}").font(LUCIDE)
}

/// `cloudy` from Lucide.
pub fn cloudy<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E217}").font(LUCIDE)
}

/// `clover` from Lucide.
pub fn clover<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E092}").font(LUCIDE)
}

/// `club` from Lucide.
pub fn club<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E496}").font(LUCIDE)
}

/// `code` from Lucide.
pub fn code<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E093}").font(LUCIDE)
}

/// `code-2` from Lucide.
pub fn code_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E206}").font(LUCIDE)
}

/// `code-square` from Lucide.
pub fn code_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E16B}").font(LUCIDE)
}

/// `code-xml` from Lucide.
pub fn code_xml<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E206}").font(LUCIDE)
}

/// `coffee` from Lucide.
pub fn coffee<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E096}").font(LUCIDE)
}

/// `cog` from Lucide.
pub fn cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E30B}").font(LUCIDE)
}

/// `coins` from Lucide.
pub fn coins<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E097}").font(LUCIDE)
}

/// `columns` from Lucide.
pub fn columns<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E098}").font(LUCIDE)
}

/// `columns-2` from Lucide.
pub fn columns_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E098}").font(LUCIDE)
}

/// `columns-3` from Lucide.
pub fn columns_3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E099}").font(LUCIDE)
}

/// `columns-3-cog` from Lucide.
pub fn columns_3_cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E661}").font(LUCIDE)
}

/// `columns-4` from Lucide.
pub fn columns_4<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E589}").font(LUCIDE)
}

/// `columns-settings` from Lucide.
pub fn columns_settings<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E661}").font(LUCIDE)
}

/// `combine` from Lucide.
pub fn combine<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E44C}").font(LUCIDE)
}

/// `command` from Lucide.
pub fn command<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E09A}").font(LUCIDE)
}

/// `compass` from Lucide.
pub fn compass<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E09B}").font(LUCIDE)
}

/// `component` from Lucide.
pub fn component<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2AD}").font(LUCIDE)
}

/// `computer` from Lucide.
pub fn computer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4E4}").font(LUCIDE)
}

/// `concierge-bell` from Lucide.
pub fn concierge_bell<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E378}").font(LUCIDE)
}

/// `cone` from Lucide.
pub fn cone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E523}").font(LUCIDE)
}

/// `construction` from Lucide.
pub fn construction<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B4}").font(LUCIDE)
}

/// `contact` from Lucide.
pub fn contact<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E09C}").font(LUCIDE)
}

/// `contact-2` from Lucide.
pub fn contact_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E463}").font(LUCIDE)
}

/// `contact-round` from Lucide.
pub fn contact_round<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E463}").font(LUCIDE)
}

/// `container` from Lucide.
pub fn container<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4D5}").font(LUCIDE)
}

/// `contrast` from Lucide.
pub fn contrast<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E09D}").font(LUCIDE)
}

/// `cookie` from Lucide.
pub fn cookie<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E26B}").font(LUCIDE)
}

/// `cooking-pot` from Lucide.
pub fn cooking_pot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E584}").font(LUCIDE)
}

/// `copy` from Lucide.
pub fn copy<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E09E}").font(LUCIDE)
}

/// `copy-check` from Lucide.
pub fn copy_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3FB}").font(LUCIDE)
}

/// `copy-minus` from Lucide.
pub fn copy_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3FC}").font(LUCIDE)
}

/// `copy-plus` from Lucide.
pub fn copy_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3FD}").font(LUCIDE)
}

/// `copy-slash` from Lucide.
pub fn copy_slash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3FE}").font(LUCIDE)
}

/// `copy-x` from Lucide.
pub fn copy_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3FF}").font(LUCIDE)
}

/// `copyleft` from Lucide.
pub fn copyleft<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E09F}").font(LUCIDE)
}

/// `copyright` from Lucide.
pub fn copyright<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0A0}").font(LUCIDE)
}

/// `corner-down-left` from Lucide.
pub fn corner_down_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0A1}").font(LUCIDE)
}

/// `corner-down-right` from Lucide.
pub fn corner_down_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0A2}").font(LUCIDE)
}

/// `corner-left-down` from Lucide.
pub fn corner_left_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0A3}").font(LUCIDE)
}

/// `corner-left-up` from Lucide.
pub fn corner_left_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0A4}").font(LUCIDE)
}

/// `corner-right-down` from Lucide.
pub fn corner_right_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0A5}").font(LUCIDE)
}

/// `corner-right-up` from Lucide.
pub fn corner_right_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0A6}").font(LUCIDE)
}

/// `corner-up-left` from Lucide.
pub fn corner_up_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0A7}").font(LUCIDE)
}

/// `corner-up-right` from Lucide.
pub fn corner_up_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0A8}").font(LUCIDE)
}

/// `cpu` from Lucide.
pub fn cpu<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0A9}").font(LUCIDE)
}

/// `creative-commons` from Lucide.
pub fn creative_commons<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B2}").font(LUCIDE)
}

/// `credit-card` from Lucide.
pub fn credit_card<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0AA}").font(LUCIDE)
}

/// `credit-card-check` from Lucide.
pub fn credit_card_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E743}").font(LUCIDE)
}

/// `credit-card-minus` from Lucide.
pub fn credit_card_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E744}").font(LUCIDE)
}

/// `credit-card-plus` from Lucide.
pub fn credit_card_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E74B}").font(LUCIDE)
}

/// `credit-card-x` from Lucide.
pub fn credit_card_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E745}").font(LUCIDE)
}

/// `croissant` from Lucide.
pub fn croissant<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2AE}").font(LUCIDE)
}

/// `crop` from Lucide.
pub fn crop<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0AB}").font(LUCIDE)
}

/// `cross` from Lucide.
pub fn cross<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1E5}").font(LUCIDE)
}

/// `crosshair` from Lucide.
pub fn crosshair<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0AC}").font(LUCIDE)
}

/// `crown` from Lucide.
pub fn crown<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1D6}").font(LUCIDE)
}

/// `cuboid` from Lucide.
pub fn cuboid<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E524}").font(LUCIDE)
}

/// `cup-soda` from Lucide.
pub fn cup_soda<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2D1}").font(LUCIDE)
}

/// `curly-braces` from Lucide.
pub fn curly_braces<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E36A}").font(LUCIDE)
}

/// `currency` from Lucide.
pub fn currency<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E230}").font(LUCIDE)
}

/// `cylinder` from Lucide.
pub fn cylinder<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E525}").font(LUCIDE)
}

/// `dam` from Lucide.
pub fn dam<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E606}").font(LUCIDE)
}

/// `database` from Lucide.
pub fn database<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0AD}").font(LUCIDE)
}

/// `database-arrow-down` from Lucide.
pub fn database_arrow_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E716}").font(LUCIDE)
}

/// `database-arrow-up` from Lucide.
pub fn database_arrow_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E717}").font(LUCIDE)
}

/// `database-backup` from Lucide.
pub fn database_backup<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3AB}").font(LUCIDE)
}

/// `database-check` from Lucide.
pub fn database_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E718}").font(LUCIDE)
}

/// `database-minus` from Lucide.
pub fn database_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E719}").font(LUCIDE)
}

/// `database-plus` from Lucide.
pub fn database_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E71A}").font(LUCIDE)
}

/// `database-search` from Lucide.
pub fn database_search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6B1}").font(LUCIDE)
}

/// `database-x` from Lucide.
pub fn database_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E71B}").font(LUCIDE)
}

/// `database-zap` from Lucide.
pub fn database_zap<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E50B}").font(LUCIDE)
}

/// `decimals-arrow-left` from Lucide.
pub fn decimals_arrow_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E65C}").font(LUCIDE)
}

/// `decimals-arrow-right` from Lucide.
pub fn decimals_arrow_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E65D}").font(LUCIDE)
}

/// `delete` from Lucide.
pub fn delete<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0AE}").font(LUCIDE)
}

/// `dessert` from Lucide.
pub fn dessert<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4BB}").font(LUCIDE)
}

/// `diameter` from Lucide.
pub fn diameter<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E526}").font(LUCIDE)
}

/// `diamond` from Lucide.
pub fn diamond<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2D2}").font(LUCIDE)
}

/// `diamond-minus` from Lucide.
pub fn diamond_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5E1}").font(LUCIDE)
}

/// `diamond-percent` from Lucide.
pub fn diamond_percent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E51B}").font(LUCIDE)
}

/// `diamond-plus` from Lucide.
pub fn diamond_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5E2}").font(LUCIDE)
}

/// `dice-1` from Lucide.
pub fn dice_1<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E287}").font(LUCIDE)
}

/// `dice-2` from Lucide.
pub fn dice_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E288}").font(LUCIDE)
}

/// `dice-3` from Lucide.
pub fn dice_3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E289}").font(LUCIDE)
}

/// `dice-4` from Lucide.
pub fn dice_4<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E28A}").font(LUCIDE)
}

/// `dice-5` from Lucide.
pub fn dice_5<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E28B}").font(LUCIDE)
}

/// `dice-6` from Lucide.
pub fn dice_6<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E28C}").font(LUCIDE)
}

/// `dices` from Lucide.
pub fn dices<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2C5}").font(LUCIDE)
}

/// `diff` from Lucide.
pub fn diff<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E30C}").font(LUCIDE)
}

/// `disc` from Lucide.
pub fn disc<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0AF}").font(LUCIDE)
}

/// `disc-2` from Lucide.
pub fn disc_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3F6}").font(LUCIDE)
}

/// `disc-3` from Lucide.
pub fn disc_3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E494}").font(LUCIDE)
}

/// `disc-album` from Lucide.
pub fn disc_album<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E55C}").font(LUCIDE)
}

/// `divide` from Lucide.
pub fn divide<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0B0}").font(LUCIDE)
}

/// `divide-circle` from Lucide.
pub fn divide_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E07D}").font(LUCIDE)
}

/// `divide-square` from Lucide.
pub fn divide_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E16D}").font(LUCIDE)
}

/// `dna` from Lucide.
pub fn dna<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E393}").font(LUCIDE)
}

/// `dna-off` from Lucide.
pub fn dna_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E394}").font(LUCIDE)
}

/// `dock` from Lucide.
pub fn dock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5D3}").font(LUCIDE)
}

/// `dog` from Lucide.
pub fn dog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E38D}").font(LUCIDE)
}

/// `dollar-sign` from Lucide.
pub fn dollar_sign<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0B1}").font(LUCIDE)
}

/// `donut` from Lucide.
pub fn donut<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4BC}").font(LUCIDE)
}

/// `door-closed` from Lucide.
pub fn door_closed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D5}").font(LUCIDE)
}

/// `door-closed-locked` from Lucide.
pub fn door_closed_locked<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E664}").font(LUCIDE)
}

/// `door-open` from Lucide.
pub fn door_open<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D6}").font(LUCIDE)
}

/// `dot` from Lucide.
pub fn dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E44F}").font(LUCIDE)
}

/// `dot-square` from Lucide.
pub fn dot_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E16E}").font(LUCIDE)
}

/// `download` from Lucide.
pub fn download<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0B2}").font(LUCIDE)
}

/// `download-cloud` from Lucide.
pub fn download_cloud<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E089}").font(LUCIDE)
}

/// `drafting-compass` from Lucide.
pub fn drafting_compass<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E527}").font(LUCIDE)
}

/// `drama` from Lucide.
pub fn drama<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E521}").font(LUCIDE)
}

/// `drill` from Lucide.
pub fn drill<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E58D}").font(LUCIDE)
}

/// `drone` from Lucide.
pub fn drone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E676}").font(LUCIDE)
}

/// `droplet` from Lucide.
pub fn droplet<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0B4}").font(LUCIDE)
}

/// `droplet-off` from Lucide.
pub fn droplet_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E638}").font(LUCIDE)
}

/// `droplets` from Lucide.
pub fn droplets<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0B5}").font(LUCIDE)
}

/// `drum` from Lucide.
pub fn drum<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E55D}").font(LUCIDE)
}

/// `drumstick` from Lucide.
pub fn drumstick<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E25B}").font(LUCIDE)
}

/// `dumbbell` from Lucide.
pub fn dumbbell<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3A1}").font(LUCIDE)
}

/// `ear` from Lucide.
pub fn ear<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E382}").font(LUCIDE)
}

/// `ear-off` from Lucide.
pub fn ear_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E383}").font(LUCIDE)
}

/// `earth` from Lucide.
pub fn earth<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1F3}").font(LUCIDE)
}

/// `earth-lock` from Lucide.
pub fn earth_lock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5CC}").font(LUCIDE)
}

/// `eclipse` from Lucide.
pub fn eclipse<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E59D}").font(LUCIDE)
}

/// `edit` from Lucide.
pub fn edit<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E172}").font(LUCIDE)
}

/// `edit-2` from Lucide.
pub fn edit_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E12F}").font(LUCIDE)
}

/// `edit-3` from Lucide.
pub fn edit_3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E130}").font(LUCIDE)
}

/// `egg` from Lucide.
pub fn egg<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E25D}").font(LUCIDE)
}

/// `egg-fried` from Lucide.
pub fn egg_fried<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E351}").font(LUCIDE)
}

/// `egg-off` from Lucide.
pub fn egg_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E395}").font(LUCIDE)
}

/// `eject` from Lucide.
pub fn eject<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E733}").font(LUCIDE)
}

/// `ellipse` from Lucide.
pub fn ellipse<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6B2}").font(LUCIDE)
}

/// `ellipsis` from Lucide.
pub fn ellipsis<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0B6}").font(LUCIDE)
}

/// `ellipsis-vertical` from Lucide.
pub fn ellipsis_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0B7}").font(LUCIDE)
}

/// `equal` from Lucide.
pub fn equal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1BD}").font(LUCIDE)
}

/// `equal-approximately` from Lucide.
pub fn equal_approximately<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E634}").font(LUCIDE)
}

/// `equal-not` from Lucide.
pub fn equal_not<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1BE}").font(LUCIDE)
}

/// `equal-square` from Lucide.
pub fn equal_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E16F}").font(LUCIDE)
}

/// `eraser` from Lucide.
pub fn eraser<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E28F}").font(LUCIDE)
}

/// `ethernet-port` from Lucide.
pub fn ethernet_port<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E620}").font(LUCIDE)
}

/// `euro` from Lucide.
pub fn euro<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0B8}").font(LUCIDE)
}

/// `ev-charger` from Lucide.
pub fn ev_charger<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E697}").font(LUCIDE)
}

/// `expand` from Lucide.
pub fn expand<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E21A}").font(LUCIDE)
}

/// `external-link` from Lucide.
pub fn external_link<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0B9}").font(LUCIDE)
}

/// `eye` from Lucide.
pub fn eye<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0BA}").font(LUCIDE)
}

/// `eye-closed` from Lucide.
pub fn eye_closed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E62E}").font(LUCIDE)
}

/// `eye-dashed` from Lucide.
pub fn eye_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E701}").font(LUCIDE)
}

/// `eye-off` from Lucide.
pub fn eye_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0BB}").font(LUCIDE)
}

/// `face-angry` from Lucide.
pub fn face_angry<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2FC}").font(LUCIDE)
}

/// `face-expressionless` from Lucide.
pub fn face_expressionless<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2FD}").font(LUCIDE)
}

/// `face-grinning` from Lucide.
pub fn face_grinning<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E300}").font(LUCIDE)
}

/// `face-neutral` from Lucide.
pub fn face_neutral<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E114}").font(LUCIDE)
}

/// `face-slightly-frowning` from Lucide.
pub fn face_slightly_frowning<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0DB}").font(LUCIDE)
}

/// `face-slightly-smiling` from Lucide.
pub fn face_slightly_smiling<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E164}").font(LUCIDE)
}

/// `face-slightly-smiling-plus` from Lucide.
pub fn face_slightly_smiling_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E301}").font(LUCIDE)
}

/// `factory` from Lucide.
pub fn factory<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E29F}").font(LUCIDE)
}

/// `fan` from Lucide.
pub fn fan<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E379}").font(LUCIDE)
}

/// `fast-forward` from Lucide.
pub fn fast_forward<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0BD}").font(LUCIDE)
}

/// `feather` from Lucide.
pub fn feather<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0BE}").font(LUCIDE)
}

/// `fence` from Lucide.
pub fn fence<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E582}").font(LUCIDE)
}

/// `ferris-wheel` from Lucide.
pub fn ferris_wheel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E47F}").font(LUCIDE)
}

/// `file` from Lucide.
pub fn file<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0C0}").font(LUCIDE)
}

/// `file-archive` from Lucide.
pub fn file_archive<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E30D}").font(LUCIDE)
}

/// `file-audio` from Lucide.
pub fn file_audio<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E31A}").font(LUCIDE)
}

/// `file-audio-2` from Lucide.
pub fn file_audio_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E31A}").font(LUCIDE)
}

/// `file-axis-3-d` from Lucide.
pub fn file_axis_3_d<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E30E}").font(LUCIDE)
}

/// `file-axis-3d` from Lucide.
pub fn file_axis_3d<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E30E}").font(LUCIDE)
}

/// `file-badge` from Lucide.
pub fn file_badge<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E30F}").font(LUCIDE)
}

/// `file-badge-2` from Lucide.
pub fn file_badge_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E30F}").font(LUCIDE)
}

/// `file-bar-chart` from Lucide.
pub fn file_bar_chart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E312}").font(LUCIDE)
}

/// `file-bar-chart-2` from Lucide.
pub fn file_bar_chart_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E311}").font(LUCIDE)
}

/// `file-box` from Lucide.
pub fn file_box<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E310}").font(LUCIDE)
}

/// `file-braces` from Lucide.
pub fn file_braces<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E36B}").font(LUCIDE)
}

/// `file-braces-corner` from Lucide.
pub fn file_braces_corner<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E36C}").font(LUCIDE)
}

/// `file-chart-column` from Lucide.
pub fn file_chart_column<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E311}").font(LUCIDE)
}

/// `file-chart-column-increasing` from Lucide.
pub fn file_chart_column_increasing<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E312}").font(LUCIDE)
}

/// `file-chart-line` from Lucide.
pub fn file_chart_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E313}").font(LUCIDE)
}

/// `file-chart-pie` from Lucide.
pub fn file_chart_pie<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E314}").font(LUCIDE)
}

/// `file-check` from Lucide.
pub fn file_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0C1}").font(LUCIDE)
}

/// `file-check-2` from Lucide.
pub fn file_check_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0C2}").font(LUCIDE)
}

/// `file-check-corner` from Lucide.
pub fn file_check_corner<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0C2}").font(LUCIDE)
}

/// `file-clock` from Lucide.
pub fn file_clock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E315}").font(LUCIDE)
}

/// `file-code` from Lucide.
pub fn file_code<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0C3}").font(LUCIDE)
}

/// `file-code-2` from Lucide.
pub fn file_code_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E45E}").font(LUCIDE)
}

/// `file-code-corner` from Lucide.
pub fn file_code_corner<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E45E}").font(LUCIDE)
}

/// `file-cog` from Lucide.
pub fn file_cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E316}").font(LUCIDE)
}

/// `file-cog-2` from Lucide.
pub fn file_cog_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E316}").font(LUCIDE)
}

/// `file-diff` from Lucide.
pub fn file_diff<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E317}").font(LUCIDE)
}

/// `file-digit` from Lucide.
pub fn file_digit<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0C4}").font(LUCIDE)
}

/// `file-down` from Lucide.
pub fn file_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E318}").font(LUCIDE)
}

/// `file-edit` from Lucide.
pub fn file_edit<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E31F}").font(LUCIDE)
}

/// `file-exclamation-point` from Lucide.
pub fn file_exclamation_point<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E319}").font(LUCIDE)
}

/// `file-headphone` from Lucide.
pub fn file_headphone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E31A}").font(LUCIDE)
}

/// `file-heart` from Lucide.
pub fn file_heart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E31B}").font(LUCIDE)
}

/// `file-image` from Lucide.
pub fn file_image<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E31C}").font(LUCIDE)
}

/// `file-input` from Lucide.
pub fn file_input<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0C5}").font(LUCIDE)
}

/// `file-json` from Lucide.
pub fn file_json<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E36B}").font(LUCIDE)
}

/// `file-json-2` from Lucide.
pub fn file_json_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E36C}").font(LUCIDE)
}

/// `file-key` from Lucide.
pub fn file_key<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E31D}").font(LUCIDE)
}

/// `file-key-2` from Lucide.
pub fn file_key_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E31D}").font(LUCIDE)
}

/// `file-line-chart` from Lucide.
pub fn file_line_chart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E313}").font(LUCIDE)
}

/// `file-lock` from Lucide.
pub fn file_lock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E31E}").font(LUCIDE)
}

/// `file-lock-2` from Lucide.
pub fn file_lock_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E31E}").font(LUCIDE)
}

/// `file-minus` from Lucide.
pub fn file_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0C6}").font(LUCIDE)
}

/// `file-minus-2` from Lucide.
pub fn file_minus_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0C7}").font(LUCIDE)
}

/// `file-minus-corner` from Lucide.
pub fn file_minus_corner<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0C7}").font(LUCIDE)
}

/// `file-music` from Lucide.
pub fn file_music<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E55E}").font(LUCIDE)
}

/// `file-output` from Lucide.
pub fn file_output<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0C8}").font(LUCIDE)
}

/// `file-pen` from Lucide.
pub fn file_pen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E31F}").font(LUCIDE)
}

/// `file-pen-line` from Lucide.
pub fn file_pen_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E320}").font(LUCIDE)
}

/// `file-pie-chart` from Lucide.
pub fn file_pie_chart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E314}").font(LUCIDE)
}

/// `file-play` from Lucide.
pub fn file_play<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E321}").font(LUCIDE)
}

/// `file-plus` from Lucide.
pub fn file_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0C9}").font(LUCIDE)
}

/// `file-plus-2` from Lucide.
pub fn file_plus_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0CA}").font(LUCIDE)
}

/// `file-plus-corner` from Lucide.
pub fn file_plus_corner<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0CA}").font(LUCIDE)
}

/// `file-question` from Lucide.
pub fn file_question<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E322}").font(LUCIDE)
}

/// `file-question-mark` from Lucide.
pub fn file_question_mark<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E322}").font(LUCIDE)
}

/// `file-scan` from Lucide.
pub fn file_scan<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E323}").font(LUCIDE)
}

/// `file-search` from Lucide.
pub fn file_search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0CB}").font(LUCIDE)
}

/// `file-search-2` from Lucide.
pub fn file_search_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E324}").font(LUCIDE)
}

/// `file-search-corner` from Lucide.
pub fn file_search_corner<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E324}").font(LUCIDE)
}

/// `file-signal` from Lucide.
pub fn file_signal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E325}").font(LUCIDE)
}

/// `file-signature` from Lucide.
pub fn file_signature<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E320}").font(LUCIDE)
}

/// `file-sliders` from Lucide.
pub fn file_sliders<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A0}").font(LUCIDE)
}

/// `file-spreadsheet` from Lucide.
pub fn file_spreadsheet<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E326}").font(LUCIDE)
}

/// `file-stack` from Lucide.
pub fn file_stack<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A1}").font(LUCIDE)
}

/// `file-symlink` from Lucide.
pub fn file_symlink<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E327}").font(LUCIDE)
}

/// `file-terminal` from Lucide.
pub fn file_terminal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E328}").font(LUCIDE)
}

/// `file-text` from Lucide.
pub fn file_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0CC}").font(LUCIDE)
}

/// `file-type` from Lucide.
pub fn file_type<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E329}").font(LUCIDE)
}

/// `file-type-2` from Lucide.
pub fn file_type_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E36D}").font(LUCIDE)
}

/// `file-type-corner` from Lucide.
pub fn file_type_corner<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E36D}").font(LUCIDE)
}

/// `file-up` from Lucide.
pub fn file_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E32A}").font(LUCIDE)
}

/// `file-user` from Lucide.
pub fn file_user<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E62D}").font(LUCIDE)
}

/// `file-video` from Lucide.
pub fn file_video<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E321}").font(LUCIDE)
}

/// `file-video-2` from Lucide.
pub fn file_video_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E32B}").font(LUCIDE)
}

/// `file-video-camera` from Lucide.
pub fn file_video_camera<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E32B}").font(LUCIDE)
}

/// `file-volume` from Lucide.
pub fn file_volume<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E32C}").font(LUCIDE)
}

/// `file-volume-2` from Lucide.
pub fn file_volume_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E325}").font(LUCIDE)
}

/// `file-warning` from Lucide.
pub fn file_warning<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E319}").font(LUCIDE)
}

/// `file-x` from Lucide.
pub fn file_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0CD}").font(LUCIDE)
}

/// `file-x-2` from Lucide.
pub fn file_x_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0CE}").font(LUCIDE)
}

/// `file-x-corner` from Lucide.
pub fn file_x_corner<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0CE}").font(LUCIDE)
}

/// `files` from Lucide.
pub fn files<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0CF}").font(LUCIDE)
}

/// `film` from Lucide.
pub fn film<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0D0}").font(LUCIDE)
}

/// `filter` from Lucide.
pub fn filter<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0DC}").font(LUCIDE)
}

/// `filter-x` from Lucide.
pub fn filter_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B5}").font(LUCIDE)
}

/// `fingerprint` from Lucide.
pub fn fingerprint<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2CB}").font(LUCIDE)
}

/// `fingerprint-pattern` from Lucide.
pub fn fingerprint_pattern<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2CB}").font(LUCIDE)
}

/// `fire-extinguisher` from Lucide.
pub fn fire_extinguisher<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E57E}").font(LUCIDE)
}

/// `fish` from Lucide.
pub fn fish<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3A6}").font(LUCIDE)
}

/// `fish-off` from Lucide.
pub fn fish_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B0}").font(LUCIDE)
}

/// `fish-symbol` from Lucide.
pub fn fish_symbol<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4F4}").font(LUCIDE)
}

/// `fishing-hook` from Lucide.
pub fn fishing_hook<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6E4}").font(LUCIDE)
}

/// `fishing-rod` from Lucide.
pub fn fishing_rod<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6B3}").font(LUCIDE)
}

/// `flag` from Lucide.
pub fn flag<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0D1}").font(LUCIDE)
}

/// `flag-off` from Lucide.
pub fn flag_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E292}").font(LUCIDE)
}

/// `flag-triangle-left` from Lucide.
pub fn flag_triangle_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E237}").font(LUCIDE)
}

/// `flag-triangle-right` from Lucide.
pub fn flag_triangle_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E238}").font(LUCIDE)
}

/// `flame` from Lucide.
pub fn flame<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0D2}").font(LUCIDE)
}

/// `flame-kindling` from Lucide.
pub fn flame_kindling<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E53A}").font(LUCIDE)
}

/// `flashlight` from Lucide.
pub fn flashlight<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0D3}").font(LUCIDE)
}

/// `flashlight-off` from Lucide.
pub fn flashlight_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0D4}").font(LUCIDE)
}

/// `flask-conical` from Lucide.
pub fn flask_conical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0D5}").font(LUCIDE)
}

/// `flask-conical-off` from Lucide.
pub fn flask_conical_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E396}").font(LUCIDE)
}

/// `flask-round` from Lucide.
pub fn flask_round<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0D6}").font(LUCIDE)
}

/// `flip-horizontal` from Lucide.
pub fn flip_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E35D}").font(LUCIDE)
}

/// `flip-horizontal-2` from Lucide.
pub fn flip_horizontal_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E35E}").font(LUCIDE)
}

/// `flip-vertical` from Lucide.
pub fn flip_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E35F}").font(LUCIDE)
}

/// `flip-vertical-2` from Lucide.
pub fn flip_vertical_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E360}").font(LUCIDE)
}

/// `flower` from Lucide.
pub fn flower<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2D3}").font(LUCIDE)
}

/// `flower-2` from Lucide.
pub fn flower_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2D4}").font(LUCIDE)
}

/// `focus` from Lucide.
pub fn focus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E29E}").font(LUCIDE)
}

/// `fold-horizontal` from Lucide.
pub fn fold_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E43B}").font(LUCIDE)
}

/// `fold-vertical` from Lucide.
pub fn fold_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E43C}").font(LUCIDE)
}

/// `folder` from Lucide.
pub fn folder<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0D7}").font(LUCIDE)
}

/// `folder-archive` from Lucide.
pub fn folder_archive<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E32D}").font(LUCIDE)
}

/// `folder-bookmark` from Lucide.
pub fn folder_bookmark<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6F1}").font(LUCIDE)
}

/// `folder-check` from Lucide.
pub fn folder_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E32E}").font(LUCIDE)
}

/// `folder-clock` from Lucide.
pub fn folder_clock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E32F}").font(LUCIDE)
}

/// `folder-closed` from Lucide.
pub fn folder_closed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E330}").font(LUCIDE)
}

/// `folder-code` from Lucide.
pub fn folder_code<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5FB}").font(LUCIDE)
}

/// `folder-cog` from Lucide.
pub fn folder_cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E331}").font(LUCIDE)
}

/// `folder-cog-2` from Lucide.
pub fn folder_cog_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E331}").font(LUCIDE)
}

/// `folder-dot` from Lucide.
pub fn folder_dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4C5}").font(LUCIDE)
}

/// `folder-down` from Lucide.
pub fn folder_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E332}").font(LUCIDE)
}

/// `folder-edit` from Lucide.
pub fn folder_edit<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E338}").font(LUCIDE)
}

/// `folder-git` from Lucide.
pub fn folder_git<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E409}").font(LUCIDE)
}

/// `folder-git-2` from Lucide.
pub fn folder_git_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E40A}").font(LUCIDE)
}

/// `folder-heart` from Lucide.
pub fn folder_heart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E333}").font(LUCIDE)
}

/// `folder-input` from Lucide.
pub fn folder_input<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E334}").font(LUCIDE)
}

/// `folder-kanban` from Lucide.
pub fn folder_kanban<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4C6}").font(LUCIDE)
}

/// `folder-key` from Lucide.
pub fn folder_key<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E335}").font(LUCIDE)
}

/// `folder-lock` from Lucide.
pub fn folder_lock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E336}").font(LUCIDE)
}

/// `folder-minus` from Lucide.
pub fn folder_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0D8}").font(LUCIDE)
}

/// `folder-open` from Lucide.
pub fn folder_open<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E247}").font(LUCIDE)
}

/// `folder-open-dot` from Lucide.
pub fn folder_open_dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4C7}").font(LUCIDE)
}

/// `folder-output` from Lucide.
pub fn folder_output<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E337}").font(LUCIDE)
}

/// `folder-pen` from Lucide.
pub fn folder_pen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E338}").font(LUCIDE)
}

/// `folder-plus` from Lucide.
pub fn folder_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0D9}").font(LUCIDE)
}

/// `folder-root` from Lucide.
pub fn folder_root<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4C8}").font(LUCIDE)
}

/// `folder-search` from Lucide.
pub fn folder_search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E339}").font(LUCIDE)
}

/// `folder-search-2` from Lucide.
pub fn folder_search_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E33A}").font(LUCIDE)
}

/// `folder-symlink` from Lucide.
pub fn folder_symlink<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E33B}").font(LUCIDE)
}

/// `folder-sync` from Lucide.
pub fn folder_sync<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4C9}").font(LUCIDE)
}

/// `folder-tree` from Lucide.
pub fn folder_tree<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E33C}").font(LUCIDE)
}

/// `folder-up` from Lucide.
pub fn folder_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E33D}").font(LUCIDE)
}

/// `folder-x` from Lucide.
pub fn folder_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E33E}").font(LUCIDE)
}

/// `folders` from Lucide.
pub fn folders<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E33F}").font(LUCIDE)
}

/// `footprints` from Lucide.
pub fn footprints<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B9}").font(LUCIDE)
}

/// `fork-knife` from Lucide.
pub fn fork_knife<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2F6}").font(LUCIDE)
}

/// `fork-knife-crossed` from Lucide.
pub fn fork_knife_crossed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2F7}").font(LUCIDE)
}

/// `forklift` from Lucide.
pub fn forklift<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3C1}").font(LUCIDE)
}

/// `form` from Lucide.
pub fn form<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6A8}").font(LUCIDE)
}

/// `form-input` from Lucide.
pub fn form_input<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E21F}").font(LUCIDE)
}

/// `forward` from Lucide.
pub fn forward<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E229}").font(LUCIDE)
}

/// `frame` from Lucide.
pub fn frame<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E291}").font(LUCIDE)
}

/// `frown` from Lucide.
pub fn frown<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0DB}").font(LUCIDE)
}

/// `fuel` from Lucide.
pub fn fuel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2AF}").font(LUCIDE)
}

/// `fullscreen` from Lucide.
pub fn fullscreen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E534}").font(LUCIDE)
}

/// `function-square` from Lucide.
pub fn function_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E22D}").font(LUCIDE)
}

/// `funnel` from Lucide.
pub fn funnel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0DC}").font(LUCIDE)
}

/// `funnel-plus` from Lucide.
pub fn funnel_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0DD}").font(LUCIDE)
}

/// `funnel-x` from Lucide.
pub fn funnel_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B5}").font(LUCIDE)
}

/// `galaxy` from Lucide.
pub fn galaxy<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E73F}").font(LUCIDE)
}

/// `gallery-horizontal` from Lucide.
pub fn gallery_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4CE}").font(LUCIDE)
}

/// `gallery-horizontal-end` from Lucide.
pub fn gallery_horizontal_end<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4CF}").font(LUCIDE)
}

/// `gallery-thumbnails` from Lucide.
pub fn gallery_thumbnails<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4D0}").font(LUCIDE)
}

/// `gallery-vertical` from Lucide.
pub fn gallery_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4D1}").font(LUCIDE)
}

/// `gallery-vertical-end` from Lucide.
pub fn gallery_vertical_end<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4D2}").font(LUCIDE)
}

/// `gamepad` from Lucide.
pub fn gamepad<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0DE}").font(LUCIDE)
}

/// `gamepad-2` from Lucide.
pub fn gamepad_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0DF}").font(LUCIDE)
}

/// `gamepad-directional` from Lucide.
pub fn gamepad_directional<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E69B}").font(LUCIDE)
}

/// `gantt-chart` from Lucide.
pub fn gantt_chart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4C4}").font(LUCIDE)
}

/// `gantt-chart-square` from Lucide.
pub fn gantt_chart_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E169}").font(LUCIDE)
}

/// `gauge` from Lucide.
pub fn gauge<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1BF}").font(LUCIDE)
}

/// `gauge-circle` from Lucide.
pub fn gauge_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4E1}").font(LUCIDE)
}

/// `gavel` from Lucide.
pub fn gavel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0E0}").font(LUCIDE)
}

/// `gem` from Lucide.
pub fn gem<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E242}").font(LUCIDE)
}

/// `georgian-lari` from Lucide.
pub fn georgian_lari<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E678}").font(LUCIDE)
}

/// `ghost` from Lucide.
pub fn ghost<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E20E}").font(LUCIDE)
}

/// `gift` from Lucide.
pub fn gift<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0E1}").font(LUCIDE)
}

/// `git-branch` from Lucide.
pub fn git_branch<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0E2}").font(LUCIDE)
}

/// `git-branch-minus` from Lucide.
pub fn git_branch_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E69C}").font(LUCIDE)
}

/// `git-branch-plus` from Lucide.
pub fn git_branch_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1F4}").font(LUCIDE)
}

/// `git-commit` from Lucide.
pub fn git_commit<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0E3}").font(LUCIDE)
}

/// `git-commit-horizontal` from Lucide.
pub fn git_commit_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0E3}").font(LUCIDE)
}

/// `git-commit-vertical` from Lucide.
pub fn git_commit_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E552}").font(LUCIDE)
}

/// `git-compare` from Lucide.
pub fn git_compare<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E359}").font(LUCIDE)
}

/// `git-compare-arrows` from Lucide.
pub fn git_compare_arrows<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E553}").font(LUCIDE)
}

/// `git-fork` from Lucide.
pub fn git_fork<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E28D}").font(LUCIDE)
}

/// `git-graph` from Lucide.
pub fn git_graph<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E554}").font(LUCIDE)
}

/// `git-merge` from Lucide.
pub fn git_merge<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0E4}").font(LUCIDE)
}

/// `git-merge-conflict` from Lucide.
pub fn git_merge_conflict<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6B4}").font(LUCIDE)
}

/// `git-pull-request` from Lucide.
pub fn git_pull_request<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0E5}").font(LUCIDE)
}

/// `git-pull-request-arrow` from Lucide.
pub fn git_pull_request_arrow<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E555}").font(LUCIDE)
}

/// `git-pull-request-closed` from Lucide.
pub fn git_pull_request_closed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E35A}").font(LUCIDE)
}

/// `git-pull-request-create` from Lucide.
pub fn git_pull_request_create<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E556}").font(LUCIDE)
}

/// `git-pull-request-create-arrow` from Lucide.
pub fn git_pull_request_create_arrow<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E557}").font(LUCIDE)
}

/// `git-pull-request-draft` from Lucide.
pub fn git_pull_request_draft<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E35B}").font(LUCIDE)
}

/// `glass-water` from Lucide.
pub fn glass_water<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2D5}").font(LUCIDE)
}

/// `glasses` from Lucide.
pub fn glasses<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E20D}").font(LUCIDE)
}

/// `globe` from Lucide.
pub fn globe<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0E8}").font(LUCIDE)
}

/// `globe-2` from Lucide.
pub fn globe_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1F3}").font(LUCIDE)
}

/// `globe-check` from Lucide.
pub fn globe_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6FC}").font(LUCIDE)
}

/// `globe-lock` from Lucide.
pub fn globe_lock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5CD}").font(LUCIDE)
}

/// `globe-off` from Lucide.
pub fn globe_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6B5}").font(LUCIDE)
}

/// `globe-x` from Lucide.
pub fn globe_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6B6}").font(LUCIDE)
}

/// `goal` from Lucide.
pub fn goal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A5}").font(LUCIDE)
}

/// `gpu` from Lucide.
pub fn gpu<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E66A}").font(LUCIDE)
}

/// `grab` from Lucide.
pub fn grab<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1E6}").font(LUCIDE)
}

/// `graduation-cap` from Lucide.
pub fn graduation_cap<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E234}").font(LUCIDE)
}

/// `grape` from Lucide.
pub fn grape<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E352}").font(LUCIDE)
}

/// `grid` from Lucide.
pub fn grid<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0E9}").font(LUCIDE)
}

/// `grid-2-x-2` from Lucide.
pub fn grid_2_x_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4FF}").font(LUCIDE)
}

/// `grid-2-x-2-check` from Lucide.
pub fn grid_2_x_2_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5E4}").font(LUCIDE)
}

/// `grid-2-x-2-plus` from Lucide.
pub fn grid_2_x_2_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E628}").font(LUCIDE)
}

/// `grid-2-x-2-x` from Lucide.
pub fn grid_2_x_2_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5E5}").font(LUCIDE)
}

/// `grid-2x2` from Lucide.
pub fn grid_2x2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4FF}").font(LUCIDE)
}

/// `grid-2x2-check` from Lucide.
pub fn grid_2x2_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5E4}").font(LUCIDE)
}

/// `grid-2x2-plus` from Lucide.
pub fn grid_2x2_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E628}").font(LUCIDE)
}

/// `grid-2x2-x` from Lucide.
pub fn grid_2x2_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5E5}").font(LUCIDE)
}

/// `grid-3-x-3` from Lucide.
pub fn grid_3_x_3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0E9}").font(LUCIDE)
}

/// `grid-3x2` from Lucide.
pub fn grid_3x2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E66F}").font(LUCIDE)
}

/// `grid-3x3` from Lucide.
pub fn grid_3x3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0E9}").font(LUCIDE)
}

/// `grip` from Lucide.
pub fn grip<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B1}").font(LUCIDE)
}

/// `grip-horizontal` from Lucide.
pub fn grip_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0EA}").font(LUCIDE)
}

/// `grip-vertical` from Lucide.
pub fn grip_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0EB}").font(LUCIDE)
}

/// `group` from Lucide.
pub fn group<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E464}").font(LUCIDE)
}

/// `guitar` from Lucide.
pub fn guitar<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E55F}").font(LUCIDE)
}

/// `ham` from Lucide.
pub fn ham<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5D7}").font(LUCIDE)
}

/// `hamburger` from Lucide.
pub fn hamburger<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E665}").font(LUCIDE)
}

/// `hammer` from Lucide.
pub fn hammer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0EC}").font(LUCIDE)
}

/// `hand` from Lucide.
pub fn hand<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1D7}").font(LUCIDE)
}

/// `hand-coins` from Lucide.
pub fn hand_coins<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5B8}").font(LUCIDE)
}

/// `hand-fist` from Lucide.
pub fn hand_fist<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E68B}").font(LUCIDE)
}

/// `hand-grab` from Lucide.
pub fn hand_grab<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1E6}").font(LUCIDE)
}

/// `hand-heart` from Lucide.
pub fn hand_heart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5B9}").font(LUCIDE)
}

/// `hand-helping` from Lucide.
pub fn hand_helping<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B8}").font(LUCIDE)
}

/// `hand-metal` from Lucide.
pub fn hand_metal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E22C}").font(LUCIDE)
}

/// `hand-platter` from Lucide.
pub fn hand_platter<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5BA}").font(LUCIDE)
}

/// `handbag` from Lucide.
pub fn handbag<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E689}").font(LUCIDE)
}

/// `handshake` from Lucide.
pub fn handshake<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5C0}").font(LUCIDE)
}

/// `hard-drive` from Lucide.
pub fn hard_drive<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0ED}").font(LUCIDE)
}

/// `hard-drive-download` from Lucide.
pub fn hard_drive_download<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4E5}").font(LUCIDE)
}

/// `hard-drive-upload` from Lucide.
pub fn hard_drive_upload<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4E6}").font(LUCIDE)
}

/// `hard-hat` from Lucide.
pub fn hard_hat<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0EE}").font(LUCIDE)
}

/// `hash` from Lucide.
pub fn hash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0EF}").font(LUCIDE)
}

/// `hat-glasses` from Lucide.
pub fn hat_glasses<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E683}").font(LUCIDE)
}

/// `haze` from Lucide.
pub fn haze<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0F0}").font(LUCIDE)
}

/// `hd` from Lucide.
pub fn hd<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6E5}").font(LUCIDE)
}

/// `hdmi-port` from Lucide.
pub fn hdmi_port<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4E7}").font(LUCIDE)
}

/// `heading` from Lucide.
pub fn heading<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E384}").font(LUCIDE)
}

/// `heading-1` from Lucide.
pub fn heading_1<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E385}").font(LUCIDE)
}

/// `heading-2` from Lucide.
pub fn heading_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E386}").font(LUCIDE)
}

/// `heading-3` from Lucide.
pub fn heading_3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E387}").font(LUCIDE)
}

/// `heading-4` from Lucide.
pub fn heading_4<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E388}").font(LUCIDE)
}

/// `heading-5` from Lucide.
pub fn heading_5<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E389}").font(LUCIDE)
}

/// `heading-6` from Lucide.
pub fn heading_6<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E38A}").font(LUCIDE)
}

/// `headphone-off` from Lucide.
pub fn headphone_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E629}").font(LUCIDE)
}

/// `headphones` from Lucide.
pub fn headphones<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0F1}").font(LUCIDE)
}

/// `headset` from Lucide.
pub fn headset<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5BD}").font(LUCIDE)
}

/// `heart` from Lucide.
pub fn heart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0F2}").font(LUCIDE)
}

/// `heart-crack` from Lucide.
pub fn heart_crack<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2D6}").font(LUCIDE)
}

/// `heart-handshake` from Lucide.
pub fn heart_handshake<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2D7}").font(LUCIDE)
}

/// `heart-minus` from Lucide.
pub fn heart_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E651}").font(LUCIDE)
}

/// `heart-off` from Lucide.
pub fn heart_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E295}").font(LUCIDE)
}

/// `heart-plus` from Lucide.
pub fn heart_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E652}").font(LUCIDE)
}

/// `heart-pulse` from Lucide.
pub fn heart_pulse<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E36E}").font(LUCIDE)
}

/// `heart-x` from Lucide.
pub fn heart_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6ED}").font(LUCIDE)
}

/// `heater` from Lucide.
pub fn heater<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E58E}").font(LUCIDE)
}

/// `helicopter` from Lucide.
pub fn helicopter<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E69D}").font(LUCIDE)
}

/// `help-circle` from Lucide.
pub fn help_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E082}").font(LUCIDE)
}

/// `helping-hand` from Lucide.
pub fn helping_hand<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B8}").font(LUCIDE)
}

/// `hexagon` from Lucide.
pub fn hexagon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0F3}").font(LUCIDE)
}

/// `highlighter` from Lucide.
pub fn highlighter<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0F4}").font(LUCIDE)
}

/// `history` from Lucide.
pub fn history<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1F5}").font(LUCIDE)
}

/// `home` from Lucide.
pub fn home<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0F5}").font(LUCIDE)
}

/// `hop` from Lucide.
pub fn hop<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E397}").font(LUCIDE)
}

/// `hop-off` from Lucide.
pub fn hop_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E398}").font(LUCIDE)
}

/// `hospital` from Lucide.
pub fn hospital<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5D8}").font(LUCIDE)
}

/// `hotel` from Lucide.
pub fn hotel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3E2}").font(LUCIDE)
}

/// `hourglass` from Lucide.
pub fn hourglass<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E296}").font(LUCIDE)
}

/// `house` from Lucide.
pub fn house<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0F5}").font(LUCIDE)
}

/// `house-heart` from Lucide.
pub fn house_heart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E695}").font(LUCIDE)
}

/// `house-plug` from Lucide.
pub fn house_plug<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5F0}").font(LUCIDE)
}

/// `house-plus` from Lucide.
pub fn house_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5F1}").font(LUCIDE)
}

/// `house-wifi` from Lucide.
pub fn house_wifi<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E63C}").font(LUCIDE)
}

/// `ice-cream` from Lucide.
pub fn ice_cream<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E353}").font(LUCIDE)
}

/// `ice-cream-2` from Lucide.
pub fn ice_cream_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3A7}").font(LUCIDE)
}

/// `ice-cream-bowl` from Lucide.
pub fn ice_cream_bowl<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3A7}").font(LUCIDE)
}

/// `ice-cream-cone` from Lucide.
pub fn ice_cream_cone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E353}").font(LUCIDE)
}

/// `id-card` from Lucide.
pub fn id_card<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E617}").font(LUCIDE)
}

/// `id-card-lanyard` from Lucide.
pub fn id_card_lanyard<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E670}").font(LUCIDE)
}

/// `image` from Lucide.
pub fn image<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0F6}").font(LUCIDE)
}

/// `image-down` from Lucide.
pub fn image_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E53C}").font(LUCIDE)
}

/// `image-minus` from Lucide.
pub fn image_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1F6}").font(LUCIDE)
}

/// `image-off` from Lucide.
pub fn image_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1C0}").font(LUCIDE)
}

/// `image-play` from Lucide.
pub fn image_play<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5DF}").font(LUCIDE)
}

/// `image-plus` from Lucide.
pub fn image_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1F7}").font(LUCIDE)
}

/// `image-up` from Lucide.
pub fn image_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5CB}").font(LUCIDE)
}

/// `image-upscale` from Lucide.
pub fn image_upscale<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E637}").font(LUCIDE)
}

/// `images` from Lucide.
pub fn images<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5C4}").font(LUCIDE)
}

/// `import` from Lucide.
pub fn import<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E22F}").font(LUCIDE)
}

/// `inbox` from Lucide.
pub fn inbox<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0F7}").font(LUCIDE)
}

/// `indent` from Lucide.
pub fn indent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E108}").font(LUCIDE)
}

/// `indent-decrease` from Lucide.
pub fn indent_decrease<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E107}").font(LUCIDE)
}

/// `indent-increase` from Lucide.
pub fn indent_increase<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E108}").font(LUCIDE)
}

/// `indian-rupee` from Lucide.
pub fn indian_rupee<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0F8}").font(LUCIDE)
}

/// `infinity` from Lucide.
pub fn infinity<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1E7}").font(LUCIDE)
}

/// `info` from Lucide.
pub fn info<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0F9}").font(LUCIDE)
}

/// `inspect` from Lucide.
pub fn inspect<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E202}").font(LUCIDE)
}

/// `inspection-panel` from Lucide.
pub fn inspection_panel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E583}").font(LUCIDE)
}

/// `italic` from Lucide.
pub fn italic<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0FB}").font(LUCIDE)
}

/// `iteration-ccw` from Lucide.
pub fn iteration_ccw<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E423}").font(LUCIDE)
}

/// `iteration-cw` from Lucide.
pub fn iteration_cw<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E424}").font(LUCIDE)
}

/// `japanese-yen` from Lucide.
pub fn japanese_yen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0FC}").font(LUCIDE)
}

/// `joystick` from Lucide.
pub fn joystick<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E355}").font(LUCIDE)
}

/// `kanban` from Lucide.
pub fn kanban<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4DC}").font(LUCIDE)
}

/// `kanban-square` from Lucide.
pub fn kanban_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E170}").font(LUCIDE)
}

/// `kanban-square-dashed` from Lucide.
pub fn kanban_square_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E16C}").font(LUCIDE)
}

/// `kayak` from Lucide.
pub fn kayak<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E68F}").font(LUCIDE)
}

/// `key` from Lucide.
pub fn key<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0FD}").font(LUCIDE)
}

/// `key-round` from Lucide.
pub fn key_round<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A3}").font(LUCIDE)
}

/// `key-square` from Lucide.
pub fn key_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A4}").font(LUCIDE)
}

/// `keyboard` from Lucide.
pub fn keyboard<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E284}").font(LUCIDE)
}

/// `keyboard-music` from Lucide.
pub fn keyboard_music<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E560}").font(LUCIDE)
}

/// `keyboard-off` from Lucide.
pub fn keyboard_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5DE}").font(LUCIDE)
}

/// `lamp` from Lucide.
pub fn lamp<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2D8}").font(LUCIDE)
}

/// `lamp-ceiling` from Lucide.
pub fn lamp_ceiling<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2D9}").font(LUCIDE)
}

/// `lamp-desk` from Lucide.
pub fn lamp_desk<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2DA}").font(LUCIDE)
}

/// `lamp-floor` from Lucide.
pub fn lamp_floor<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2DB}").font(LUCIDE)
}

/// `lamp-wall-down` from Lucide.
pub fn lamp_wall_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2DC}").font(LUCIDE)
}

/// `lamp-wall-up` from Lucide.
pub fn lamp_wall_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2DD}").font(LUCIDE)
}

/// `land-plot` from Lucide.
pub fn land_plot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E528}").font(LUCIDE)
}

/// `landmark` from Lucide.
pub fn landmark<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E23A}").font(LUCIDE)
}

/// `languages` from Lucide.
pub fn languages<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0FE}").font(LUCIDE)
}

/// `laptop` from Lucide.
pub fn laptop<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1CD}").font(LUCIDE)
}

/// `laptop-2` from Lucide.
pub fn laptop_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1D8}").font(LUCIDE)
}

/// `laptop-minimal` from Lucide.
pub fn laptop_minimal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1D8}").font(LUCIDE)
}

/// `laptop-minimal-check` from Lucide.
pub fn laptop_minimal_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E632}").font(LUCIDE)
}

/// `lasso` from Lucide.
pub fn lasso<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1CE}").font(LUCIDE)
}

/// `lasso-select` from Lucide.
pub fn lasso_select<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1CF}").font(LUCIDE)
}

/// `laugh` from Lucide.
pub fn laugh<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E300}").font(LUCIDE)
}

/// `layer-arrow-down` from Lucide.
pub fn layer_arrow_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E72A}").font(LUCIDE)
}

/// `layer-arrow-up` from Lucide.
pub fn layer_arrow_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E72B}").font(LUCIDE)
}

/// `layers` from Lucide.
pub fn layers<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E529}").font(LUCIDE)
}

/// `layers-2` from Lucide.
pub fn layers_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E52A}").font(LUCIDE)
}

/// `layers-3` from Lucide.
pub fn layers_3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E529}").font(LUCIDE)
}

/// `layers-arrow-down` from Lucide.
pub fn layers_arrow_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E72C}").font(LUCIDE)
}

/// `layers-arrow-up` from Lucide.
pub fn layers_arrow_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E72D}").font(LUCIDE)
}

/// `layers-minus` from Lucide.
pub fn layers_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6EE}").font(LUCIDE)
}

/// `layers-plus` from Lucide.
pub fn layers_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6E6}").font(LUCIDE)
}

/// `layout` from Lucide.
pub fn layout<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E12C}").font(LUCIDE)
}

/// `layout-dashboard` from Lucide.
pub fn layout_dashboard<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1C1}").font(LUCIDE)
}

/// `layout-freeform` from Lucide.
pub fn layout_freeform<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E723}").font(LUCIDE)
}

/// `layout-grid` from Lucide.
pub fn layout_grid<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0FF}").font(LUCIDE)
}

/// `layout-list` from Lucide.
pub fn layout_list<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1D9}").font(LUCIDE)
}

/// `layout-panel-left` from Lucide.
pub fn layout_panel_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E470}").font(LUCIDE)
}

/// `layout-panel-top` from Lucide.
pub fn layout_panel_top<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E471}").font(LUCIDE)
}

/// `layout-template` from Lucide.
pub fn layout_template<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E207}").font(LUCIDE)
}

/// `leaf` from Lucide.
pub fn leaf<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2DE}").font(LUCIDE)
}

/// `leafy-green` from Lucide.
pub fn leafy_green<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E46F}").font(LUCIDE)
}

/// `lectern` from Lucide.
pub fn lectern<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5E9}").font(LUCIDE)
}

/// `lens-concave` from Lucide.
pub fn lens_concave<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6B7}").font(LUCIDE)
}

/// `lens-convex` from Lucide.
pub fn lens_convex<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6B8}").font(LUCIDE)
}

/// `letter-text` from Lucide.
pub fn letter_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E605}").font(LUCIDE)
}

/// `library` from Lucide.
pub fn library<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E100}").font(LUCIDE)
}

/// `library-big` from Lucide.
pub fn library_big<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E54E}").font(LUCIDE)
}

/// `library-square` from Lucide.
pub fn library_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E54F}").font(LUCIDE)
}

/// `life-buoy` from Lucide.
pub fn life_buoy<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E101}").font(LUCIDE)
}

/// `ligature` from Lucide.
pub fn ligature<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E43A}").font(LUCIDE)
}

/// `lightbulb` from Lucide.
pub fn lightbulb<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1C2}").font(LUCIDE)
}

/// `lightbulb-off` from Lucide.
pub fn lightbulb_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E208}").font(LUCIDE)
}

/// `line-chart` from Lucide.
pub fn line_chart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A5}").font(LUCIDE)
}

/// `line-dot-right-horizontal` from Lucide.
pub fn line_dot_right_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6B9}").font(LUCIDE)
}

/// `line-squiggle` from Lucide.
pub fn line_squiggle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E67A}").font(LUCIDE)
}

/// `line-style` from Lucide.
pub fn line_style<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6DA}").font(LUCIDE)
}

/// `link` from Lucide.
pub fn link<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E102}").font(LUCIDE)
}

/// `link-2` from Lucide.
pub fn link_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E103}").font(LUCIDE)
}

/// `link-2-off` from Lucide.
pub fn link_2_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E104}").font(LUCIDE)
}

/// `list` from Lucide.
pub fn list<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E106}").font(LUCIDE)
}

/// `list-check` from Lucide.
pub fn list_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5FA}").font(LUCIDE)
}

/// `list-checks` from Lucide.
pub fn list_checks<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1D0}").font(LUCIDE)
}

/// `list-chevrons-down-up` from Lucide.
pub fn list_chevrons_down_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E694}").font(LUCIDE)
}

/// `list-chevrons-up-down` from Lucide.
pub fn list_chevrons_up_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E696}").font(LUCIDE)
}

/// `list-clock` from Lucide.
pub fn list_clock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E738}").font(LUCIDE)
}

/// `list-collapse` from Lucide.
pub fn list_collapse<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E59B}").font(LUCIDE)
}

/// `list-end` from Lucide.
pub fn list_end<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2DF}").font(LUCIDE)
}

/// `list-filter` from Lucide.
pub fn list_filter<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E460}").font(LUCIDE)
}

/// `list-filter-plus` from Lucide.
pub fn list_filter_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E639}").font(LUCIDE)
}

/// `list-indent-decrease` from Lucide.
pub fn list_indent_decrease<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E107}").font(LUCIDE)
}

/// `list-indent-increase` from Lucide.
pub fn list_indent_increase<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E108}").font(LUCIDE)
}

/// `list-minus` from Lucide.
pub fn list_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E23E}").font(LUCIDE)
}

/// `list-music` from Lucide.
pub fn list_music<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2E0}").font(LUCIDE)
}

/// `list-ordered` from Lucide.
pub fn list_ordered<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1D1}").font(LUCIDE)
}

/// `list-plus` from Lucide.
pub fn list_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E23F}").font(LUCIDE)
}

/// `list-restart` from Lucide.
pub fn list_restart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E452}").font(LUCIDE)
}

/// `list-sort-ascending` from Lucide.
pub fn list_sort_ascending<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E702}").font(LUCIDE)
}

/// `list-sort-descending` from Lucide.
pub fn list_sort_descending<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E703}").font(LUCIDE)
}

/// `list-start` from Lucide.
pub fn list_start<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2E1}").font(LUCIDE)
}

/// `list-todo` from Lucide.
pub fn list_todo<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4C3}").font(LUCIDE)
}

/// `list-tree` from Lucide.
pub fn list_tree<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E408}").font(LUCIDE)
}

/// `list-video` from Lucide.
pub fn list_video<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2E2}").font(LUCIDE)
}

/// `list-x` from Lucide.
pub fn list_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E240}").font(LUCIDE)
}

/// `loader` from Lucide.
pub fn loader<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E109}").font(LUCIDE)
}

/// `loader-2` from Lucide.
pub fn loader_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E10A}").font(LUCIDE)
}

/// `loader-circle` from Lucide.
pub fn loader_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E10A}").font(LUCIDE)
}

/// `loader-pinwheel` from Lucide.
pub fn loader_pinwheel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5E6}").font(LUCIDE)
}

/// `locate` from Lucide.
pub fn locate<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1DA}").font(LUCIDE)
}

/// `locate-fixed` from Lucide.
pub fn locate_fixed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1DB}").font(LUCIDE)
}

/// `locate-off` from Lucide.
pub fn locate_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E282}").font(LUCIDE)
}

/// `location-edit` from Lucide.
pub fn location_edit<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E655}").font(LUCIDE)
}

/// `lock` from Lucide.
pub fn lock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E10B}").font(LUCIDE)
}

/// `lock-keyhole` from Lucide.
pub fn lock_keyhole<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E531}").font(LUCIDE)
}

/// `lock-keyhole-open` from Lucide.
pub fn lock_keyhole_open<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E532}").font(LUCIDE)
}

/// `lock-open` from Lucide.
pub fn lock_open<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E10C}").font(LUCIDE)
}

/// `log-in` from Lucide.
pub fn log_in<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E10D}").font(LUCIDE)
}

/// `log-out` from Lucide.
pub fn log_out<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E10E}").font(LUCIDE)
}

/// `logs` from Lucide.
pub fn logs<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5F4}").font(LUCIDE)
}

/// `lollipop` from Lucide.
pub fn lollipop<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4BD}").font(LUCIDE)
}

/// `luggage` from Lucide.
pub fn luggage<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2CA}").font(LUCIDE)
}

/// `m-square` from Lucide.
pub fn m_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E503}").font(LUCIDE)
}

/// `magnet` from Lucide.
pub fn magnet<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2B5}").font(LUCIDE)
}

/// `mail` from Lucide.
pub fn mail<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E10F}").font(LUCIDE)
}

/// `mail-badge` from Lucide.
pub fn mail_badge<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E734}").font(LUCIDE)
}

/// `mail-check` from Lucide.
pub fn mail_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E361}").font(LUCIDE)
}

/// `mail-clock` from Lucide.
pub fn mail_clock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E73E}").font(LUCIDE)
}

/// `mail-minus` from Lucide.
pub fn mail_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E362}").font(LUCIDE)
}

/// `mail-open` from Lucide.
pub fn mail_open<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E363}").font(LUCIDE)
}

/// `mail-plus` from Lucide.
pub fn mail_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E364}").font(LUCIDE)
}

/// `mail-question` from Lucide.
pub fn mail_question<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E365}").font(LUCIDE)
}

/// `mail-question-mark` from Lucide.
pub fn mail_question_mark<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E365}").font(LUCIDE)
}

/// `mail-search` from Lucide.
pub fn mail_search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E366}").font(LUCIDE)
}

/// `mail-warning` from Lucide.
pub fn mail_warning<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E367}").font(LUCIDE)
}

/// `mail-x` from Lucide.
pub fn mail_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E368}").font(LUCIDE)
}

/// `mailbox` from Lucide.
pub fn mailbox<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D4}").font(LUCIDE)
}

/// `mails` from Lucide.
pub fn mails<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E369}").font(LUCIDE)
}

/// `map` from Lucide.
pub fn map<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E110}").font(LUCIDE)
}

/// `map-minus` from Lucide.
pub fn map_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E686}").font(LUCIDE)
}

/// `map-pin` from Lucide.
pub fn map_pin<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E111}").font(LUCIDE)
}

/// `map-pin-check` from Lucide.
pub fn map_pin_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E60F}").font(LUCIDE)
}

/// `map-pin-check-inside` from Lucide.
pub fn map_pin_check_inside<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E610}").font(LUCIDE)
}

/// `map-pin-house` from Lucide.
pub fn map_pin_house<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E61C}").font(LUCIDE)
}

/// `map-pin-minus` from Lucide.
pub fn map_pin_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E611}").font(LUCIDE)
}

/// `map-pin-minus-inside` from Lucide.
pub fn map_pin_minus_inside<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E612}").font(LUCIDE)
}

/// `map-pin-off` from Lucide.
pub fn map_pin_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A6}").font(LUCIDE)
}

/// `map-pin-pen` from Lucide.
pub fn map_pin_pen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E655}").font(LUCIDE)
}

/// `map-pin-plus` from Lucide.
pub fn map_pin_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E613}").font(LUCIDE)
}

/// `map-pin-plus-inside` from Lucide.
pub fn map_pin_plus_inside<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E614}").font(LUCIDE)
}

/// `map-pin-search` from Lucide.
pub fn map_pin_search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6DE}").font(LUCIDE)
}

/// `map-pin-x` from Lucide.
pub fn map_pin_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E615}").font(LUCIDE)
}

/// `map-pin-x-inside` from Lucide.
pub fn map_pin_x_inside<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E616}").font(LUCIDE)
}

/// `map-pinned` from Lucide.
pub fn map_pinned<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E53D}").font(LUCIDE)
}

/// `map-plus` from Lucide.
pub fn map_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E63F}").font(LUCIDE)
}

/// `mars` from Lucide.
pub fn mars<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E641}").font(LUCIDE)
}

/// `mars-stroke` from Lucide.
pub fn mars_stroke<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E642}").font(LUCIDE)
}

/// `martini` from Lucide.
pub fn martini<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2E3}").font(LUCIDE)
}

/// `maximize` from Lucide.
pub fn maximize<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E112}").font(LUCIDE)
}

/// `maximize-2` from Lucide.
pub fn maximize_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E113}").font(LUCIDE)
}

/// `medal` from Lucide.
pub fn medal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E36F}").font(LUCIDE)
}

/// `megaphone` from Lucide.
pub fn megaphone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E235}").font(LUCIDE)
}

/// `megaphone-off` from Lucide.
pub fn megaphone_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E370}").font(LUCIDE)
}

/// `meh` from Lucide.
pub fn meh<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E114}").font(LUCIDE)
}

/// `memory-stick` from Lucide.
pub fn memory_stick<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E445}").font(LUCIDE)
}

/// `menu` from Lucide.
pub fn menu<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E115}").font(LUCIDE)
}

/// `menu-square` from Lucide.
pub fn menu_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E453}").font(LUCIDE)
}

/// `merge` from Lucide.
pub fn merge<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E43F}").font(LUCIDE)
}

/// `message-circle` from Lucide.
pub fn message_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E116}").font(LUCIDE)
}

/// `message-circle-check` from Lucide.
pub fn message_circle_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6BA}").font(LUCIDE)
}

/// `message-circle-code` from Lucide.
pub fn message_circle_code<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E562}").font(LUCIDE)
}

/// `message-circle-dashed` from Lucide.
pub fn message_circle_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E563}").font(LUCIDE)
}

/// `message-circle-dashed-check` from Lucide.
pub fn message_circle_dashed_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E746}").font(LUCIDE)
}

/// `message-circle-heart` from Lucide.
pub fn message_circle_heart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E564}").font(LUCIDE)
}

/// `message-circle-more` from Lucide.
pub fn message_circle_more<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E565}").font(LUCIDE)
}

/// `message-circle-off` from Lucide.
pub fn message_circle_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E566}").font(LUCIDE)
}

/// `message-circle-plus` from Lucide.
pub fn message_circle_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E567}").font(LUCIDE)
}

/// `message-circle-question` from Lucide.
pub fn message_circle_question<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E568}").font(LUCIDE)
}

/// `message-circle-question-mark` from Lucide.
pub fn message_circle_question_mark<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E568}").font(LUCIDE)
}

/// `message-circle-reply` from Lucide.
pub fn message_circle_reply<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E569}").font(LUCIDE)
}

/// `message-circle-warning` from Lucide.
pub fn message_circle_warning<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E56A}").font(LUCIDE)
}

/// `message-circle-x` from Lucide.
pub fn message_circle_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E56B}").font(LUCIDE)
}

/// `message-square` from Lucide.
pub fn message_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E117}").font(LUCIDE)
}

/// `message-square-check` from Lucide.
pub fn message_square_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6BB}").font(LUCIDE)
}

/// `message-square-code` from Lucide.
pub fn message_square_code<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E56C}").font(LUCIDE)
}

/// `message-square-dashed` from Lucide.
pub fn message_square_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E40B}").font(LUCIDE)
}

/// `message-square-diff` from Lucide.
pub fn message_square_diff<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E56D}").font(LUCIDE)
}

/// `message-square-dot` from Lucide.
pub fn message_square_dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E56E}").font(LUCIDE)
}

/// `message-square-heart` from Lucide.
pub fn message_square_heart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E56F}").font(LUCIDE)
}

/// `message-square-lock` from Lucide.
pub fn message_square_lock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E62C}").font(LUCIDE)
}

/// `message-square-more` from Lucide.
pub fn message_square_more<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E570}").font(LUCIDE)
}

/// `message-square-off` from Lucide.
pub fn message_square_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E571}").font(LUCIDE)
}

/// `message-square-plus` from Lucide.
pub fn message_square_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E40C}").font(LUCIDE)
}

/// `message-square-quote` from Lucide.
pub fn message_square_quote<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E572}").font(LUCIDE)
}

/// `message-square-reply` from Lucide.
pub fn message_square_reply<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E573}").font(LUCIDE)
}

/// `message-square-share` from Lucide.
pub fn message_square_share<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E574}").font(LUCIDE)
}

/// `message-square-text` from Lucide.
pub fn message_square_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E575}").font(LUCIDE)
}

/// `message-square-warning` from Lucide.
pub fn message_square_warning<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E576}").font(LUCIDE)
}

/// `message-square-x` from Lucide.
pub fn message_square_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E577}").font(LUCIDE)
}

/// `messages-square` from Lucide.
pub fn messages_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E40D}").font(LUCIDE)
}

/// `metronome` from Lucide.
pub fn metronome<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6BC}").font(LUCIDE)
}

/// `mic` from Lucide.
pub fn mic<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E118}").font(LUCIDE)
}

/// `mic-2` from Lucide.
pub fn mic_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E349}").font(LUCIDE)
}

/// `mic-audio-lines` from Lucide.
pub fn mic_audio_lines<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E724}").font(LUCIDE)
}

/// `mic-off` from Lucide.
pub fn mic_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E119}").font(LUCIDE)
}

/// `mic-signal` from Lucide.
pub fn mic_signal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1FA}").font(LUCIDE)
}

/// `mic-vocal` from Lucide.
pub fn mic_vocal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E349}").font(LUCIDE)
}

/// `microchip` from Lucide.
pub fn microchip<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E61A}").font(LUCIDE)
}

/// `microscope` from Lucide.
pub fn microscope<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2E4}").font(LUCIDE)
}

/// `microwave` from Lucide.
pub fn microwave<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E37A}").font(LUCIDE)
}

/// `midi-port` from Lucide.
pub fn midi_port<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E739}").font(LUCIDE)
}

/// `milestone` from Lucide.
pub fn milestone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E298}").font(LUCIDE)
}

/// `milk` from Lucide.
pub fn milk<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E399}").font(LUCIDE)
}

/// `milk-off` from Lucide.
pub fn milk_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E39A}").font(LUCIDE)
}

/// `minimize` from Lucide.
pub fn minimize<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E11A}").font(LUCIDE)
}

/// `minimize-2` from Lucide.
pub fn minimize_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E11B}").font(LUCIDE)
}

/// `minus` from Lucide.
pub fn minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E11C}").font(LUCIDE)
}

/// `minus-circle` from Lucide.
pub fn minus_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E07E}").font(LUCIDE)
}

/// `minus-square` from Lucide.
pub fn minus_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E171}").font(LUCIDE)
}

/// `mirror-rectangular` from Lucide.
pub fn mirror_rectangular<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6BD}").font(LUCIDE)
}

/// `mirror-round` from Lucide.
pub fn mirror_round<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6BE}").font(LUCIDE)
}

/// `monitor` from Lucide.
pub fn monitor<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E11D}").font(LUCIDE)
}

/// `monitor-check` from Lucide.
pub fn monitor_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E482}").font(LUCIDE)
}

/// `monitor-cloud` from Lucide.
pub fn monitor_cloud<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E699}").font(LUCIDE)
}

/// `monitor-cog` from Lucide.
pub fn monitor_cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E603}").font(LUCIDE)
}

/// `monitor-dot` from Lucide.
pub fn monitor_dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E483}").font(LUCIDE)
}

/// `monitor-down` from Lucide.
pub fn monitor_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E421}").font(LUCIDE)
}

/// `monitor-off` from Lucide.
pub fn monitor_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1DC}").font(LUCIDE)
}

/// `monitor-pause` from Lucide.
pub fn monitor_pause<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E484}").font(LUCIDE)
}

/// `monitor-play` from Lucide.
pub fn monitor_play<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E485}").font(LUCIDE)
}

/// `monitor-smartphone` from Lucide.
pub fn monitor_smartphone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3A2}").font(LUCIDE)
}

/// `monitor-speaker` from Lucide.
pub fn monitor_speaker<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E210}").font(LUCIDE)
}

/// `monitor-stop` from Lucide.
pub fn monitor_stop<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E486}").font(LUCIDE)
}

/// `monitor-up` from Lucide.
pub fn monitor_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E422}").font(LUCIDE)
}

/// `monitor-x` from Lucide.
pub fn monitor_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E487}").font(LUCIDE)
}

/// `moon` from Lucide.
pub fn moon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E11E}").font(LUCIDE)
}

/// `moon-star` from Lucide.
pub fn moon_star<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E410}").font(LUCIDE)
}

/// `mop` from Lucide.
pub fn mop<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E73B}").font(LUCIDE)
}

/// `mop-sparkles` from Lucide.
pub fn mop_sparkles<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E73A}").font(LUCIDE)
}

/// `more-horizontal` from Lucide.
pub fn more_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0B6}").font(LUCIDE)
}

/// `more-vertical` from Lucide.
pub fn more_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E0B7}").font(LUCIDE)
}

/// `mosque` from Lucide.
pub fn mosque<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E725}").font(LUCIDE)
}

/// `motorbike` from Lucide.
pub fn motorbike<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E698}").font(LUCIDE)
}

/// `mountain` from Lucide.
pub fn mountain<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E231}").font(LUCIDE)
}

/// `mountain-snow` from Lucide.
pub fn mountain_snow<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E232}").font(LUCIDE)
}

/// `mouse` from Lucide.
pub fn mouse<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E28E}").font(LUCIDE)
}

/// `mouse-left` from Lucide.
pub fn mouse_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6BF}").font(LUCIDE)
}

/// `mouse-off` from Lucide.
pub fn mouse_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5DB}").font(LUCIDE)
}

/// `mouse-pointer` from Lucide.
pub fn mouse_pointer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E11F}").font(LUCIDE)
}

/// `mouse-pointer-2` from Lucide.
pub fn mouse_pointer_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1C3}").font(LUCIDE)
}

/// `mouse-pointer-2-off` from Lucide.
pub fn mouse_pointer_2_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6A6}").font(LUCIDE)
}

/// `mouse-pointer-ban` from Lucide.
pub fn mouse_pointer_ban<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5E7}").font(LUCIDE)
}

/// `mouse-pointer-click` from Lucide.
pub fn mouse_pointer_click<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E120}").font(LUCIDE)
}

/// `mouse-pointer-square-dashed` from Lucide.
pub fn mouse_pointer_square_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E509}").font(LUCIDE)
}

/// `mouse-right` from Lucide.
pub fn mouse_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6C0}").font(LUCIDE)
}

/// `move` from Lucide.
pub fn move_icon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E121}").font(LUCIDE)
}

/// `move-3-d` from Lucide.
pub fn move_3_d<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2E5}").font(LUCIDE)
}

/// `move-3d` from Lucide.
pub fn move_3d<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2E5}").font(LUCIDE)
}

/// `move-diagonal` from Lucide.
pub fn move_diagonal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1C4}").font(LUCIDE)
}

/// `move-diagonal-2` from Lucide.
pub fn move_diagonal_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1C5}").font(LUCIDE)
}

/// `move-down` from Lucide.
pub fn move_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E48C}").font(LUCIDE)
}

/// `move-down-left` from Lucide.
pub fn move_down_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E48D}").font(LUCIDE)
}

/// `move-down-right` from Lucide.
pub fn move_down_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E48E}").font(LUCIDE)
}

/// `move-horizontal` from Lucide.
pub fn move_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1C6}").font(LUCIDE)
}

/// `move-left` from Lucide.
pub fn move_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E48F}").font(LUCIDE)
}

/// `move-right` from Lucide.
pub fn move_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E490}").font(LUCIDE)
}

/// `move-up` from Lucide.
pub fn move_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E491}").font(LUCIDE)
}

/// `move-up-left` from Lucide.
pub fn move_up_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E492}").font(LUCIDE)
}

/// `move-up-right` from Lucide.
pub fn move_up_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E493}").font(LUCIDE)
}

/// `move-vertical` from Lucide.
pub fn move_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1C7}").font(LUCIDE)
}

/// `music` from Lucide.
pub fn music<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E122}").font(LUCIDE)
}

/// `music-2` from Lucide.
pub fn music_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E34A}").font(LUCIDE)
}

/// `music-3` from Lucide.
pub fn music_3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E34B}").font(LUCIDE)
}

/// `music-4` from Lucide.
pub fn music_4<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E34C}").font(LUCIDE)
}

/// `navigation` from Lucide.
pub fn navigation<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E123}").font(LUCIDE)
}

/// `navigation-2` from Lucide.
pub fn navigation_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E124}").font(LUCIDE)
}

/// `navigation-2-off` from Lucide.
pub fn navigation_2_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A7}").font(LUCIDE)
}

/// `navigation-off` from Lucide.
pub fn navigation_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A8}").font(LUCIDE)
}

/// `network` from Lucide.
pub fn network<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E125}").font(LUCIDE)
}

/// `newspaper` from Lucide.
pub fn newspaper<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E348}").font(LUCIDE)
}

/// `nfc` from Lucide.
pub fn nfc<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3C3}").font(LUCIDE)
}

/// `non-binary` from Lucide.
pub fn non_binary<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E643}").font(LUCIDE)
}

/// `notebook` from Lucide.
pub fn notebook<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E595}").font(LUCIDE)
}

/// `notebook-pen` from Lucide.
pub fn notebook_pen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E596}").font(LUCIDE)
}

/// `notebook-tabs` from Lucide.
pub fn notebook_tabs<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E597}").font(LUCIDE)
}

/// `notebook-text` from Lucide.
pub fn notebook_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E598}").font(LUCIDE)
}

/// `notepad-text` from Lucide.
pub fn notepad_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E599}").font(LUCIDE)
}

/// `notepad-text-dashed` from Lucide.
pub fn notepad_text_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E59A}").font(LUCIDE)
}

/// `nut` from Lucide.
pub fn nut<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E39B}").font(LUCIDE)
}

/// `nut-off` from Lucide.
pub fn nut_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E39C}").font(LUCIDE)
}

/// `octagon` from Lucide.
pub fn octagon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E126}").font(LUCIDE)
}

/// `octagon-alert` from Lucide.
pub fn octagon_alert<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E127}").font(LUCIDE)
}

/// `octagon-minus` from Lucide.
pub fn octagon_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E627}").font(LUCIDE)
}

/// `octagon-pause` from Lucide.
pub fn octagon_pause<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E21B}").font(LUCIDE)
}

/// `octagon-x` from Lucide.
pub fn octagon_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E128}").font(LUCIDE)
}

/// `omega` from Lucide.
pub fn omega<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E619}").font(LUCIDE)
}

/// `option` from Lucide.
pub fn option<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1F8}").font(LUCIDE)
}

/// `orbit` from Lucide.
pub fn orbit<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3E7}").font(LUCIDE)
}

/// `origami` from Lucide.
pub fn origami<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5E3}").font(LUCIDE)
}

/// `outdent` from Lucide.
pub fn outdent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E107}").font(LUCIDE)
}

/// `package` from Lucide.
pub fn package<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E129}").font(LUCIDE)
}

/// `package-2` from Lucide.
pub fn package_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E340}").font(LUCIDE)
}

/// `package-check` from Lucide.
pub fn package_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E266}").font(LUCIDE)
}

/// `package-minus` from Lucide.
pub fn package_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E267}").font(LUCIDE)
}

/// `package-open` from Lucide.
pub fn package_open<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2CC}").font(LUCIDE)
}

/// `package-plus` from Lucide.
pub fn package_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E268}").font(LUCIDE)
}

/// `package-search` from Lucide.
pub fn package_search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E269}").font(LUCIDE)
}

/// `package-x` from Lucide.
pub fn package_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E26A}").font(LUCIDE)
}

/// `paint-bucket` from Lucide.
pub fn paint_bucket<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2E6}").font(LUCIDE)
}

/// `paint-roller` from Lucide.
pub fn paint_roller<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E59E}").font(LUCIDE)
}

/// `paintbrush` from Lucide.
pub fn paintbrush<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2E7}").font(LUCIDE)
}

/// `paintbrush-2` from Lucide.
pub fn paintbrush_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2E8}").font(LUCIDE)
}

/// `paintbrush-vertical` from Lucide.
pub fn paintbrush_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2E8}").font(LUCIDE)
}

/// `palette` from Lucide.
pub fn palette<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1DD}").font(LUCIDE)
}

/// `palmtree` from Lucide.
pub fn palmtree<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E281}").font(LUCIDE)
}

/// `panda` from Lucide.
pub fn panda<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E668}").font(LUCIDE)
}

/// `panel-bottom` from Lucide.
pub fn panel_bottom<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E42C}").font(LUCIDE)
}

/// `panel-bottom-close` from Lucide.
pub fn panel_bottom_close<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E42D}").font(LUCIDE)
}

/// `panel-bottom-dashed` from Lucide.
pub fn panel_bottom_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E42E}").font(LUCIDE)
}

/// `panel-bottom-inactive` from Lucide.
pub fn panel_bottom_inactive<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E42E}").font(LUCIDE)
}

/// `panel-bottom-open` from Lucide.
pub fn panel_bottom_open<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E42F}").font(LUCIDE)
}

/// `panel-left` from Lucide.
pub fn panel_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E12A}").font(LUCIDE)
}

/// `panel-left-close` from Lucide.
pub fn panel_left_close<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E21C}").font(LUCIDE)
}

/// `panel-left-dashed` from Lucide.
pub fn panel_left_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E430}").font(LUCIDE)
}

/// `panel-left-inactive` from Lucide.
pub fn panel_left_inactive<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E430}").font(LUCIDE)
}

/// `panel-left-open` from Lucide.
pub fn panel_left_open<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E21D}").font(LUCIDE)
}

/// `panel-left-right-dashed` from Lucide.
pub fn panel_left_right_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E692}").font(LUCIDE)
}

/// `panel-right` from Lucide.
pub fn panel_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E431}").font(LUCIDE)
}

/// `panel-right-close` from Lucide.
pub fn panel_right_close<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E432}").font(LUCIDE)
}

/// `panel-right-dashed` from Lucide.
pub fn panel_right_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E433}").font(LUCIDE)
}

/// `panel-right-inactive` from Lucide.
pub fn panel_right_inactive<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E433}").font(LUCIDE)
}

/// `panel-right-open` from Lucide.
pub fn panel_right_open<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E434}").font(LUCIDE)
}

/// `panel-top` from Lucide.
pub fn panel_top<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E435}").font(LUCIDE)
}

/// `panel-top-bottom-dashed` from Lucide.
pub fn panel_top_bottom_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E693}").font(LUCIDE)
}

/// `panel-top-close` from Lucide.
pub fn panel_top_close<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E436}").font(LUCIDE)
}

/// `panel-top-dashed` from Lucide.
pub fn panel_top_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E437}").font(LUCIDE)
}

/// `panel-top-inactive` from Lucide.
pub fn panel_top_inactive<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E437}").font(LUCIDE)
}

/// `panel-top-open` from Lucide.
pub fn panel_top_open<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E438}").font(LUCIDE)
}

/// `panels-left-bottom` from Lucide.
pub fn panels_left_bottom<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E12B}").font(LUCIDE)
}

/// `panels-left-right` from Lucide.
pub fn panels_left_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E099}").font(LUCIDE)
}

/// `panels-right-bottom` from Lucide.
pub fn panels_right_bottom<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E588}").font(LUCIDE)
}

/// `panels-top-bottom` from Lucide.
pub fn panels_top_bottom<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E58A}").font(LUCIDE)
}

/// `panels-top-left` from Lucide.
pub fn panels_top_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E12C}").font(LUCIDE)
}

/// `paper-bag` from Lucide.
pub fn paper_bag<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E71D}").font(LUCIDE)
}

/// `paperclip` from Lucide.
pub fn paperclip<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E12D}").font(LUCIDE)
}

/// `parasol` from Lucide.
pub fn parasol<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6FD}").font(LUCIDE)
}

/// `parentheses` from Lucide.
pub fn parentheses<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E444}").font(LUCIDE)
}

/// `parking-circle` from Lucide.
pub fn parking_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3C9}").font(LUCIDE)
}

/// `parking-circle-off` from Lucide.
pub fn parking_circle_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3CA}").font(LUCIDE)
}

/// `parking-meter` from Lucide.
pub fn parking_meter<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E500}").font(LUCIDE)
}

/// `parking-square` from Lucide.
pub fn parking_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3CB}").font(LUCIDE)
}

/// `parking-square-off` from Lucide.
pub fn parking_square_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3CC}").font(LUCIDE)
}

/// `party-popper` from Lucide.
pub fn party_popper<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E343}").font(LUCIDE)
}

/// `pause` from Lucide.
pub fn pause<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E12E}").font(LUCIDE)
}

/// `pause-circle` from Lucide.
pub fn pause_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E07F}").font(LUCIDE)
}

/// `pause-octagon` from Lucide.
pub fn pause_octagon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E21B}").font(LUCIDE)
}

/// `paw-print` from Lucide.
pub fn paw_print<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4F5}").font(LUCIDE)
}

/// `pc-case` from Lucide.
pub fn pc_case<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E446}").font(LUCIDE)
}

/// `pen` from Lucide.
pub fn pen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E12F}").font(LUCIDE)
}

/// `pen-box` from Lucide.
pub fn pen_box<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E172}").font(LUCIDE)
}

/// `pen-line` from Lucide.
pub fn pen_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E130}").font(LUCIDE)
}

/// `pen-off` from Lucide.
pub fn pen_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5EE}").font(LUCIDE)
}

/// `pen-square` from Lucide.
pub fn pen_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E172}").font(LUCIDE)
}

/// `pen-tool` from Lucide.
pub fn pen_tool<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E131}").font(LUCIDE)
}

/// `pencil` from Lucide.
pub fn pencil<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1F9}").font(LUCIDE)
}

/// `pencil-line` from Lucide.
pub fn pencil_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4F0}").font(LUCIDE)
}

/// `pencil-off` from Lucide.
pub fn pencil_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5EF}").font(LUCIDE)
}

/// `pencil-ruler` from Lucide.
pub fn pencil_ruler<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4F1}").font(LUCIDE)
}

/// `pencil-sparkles` from Lucide.
pub fn pencil_sparkles<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E70F}").font(LUCIDE)
}

/// `pentagon` from Lucide.
pub fn pentagon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E52B}").font(LUCIDE)
}

/// `percent` from Lucide.
pub fn percent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E132}").font(LUCIDE)
}

/// `percent-circle` from Lucide.
pub fn percent_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E51A}").font(LUCIDE)
}

/// `percent-diamond` from Lucide.
pub fn percent_diamond<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E51B}").font(LUCIDE)
}

/// `percent-square` from Lucide.
pub fn percent_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E51C}").font(LUCIDE)
}

/// `person-standing` from Lucide.
pub fn person_standing<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E21E}").font(LUCIDE)
}

/// `phi` from Lucide.
pub fn phi<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E71C}").font(LUCIDE)
}

/// `philippine-peso` from Lucide.
pub fn philippine_peso<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E604}").font(LUCIDE)
}

/// `phone` from Lucide.
pub fn phone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E133}").font(LUCIDE)
}

/// `phone-call` from Lucide.
pub fn phone_call<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E134}").font(LUCIDE)
}

/// `phone-forwarded` from Lucide.
pub fn phone_forwarded<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E135}").font(LUCIDE)
}

/// `phone-incoming` from Lucide.
pub fn phone_incoming<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E136}").font(LUCIDE)
}

/// `phone-missed` from Lucide.
pub fn phone_missed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E137}").font(LUCIDE)
}

/// `phone-off` from Lucide.
pub fn phone_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E138}").font(LUCIDE)
}

/// `phone-outgoing` from Lucide.
pub fn phone_outgoing<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E139}").font(LUCIDE)
}

/// `pi` from Lucide.
pub fn pi<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E472}").font(LUCIDE)
}

/// `pi-square` from Lucide.
pub fn pi_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E488}").font(LUCIDE)
}

/// `piano` from Lucide.
pub fn piano<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E561}").font(LUCIDE)
}

/// `pickaxe` from Lucide.
pub fn pickaxe<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5C6}").font(LUCIDE)
}

/// `picture-in-picture` from Lucide.
pub fn picture_in_picture<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3AE}").font(LUCIDE)
}

/// `picture-in-picture-2` from Lucide.
pub fn picture_in_picture_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3AF}").font(LUCIDE)
}

/// `pie-chart` from Lucide.
pub fn pie_chart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E06B}").font(LUCIDE)
}

/// `piggy-bank` from Lucide.
pub fn piggy_bank<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E13A}").font(LUCIDE)
}

/// `pilcrow` from Lucide.
pub fn pilcrow<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3A3}").font(LUCIDE)
}

/// `pilcrow-left` from Lucide.
pub fn pilcrow_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5DC}").font(LUCIDE)
}

/// `pilcrow-right` from Lucide.
pub fn pilcrow_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5DD}").font(LUCIDE)
}

/// `pilcrow-square` from Lucide.
pub fn pilcrow_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E48B}").font(LUCIDE)
}

/// `pill` from Lucide.
pub fn pill<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3BD}").font(LUCIDE)
}

/// `pill-bottle` from Lucide.
pub fn pill_bottle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5EA}").font(LUCIDE)
}

/// `pin` from Lucide.
pub fn pin<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E259}").font(LUCIDE)
}

/// `pin-off` from Lucide.
pub fn pin_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2B6}").font(LUCIDE)
}

/// `pipette` from Lucide.
pub fn pipette<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E13B}").font(LUCIDE)
}

/// `pizza` from Lucide.
pub fn pizza<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E354}").font(LUCIDE)
}

/// `plane` from Lucide.
pub fn plane<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1DE}").font(LUCIDE)
}

/// `plane-landing` from Lucide.
pub fn plane_landing<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3CD}").font(LUCIDE)
}

/// `plane-takeoff` from Lucide.
pub fn plane_takeoff<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3CE}").font(LUCIDE)
}

/// `play` from Lucide.
pub fn play<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E13C}").font(LUCIDE)
}

/// `play-circle` from Lucide.
pub fn play_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E080}").font(LUCIDE)
}

/// `play-off` from Lucide.
pub fn play_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6FE}").font(LUCIDE)
}

/// `play-square` from Lucide.
pub fn play_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E481}").font(LUCIDE)
}

/// `playing-card` from Lucide.
pub fn playing_card<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E747}").font(LUCIDE)
}

/// `playing-cards` from Lucide.
pub fn playing_cards<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E749}").font(LUCIDE)
}

/// `playing-cards-fan` from Lucide.
pub fn playing_cards_fan<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E748}").font(LUCIDE)
}

/// `plug` from Lucide.
pub fn plug<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E37F}").font(LUCIDE)
}

/// `plug-2` from Lucide.
pub fn plug_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E380}").font(LUCIDE)
}

/// `plug-zap` from Lucide.
pub fn plug_zap<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E45C}").font(LUCIDE)
}

/// `plug-zap-2` from Lucide.
pub fn plug_zap_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E45C}").font(LUCIDE)
}

/// `plus` from Lucide.
pub fn plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E13D}").font(LUCIDE)
}

/// `plus-circle` from Lucide.
pub fn plus_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E081}").font(LUCIDE)
}

/// `plus-square` from Lucide.
pub fn plus_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E173}").font(LUCIDE)
}

/// `pocket-knife` from Lucide.
pub fn pocket_knife<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A0}").font(LUCIDE)
}

/// `podcast` from Lucide.
pub fn podcast<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1FA}").font(LUCIDE)
}

/// `podium` from Lucide.
pub fn podium<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E704}").font(LUCIDE)
}

/// `pointer` from Lucide.
pub fn pointer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1E8}").font(LUCIDE)
}

/// `pointer-off` from Lucide.
pub fn pointer_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E57F}").font(LUCIDE)
}

/// `popcorn` from Lucide.
pub fn popcorn<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4BE}").font(LUCIDE)
}

/// `popsicle` from Lucide.
pub fn popsicle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4BF}").font(LUCIDE)
}

/// `pound-sterling` from Lucide.
pub fn pound_sterling<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E13F}").font(LUCIDE)
}

/// `power` from Lucide.
pub fn power<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E140}").font(LUCIDE)
}

/// `power-circle` from Lucide.
pub fn power_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E550}").font(LUCIDE)
}

/// `power-off` from Lucide.
pub fn power_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E209}").font(LUCIDE)
}

/// `power-square` from Lucide.
pub fn power_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E551}").font(LUCIDE)
}

/// `presentation` from Lucide.
pub fn presentation<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4AE}").font(LUCIDE)
}

/// `printer` from Lucide.
pub fn printer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E141}").font(LUCIDE)
}

/// `printer-check` from Lucide.
pub fn printer_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5F5}").font(LUCIDE)
}

/// `printer-x` from Lucide.
pub fn printer_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6C1}").font(LUCIDE)
}

/// `projector` from Lucide.
pub fn projector<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4AF}").font(LUCIDE)
}

/// `proportions` from Lucide.
pub fn proportions<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5CF}").font(LUCIDE)
}

/// `puzzle` from Lucide.
pub fn puzzle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E29C}").font(LUCIDE)
}

/// `pyramid` from Lucide.
pub fn pyramid<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E52C}").font(LUCIDE)
}

/// `qr-code` from Lucide.
pub fn qr_code<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1DF}").font(LUCIDE)
}

/// `quote` from Lucide.
pub fn quote<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E239}").font(LUCIDE)
}

/// `rabbit` from Lucide.
pub fn rabbit<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4F6}").font(LUCIDE)
}

/// `radar` from Lucide.
pub fn radar<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E497}").font(LUCIDE)
}

/// `radiation` from Lucide.
pub fn radiation<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E442}").font(LUCIDE)
}

/// `radical` from Lucide.
pub fn radical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5C2}").font(LUCIDE)
}

/// `radio` from Lucide.
pub fn radio<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E142}").font(LUCIDE)
}

/// `radio-off` from Lucide.
pub fn radio_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6DD}").font(LUCIDE)
}

/// `radio-receiver` from Lucide.
pub fn radio_receiver<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1FB}").font(LUCIDE)
}

/// `radio-tower` from Lucide.
pub fn radio_tower<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E404}").font(LUCIDE)
}

/// `radius` from Lucide.
pub fn radius<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E52D}").font(LUCIDE)
}

/// `rainbow` from Lucide.
pub fn rainbow<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4C2}").font(LUCIDE)
}

/// `rat` from Lucide.
pub fn rat<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3EB}").font(LUCIDE)
}

/// `ratio` from Lucide.
pub fn ratio<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4E8}").font(LUCIDE)
}

/// `receipt` from Lucide.
pub fn receipt<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D3}").font(LUCIDE)
}

/// `receipt-cent` from Lucide.
pub fn receipt_cent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A5}").font(LUCIDE)
}

/// `receipt-euro` from Lucide.
pub fn receipt_euro<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A6}").font(LUCIDE)
}

/// `receipt-indian-rupee` from Lucide.
pub fn receipt_indian_rupee<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A7}").font(LUCIDE)
}

/// `receipt-japanese-yen` from Lucide.
pub fn receipt_japanese_yen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A8}").font(LUCIDE)
}

/// `receipt-pound-sterling` from Lucide.
pub fn receipt_pound_sterling<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A9}").font(LUCIDE)
}

/// `receipt-russian-ruble` from Lucide.
pub fn receipt_russian_ruble<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5AA}").font(LUCIDE)
}

/// `receipt-swiss-franc` from Lucide.
pub fn receipt_swiss_franc<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5AB}").font(LUCIDE)
}

/// `receipt-text` from Lucide.
pub fn receipt_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5AC}").font(LUCIDE)
}

/// `receipt-turkish-lira` from Lucide.
pub fn receipt_turkish_lira<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E67F}").font(LUCIDE)
}

/// `rectangle-circle` from Lucide.
pub fn rectangle_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E673}").font(LUCIDE)
}

/// `rectangle-ellipsis` from Lucide.
pub fn rectangle_ellipsis<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E21F}").font(LUCIDE)
}

/// `rectangle-goggles` from Lucide.
pub fn rectangle_goggles<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E656}").font(LUCIDE)
}

/// `rectangle-horizontal` from Lucide.
pub fn rectangle_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E376}").font(LUCIDE)
}

/// `rectangle-vertical` from Lucide.
pub fn rectangle_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E377}").font(LUCIDE)
}

/// `recycle` from Lucide.
pub fn recycle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2E9}").font(LUCIDE)
}

/// `redo` from Lucide.
pub fn redo<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E143}").font(LUCIDE)
}

/// `redo-2` from Lucide.
pub fn redo_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A0}").font(LUCIDE)
}

/// `redo-dot` from Lucide.
pub fn redo_dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E450}").font(LUCIDE)
}

/// `refresh-ccw` from Lucide.
pub fn refresh_ccw<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E144}").font(LUCIDE)
}

/// `refresh-ccw-dot` from Lucide.
pub fn refresh_ccw_dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B2}").font(LUCIDE)
}

/// `refresh-cw` from Lucide.
pub fn refresh_cw<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E145}").font(LUCIDE)
}

/// `refresh-cw-off` from Lucide.
pub fn refresh_cw_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E498}").font(LUCIDE)
}

/// `refrigerator` from Lucide.
pub fn refrigerator<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E37B}").font(LUCIDE)
}

/// `regex` from Lucide.
pub fn regex<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1FC}").font(LUCIDE)
}

/// `remove-formatting` from Lucide.
pub fn remove_formatting<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B3}").font(LUCIDE)
}

/// `repeat` from Lucide.
pub fn repeat<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E146}").font(LUCIDE)
}

/// `repeat-1` from Lucide.
pub fn repeat_1<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1FD}").font(LUCIDE)
}

/// `repeat-2` from Lucide.
pub fn repeat_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E411}").font(LUCIDE)
}

/// `repeat-off` from Lucide.
pub fn repeat_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6F3}").font(LUCIDE)
}

/// `replace` from Lucide.
pub fn replace<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3DB}").font(LUCIDE)
}

/// `replace-all` from Lucide.
pub fn replace_all<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3DC}").font(LUCIDE)
}

/// `reply` from Lucide.
pub fn reply<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E22A}").font(LUCIDE)
}

/// `reply-all` from Lucide.
pub fn reply_all<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E22B}").font(LUCIDE)
}

/// `rewind` from Lucide.
pub fn rewind<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E147}").font(LUCIDE)
}

/// `ribbon` from Lucide.
pub fn ribbon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E558}").font(LUCIDE)
}

/// `road` from Lucide.
pub fn road<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6D9}").font(LUCIDE)
}

/// `robot-arm` from Lucide.
pub fn robot_arm<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E740}").font(LUCIDE)
}

/// `robot-vacuum` from Lucide.
pub fn robot_vacuum<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E74A}").font(LUCIDE)
}

/// `rocket` from Lucide.
pub fn rocket<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E286}").font(LUCIDE)
}

/// `rocking-chair` from Lucide.
pub fn rocking_chair<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E233}").font(LUCIDE)
}

/// `roller-coaster` from Lucide.
pub fn roller_coaster<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E480}").font(LUCIDE)
}

/// `rose` from Lucide.
pub fn rose<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E691}").font(LUCIDE)
}

/// `rotate-3-d` from Lucide.
pub fn rotate_3_d<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2EA}").font(LUCIDE)
}

/// `rotate-3d` from Lucide.
pub fn rotate_3d<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2EA}").font(LUCIDE)
}

/// `rotate-ccw` from Lucide.
pub fn rotate_ccw<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E148}").font(LUCIDE)
}

/// `rotate-ccw-clock` from Lucide.
pub fn rotate_ccw_clock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1F5}").font(LUCIDE)
}

/// `rotate-ccw-key` from Lucide.
pub fn rotate_ccw_key<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E650}").font(LUCIDE)
}

/// `rotate-ccw-square` from Lucide.
pub fn rotate_ccw_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5D0}").font(LUCIDE)
}

/// `rotate-cw` from Lucide.
pub fn rotate_cw<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E149}").font(LUCIDE)
}

/// `rotate-cw-fading-clock` from Lucide.
pub fn rotate_cw_fading_clock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E726}").font(LUCIDE)
}

/// `rotate-cw-square` from Lucide.
pub fn rotate_cw_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5D1}").font(LUCIDE)
}

/// `route` from Lucide.
pub fn route<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E53E}").font(LUCIDE)
}

/// `route-off` from Lucide.
pub fn route_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E53F}").font(LUCIDE)
}

/// `router` from Lucide.
pub fn router<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3BF}").font(LUCIDE)
}

/// `rows` from Lucide.
pub fn rows<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E439}").font(LUCIDE)
}

/// `rows-2` from Lucide.
pub fn rows_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E439}").font(LUCIDE)
}

/// `rows-3` from Lucide.
pub fn rows_3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E58A}").font(LUCIDE)
}

/// `rows-4` from Lucide.
pub fn rows_4<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E58B}").font(LUCIDE)
}

/// `rss` from Lucide.
pub fn rss<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E14A}").font(LUCIDE)
}

/// `ruler` from Lucide.
pub fn ruler<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E14B}").font(LUCIDE)
}

/// `ruler-dimension-line` from Lucide.
pub fn ruler_dimension_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E662}").font(LUCIDE)
}

/// `russian-ruble` from Lucide.
pub fn russian_ruble<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E14C}").font(LUCIDE)
}

/// `sailboat` from Lucide.
pub fn sailboat<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E37E}").font(LUCIDE)
}

/// `salad` from Lucide.
pub fn salad<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3A8}").font(LUCIDE)
}

/// `sandwich` from Lucide.
pub fn sandwich<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3A9}").font(LUCIDE)
}

/// `satellite` from Lucide.
pub fn satellite<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E447}").font(LUCIDE)
}

/// `satellite-dish` from Lucide.
pub fn satellite_dish<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E448}").font(LUCIDE)
}

/// `saudi-riyal` from Lucide.
pub fn saudi_riyal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E64B}").font(LUCIDE)
}

/// `save` from Lucide.
pub fn save<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E14D}").font(LUCIDE)
}

/// `save-all` from Lucide.
pub fn save_all<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E40F}").font(LUCIDE)
}

/// `save-check` from Lucide.
pub fn save_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E710}").font(LUCIDE)
}

/// `save-off` from Lucide.
pub fn save_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5F3}").font(LUCIDE)
}

/// `save-pen` from Lucide.
pub fn save_pen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E705}").font(LUCIDE)
}

/// `save-plus` from Lucide.
pub fn save_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E706}").font(LUCIDE)
}

/// `scale` from Lucide.
pub fn scale<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E212}").font(LUCIDE)
}

/// `scale-3-d` from Lucide.
pub fn scale_3_d<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2EB}").font(LUCIDE)
}

/// `scale-3d` from Lucide.
pub fn scale_3d<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2EB}").font(LUCIDE)
}

/// `scaling` from Lucide.
pub fn scaling<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2EC}").font(LUCIDE)
}

/// `scan` from Lucide.
pub fn scan<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E257}").font(LUCIDE)
}

/// `scan-barcode` from Lucide.
pub fn scan_barcode<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E535}").font(LUCIDE)
}

/// `scan-box` from Lucide.
pub fn scan_box<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E720}").font(LUCIDE)
}

/// `scan-eye` from Lucide.
pub fn scan_eye<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E536}").font(LUCIDE)
}

/// `scan-face` from Lucide.
pub fn scan_face<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E371}").font(LUCIDE)
}

/// `scan-heart` from Lucide.
pub fn scan_heart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E63A}").font(LUCIDE)
}

/// `scan-line` from Lucide.
pub fn scan_line<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E258}").font(LUCIDE)
}

/// `scan-qr-code` from Lucide.
pub fn scan_qr_code<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5F6}").font(LUCIDE)
}

/// `scan-search` from Lucide.
pub fn scan_search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E537}").font(LUCIDE)
}

/// `scan-square` from Lucide.
pub fn scan_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E727}").font(LUCIDE)
}

/// `scan-text` from Lucide.
pub fn scan_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E538}").font(LUCIDE)
}

/// `scatter-chart` from Lucide.
pub fn scatter_chart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E48A}").font(LUCIDE)
}

/// `school` from Lucide.
pub fn school<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3E3}").font(LUCIDE)
}

/// `school-2` from Lucide.
pub fn school_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3E5}").font(LUCIDE)
}

/// `scissors` from Lucide.
pub fn scissors<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E14E}").font(LUCIDE)
}

/// `scissors-line-dashed` from Lucide.
pub fn scissors_line_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4E9}").font(LUCIDE)
}

/// `scissors-square` from Lucide.
pub fn scissors_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4EC}").font(LUCIDE)
}

/// `scissors-square-dashed-bottom` from Lucide.
pub fn scissors_square_dashed_bottom<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4EB}").font(LUCIDE)
}

/// `scooter` from Lucide.
pub fn scooter<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6AC}").font(LUCIDE)
}

/// `screen-share` from Lucide.
pub fn screen_share<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E14F}").font(LUCIDE)
}

/// `screen-share-off` from Lucide.
pub fn screen_share_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E150}").font(LUCIDE)
}

/// `scroll` from Lucide.
pub fn scroll<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2ED}").font(LUCIDE)
}

/// `scroll-text` from Lucide.
pub fn scroll_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E45F}").font(LUCIDE)
}

/// `search` from Lucide.
pub fn search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E151}").font(LUCIDE)
}

/// `search-alert` from Lucide.
pub fn search_alert<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6E7}").font(LUCIDE)
}

/// `search-check` from Lucide.
pub fn search_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4AA}").font(LUCIDE)
}

/// `search-code` from Lucide.
pub fn search_code<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4AB}").font(LUCIDE)
}

/// `search-slash` from Lucide.
pub fn search_slash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4AC}").font(LUCIDE)
}

/// `search-x` from Lucide.
pub fn search_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4AD}").font(LUCIDE)
}

/// `section` from Lucide.
pub fn section<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5E8}").font(LUCIDE)
}

/// `send` from Lucide.
pub fn send<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E152}").font(LUCIDE)
}

/// `send-horizonal` from Lucide.
pub fn send_horizonal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4F2}").font(LUCIDE)
}

/// `send-horizontal` from Lucide.
pub fn send_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4F2}").font(LUCIDE)
}

/// `send-to-back` from Lucide.
pub fn send_to_back<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4F3}").font(LUCIDE)
}

/// `separator-horizontal` from Lucide.
pub fn separator_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1C8}").font(LUCIDE)
}

/// `separator-vertical` from Lucide.
pub fn separator_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1C9}").font(LUCIDE)
}

/// `server` from Lucide.
pub fn server<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E153}").font(LUCIDE)
}

/// `server-cog` from Lucide.
pub fn server_cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E341}").font(LUCIDE)
}

/// `server-crash` from Lucide.
pub fn server_crash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1E9}").font(LUCIDE)
}

/// `server-off` from Lucide.
pub fn server_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1EA}").font(LUCIDE)
}

/// `server-plus` from Lucide.
pub fn server_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E721}").font(LUCIDE)
}

/// `settings` from Lucide.
pub fn settings<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E154}").font(LUCIDE)
}

/// `settings-2` from Lucide.
pub fn settings_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E245}").font(LUCIDE)
}

/// `shapes` from Lucide.
pub fn shapes<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B3}").font(LUCIDE)
}

/// `share` from Lucide.
pub fn share<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E155}").font(LUCIDE)
}

/// `share-2` from Lucide.
pub fn share_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E156}").font(LUCIDE)
}

/// `sheet` from Lucide.
pub fn sheet<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E157}").font(LUCIDE)
}

/// `shell` from Lucide.
pub fn shell<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4F7}").font(LUCIDE)
}

/// `shelving-unit` from Lucide.
pub fn shelving_unit<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6C2}").font(LUCIDE)
}

/// `shield` from Lucide.
pub fn shield<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E158}").font(LUCIDE)
}

/// `shield-alert` from Lucide.
pub fn shield_alert<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1FE}").font(LUCIDE)
}

/// `shield-ban` from Lucide.
pub fn shield_ban<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E159}").font(LUCIDE)
}

/// `shield-check` from Lucide.
pub fn shield_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1FF}").font(LUCIDE)
}

/// `shield-close` from Lucide.
pub fn shield_close<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E200}").font(LUCIDE)
}

/// `shield-cog` from Lucide.
pub fn shield_cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6E9}").font(LUCIDE)
}

/// `shield-cog-corner` from Lucide.
pub fn shield_cog_corner<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6E8}").font(LUCIDE)
}

/// `shield-ellipsis` from Lucide.
pub fn shield_ellipsis<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E516}").font(LUCIDE)
}

/// `shield-half` from Lucide.
pub fn shield_half<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E517}").font(LUCIDE)
}

/// `shield-keyhole` from Lucide.
pub fn shield_keyhole<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E722}").font(LUCIDE)
}

/// `shield-lock` from Lucide.
pub fn shield_lock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E731}").font(LUCIDE)
}

/// `shield-minus` from Lucide.
pub fn shield_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E518}").font(LUCIDE)
}

/// `shield-off` from Lucide.
pub fn shield_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E15A}").font(LUCIDE)
}

/// `shield-plus` from Lucide.
pub fn shield_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E519}").font(LUCIDE)
}

/// `shield-question` from Lucide.
pub fn shield_question<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E40E}").font(LUCIDE)
}

/// `shield-question-mark` from Lucide.
pub fn shield_question_mark<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E40E}").font(LUCIDE)
}

/// `shield-user` from Lucide.
pub fn shield_user<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E647}").font(LUCIDE)
}

/// `shield-x` from Lucide.
pub fn shield_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E200}").font(LUCIDE)
}

/// `ship` from Lucide.
pub fn ship<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3BA}").font(LUCIDE)
}

/// `ship-cargo` from Lucide.
pub fn ship_cargo<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E741}").font(LUCIDE)
}

/// `ship-wheel` from Lucide.
pub fn ship_wheel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E502}").font(LUCIDE)
}

/// `shirt` from Lucide.
pub fn shirt<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1CA}").font(LUCIDE)
}

/// `shopping-bag` from Lucide.
pub fn shopping_bag<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E15B}").font(LUCIDE)
}

/// `shopping-basket` from Lucide.
pub fn shopping_basket<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4EA}").font(LUCIDE)
}

/// `shopping-cart` from Lucide.
pub fn shopping_cart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E15C}").font(LUCIDE)
}

/// `shovel` from Lucide.
pub fn shovel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E15D}").font(LUCIDE)
}

/// `shower-head` from Lucide.
pub fn shower_head<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E37C}").font(LUCIDE)
}

/// `shredder` from Lucide.
pub fn shredder<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E65B}").font(LUCIDE)
}

/// `shrimp` from Lucide.
pub fn shrimp<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E649}").font(LUCIDE)
}

/// `shrink` from Lucide.
pub fn shrink<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E220}").font(LUCIDE)
}

/// `shrub` from Lucide.
pub fn shrub<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2EE}").font(LUCIDE)
}

/// `shuffle` from Lucide.
pub fn shuffle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E15E}").font(LUCIDE)
}

/// `sidebar` from Lucide.
pub fn sidebar<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E12A}").font(LUCIDE)
}

/// `sidebar-close` from Lucide.
pub fn sidebar_close<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E21C}").font(LUCIDE)
}

/// `sidebar-open` from Lucide.
pub fn sidebar_open<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E21D}").font(LUCIDE)
}

/// `sigma` from Lucide.
pub fn sigma<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E201}").font(LUCIDE)
}

/// `sigma-square` from Lucide.
pub fn sigma_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E489}").font(LUCIDE)
}

/// `signal` from Lucide.
pub fn signal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E25F}").font(LUCIDE)
}

/// `signal-high` from Lucide.
pub fn signal_high<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E260}").font(LUCIDE)
}

/// `signal-low` from Lucide.
pub fn signal_low<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E261}").font(LUCIDE)
}

/// `signal-medium` from Lucide.
pub fn signal_medium<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E262}").font(LUCIDE)
}

/// `signal-zero` from Lucide.
pub fn signal_zero<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E263}").font(LUCIDE)
}

/// `signature` from Lucide.
pub fn signature<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5F2}").font(LUCIDE)
}

/// `signpost` from Lucide.
pub fn signpost<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E540}").font(LUCIDE)
}

/// `signpost-big` from Lucide.
pub fn signpost_big<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E541}").font(LUCIDE)
}

/// `siren` from Lucide.
pub fn siren<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2EF}").font(LUCIDE)
}

/// `skip-back` from Lucide.
pub fn skip_back<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E15F}").font(LUCIDE)
}

/// `skip-forward` from Lucide.
pub fn skip_forward<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E160}").font(LUCIDE)
}

/// `skull` from Lucide.
pub fn skull<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E221}").font(LUCIDE)
}

/// `slash` from Lucide.
pub fn slash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E51D}").font(LUCIDE)
}

/// `slash-square` from Lucide.
pub fn slash_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E174}").font(LUCIDE)
}

/// `slice` from Lucide.
pub fn slice<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2F0}").font(LUCIDE)
}

/// `sliders` from Lucide.
pub fn sliders<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E162}").font(LUCIDE)
}

/// `sliders-horizontal` from Lucide.
pub fn sliders_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E29A}").font(LUCIDE)
}

/// `sliders-vertical` from Lucide.
pub fn sliders_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E162}").font(LUCIDE)
}

/// `smartphone` from Lucide.
pub fn smartphone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E163}").font(LUCIDE)
}

/// `smartphone-charging` from Lucide.
pub fn smartphone_charging<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E22E}").font(LUCIDE)
}

/// `smartphone-nfc` from Lucide.
pub fn smartphone_nfc<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3C4}").font(LUCIDE)
}

/// `smile` from Lucide.
pub fn smile<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E164}").font(LUCIDE)
}

/// `smile-plus` from Lucide.
pub fn smile_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E301}").font(LUCIDE)
}

/// `snail` from Lucide.
pub fn snail<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4F8}").font(LUCIDE)
}

/// `snowflake` from Lucide.
pub fn snowflake<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E165}").font(LUCIDE)
}

/// `soap-dispenser-droplet` from Lucide.
pub fn soap_dispenser_droplet<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E669}").font(LUCIDE)
}

/// `sofa` from Lucide.
pub fn sofa<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2C4}").font(LUCIDE)
}

/// `solar-panel` from Lucide.
pub fn solar_panel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E69F}").font(LUCIDE)
}

/// `sort-asc` from Lucide.
pub fn sort_asc<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E04C}").font(LUCIDE)
}

/// `sort-desc` from Lucide.
pub fn sort_desc<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E047}").font(LUCIDE)
}

/// `soup` from Lucide.
pub fn soup<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3AA}").font(LUCIDE)
}

/// `space` from Lucide.
pub fn space<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3DD}").font(LUCIDE)
}

/// `spade` from Lucide.
pub fn spade<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E499}").font(LUCIDE)
}

/// `sparkle` from Lucide.
pub fn sparkle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E47E}").font(LUCIDE)
}

/// `sparkles` from Lucide.
pub fn sparkles<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E412}").font(LUCIDE)
}

/// `speaker` from Lucide.
pub fn speaker<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E166}").font(LUCIDE)
}

/// `speech` from Lucide.
pub fn speech<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E51E}").font(LUCIDE)
}

/// `spell-check` from Lucide.
pub fn spell_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E49A}").font(LUCIDE)
}

/// `spell-check-2` from Lucide.
pub fn spell_check_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E49B}").font(LUCIDE)
}

/// `spline` from Lucide.
pub fn spline<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E38B}").font(LUCIDE)
}

/// `spline-pointer` from Lucide.
pub fn spline_pointer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E64F}").font(LUCIDE)
}

/// `split` from Lucide.
pub fn split<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E440}").font(LUCIDE)
}

/// `split-square-horizontal` from Lucide.
pub fn split_square_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B6}").font(LUCIDE)
}

/// `split-square-vertical` from Lucide.
pub fn split_square_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B7}").font(LUCIDE)
}

/// `spool` from Lucide.
pub fn spool<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E677}").font(LUCIDE)
}

/// `sport-shoe` from Lucide.
pub fn sport_shoe<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6DB}").font(LUCIDE)
}

/// `spotlight` from Lucide.
pub fn spotlight<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E682}").font(LUCIDE)
}

/// `spray-can` from Lucide.
pub fn spray_can<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E495}").font(LUCIDE)
}

/// `sprout` from Lucide.
pub fn sprout<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1EB}").font(LUCIDE)
}

/// `square` from Lucide.
pub fn square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E167}").font(LUCIDE)
}

/// `square-activity` from Lucide.
pub fn square_activity<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B4}").font(LUCIDE)
}

/// `square-arrow-down` from Lucide.
pub fn square_arrow_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E427}").font(LUCIDE)
}

/// `square-arrow-down-left` from Lucide.
pub fn square_arrow_down_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B5}").font(LUCIDE)
}

/// `square-arrow-down-right` from Lucide.
pub fn square_arrow_down_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B6}").font(LUCIDE)
}

/// `square-arrow-left` from Lucide.
pub fn square_arrow_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E428}").font(LUCIDE)
}

/// `square-arrow-out-down-left` from Lucide.
pub fn square_arrow_out_down_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A1}").font(LUCIDE)
}

/// `square-arrow-out-down-right` from Lucide.
pub fn square_arrow_out_down_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A2}").font(LUCIDE)
}

/// `square-arrow-out-up-left` from Lucide.
pub fn square_arrow_out_up_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A3}").font(LUCIDE)
}

/// `square-arrow-out-up-right` from Lucide.
pub fn square_arrow_out_up_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5A4}").font(LUCIDE)
}

/// `square-arrow-right` from Lucide.
pub fn square_arrow_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E429}").font(LUCIDE)
}

/// `square-arrow-right-enter` from Lucide.
pub fn square_arrow_right_enter<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6C3}").font(LUCIDE)
}

/// `square-arrow-right-exit` from Lucide.
pub fn square_arrow_right_exit<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6C4}").font(LUCIDE)
}

/// `square-arrow-up` from Lucide.
pub fn square_arrow_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E42A}").font(LUCIDE)
}

/// `square-arrow-up-left` from Lucide.
pub fn square_arrow_up_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B7}").font(LUCIDE)
}

/// `square-arrow-up-right` from Lucide.
pub fn square_arrow_up_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4B8}").font(LUCIDE)
}

/// `square-asterisk` from Lucide.
pub fn square_asterisk<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E168}").font(LUCIDE)
}

/// `square-bottom-dashed-scissors` from Lucide.
pub fn square_bottom_dashed_scissors<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4EB}").font(LUCIDE)
}

/// `square-centerline-dashed-horizontal` from Lucide.
pub fn square_centerline_dashed_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6C5}").font(LUCIDE)
}

/// `square-centerline-dashed-vertical` from Lucide.
pub fn square_centerline_dashed_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6C6}").font(LUCIDE)
}

/// `square-chart-gantt` from Lucide.
pub fn square_chart_gantt<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E169}").font(LUCIDE)
}

/// `square-check` from Lucide.
pub fn square_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E559}").font(LUCIDE)
}

/// `square-check-big` from Lucide.
pub fn square_check_big<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E16A}").font(LUCIDE)
}

/// `square-chevron-down` from Lucide.
pub fn square_chevron_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3CF}").font(LUCIDE)
}

/// `square-chevron-left` from Lucide.
pub fn square_chevron_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D0}").font(LUCIDE)
}

/// `square-chevron-right` from Lucide.
pub fn square_chevron_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D1}").font(LUCIDE)
}

/// `square-chevron-up` from Lucide.
pub fn square_chevron_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3D2}").font(LUCIDE)
}

/// `square-code` from Lucide.
pub fn square_code<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E16B}").font(LUCIDE)
}

/// `square-dashed` from Lucide.
pub fn square_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1CB}").font(LUCIDE)
}

/// `square-dashed-bottom` from Lucide.
pub fn square_dashed_bottom<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4C0}").font(LUCIDE)
}

/// `square-dashed-bottom-code` from Lucide.
pub fn square_dashed_bottom_code<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4C1}").font(LUCIDE)
}

/// `square-dashed-kanban` from Lucide.
pub fn square_dashed_kanban<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E16C}").font(LUCIDE)
}

/// `square-dashed-mouse-pointer` from Lucide.
pub fn square_dashed_mouse_pointer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E509}").font(LUCIDE)
}

/// `square-dashed-text` from Lucide.
pub fn square_dashed_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6EA}").font(LUCIDE)
}

/// `square-dashed-top-solid` from Lucide.
pub fn square_dashed_top_solid<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E66C}").font(LUCIDE)
}

/// `square-dimensions` from Lucide.
pub fn square_dimensions<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E73C}").font(LUCIDE)
}

/// `square-divide` from Lucide.
pub fn square_divide<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E16D}").font(LUCIDE)
}

/// `square-dot` from Lucide.
pub fn square_dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E16E}").font(LUCIDE)
}

/// `square-equal` from Lucide.
pub fn square_equal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E16F}").font(LUCIDE)
}

/// `square-function` from Lucide.
pub fn square_function<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E22D}").font(LUCIDE)
}

/// `square-gantt-chart` from Lucide.
pub fn square_gantt_chart<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E169}").font(LUCIDE)
}

/// `square-kanban` from Lucide.
pub fn square_kanban<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E170}").font(LUCIDE)
}

/// `square-library` from Lucide.
pub fn square_library<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E54F}").font(LUCIDE)
}

/// `square-m` from Lucide.
pub fn square_m<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E503}").font(LUCIDE)
}

/// `square-menu` from Lucide.
pub fn square_menu<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E453}").font(LUCIDE)
}

/// `square-minus` from Lucide.
pub fn square_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E171}").font(LUCIDE)
}

/// `square-mouse-pointer` from Lucide.
pub fn square_mouse_pointer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E202}").font(LUCIDE)
}

/// `square-off` from Lucide.
pub fn square_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E728}").font(LUCIDE)
}

/// `square-parking` from Lucide.
pub fn square_parking<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3CB}").font(LUCIDE)
}

/// `square-parking-off` from Lucide.
pub fn square_parking_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3CC}").font(LUCIDE)
}

/// `square-pause` from Lucide.
pub fn square_pause<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E684}").font(LUCIDE)
}

/// `square-pen` from Lucide.
pub fn square_pen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E172}").font(LUCIDE)
}

/// `square-percent` from Lucide.
pub fn square_percent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E51C}").font(LUCIDE)
}

/// `square-pi` from Lucide.
pub fn square_pi<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E488}").font(LUCIDE)
}

/// `square-pilcrow` from Lucide.
pub fn square_pilcrow<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E48B}").font(LUCIDE)
}

/// `square-play` from Lucide.
pub fn square_play<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E481}").font(LUCIDE)
}

/// `square-plus` from Lucide.
pub fn square_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E173}").font(LUCIDE)
}

/// `square-power` from Lucide.
pub fn square_power<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E551}").font(LUCIDE)
}

/// `square-radical` from Lucide.
pub fn square_radical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5C3}").font(LUCIDE)
}

/// `square-round-corner` from Lucide.
pub fn square_round_corner<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E648}").font(LUCIDE)
}

/// `square-scissors` from Lucide.
pub fn square_scissors<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4EC}").font(LUCIDE)
}

/// `square-sigma` from Lucide.
pub fn square_sigma<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E489}").font(LUCIDE)
}

/// `square-slash` from Lucide.
pub fn square_slash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E174}").font(LUCIDE)
}

/// `square-split-horizontal` from Lucide.
pub fn square_split_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B6}").font(LUCIDE)
}

/// `square-split-vertical` from Lucide.
pub fn square_split_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3B7}").font(LUCIDE)
}

/// `square-square` from Lucide.
pub fn square_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E60E}").font(LUCIDE)
}

/// `square-stack` from Lucide.
pub fn square_stack<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4A2}").font(LUCIDE)
}

/// `square-star` from Lucide.
pub fn square_star<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E68E}").font(LUCIDE)
}

/// `square-stop` from Lucide.
pub fn square_stop<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E685}").font(LUCIDE)
}

/// `square-terminal` from Lucide.
pub fn square_terminal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E20A}").font(LUCIDE)
}

/// `square-text` from Lucide.
pub fn square_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E736}").font(LUCIDE)
}

/// `square-user` from Lucide.
pub fn square_user<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E465}").font(LUCIDE)
}

/// `square-user-round` from Lucide.
pub fn square_user_round<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E466}").font(LUCIDE)
}

/// `square-x` from Lucide.
pub fn square_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E175}").font(LUCIDE)
}

/// `squares-exclude` from Lucide.
pub fn squares_exclude<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E657}").font(LUCIDE)
}

/// `squares-intersect` from Lucide.
pub fn squares_intersect<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E658}").font(LUCIDE)
}

/// `squares-subtract` from Lucide.
pub fn squares_subtract<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E659}").font(LUCIDE)
}

/// `squares-unite` from Lucide.
pub fn squares_unite<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E65A}").font(LUCIDE)
}

/// `squircle` from Lucide.
pub fn squircle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E57A}").font(LUCIDE)
}

/// `squircle-dashed` from Lucide.
pub fn squircle_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E679}").font(LUCIDE)
}

/// `squirrel` from Lucide.
pub fn squirrel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E49F}").font(LUCIDE)
}

/// `stamp` from Lucide.
pub fn stamp<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3BB}").font(LUCIDE)
}

/// `star` from Lucide.
pub fn star<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E176}").font(LUCIDE)
}

/// `star-check` from Lucide.
pub fn star_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E707}").font(LUCIDE)
}

/// `star-half` from Lucide.
pub fn star_half<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E20B}").font(LUCIDE)
}

/// `star-minus` from Lucide.
pub fn star_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E708}").font(LUCIDE)
}

/// `star-off` from Lucide.
pub fn star_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2B0}").font(LUCIDE)
}

/// `star-plus` from Lucide.
pub fn star_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E709}").font(LUCIDE)
}

/// `star-x` from Lucide.
pub fn star_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E70A}").font(LUCIDE)
}

/// `stars` from Lucide.
pub fn stars<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E412}").font(LUCIDE)
}

/// `step-back` from Lucide.
pub fn step_back<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3E9}").font(LUCIDE)
}

/// `step-forward` from Lucide.
pub fn step_forward<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3EA}").font(LUCIDE)
}

/// `stethoscope` from Lucide.
pub fn stethoscope<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2F1}").font(LUCIDE)
}

/// `sticker` from Lucide.
pub fn sticker<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E302}").font(LUCIDE)
}

/// `sticky-note` from Lucide.
pub fn sticky_note<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E303}").font(LUCIDE)
}

/// `sticky-note-check` from Lucide.
pub fn sticky_note_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6F6}").font(LUCIDE)
}

/// `sticky-note-minus` from Lucide.
pub fn sticky_note_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6F7}").font(LUCIDE)
}

/// `sticky-note-off` from Lucide.
pub fn sticky_note_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6F8}").font(LUCIDE)
}

/// `sticky-note-plus` from Lucide.
pub fn sticky_note_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6F9}").font(LUCIDE)
}

/// `sticky-note-x` from Lucide.
pub fn sticky_note_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6FA}").font(LUCIDE)
}

/// `sticky-notes` from Lucide.
pub fn sticky_notes<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6FB}").font(LUCIDE)
}

/// `stone` from Lucide.
pub fn stone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6EB}").font(LUCIDE)
}

/// `stop-circle` from Lucide.
pub fn stop_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E083}").font(LUCIDE)
}

/// `store` from Lucide.
pub fn store<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3E4}").font(LUCIDE)
}

/// `stretch-horizontal` from Lucide.
pub fn stretch_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E27C}").font(LUCIDE)
}

/// `stretch-vertical` from Lucide.
pub fn stretch_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E27D}").font(LUCIDE)
}

/// `strikethrough` from Lucide.
pub fn strikethrough<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E177}").font(LUCIDE)
}

/// `subscript` from Lucide.
pub fn subscript<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E25C}").font(LUCIDE)
}

/// `subtitles` from Lucide.
pub fn subtitles<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3A4}").font(LUCIDE)
}

/// `summary` from Lucide.
pub fn summary<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E711}").font(LUCIDE)
}

/// `sun` from Lucide.
pub fn sun<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E178}").font(LUCIDE)
}

/// `sun-dim` from Lucide.
pub fn sun_dim<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E299}").font(LUCIDE)
}

/// `sun-medium` from Lucide.
pub fn sun_medium<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2B1}").font(LUCIDE)
}

/// `sun-moon` from Lucide.
pub fn sun_moon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2B2}").font(LUCIDE)
}

/// `sun-snow` from Lucide.
pub fn sun_snow<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E372}").font(LUCIDE)
}

/// `sunrise` from Lucide.
pub fn sunrise<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E179}").font(LUCIDE)
}

/// `sunset` from Lucide.
pub fn sunset<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E17A}").font(LUCIDE)
}

/// `superscript` from Lucide.
pub fn superscript<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E25E}").font(LUCIDE)
}

/// `swatch-book` from Lucide.
pub fn swatch_book<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E59F}").font(LUCIDE)
}

/// `swiss-franc` from Lucide.
pub fn swiss_franc<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E17B}").font(LUCIDE)
}

/// `switch-camera` from Lucide.
pub fn switch_camera<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E17C}").font(LUCIDE)
}

/// `sword` from Lucide.
pub fn sword<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2B3}").font(LUCIDE)
}

/// `swords` from Lucide.
pub fn swords<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2B4}").font(LUCIDE)
}

/// `syringe` from Lucide.
pub fn syringe<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2F2}").font(LUCIDE)
}

/// `table` from Lucide.
pub fn table<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E17D}").font(LUCIDE)
}

/// `table-2` from Lucide.
pub fn table_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2F9}").font(LUCIDE)
}

/// `table-cells-merge` from Lucide.
pub fn table_cells_merge<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5C7}").font(LUCIDE)
}

/// `table-cells-split` from Lucide.
pub fn table_cells_split<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5C8}").font(LUCIDE)
}

/// `table-columns-split` from Lucide.
pub fn table_columns_split<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5C9}").font(LUCIDE)
}

/// `table-config` from Lucide.
pub fn table_config<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E661}").font(LUCIDE)
}

/// `table-of-contents` from Lucide.
pub fn table_of_contents<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E61E}").font(LUCIDE)
}

/// `table-properties` from Lucide.
pub fn table_properties<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4DB}").font(LUCIDE)
}

/// `table-rows-split` from Lucide.
pub fn table_rows_split<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5CA}").font(LUCIDE)
}

/// `tablet` from Lucide.
pub fn tablet<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E17E}").font(LUCIDE)
}

/// `tablet-smartphone` from Lucide.
pub fn tablet_smartphone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E50A}").font(LUCIDE)
}

/// `tablets` from Lucide.
pub fn tablets<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3BE}").font(LUCIDE)
}

/// `tag` from Lucide.
pub fn tag<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E17F}").font(LUCIDE)
}

/// `tag-plus` from Lucide.
pub fn tag_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E712}").font(LUCIDE)
}

/// `tag-x` from Lucide.
pub fn tag_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E713}").font(LUCIDE)
}

/// `tags` from Lucide.
pub fn tags<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E35C}").font(LUCIDE)
}

/// `tally-1` from Lucide.
pub fn tally_1<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4D6}").font(LUCIDE)
}

/// `tally-2` from Lucide.
pub fn tally_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4D7}").font(LUCIDE)
}

/// `tally-3` from Lucide.
pub fn tally_3<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4D8}").font(LUCIDE)
}

/// `tally-4` from Lucide.
pub fn tally_4<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4D9}").font(LUCIDE)
}

/// `tally-5` from Lucide.
pub fn tally_5<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4DA}").font(LUCIDE)
}

/// `tangent` from Lucide.
pub fn tangent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E52E}").font(LUCIDE)
}

/// `target` from Lucide.
pub fn target<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E180}").font(LUCIDE)
}

/// `telescope` from Lucide.
pub fn telescope<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5C5}").font(LUCIDE)
}

/// `tent` from Lucide.
pub fn tent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E227}").font(LUCIDE)
}

/// `tent-tree` from Lucide.
pub fn tent_tree<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E53B}").font(LUCIDE)
}

/// `terminal` from Lucide.
pub fn terminal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E181}").font(LUCIDE)
}

/// `terminal-square` from Lucide.
pub fn terminal_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E20A}").font(LUCIDE)
}

/// `test-tube` from Lucide.
pub fn test_tube<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E405}").font(LUCIDE)
}

/// `test-tube-2` from Lucide.
pub fn test_tube_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E406}").font(LUCIDE)
}

/// `test-tube-diagonal` from Lucide.
pub fn test_tube_diagonal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E406}").font(LUCIDE)
}

/// `test-tubes` from Lucide.
pub fn test_tubes<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E407}").font(LUCIDE)
}

/// `text` from Lucide.
pub fn text_icon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E185}").font(LUCIDE)
}

/// `text-align-center` from Lucide.
pub fn text_align_center<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E182}").font(LUCIDE)
}

/// `text-align-end` from Lucide.
pub fn text_align_end<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E183}").font(LUCIDE)
}

/// `text-align-justify` from Lucide.
pub fn text_align_justify<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E184}").font(LUCIDE)
}

/// `text-align-start` from Lucide.
pub fn text_align_start<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E185}").font(LUCIDE)
}

/// `text-cursor` from Lucide.
pub fn text_cursor<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E264}").font(LUCIDE)
}

/// `text-cursor-input` from Lucide.
pub fn text_cursor_input<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E265}").font(LUCIDE)
}

/// `text-initial` from Lucide.
pub fn text_initial<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E605}").font(LUCIDE)
}

/// `text-quote` from Lucide.
pub fn text_quote<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E49E}").font(LUCIDE)
}

/// `text-search` from Lucide.
pub fn text_search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5AD}").font(LUCIDE)
}

/// `text-select` from Lucide.
pub fn text_select<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6EA}").font(LUCIDE)
}

/// `text-selection` from Lucide.
pub fn text_selection<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3DE}").font(LUCIDE)
}

/// `text-wrap` from Lucide.
pub fn text_wrap<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E248}").font(LUCIDE)
}

/// `theater` from Lucide.
pub fn theater<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E522}").font(LUCIDE)
}

/// `thermometer` from Lucide.
pub fn thermometer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E186}").font(LUCIDE)
}

/// `thermometer-snowflake` from Lucide.
pub fn thermometer_snowflake<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E187}").font(LUCIDE)
}

/// `thermometer-sun` from Lucide.
pub fn thermometer_sun<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E188}").font(LUCIDE)
}

/// `thumbs-down` from Lucide.
pub fn thumbs_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E189}").font(LUCIDE)
}

/// `thumbs-up` from Lucide.
pub fn thumbs_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E18A}").font(LUCIDE)
}

/// `ticket` from Lucide.
pub fn ticket<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E20F}").font(LUCIDE)
}

/// `ticket-check` from Lucide.
pub fn ticket_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5AE}").font(LUCIDE)
}

/// `ticket-minus` from Lucide.
pub fn ticket_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5AF}").font(LUCIDE)
}

/// `ticket-percent` from Lucide.
pub fn ticket_percent<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5B0}").font(LUCIDE)
}

/// `ticket-plus` from Lucide.
pub fn ticket_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5B1}").font(LUCIDE)
}

/// `ticket-slash` from Lucide.
pub fn ticket_slash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5B2}").font(LUCIDE)
}

/// `ticket-x` from Lucide.
pub fn ticket_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5B3}").font(LUCIDE)
}

/// `tickets` from Lucide.
pub fn tickets<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E622}").font(LUCIDE)
}

/// `tickets-plane` from Lucide.
pub fn tickets_plane<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E623}").font(LUCIDE)
}

/// `timeline` from Lucide.
pub fn timeline<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6EC}").font(LUCIDE)
}

/// `timer` from Lucide.
pub fn timer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1E0}").font(LUCIDE)
}

/// `timer-off` from Lucide.
pub fn timer_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E249}").font(LUCIDE)
}

/// `timer-reset` from Lucide.
pub fn timer_reset<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E236}").font(LUCIDE)
}

/// `toggle-left` from Lucide.
pub fn toggle_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E18B}").font(LUCIDE)
}

/// `toggle-right` from Lucide.
pub fn toggle_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E18C}").font(LUCIDE)
}

/// `toilet` from Lucide.
pub fn toilet<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E635}").font(LUCIDE)
}

/// `tool-case` from Lucide.
pub fn tool_case<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E67D}").font(LUCIDE)
}

/// `toolbox` from Lucide.
pub fn toolbox<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6B0}").font(LUCIDE)
}

/// `tornado` from Lucide.
pub fn tornado<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E218}").font(LUCIDE)
}

/// `torus` from Lucide.
pub fn torus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E52F}").font(LUCIDE)
}

/// `touchpad` from Lucide.
pub fn touchpad<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E449}").font(LUCIDE)
}

/// `touchpad-off` from Lucide.
pub fn touchpad_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E44A}").font(LUCIDE)
}

/// `towel-rack` from Lucide.
pub fn towel_rack<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6C7}").font(LUCIDE)
}

/// `tower-control` from Lucide.
pub fn tower_control<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3BC}").font(LUCIDE)
}

/// `toy-brick` from Lucide.
pub fn toy_brick<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E347}").font(LUCIDE)
}

/// `tractor` from Lucide.
pub fn tractor<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E504}").font(LUCIDE)
}

/// `traffic-cone` from Lucide.
pub fn traffic_cone<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E505}").font(LUCIDE)
}

/// `trailer` from Lucide.
pub fn trailer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E742}").font(LUCIDE)
}

/// `train` from Lucide.
pub fn train<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A9}").font(LUCIDE)
}

/// `train-front` from Lucide.
pub fn train_front<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E506}").font(LUCIDE)
}

/// `train-front-tunnel` from Lucide.
pub fn train_front_tunnel<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E507}").font(LUCIDE)
}

/// `train-track` from Lucide.
pub fn train_track<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E508}").font(LUCIDE)
}

/// `tram-front` from Lucide.
pub fn tram_front<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A9}").font(LUCIDE)
}

/// `transgender` from Lucide.
pub fn transgender<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E644}").font(LUCIDE)
}

/// `trash` from Lucide.
pub fn trash<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E18D}").font(LUCIDE)
}

/// `trash-2` from Lucide.
pub fn trash_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E18E}").font(LUCIDE)
}

/// `tree-deciduous` from Lucide.
pub fn tree_deciduous<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2F3}").font(LUCIDE)
}

/// `tree-palm` from Lucide.
pub fn tree_palm<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E281}").font(LUCIDE)
}

/// `tree-pine` from Lucide.
pub fn tree_pine<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2F4}").font(LUCIDE)
}

/// `trees` from Lucide.
pub fn trees<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2F5}").font(LUCIDE)
}

/// `trending-down` from Lucide.
pub fn trending_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E190}").font(LUCIDE)
}

/// `trending-up` from Lucide.
pub fn trending_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E191}").font(LUCIDE)
}

/// `trending-up-down` from Lucide.
pub fn trending_up_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E625}").font(LUCIDE)
}

/// `triangle` from Lucide.
pub fn triangle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E192}").font(LUCIDE)
}

/// `triangle-alert` from Lucide.
pub fn triangle_alert<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E193}").font(LUCIDE)
}

/// `triangle-dashed` from Lucide.
pub fn triangle_dashed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E63D}").font(LUCIDE)
}

/// `triangle-right` from Lucide.
pub fn triangle_right<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4ED}").font(LUCIDE)
}

/// `trophy` from Lucide.
pub fn trophy<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E373}").font(LUCIDE)
}

/// `truck` from Lucide.
pub fn truck<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E194}").font(LUCIDE)
}

/// `truck-electric` from Lucide.
pub fn truck_electric<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E65F}").font(LUCIDE)
}

/// `turkish-lira` from Lucide.
pub fn turkish_lira<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E680}").font(LUCIDE)
}

/// `turntable` from Lucide.
pub fn turntable<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E68C}").font(LUCIDE)
}

/// `turtle` from Lucide.
pub fn turtle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4F9}").font(LUCIDE)
}

/// `tv` from Lucide.
pub fn tv<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E195}").font(LUCIDE)
}

/// `tv-2` from Lucide.
pub fn tv_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E203}").font(LUCIDE)
}

/// `tv-minimal` from Lucide.
pub fn tv_minimal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E203}").font(LUCIDE)
}

/// `tv-minimal-play` from Lucide.
pub fn tv_minimal_play<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5EC}").font(LUCIDE)
}

/// `type` from Lucide.
pub fn type_icon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E198}").font(LUCIDE)
}

/// `type-outline` from Lucide.
pub fn type_outline<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E602}").font(LUCIDE)
}

/// `umbrella` from Lucide.
pub fn umbrella<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E199}").font(LUCIDE)
}

/// `umbrella-off` from Lucide.
pub fn umbrella_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E543}").font(LUCIDE)
}

/// `underline` from Lucide.
pub fn underline<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E19A}").font(LUCIDE)
}

/// `undo` from Lucide.
pub fn undo<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E19B}").font(LUCIDE)
}

/// `undo-2` from Lucide.
pub fn undo_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2A1}").font(LUCIDE)
}

/// `undo-dot` from Lucide.
pub fn undo_dot<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E451}").font(LUCIDE)
}

/// `unfold-horizontal` from Lucide.
pub fn unfold_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E43D}").font(LUCIDE)
}

/// `unfold-vertical` from Lucide.
pub fn unfold_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E43E}").font(LUCIDE)
}

/// `ungroup` from Lucide.
pub fn ungroup<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E467}").font(LUCIDE)
}

/// `university` from Lucide.
pub fn university<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3E5}").font(LUCIDE)
}

/// `unlink` from Lucide.
pub fn unlink<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E19C}").font(LUCIDE)
}

/// `unlink-2` from Lucide.
pub fn unlink_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E19D}").font(LUCIDE)
}

/// `unlock` from Lucide.
pub fn unlock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E10C}").font(LUCIDE)
}

/// `unlock-keyhole` from Lucide.
pub fn unlock_keyhole<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E532}").font(LUCIDE)
}

/// `unplug` from Lucide.
pub fn unplug<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E45D}").font(LUCIDE)
}

/// `upload` from Lucide.
pub fn upload<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E19E}").font(LUCIDE)
}

/// `upload-cloud` from Lucide.
pub fn upload_cloud<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E091}").font(LUCIDE)
}

/// `usb` from Lucide.
pub fn usb<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E356}").font(LUCIDE)
}

/// `usb-c-port` from Lucide.
pub fn usb_c_port<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E73D}").font(LUCIDE)
}

/// `user` from Lucide.
pub fn user<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E19F}").font(LUCIDE)
}

/// `user-2` from Lucide.
pub fn user_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E468}").font(LUCIDE)
}

/// `user-check` from Lucide.
pub fn user_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1A0}").font(LUCIDE)
}

/// `user-check-2` from Lucide.
pub fn user_check_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E469}").font(LUCIDE)
}

/// `user-circle` from Lucide.
pub fn user_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E461}").font(LUCIDE)
}

/// `user-circle-2` from Lucide.
pub fn user_circle_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E462}").font(LUCIDE)
}

/// `user-cog` from Lucide.
pub fn user_cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E342}").font(LUCIDE)
}

/// `user-cog-2` from Lucide.
pub fn user_cog_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E46A}").font(LUCIDE)
}

/// `user-key` from Lucide.
pub fn user_key<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6C8}").font(LUCIDE)
}

/// `user-lock` from Lucide.
pub fn user_lock<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E660}").font(LUCIDE)
}

/// `user-minus` from Lucide.
pub fn user_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1A1}").font(LUCIDE)
}

/// `user-minus-2` from Lucide.
pub fn user_minus_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E46B}").font(LUCIDE)
}

/// `user-pen` from Lucide.
pub fn user_pen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5FC}").font(LUCIDE)
}

/// `user-plus` from Lucide.
pub fn user_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1A2}").font(LUCIDE)
}

/// `user-plus-2` from Lucide.
pub fn user_plus_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E46C}").font(LUCIDE)
}

/// `user-round` from Lucide.
pub fn user_round<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E468}").font(LUCIDE)
}

/// `user-round-arrow-left` from Lucide.
pub fn user_round_arrow_left<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E714}").font(LUCIDE)
}

/// `user-round-check` from Lucide.
pub fn user_round_check<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E469}").font(LUCIDE)
}

/// `user-round-cog` from Lucide.
pub fn user_round_cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E46A}").font(LUCIDE)
}

/// `user-round-key` from Lucide.
pub fn user_round_key<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6C9}").font(LUCIDE)
}

/// `user-round-minus` from Lucide.
pub fn user_round_minus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E46B}").font(LUCIDE)
}

/// `user-round-pen` from Lucide.
pub fn user_round_pen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5FD}").font(LUCIDE)
}

/// `user-round-plus` from Lucide.
pub fn user_round_plus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E46C}").font(LUCIDE)
}

/// `user-round-search` from Lucide.
pub fn user_round_search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E578}").font(LUCIDE)
}

/// `user-round-x` from Lucide.
pub fn user_round_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E46D}").font(LUCIDE)
}

/// `user-search` from Lucide.
pub fn user_search<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E579}").font(LUCIDE)
}

/// `user-shield` from Lucide.
pub fn user_shield<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E729}").font(LUCIDE)
}

/// `user-square` from Lucide.
pub fn user_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E465}").font(LUCIDE)
}

/// `user-square-2` from Lucide.
pub fn user_square_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E466}").font(LUCIDE)
}

/// `user-star` from Lucide.
pub fn user_star<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E687}").font(LUCIDE)
}

/// `user-x` from Lucide.
pub fn user_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1A3}").font(LUCIDE)
}

/// `user-x-2` from Lucide.
pub fn user_x_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E46D}").font(LUCIDE)
}

/// `users` from Lucide.
pub fn users<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1A4}").font(LUCIDE)
}

/// `users-2` from Lucide.
pub fn users_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E46E}").font(LUCIDE)
}

/// `users-round` from Lucide.
pub fn users_round<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E46E}").font(LUCIDE)
}

/// `utensils` from Lucide.
pub fn utensils<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2F6}").font(LUCIDE)
}

/// `utensils-crossed` from Lucide.
pub fn utensils_crossed<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2F7}").font(LUCIDE)
}

/// `utility-pole` from Lucide.
pub fn utility_pole<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3C2}").font(LUCIDE)
}

/// `van` from Lucide.
pub fn van<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6AD}").font(LUCIDE)
}

/// `variable` from Lucide.
pub fn variable<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E473}").font(LUCIDE)
}

/// `vault` from Lucide.
pub fn vault<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E58F}").font(LUCIDE)
}

/// `vector-square` from Lucide.
pub fn vector_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E67C}").font(LUCIDE)
}

/// `vegan` from Lucide.
pub fn vegan<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E39D}").font(LUCIDE)
}

/// `venetian-mask` from Lucide.
pub fn venetian_mask<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2AA}").font(LUCIDE)
}

/// `venus` from Lucide.
pub fn venus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E645}").font(LUCIDE)
}

/// `venus-and-mars` from Lucide.
pub fn venus_and_mars<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E646}").font(LUCIDE)
}

/// `verified` from Lucide.
pub fn verified<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E241}").font(LUCIDE)
}

/// `vibrate` from Lucide.
pub fn vibrate<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E223}").font(LUCIDE)
}

/// `vibrate-off` from Lucide.
pub fn vibrate_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E29D}").font(LUCIDE)
}

/// `video` from Lucide.
pub fn video<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1A5}").font(LUCIDE)
}

/// `video-off` from Lucide.
pub fn video_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1A6}").font(LUCIDE)
}

/// `videotape` from Lucide.
pub fn videotape<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4CB}").font(LUCIDE)
}

/// `view` from Lucide.
pub fn view<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1A7}").font(LUCIDE)
}

/// `voicemail` from Lucide.
pub fn voicemail<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1A8}").font(LUCIDE)
}

/// `volleyball` from Lucide.
pub fn volleyball<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E62F}").font(LUCIDE)
}

/// `volume` from Lucide.
pub fn volume<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1A9}").font(LUCIDE)
}

/// `volume-1` from Lucide.
pub fn volume_1<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1AA}").font(LUCIDE)
}

/// `volume-2` from Lucide.
pub fn volume_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1AB}").font(LUCIDE)
}

/// `volume-off` from Lucide.
pub fn volume_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E626}").font(LUCIDE)
}

/// `volume-x` from Lucide.
pub fn volume_x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1AC}").font(LUCIDE)
}

/// `vote` from Lucide.
pub fn vote<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3AD}").font(LUCIDE)
}

/// `wallet` from Lucide.
pub fn wallet<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E204}").font(LUCIDE)
}

/// `wallet-2` from Lucide.
pub fn wallet_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4CD}").font(LUCIDE)
}

/// `wallet-cards` from Lucide.
pub fn wallet_cards<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4CC}").font(LUCIDE)
}

/// `wallet-minimal` from Lucide.
pub fn wallet_minimal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E4CD}").font(LUCIDE)
}

/// `wallpaper` from Lucide.
pub fn wallpaper<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E44B}").font(LUCIDE)
}

/// `wand` from Lucide.
pub fn wand<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E246}").font(LUCIDE)
}

/// `wand-2` from Lucide.
pub fn wand_2<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E357}").font(LUCIDE)
}

/// `wand-sparkles` from Lucide.
pub fn wand_sparkles<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E357}").font(LUCIDE)
}

/// `warehouse` from Lucide.
pub fn warehouse<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3E6}").font(LUCIDE)
}

/// `washing-machine` from Lucide.
pub fn washing_machine<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E590}").font(LUCIDE)
}

/// `watch` from Lucide.
pub fn watch<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1AD}").font(LUCIDE)
}

/// `waves` from Lucide.
pub fn waves<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E283}").font(LUCIDE)
}

/// `waves-arrow-down` from Lucide.
pub fn waves_arrow_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6A9}").font(LUCIDE)
}

/// `waves-arrow-up` from Lucide.
pub fn waves_arrow_up<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6AA}").font(LUCIDE)
}

/// `waves-horizontal` from Lucide.
pub fn waves_horizontal<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E283}").font(LUCIDE)
}

/// `waves-ladder` from Lucide.
pub fn waves_ladder<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E63B}").font(LUCIDE)
}

/// `waves-vertical` from Lucide.
pub fn waves_vertical<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6F2}").font(LUCIDE)
}

/// `waypoints` from Lucide.
pub fn waypoints<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E542}").font(LUCIDE)
}

/// `webcam` from Lucide.
pub fn webcam<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E205}").font(LUCIDE)
}

/// `webcam-off` from Lucide.
pub fn webcam_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6FF}").font(LUCIDE)
}

/// `webhook` from Lucide.
pub fn webhook<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E374}").font(LUCIDE)
}

/// `webhook-off` from Lucide.
pub fn webhook_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5B7}").font(LUCIDE)
}

/// `weight` from Lucide.
pub fn weight<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E530}").font(LUCIDE)
}

/// `weight-tilde` from Lucide.
pub fn weight_tilde<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6AE}").font(LUCIDE)
}

/// `wheat` from Lucide.
pub fn wheat<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E39E}").font(LUCIDE)
}

/// `wheat-off` from Lucide.
pub fn wheat_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E39F}").font(LUCIDE)
}

/// `whole-word` from Lucide.
pub fn whole_word<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3DF}").font(LUCIDE)
}

/// `wifi` from Lucide.
pub fn wifi<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1AE}").font(LUCIDE)
}

/// `wifi-cog` from Lucide.
pub fn wifi_cog<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E674}").font(LUCIDE)
}

/// `wifi-high` from Lucide.
pub fn wifi_high<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5F7}").font(LUCIDE)
}

/// `wifi-low` from Lucide.
pub fn wifi_low<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5F8}").font(LUCIDE)
}

/// `wifi-off` from Lucide.
pub fn wifi_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1AF}").font(LUCIDE)
}

/// `wifi-pen` from Lucide.
pub fn wifi_pen<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E663}").font(LUCIDE)
}

/// `wifi-sync` from Lucide.
pub fn wifi_sync<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E681}").font(LUCIDE)
}

/// `wifi-zero` from Lucide.
pub fn wifi_zero<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5F9}").font(LUCIDE)
}

/// `wind` from Lucide.
pub fn wind<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1B0}").font(LUCIDE)
}

/// `wind-arrow-down` from Lucide.
pub fn wind_arrow_down<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E631}").font(LUCIDE)
}

/// `wine` from Lucide.
pub fn wine<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E2F8}").font(LUCIDE)
}

/// `wine-off` from Lucide.
pub fn wine_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E3A0}").font(LUCIDE)
}

/// `workflow` from Lucide.
pub fn workflow<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E425}").font(LUCIDE)
}

/// `worm` from Lucide.
pub fn worm<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E5DA}").font(LUCIDE)
}

/// `wrap-text` from Lucide.
pub fn wrap_text<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E248}").font(LUCIDE)
}

/// `wrench` from Lucide.
pub fn wrench<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1B1}").font(LUCIDE)
}

/// `wrench-off` from Lucide.
pub fn wrench_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E70B}").font(LUCIDE)
}

/// `x` from Lucide.
pub fn x<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1B2}").font(LUCIDE)
}

/// `x-circle` from Lucide.
pub fn x_circle<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E084}").font(LUCIDE)
}

/// `x-line-top` from Lucide.
pub fn x_line_top<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6CA}").font(LUCIDE)
}

/// `x-octagon` from Lucide.
pub fn x_octagon<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E128}").font(LUCIDE)
}

/// `x-square` from Lucide.
pub fn x_square<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E175}").font(LUCIDE)
}

/// `zap` from Lucide.
pub fn zap<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1B4}").font(LUCIDE)
}

/// `zap-off` from Lucide.
pub fn zap_off<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1B5}").font(LUCIDE)
}

/// `zodiac-aquarius` from Lucide.
pub fn zodiac_aquarius<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6CB}").font(LUCIDE)
}

/// `zodiac-aries` from Lucide.
pub fn zodiac_aries<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6CC}").font(LUCIDE)
}

/// `zodiac-cancer` from Lucide.
pub fn zodiac_cancer<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6CD}").font(LUCIDE)
}

/// `zodiac-capricorn` from Lucide.
pub fn zodiac_capricorn<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6CE}").font(LUCIDE)
}

/// `zodiac-gemini` from Lucide.
pub fn zodiac_gemini<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6CF}").font(LUCIDE)
}

/// `zodiac-leo` from Lucide.
pub fn zodiac_leo<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6D0}").font(LUCIDE)
}

/// `zodiac-libra` from Lucide.
pub fn zodiac_libra<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6D1}").font(LUCIDE)
}

/// `zodiac-ophiuchus` from Lucide.
pub fn zodiac_ophiuchus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6D2}").font(LUCIDE)
}

/// `zodiac-pisces` from Lucide.
pub fn zodiac_pisces<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6D3}").font(LUCIDE)
}

/// `zodiac-sagittarius` from Lucide.
pub fn zodiac_sagittarius<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6D4}").font(LUCIDE)
}

/// `zodiac-scorpio` from Lucide.
pub fn zodiac_scorpio<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6D5}").font(LUCIDE)
}

/// `zodiac-taurus` from Lucide.
pub fn zodiac_taurus<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6D6}").font(LUCIDE)
}

/// `zodiac-virgo` from Lucide.
pub fn zodiac_virgo<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E6D7}").font(LUCIDE)
}

/// `zoom-in` from Lucide.
pub fn zoom_in<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1B6}").font(LUCIDE)
}

/// `zoom-out` from Lucide.
pub fn zoom_out<'a, Theme, Renderer>() -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text("\u{E1B7}").font(LUCIDE)
}

/// Render any icon from [`ALL_ICONS`].
pub fn render<'a, Theme, Renderer>(icon: Icon) -> Text<'a, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    iced::widget::text(icon.codepoint).font(icon.font)
}

/// Look an icon up by its upstream name.
pub fn find(name: &str) -> Option<Icon> {
    ALL_ICONS.iter().find(|icon| icon.name == name).copied()
}
