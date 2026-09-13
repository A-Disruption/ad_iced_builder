use iced::advanced::graphics::core::length::{Bounds, Constraint, Sizing};
use iced::mouse::Interaction;
use iced::widget::{scrollable, text, tooltip};
use iced::{Alignment, ContentFit, Font, Length};
use serde::{Deserialize, Serialize};

use super::types::*;

// Display implementations
impl std::fmt::Display for WidgetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The three groups the add-widget picker sorts its results into.
///
/// These are the sections the old fixed palette had, kept because they are the
/// way the widgets are already thought about: things that hold other things,
/// things that arrange them, and things that draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WidgetCategory {
    Containers,
    Layout,
    Widgets,
}

impl WidgetCategory {
    pub const ALL: [WidgetCategory; 3] = [
        WidgetCategory::Containers,
        WidgetCategory::Layout,
        WidgetCategory::Widgets,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            WidgetCategory::Containers => "Containers",
            WidgetCategory::Layout => "Layout",
            WidgetCategory::Widgets => "Widgets",
        }
    }
}

impl WidgetType {
    /// The name shown to the user.
    ///
    /// `Display` writes the variant, which is what code generation and saved
    /// projects want. This is the reading version — the same strings the fixed
    /// palette used to hard-code next to each button.
    pub fn label(&self) -> &'static str {
        match self {
            WidgetType::Container => "Container",
            WidgetType::Scrollable => "Scrollable",
            WidgetType::Row => "Row",
            WidgetType::Column => "Column",
            WidgetType::Button => "Button",
            WidgetType::Text => "Text",
            WidgetType::TextEditor => "Text Editor",
            WidgetType::TextInput => "Text Input",
            WidgetType::Checkbox => "Checkbox",
            WidgetType::Radio => "Radio",
            WidgetType::Slider => "Slider",
            WidgetType::VerticalSlider => "Vertical Slider",
            WidgetType::ProgressBar => "Progress Bar",
            WidgetType::Toggler => "Toggler",
            WidgetType::PickList => "Pick List",
            WidgetType::Space => "Space",
            WidgetType::Rule => "Rule",
            WidgetType::Image => "Image",
            WidgetType::Svg => "SVG",
            WidgetType::Tooltip => "Tooltip",
            WidgetType::ComboBox => "ComboBox",
            WidgetType::Markdown => "Markdown",
            WidgetType::MouseArea => "Mouse Area",
            WidgetType::QRCode => "QR Code",
            WidgetType::Stack => "Stack",
            WidgetType::Collapsible => "Collapsible",
            WidgetType::CollapsibleGroup => "Collapsible Group",
            WidgetType::Popover => "Popover",
            WidgetType::Menu => "Menu",
            WidgetType::Layer => "Layer",
            WidgetType::Toast => "Toasts",
            WidgetType::DatePicker => "Date Picker",
            WidgetType::Split => "Split",
            WidgetType::Float => "Float",
            WidgetType::Responsive => "Responsive",
            WidgetType::Sensor => "Sensor",
            WidgetType::Themer => "Themer",
            WidgetType::Grid => "Grid",
            WidgetType::Pin => "Pin",
            WidgetType::Table => "Table",
            WidgetType::DataTable => "Data Table",
            WidgetType::ViewReference => "View",
            WidgetType::Icon => "Icon",
        }
    }

    /// Which section of the picker this type belongs to.
    pub fn category(&self) -> WidgetCategory {
        match self {
            WidgetType::Container
            | WidgetType::Scrollable
            | WidgetType::ViewReference
            | WidgetType::Collapsible
            | WidgetType::CollapsibleGroup
            | WidgetType::Popover
            | WidgetType::Menu
            | WidgetType::Layer
            | WidgetType::Toast => WidgetCategory::Containers,

            WidgetType::Row
            | WidgetType::Column
            | WidgetType::Table
            | WidgetType::DataTable
            | WidgetType::Grid
            | WidgetType::Stack
            | WidgetType::Split
            | WidgetType::Responsive
            | WidgetType::Float
            | WidgetType::Themer => WidgetCategory::Layout,

            _ => WidgetCategory::Widgets,
        }
    }

    /// Extra words the picker's search should match on.
    ///
    /// Lets someone reach `ViewReference` by typing "sub", or `Space` by
    /// typing "gap", without those words being on screen.
    pub fn search_aliases(&self) -> &'static [&'static str] {
        match self {
            WidgetType::ViewReference => &["subview", "component", "embed"],
            WidgetType::Space => &["gap", "spacer", "filler"],
            WidgetType::Rule => &["divider", "separator", "line"],
            WidgetType::MouseArea => &["click", "press", "hover"],
            WidgetType::Sensor => &["resize", "visibility"],
            WidgetType::Popover => &["overlay", "dropdown"],
            WidgetType::Layer => &["overlay", "modal", "dialog"],
            WidgetType::Toast => &["notification", "snackbar"],
            WidgetType::Collapsible => &["accordion", "expander", "disclosure"],
            WidgetType::CollapsibleGroup => &["accordion"],
            WidgetType::DataTable => &["grid", "datagrid", "advanced table"],
            WidgetType::PickList => &["dropdown", "select"],
            WidgetType::ComboBox => &["dropdown", "select", "autocomplete"],
            WidgetType::TextInput => &["field", "entry"],
            WidgetType::TextEditor => &["multiline", "textarea"],
            WidgetType::Toggler => &["switch"],
            WidgetType::ProgressBar => &["loading"],
            WidgetType::Svg => &["vector"],
            WidgetType::Icon => &["glyph", "lucide"],
            WidgetType::Themer => &["theme"],
            WidgetType::Split => &["pane", "resizable"],
            _ => &[],
        }
    }

    /// Whether `query` should surface this type in the picker.
    ///
    /// Empty queries match everything; otherwise it is a case-insensitive
    /// substring test against the label, the variant name and the aliases.
    pub fn matches_search(&self, query: &str) -> bool {
        let query = query.trim().to_lowercase();

        if query.is_empty() {
            return true;
        }

        if self.label().to_lowercase().contains(&query) {
            return true;
        }

        if format!("{:?}", self).to_lowercase().contains(&query) {
            return true;
        }

        self.search_aliases()
            .iter()
            .any(|alias| alias.contains(&query))
    }
}

impl std::fmt::Display for ContainerAlignX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for ContainerAlignY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for OverlayPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverlayPosition::Top => write!(f, "Top"),
            OverlayPosition::Bottom => write!(f, "Bottom"),
            OverlayPosition::Left => write!(f, "Left"),
            OverlayPosition::Right => write!(f, "Right"),
        }
    }
}

impl std::fmt::Display for OverlayPositionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverlayPositionMode::Outside => write!(f, "Outside"),
            OverlayPositionMode::Inside => write!(f, "Inside"),
        }
    }
}

impl std::fmt::Display for OverlayResizeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverlayResizeMode::None => write!(f, "None"),
            OverlayResizeMode::Always => write!(f, "Always"),
            OverlayResizeMode::WithCtrl => write!(f, "With Ctrl"),
        }
    }
}

impl std::fmt::Display for OverlayAnimationPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverlayAnimationPreset::Default => write!(f, "Default"),
            OverlayAnimationPreset::Quick => write!(f, "Quick"),
            OverlayAnimationPreset::Slow => write!(f, "Slow"),
        }
    }
}

impl std::fmt::Display for DatePickerSelectionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatePickerSelectionMode::Single => write!(f, "Single"),
            DatePickerSelectionMode::Range => write!(f, "Range"),
        }
    }
}

impl std::fmt::Display for SplitStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SplitStrategy::Relative => write!(f, "Relative"),
            SplitStrategy::Start => write!(f, "Start"),
            SplitStrategy::End => write!(f, "End"),
        }
    }
}

impl std::fmt::Display for ButtonStyleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for FontType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Font> for FontType {
    fn from(a: Font) -> Self {
        match a {
            Font::DEFAULT => Self::Default,
            Font::MONOSPACE => Self::Monospace,
            _ => Self::Default,
        }
    }
}
impl From<FontType> for Font {
    fn from(c: FontType) -> Self {
        match c {
            FontType::Monospace => Self::MONOSPACE,
            FontType::Default => Self::DEFAULT,
        }
    }
}

impl std::fmt::Display for AlignmentXOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlignmentXOption::Start => write!(f, "Start"),
            AlignmentXOption::Center => write!(f, "Center"),
            AlignmentXOption::End => write!(f, "End"),
        }
    }
}

impl std::fmt::Display for AlignmentYOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlignmentYOption::Top => write!(f, "Top"),
            AlignmentYOption::Center => write!(f, "Center"),
            AlignmentYOption::Bottom => write!(f, "Bottom"),
        }
    }
}

impl std::fmt::Display for TextWrapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextWrapping::None => write!(f, "None"),
            TextWrapping::Word => write!(f, "Word"),
            TextWrapping::Glyph => write!(f, "Glyph"),
            TextWrapping::WordOrGlyph => write!(f, "WordOrGlyph"),
        }
    }
}

impl std::fmt::Display for TextShaping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextShaping::Basic => write!(f, "Basic"),
            TextShaping::Advanced => write!(f, "Advanced"),
            TextShaping::Auto => write!(f, "Auto"),
        }
    }
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Horizontal => write!(f, "Horizontal"),
            Orientation::Vertical => write!(f, "Vertical"),
        }
    }
}

impl std::fmt::Display for AlignText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlignText::Default => write!(f, "Default"),
            AlignText::Left => write!(f, "Left"),
            AlignText::Center => write!(f, "Center"),
            AlignText::Right => write!(f, "Right"),
            AlignText::Justified => write!(f, "Justified"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignmentXOption {
    Start,
    Center,
    End,
}

impl AlignmentXOption {
    // Convert our wrapper TO Iced's Alignment
    pub fn to_alignment(self) -> Alignment {
        match self {
            AlignmentXOption::Start => Alignment::Start,
            AlignmentXOption::Center => Alignment::Center,
            AlignmentXOption::End => Alignment::End,
        }
    }

    // Convert FROM Iced's Alignment to our wrapper
    pub fn from_alignment(alignment: Alignment) -> Self {
        match alignment {
            Alignment::Start => AlignmentXOption::Start,
            Alignment::Center => AlignmentXOption::Center,
            Alignment::End => AlignmentXOption::End,
        }
    }
}
impl From<Alignment> for AlignmentXOption {
    fn from(a: Alignment) -> Self {
        match a {
            Alignment::Start => Self::Start,
            Alignment::Center => Self::Center,
            Alignment::End => Self::End,
        }
    }
}
impl From<AlignmentXOption> for Alignment {
    fn from(c: AlignmentXOption) -> Self {
        match c {
            AlignmentXOption::Start => Self::Start,
            AlignmentXOption::Center => Self::Center,
            AlignmentXOption::End => Self::End,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignmentYOption {
    Top,
    Center,
    Bottom,
}

impl AlignmentYOption {
    // Convert our wrapper TO Iced's Alignment
    pub fn to_alignment(self) -> iced::alignment::Vertical {
        match self {
            AlignmentYOption::Top => iced::alignment::Vertical::Top,
            AlignmentYOption::Center => iced::alignment::Vertical::Center,
            AlignmentYOption::Bottom => iced::alignment::Vertical::Bottom,
        }
    }

    // Convert FROM Iced's Alignment to our wrapper
    pub fn from_alignment(alignment: iced::alignment::Vertical) -> Self {
        match alignment {
            iced::alignment::Vertical::Top => AlignmentYOption::Top,
            iced::alignment::Vertical::Center => AlignmentYOption::Center,
            iced::alignment::Vertical::Bottom => AlignmentYOption::Bottom,
        }
    }
}
impl From<Alignment> for AlignmentYOption {
    fn from(v: Alignment) -> Self {
        match v {
            Alignment::Start => Self::Top,
            Alignment::Center => Self::Center,
            Alignment::End => Self::Bottom,
        }
    }
}
impl From<iced::alignment::Vertical> for AlignmentYOption {
    fn from(v: iced::alignment::Vertical) -> Self {
        match v {
            iced::alignment::Vertical::Top => Self::Top,
            iced::alignment::Vertical::Center => Self::Center,
            iced::alignment::Vertical::Bottom => Self::Bottom,
        }
    }
}
impl From<AlignmentYOption> for Alignment {
    fn from(c: AlignmentYOption) -> Self {
        match c {
            AlignmentYOption::Top => Self::Start,
            AlignmentYOption::Center => Self::Center,
            AlignmentYOption::Bottom => Self::End,
        }
    }
}
impl From<AlignmentYOption> for iced::alignment::Vertical {
    fn from(c: AlignmentYOption) -> Self {
        match c {
            AlignmentYOption::Top => Self::Top,
            AlignmentYOption::Center => Self::Center,
            AlignmentYOption::Bottom => Self::Bottom,
        }
    }
}

impl From<Alignment> for ContainerAlignX {
    fn from(v: Alignment) -> Self {
        match v {
            Alignment::Start => Self::Left,
            Alignment::Center => Self::Center,
            Alignment::End => Self::Right,
        }
    }
}
impl From<iced::alignment::Horizontal> for ContainerAlignX {
    fn from(v: iced::alignment::Horizontal) -> Self {
        match v {
            iced::alignment::Horizontal::Left => Self::Left,
            iced::alignment::Horizontal::Center => Self::Center,
            iced::alignment::Horizontal::Right => Self::Right,
        }
    }
}
impl From<ContainerAlignX> for iced::alignment::Horizontal {
    fn from(c: ContainerAlignX) -> Self {
        match c {
            ContainerAlignX::Left => Self::Left,
            ContainerAlignX::Center => Self::Center,
            ContainerAlignX::Right => Self::Right,
        }
    }
}
impl From<ContainerAlignX> for Alignment {
    fn from(c: ContainerAlignX) -> Self {
        match c {
            ContainerAlignX::Left => Self::Start,
            ContainerAlignX::Center => Self::Center,
            ContainerAlignX::Right => Self::End,
        }
    }
}

// iced master takes a `text::Alignment` for text alignment, which carries a
// `Default` (writing-direction) case the layout alignments do not.
impl From<ContainerAlignX> for iced::advanced::text::Alignment {
    fn from(c: ContainerAlignX) -> Self {
        iced::alignment::Horizontal::from(c).into()
    }
}

/// The cross-axis alignment the anchored widgets take.
impl From<ContainerAlignX> for widgets::anchor::Align {
    fn from(c: ContainerAlignX) -> Self {
        match c {
            ContainerAlignX::Left => Self::Start,
            ContainerAlignX::Center => Self::Center,
            ContainerAlignX::Right => Self::End,
        }
    }
}

impl From<iced::alignment::Vertical> for ContainerAlignY {
    fn from(v: iced::alignment::Vertical) -> Self {
        match v {
            iced::alignment::Vertical::Top => Self::Top,
            iced::alignment::Vertical::Center => Self::Center,
            iced::alignment::Vertical::Bottom => Self::Bottom,
        }
    }
}
impl From<Alignment> for ContainerAlignY {
    fn from(v: Alignment) -> Self {
        match v {
            Alignment::Start => Self::Top,
            Alignment::Center => Self::Center,
            Alignment::End => Self::Bottom,
        }
    }
}
impl From<ContainerAlignY> for iced::alignment::Vertical {
    fn from(c: ContainerAlignY) -> Self {
        match c {
            ContainerAlignY::Top => Self::Top,
            ContainerAlignY::Center => Self::Center,
            ContainerAlignY::Bottom => Self::Bottom,
        }
    }
}
impl From<ContainerAlignY> for Alignment {
    fn from(c: ContainerAlignY) -> Self {
        match c {
            ContainerAlignY::Top => Self::Start,
            ContainerAlignY::Center => Self::Center,
            ContainerAlignY::Bottom => Self::End,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextWrapping {
    None,
    Word,
    Glyph,
    WordOrGlyph,
}

impl TextWrapping {
    pub fn to_wrap(self) -> text::Wrapping {
        match self {
            TextWrapping::None => text::Wrapping::None,
            TextWrapping::Word => text::Wrapping::Word,
            TextWrapping::Glyph => text::Wrapping::Glyph,
            TextWrapping::WordOrGlyph => text::Wrapping::WordOrGlyph,
        }
    }

    pub fn from_wrap(alignment: text::Wrapping) -> Self {
        match alignment {
            text::Wrapping::None => TextWrapping::None,
            text::Wrapping::Word => TextWrapping::Word,
            text::Wrapping::Glyph => TextWrapping::Glyph,
            text::Wrapping::WordOrGlyph => TextWrapping::WordOrGlyph,
        }
    }
}

impl From<text::Wrapping> for TextWrapping {
    fn from(w: text::Wrapping) -> Self {
        match w {
            text::Wrapping::None => Self::None,
            text::Wrapping::Word => Self::Word,
            text::Wrapping::Glyph => Self::Glyph,
            text::Wrapping::WordOrGlyph => Self::WordOrGlyph,
        }
    }
}
impl From<TextWrapping> for text::Wrapping {
    fn from(c: TextWrapping) -> Self {
        match c {
            TextWrapping::None => Self::None,
            TextWrapping::Word => Self::Word,
            TextWrapping::Glyph => Self::Glyph,
            TextWrapping::WordOrGlyph => Self::WordOrGlyph,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextShaping {
    Basic,
    Advanced,
    Auto,
}

impl TextShaping {
    pub fn to_shaping(self) -> text::Shaping {
        match self {
            TextShaping::Basic => text::Shaping::Basic,
            TextShaping::Advanced => text::Shaping::Advanced,
            TextShaping::Auto => text::Shaping::Auto,
        }
    }

    pub fn from_shaping(alignment: text::Shaping) -> Self {
        match alignment {
            text::Shaping::Basic => TextShaping::Basic,
            text::Shaping::Advanced => TextShaping::Advanced,
            text::Shaping::Auto => TextShaping::Auto,
        }
    }
}
impl From<text::Shaping> for TextShaping {
    fn from(s: text::Shaping) -> Self {
        match s {
            text::Shaping::Basic => Self::Basic,
            text::Shaping::Advanced => Self::Advanced,
            text::Shaping::Auto => Self::Auto,
        }
    }
}
impl From<TextShaping> for text::Shaping {
    fn from(c: TextShaping) -> Self {
        match c {
            TextShaping::Basic => Self::Basic,
            TextShaping::Advanced => Self::Advanced,
            TextShaping::Auto => Self::Auto,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ContainerAlignX {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ContainerAlignY {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OverlayPosition {
    Top,
    Bottom,
    Left,
    #[default]
    Right,
}

impl From<OverlayPosition> for widgets::anchor::Side {
    fn from(position: OverlayPosition) -> Self {
        match position {
            OverlayPosition::Top => Self::Top,
            OverlayPosition::Bottom => Self::Bottom,
            OverlayPosition::Left => Self::Left,
            OverlayPosition::Right => Self::Right,
        }
    }
}

/// Where a [`Layer`] sits against the viewport.
///
/// The same four sides the anchored widgets use, read as an edge of the window
/// rather than a side of a trigger. `Anchor` also carries corners and a centre
/// the builder does not expose yet.
impl From<OverlayPosition> for widgets::layer::Anchor {
    fn from(position: OverlayPosition) -> Self {
        match position {
            OverlayPosition::Top => Self::Top,
            OverlayPosition::Bottom => Self::Bottom,
            OverlayPosition::Left => Self::Left,
            OverlayPosition::Right => Self::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OverlayPositionMode {
    #[default]
    Outside,
    Inside,
}

impl OverlayPositionMode {
    /// Turns a gap into the signed gap the anchored widgets take.
    ///
    /// The widgets crate has no `PositionMode`: a surface always sits on the
    /// given side, and a *negative* gap is what pulls it back over the base.
    /// That is exactly what `Inside` used to mean.
    pub fn signed_gap(self, gap: f32) -> f32 {
        match self {
            Self::Outside => gap,
            Self::Inside => -gap,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OverlayResizeMode {
    #[default]
    None,
    Always,
    WithCtrl,
}

impl OverlayResizeMode {
    /// Whether a [`Layer`] should carry resize handles.
    ///
    /// `Layer::resizable` is a flag, where the old widget distinguished
    /// always-on handles from ctrl-held ones. Both of those are "resizable"
    /// now; the modifier is gone.
    pub fn is_resizable(self) -> bool {
        !matches!(self, Self::None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OverlayAnimationPreset {
    #[default]
    Default,
    Quick,
    Slow,
}

impl OverlayAnimationPreset {
    /// The open/close transition the anchored widgets take.
    pub fn motion(self) -> widgets::animation::Motion {
        match self {
            Self::Default => widgets::animation::Motion::SMOOTH,
            Self::Quick => widgets::animation::Motion::QUICK,
            Self::Slow => widgets::animation::Motion::SMOOTH
                .duration(std::time::Duration::from_millis(420)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DatePickerSelectionMode {
    #[default]
    Single,
    Range,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SplitStrategy {
    #[default]
    Relative,
    Start,
    End,
}

impl From<SplitStrategy> for iced_split::Strategy {
    fn from(strategy: SplitStrategy) -> Self {
        match strategy {
            SplitStrategy::Relative => Self::Relative,
            SplitStrategy::Start => Self::Start,
            SplitStrategy::End => Self::End,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayLengthChoice {
    Fill,
    FillPortion,
    Shrink,
    Fixed,
    Dynamic,
}

impl std::fmt::Display for OverlayLengthChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverlayLengthChoice::Fill => write!(f, "Fill"),
            OverlayLengthChoice::FillPortion => write!(f, "FillPortion"),
            OverlayLengthChoice::Shrink => write!(f, "Shrink"),
            OverlayLengthChoice::Fixed => write!(f, "Fixed"),
            OverlayLengthChoice::Dynamic => write!(f, "Dynamic"),
        }
    }
}

impl OverlayLengthChoice {
    pub fn from_state(length: Length, is_dynamic: bool) -> Self {
        if is_dynamic {
            Self::Dynamic
        } else {
            match length {
                Length::Fill => Self::Fill,
                Length::FillPortion(_) => Self::FillPortion,
                Length::Shrink => Self::Shrink,
                Length::Fixed(_) => Self::Fixed,
                // iced master added these. The builder's pickers cannot author
                // them yet -- `Bounded` only ever arrives here from the separate
                // max_width / max_height properties -- so each reduces to the
                // sizing strategy underneath it.
                Length::Fit => Self::Shrink,
                Length::Fluid(_) => Self::Fill,
                Length::Bounded { sizing, .. } => match sizing {
                    Sizing::Fit | Sizing::Shrink => Self::Shrink,
                    Sizing::Fill(1) => Self::Fill,
                    Sizing::Fill(_) => Self::FillPortion,
                },
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonStyleType {
    Primary,
    Secondary,
    Success,
    Danger,
    Text,
    Background,
    Subtle,
}

impl ButtonStyleType {
    pub fn all() -> Vec<String> {
        vec![
            "Primary".to_string(),
            "Secondary".to_string(),
            "Success".to_string(),
            "Danger".to_string(),
            "Text".to_string(),
            "Background".to_string(),
            "Subtle".to_string(),
        ]
    }

    pub fn get(name: &str) -> Option<ButtonStyleType> {
        match name {
            "Primary" => Some(ButtonStyleType::Primary),
            "Secondary" => Some(ButtonStyleType::Secondary),
            "Success" => Some(ButtonStyleType::Success),
            "Danger" => Some(ButtonStyleType::Danger),
            "Text" => Some(ButtonStyleType::Text),
            "Background" => Some(ButtonStyleType::Background),
            "Subtle" => Some(ButtonStyleType::Subtle),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContainerStyleType {
    Transparent,
    Background,
    RoundedBox,
    BorderedBox,
    Dark,
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
}

impl ContainerStyleType {
    pub fn all() -> Vec<String> {
        vec![
            "Transparent".to_string(),
            "Background".to_string(),
            "Rounded Box".to_string(),
            "Bordered Box".to_string(),
            "Dark".to_string(),
            "Primary".to_string(),
            "Secondary".to_string(),
            "Success".to_string(),
            "Danger".to_string(),
            "Warning".to_string(),
        ]
    }

    pub fn get(name: &str) -> Option<ContainerStyleType> {
        match name {
            "Transparent" => Some(ContainerStyleType::Transparent),
            "Background" => Some(ContainerStyleType::Background),
            "Rounded Box" => Some(ContainerStyleType::RoundedBox),
            "Bordered Box" => Some(ContainerStyleType::BorderedBox),
            "Dark" => Some(ContainerStyleType::Dark),
            "Primary" => Some(ContainerStyleType::Primary),
            "Secondary" => Some(ContainerStyleType::Secondary),
            "Success" => Some(ContainerStyleType::Success),
            "Danger" => Some(ContainerStyleType::Danger),
            "Warning" => Some(ContainerStyleType::Warning),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FontType {
    Default,
    Monospace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignText {
    Default,
    Left,
    Center,
    Right,
    Justified,
}

impl AlignText {
    // Convert our wrapper TO Iced's Alignment
    pub fn to_alignment(self) -> iced::advanced::text::Alignment {
        match self {
            AlignText::Default => iced::advanced::text::Alignment::Default,
            AlignText::Left => iced::advanced::text::Alignment::Left,
            AlignText::Center => iced::advanced::text::Alignment::Center,
            AlignText::Right => iced::advanced::text::Alignment::Right,
            AlignText::Justified => iced::advanced::text::Alignment::Justified,
        }
    }

    // Convert FROM Iced's Alignment to our wrapper
    pub fn from_alignment(alignment: iced::advanced::text::Alignment) -> Self {
        match alignment {
            iced::advanced::text::Alignment::Default => AlignText::Default,
            iced::advanced::text::Alignment::Left => AlignText::Left,
            iced::advanced::text::Alignment::Center => AlignText::Center,
            iced::advanced::text::Alignment::Right => AlignText::Right,
            iced::advanced::text::Alignment::Justified => AlignText::Justified,
        }
    }
}
impl From<iced::advanced::text::Alignment> for AlignText {
    fn from(a: iced::advanced::text::Alignment) -> Self {
        match a {
            iced::advanced::text::Alignment::Default => Self::Default,
            iced::advanced::text::Alignment::Left => Self::Left,
            iced::advanced::text::Alignment::Center => Self::Center,
            iced::advanced::text::Alignment::Right => Self::Right,
            iced::advanced::text::Alignment::Justified => Self::Justified,
        }
    }
}
impl From<AlignText> for iced::advanced::text::Alignment {
    fn from(c: AlignText) -> Self {
        match c {
            AlignText::Default => Self::Default,
            AlignText::Left => Self::Left,
            AlignText::Center => Self::Center,
            AlignText::Right => Self::Right,
            AlignText::Justified => Self::Justified,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirChoice {
    Vertical,
    Horizontal,
    Both,
}
impl std::fmt::Display for DirChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DirChoice::Vertical => "Vertical",
                DirChoice::Horizontal => "Horizontal",
                DirChoice::Both => "Both",
            }
        )
    }
}
impl DirChoice {
    pub fn to_choice(d: iced::widget::scrollable::Direction) -> DirChoice {
        match d {
            iced::widget::scrollable::Direction::Vertical(_) => DirChoice::Vertical,
            iced::widget::scrollable::Direction::Horizontal(_) => DirChoice::Horizontal,
            iced::widget::scrollable::Direction::Both { .. } => DirChoice::Both,
        }
    }
    pub fn from_choice(c: DirChoice) -> iced::widget::scrollable::Direction {
        match c {
            DirChoice::Vertical => {
                iced::widget::scrollable::Direction::Vertical(scrollable::Scrollbar::default())
            }
            DirChoice::Horizontal => {
                iced::widget::scrollable::Direction::Horizontal(scrollable::Scrollbar::default())
            }
            DirChoice::Both => iced::widget::scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::default(),
                horizontal: scrollable::Scrollbar::default(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorChoice {
    Start,
    End,
}
impl std::fmt::Display for AnchorChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnchorChoice::Start => "Start",
                AnchorChoice::End => "End",
            }
        )
    }
}
impl AnchorChoice {
    pub fn to_anchor(d: iced::widget::scrollable::Anchor) -> AnchorChoice {
        match d {
            iced::widget::scrollable::Anchor::Start => AnchorChoice::Start,
            iced::widget::scrollable::Anchor::End => AnchorChoice::End,
        }
    }
    pub fn from_anchor(c: AnchorChoice) -> iced::widget::scrollable::Anchor {
        match c {
            AnchorChoice::Start => iced::widget::scrollable::Anchor::Start,
            AnchorChoice::End => iced::widget::scrollable::Anchor::End,
        }
    }
}
impl From<iced::widget::scrollable::Anchor> for AnchorChoice {
    fn from(a: iced::widget::scrollable::Anchor) -> Self {
        match a {
            iced::widget::scrollable::Anchor::Start => Self::Start,
            iced::widget::scrollable::Anchor::End => Self::End,
        }
    }
}
impl From<AnchorChoice> for iced::widget::scrollable::Anchor {
    fn from(c: AnchorChoice) -> Self {
        match c {
            AnchorChoice::Start => Self::Start,
            AnchorChoice::End => Self::End,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentFitChoice {
    Contain,
    Cover,
    Fill,
    ScaleDown,
    None,
}

impl ContentFitChoice {
    pub const ALL: &'static [Self] =
        &[Self::Contain, Self::Cover, Self::Fill, Self::ScaleDown, Self::None];
}
impl std::fmt::Display for ContentFitChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ContentFitChoice::*;
        write!(
            f,
            "{}",
            match self {
                Contain => "Contain",
                Cover => "Cover",
                Fill => "Fill",
                ScaleDown => "ScaleDown",
                None => "None",
            }
        )
    }
}
impl From<ContentFit> for ContentFitChoice {
    fn from(f: ContentFit) -> Self {
        use ContentFit::*;
        match f {
            Contain => Self::Contain,
            Cover => Self::Cover,
            Fill => Self::Fill,
            ScaleDown => Self::ScaleDown,
            None => Self::None,
        }
    }
}
impl From<ContentFitChoice> for ContentFit {
    fn from(c: ContentFitChoice) -> Self {
        use ContentFitChoice::*;
        match c {
            Contain => ContentFit::Contain,
            Cover => ContentFit::Cover,
            Fill => ContentFit::Fill,
            ScaleDown => ContentFit::ScaleDown,
            None => ContentFit::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
    FollowCursor,
}
impl std::fmt::Display for TooltipPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TooltipPosition::*;
        write!(
            f,
            "{}",
            match self {
                Top => "Top",
                Bottom => "Bottom",
                Left => "Left",
                Right => "Right",
                FollowCursor => "Follow Cursor",
            }
        )
    }
}
impl From<TooltipPosition> for tooltip::Position {
    fn from(p: TooltipPosition) -> Self {
        use TooltipPosition::*;
        match p {
            Top => tooltip::Position::Top,
            Bottom => tooltip::Position::Bottom,
            Left => tooltip::Position::Left,
            Right => tooltip::Position::Right,
            FollowCursor => tooltip::Position::FollowCursor,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerSizingMode {
    Manual,  // User sets width/height separately
    CenterX, // Use center_x(length)
    CenterY, // Use center_y(length)
    Center,  // Use center(length)
}

impl std::fmt::Display for ContainerSizingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerSizingMode::Manual => write!(f, "Manual"),
            ContainerSizingMode::CenterX => write!(f, "Center X"),
            ContainerSizingMode::CenterY => write!(f, "Center Y"),
            ContainerSizingMode::Center => write!(f, "Center Both"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnHandler {
    None,
    OnAction,
    OnActionWith,
    OnActionMaybe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseInteraction {
    None,
    Hidden,
    Idle,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    ResizingHorizontally,
    ResizingVertically,
    ResizingDiagonallyUp,
    ResizingDiagonallyDown,
    ResizingColumn,
    ResizingRow,
    AllScroll,
    ZoomIn,
    ZoomOut,
}
impl std::fmt::Display for MouseInteraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MouseInteraction::None => write!(f, "None"),
            MouseInteraction::Hidden => write!(f, "Hidden"),
            MouseInteraction::Idle => write!(f, "Idle"),
            MouseInteraction::ContextMenu => write!(f, "ContextMenu"),
            MouseInteraction::Help => write!(f, "Help"),
            MouseInteraction::Pointer => write!(f, "Pointer"),
            MouseInteraction::Progress => write!(f, "Progress"),
            MouseInteraction::Wait => write!(f, "Wait"),
            MouseInteraction::Cell => write!(f, "Cell"),
            MouseInteraction::Crosshair => write!(f, "Crosshair"),
            MouseInteraction::Text => write!(f, "Text"),
            MouseInteraction::Alias => write!(f, "Alias"),
            MouseInteraction::Copy => write!(f, "Copy"),
            MouseInteraction::Move => write!(f, "Move"),
            MouseInteraction::NoDrop => write!(f, "NoDrop"),
            MouseInteraction::NotAllowed => write!(f, "NotAllowed"),
            MouseInteraction::Grab => write!(f, "Grab"),
            MouseInteraction::Grabbing => write!(f, "Grabbing"),
            MouseInteraction::ResizingHorizontally => write!(f, "ResizingHorizontally"),
            MouseInteraction::ResizingVertically => write!(f, "ResizingVertically"),
            MouseInteraction::ResizingDiagonallyUp => write!(f, "ResizingDiagonallyUp"),
            MouseInteraction::ResizingDiagonallyDown => write!(f, "ResizingDiagonallyDown"),
            MouseInteraction::ResizingColumn => write!(f, "ResizingColumn"),
            MouseInteraction::ResizingRow => write!(f, "ResizingRow"),
            MouseInteraction::AllScroll => write!(f, "AllScroll"),
            MouseInteraction::ZoomIn => write!(f, "ZoomIn"),
            MouseInteraction::ZoomOut => write!(f, "ZoomOut"),
        }
    }
}
impl From<Interaction> for MouseInteraction {
    fn from(a: Interaction) -> Self {
        match a {
            Interaction::None => Self::None,
            Interaction::Hidden => Self::Hidden,
            Interaction::Idle => Self::Idle,
            Interaction::ContextMenu => Self::ContextMenu,
            Interaction::Help => Self::Help,
            Interaction::Pointer => Self::Pointer,
            Interaction::Progress => Self::Progress,
            Interaction::Wait => Self::Wait,
            Interaction::Cell => Self::Cell,
            Interaction::Crosshair => Self::Crosshair,
            Interaction::Text => Self::Text,
            Interaction::Alias => Self::Alias,
            Interaction::Copy => Self::Copy,
            Interaction::Move => Self::Move,
            Interaction::NoDrop => Self::NoDrop,
            Interaction::NotAllowed => Self::NotAllowed,
            Interaction::Grab => Self::Grab,
            Interaction::Grabbing => Self::Grabbing,
            Interaction::ResizingHorizontally => Self::ResizingHorizontally,
            Interaction::ResizingVertically => Self::ResizingVertically,
            Interaction::ResizingDiagonallyUp => Self::ResizingDiagonallyUp,
            Interaction::ResizingDiagonallyDown => Self::ResizingDiagonallyDown,
            Interaction::ResizingColumn => Self::ResizingColumn,
            Interaction::ResizingRow => Self::ResizingRow,
            Interaction::AllScroll => Self::AllScroll,
            Interaction::ZoomIn => Self::ZoomIn,
            Interaction::ZoomOut => Self::ZoomOut,
        }
    }
}
impl From<MouseInteraction> for Interaction {
    fn from(c: MouseInteraction) -> Self {
        match c {
            MouseInteraction::None => Self::None,
            MouseInteraction::Hidden => Self::Hidden,
            MouseInteraction::Idle => Self::Idle,
            MouseInteraction::ContextMenu => Self::ContextMenu,
            MouseInteraction::Help => Self::Help,
            MouseInteraction::Pointer => Self::Pointer,
            MouseInteraction::Progress => Self::Progress,
            MouseInteraction::Wait => Self::Wait,
            MouseInteraction::Cell => Self::Cell,
            MouseInteraction::Crosshair => Self::Crosshair,
            MouseInteraction::Text => Self::Text,
            MouseInteraction::Alias => Self::Alias,
            MouseInteraction::Copy => Self::Copy,
            MouseInteraction::Move => Self::Move,
            MouseInteraction::NoDrop => Self::NoDrop,
            MouseInteraction::NotAllowed => Self::NotAllowed,
            MouseInteraction::Grab => Self::Grab,
            MouseInteraction::Grabbing => Self::Grabbing,
            MouseInteraction::ResizingHorizontally => Self::ResizingHorizontally,
            MouseInteraction::ResizingVertically => Self::ResizingVertically,
            MouseInteraction::ResizingDiagonallyUp => Self::ResizingDiagonallyUp,
            MouseInteraction::ResizingDiagonallyDown => Self::ResizingDiagonallyDown,
            MouseInteraction::ResizingColumn => Self::ResizingColumn,
            MouseInteraction::ResizingRow => Self::ResizingRow,
            MouseInteraction::AllScroll => Self::AllScroll,
            MouseInteraction::ZoomIn => Self::ZoomIn,
            MouseInteraction::ZoomOut => Self::ZoomOut,
        }
    }
}

impl MouseInteraction {
    pub const ALL: &'static [Self] = &[
        Self::None,
        Self::Hidden,
        Self::Idle,
        Self::ContextMenu,
        Self::Help,
        Self::Pointer,
        Self::Progress,
        Self::Wait,
        Self::Cell,
        Self::Crosshair,
        Self::Text,
        Self::Alias,
        Self::Copy,
        Self::Move,
        Self::NoDrop,
        Self::NotAllowed,
        Self::Grab,
        Self::Grabbing,
        Self::ResizingHorizontally,
        Self::ResizingVertically,
        Self::ResizingDiagonallyUp,
        Self::ResizingDiagonallyDown,
        Self::ResizingColumn,
        Self::ResizingRow,
        Self::AllScroll,
        Self::ZoomIn,
        Self::ZoomOut,
    ];
}

/// What the user is choosing for a Length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthChoice {
    Fill,
    FillPortion,
    /// `Length::Fit` -- the sizing most widgets start from on iced master.
    ///
    /// Takes the intrinsic size of the contents, where `Shrink` compresses
    /// them as far as they will go.
    Fit,
    Shrink,
    Fixed,
    /// `Length::Bounded` -- any of the sizings above, held inside a pixel
    /// range.
    ///
    /// iced master folded the old `max_width` / `max_height` builders into
    /// the length itself, and this is what they became.
    Bounded,
    /// `Length::Fluid` -- fills like `Fill`, but subject to one open-ended
    /// constraint.
    ///
    /// Unlike `Bounded`, a fluid length propagates its constraint outward
    /// through the layout: a container of fluid children is itself fluid.
    Fluid,
}

impl LengthChoice {
    /// Every choice, in the order the pickers offer them.
    pub const ALL: &'static [Self] = &[
        Self::Fill,
        Self::FillPortion,
        Self::Fit,
        Self::Shrink,
        Self::Fixed,
        Self::Bounded,
        Self::Fluid,
    ];

    /// The choices that survive inside a scrollable, which cannot give a
    /// child an unbounded axis to fill.
    pub const CONSTRAINED: &'static [Self] =
        &[Self::Fit, Self::Shrink, Self::Fixed, Self::Bounded];

    /// Whether this choice needs space to fill, and so is unavailable along a
    /// scrollable's scrolling axis.
    pub fn needs_fill(self) -> bool {
        matches!(self, Self::Fill | Self::FillPortion | Self::Fluid)
    }
}

impl std::fmt::Display for LengthChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthChoice::Fill => write!(f, "Fill"),
            LengthChoice::FillPortion => write!(f, "FillPortion"),
            LengthChoice::Fit => write!(f, "Fit"),
            LengthChoice::Shrink => write!(f, "Shrink"),
            LengthChoice::Fixed => write!(f, "Fixed"),
            LengthChoice::Bounded => write!(f, "Bounded"),
            LengthChoice::Fluid => write!(f, "Fluid"),
        }
    }
}

impl LengthChoice {
    pub fn from_length(len: Length) -> Self {
        match len {
            Length::Fill => LengthChoice::Fill,
            Length::FillPortion(_) => LengthChoice::FillPortion,
            Length::Shrink => LengthChoice::Shrink,
            Length::Fixed(_) => LengthChoice::Fixed,
            Length::Fit => LengthChoice::Fit,
            Length::Fluid(_) => LengthChoice::Fluid,
            Length::Bounded { .. } => LengthChoice::Bounded,
        }
    }
}

/// The growth strategy inside a [`Length::Bounded`].
///
/// Mirrors [`Sizing`], which carries its fill factor inline; the portion is
/// edited separately, so the choice does not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizingChoice {
    Fit,
    Shrink,
    Fill,
    FillPortion,
}

impl SizingChoice {
    pub const ALL: &'static [Self] = &[Self::Fit, Self::Shrink, Self::Fill, Self::FillPortion];

    pub fn from_sizing(sizing: Sizing) -> Self {
        match sizing {
            Sizing::Fit => Self::Fit,
            Sizing::Shrink => Self::Shrink,
            Sizing::Fill(1) => Self::Fill,
            Sizing::Fill(_) => Self::FillPortion,
        }
    }

    /// The [`Sizing`] this choice means, given the portion currently in hand.
    pub fn to_sizing(self, portion: u16) -> Sizing {
        match self {
            Self::Fit => Sizing::Fit,
            Self::Shrink => Sizing::Shrink,
            Self::Fill => Sizing::Fill(1),
            Self::FillPortion => Sizing::Fill(portion.max(1)),
        }
    }
}

impl std::fmt::Display for SizingChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fit => write!(f, "Fit"),
            Self::Shrink => write!(f, "Shrink"),
            Self::Fill => write!(f, "Fill"),
            Self::FillPortion => write!(f, "FillPortion"),
        }
    }
}

/// Which ends of a [`Length::Bounded`] are pinned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundsChoice {
    Min,
    Max,
    Both,
}

impl BoundsChoice {
    pub const ALL: &'static [Self] = &[Self::Min, Self::Max, Self::Both];

    pub fn from_bounds(bounds: Bounds) -> Self {
        match bounds {
            Bounds::Min(_) => Self::Min,
            Bounds::Max(_) => Self::Max,
            Bounds::Both { .. } => Self::Both,
        }
    }

    /// The [`Bounds`] this choice means, given the values currently in hand.
    pub fn to_bounds(self, min: f32, max: f32) -> Bounds {
        match self {
            Self::Min => Bounds::Min(min),
            Self::Max => Bounds::Max(max),
            Self::Both => Bounds::Both {
                min,
                max: max.max(min),
            },
        }
    }
}

impl std::fmt::Display for BoundsChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Min => write!(f, "Minimum"),
            Self::Max => write!(f, "Maximum"),
            Self::Both => write!(f, "Both"),
        }
    }
}

/// The single open-ended limit on a [`Length::Fluid`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintChoice {
    /// Never shrinks below a given size.
    Min,
    /// Never grows beyond the size the contents resolve to on their own,
    /// handing any unused share back to the siblings.
    Max,
}

impl ConstraintChoice {
    pub const ALL: &'static [Self] = &[Self::Min, Self::Max];

    pub fn from_constraint(constraint: Constraint) -> Self {
        match constraint {
            Constraint::Min(_) => Self::Min,
            Constraint::Max => Self::Max,
        }
    }

    /// The [`Constraint`] this choice means, given the minimum in hand.
    pub fn to_constraint(self, min: f32) -> Constraint {
        match self {
            Self::Min => Constraint::Min(min),
            Self::Max => Constraint::Max,
        }
    }
}

impl std::fmt::Display for ConstraintChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Min => write!(f, "At least"),
            Self::Max => write!(f, "No more than contents"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaddingMode {
    /// All four sides have the same value
    Uniform,
    /// Top/Bottom share one value, Left/Right share another
    Symmetric,
    /// Each side has its own value
    Individual,
}

impl std::fmt::Display for PaddingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaddingMode::Uniform => write!(f, "Uniform"),
            PaddingMode::Symmetric => write!(f, "Symmetric"),
            PaddingMode::Individual => write!(f, "Individual"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TextInputIconSide {
    #[default]
    Left,
    Right,
}

impl std::fmt::Display for TextInputIconSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextInputIconSide::Left => write!(f, "Left"),
            TextInputIconSide::Right => write!(f, "Right"),
        }
    }
}

pub fn length_to_string(length: Length) -> String {
    match length {
        Length::Fill => "Fill".to_string(),
        Length::Shrink => "Shrink".to_string(),
        Length::Fixed(pixels) => format!("{}", pixels),
        Length::FillPortion(p) => format!("FillPortion({p})"),
        Length::Fit => "Fit".to_string(),
        Length::Fluid(_) => "Fill".to_string(),
        Length::Bounded { bounds, sizing } => {
            let base = match sizing {
                Sizing::Fit => "Fit",
                Sizing::Shrink => "Shrink",
                Sizing::Fill(1) => "Fill",
                Sizing::Fill(_) => "FillPortion",
            };

            match bounds {
                Bounds::Min(min) => format!("{base} (min {min})"),
                Bounds::Max(max) => format!("{base} (max {max})"),
                Bounds::Both { min, max } => format!("{base} ({min}..{max})"),
            }
        }
    }
}

/// Which side of a menu row the checkmark gutter sits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MenuCheckSide {
    /// Before the row contents, sharing the gutter with the row icon.
    #[default]
    Leading,
    /// After the row contents.
    Trailing,
}

impl MenuCheckSide {
    pub const ALL: &'static [Self] = &[Self::Leading, Self::Trailing];
}

impl std::fmt::Display for MenuCheckSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leading => write!(f, "Leading"),
            Self::Trailing => write!(f, "Trailing"),
        }
    }
}

impl From<MenuCheckSide> for widgets::menu::CheckSide {
    fn from(side: MenuCheckSide) -> Self {
        match side {
            MenuCheckSide::Leading => Self::Leading,
            MenuCheckSide::Trailing => Self::Trailing,
        }
    }
}

/// What kind of row a child of a Menu is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MenuItemKind {
    /// A selectable row.
    #[default]
    Entry,
    /// A selectable row that draws a checkmark when checked.
    Toggle,
    /// A row that opens a nested panel of the rows beneath it.
    Submenu,
    /// A rule between groups of rows. Its own content is not drawn.
    Separator,
}

impl MenuItemKind {
    pub const ALL: &'static [Self] = &[
        Self::Entry,
        Self::Toggle,
        Self::Submenu,
        Self::Separator,
    ];
}

impl std::fmt::Display for MenuItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Entry => write!(f, "Entry"),
            Self::Toggle => write!(f, "Toggle"),
            Self::Submenu => write!(f, "Submenu"),
            Self::Separator => write!(f, "Separator"),
        }
    }
}

/// The accent a single toast is drawn with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ToastVariant {
    #[default]
    Neutral,
    Info,
    Warning,
    Success,
    Danger,
}

impl ToastVariant {
    pub const ALL: &'static [Self] = &[
        Self::Neutral,
        Self::Info,
        Self::Warning,
        Self::Success,
        Self::Danger,
    ];
}

impl std::fmt::Display for ToastVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Neutral => write!(f, "Neutral"),
            Self::Info => write!(f, "Info"),
            Self::Warning => write!(f, "Warning"),
            Self::Success => write!(f, "Success"),
            Self::Danger => write!(f, "Danger"),
        }
    }
}

impl From<ToastVariant> for widgets::toast::Variant {
    fn from(variant: ToastVariant) -> Self {
        match variant {
            ToastVariant::Neutral => Self::Neutral,
            ToastVariant::Info => Self::Info,
            ToastVariant::Warning => Self::Warning,
            ToastVariant::Success => Self::Success,
            ToastVariant::Danger => Self::Danger,
        }
    }
}

/// A corner or edge of the viewport a surface is anchored to.
///
/// `OverlayPosition` covers only the four sides, which is all a popover or a
/// menu needs. A layer and a toast stack can also sit in a corner or the
/// middle, so they take this instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OverlayAnchor {
    TopLeft,
    Top,
    TopRight,
    Left,
    #[default]
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl OverlayAnchor {
    pub const ALL: &'static [Self] = &[
        Self::TopLeft,
        Self::Top,
        Self::TopRight,
        Self::Left,
        Self::Center,
        Self::Right,
        Self::BottomLeft,
        Self::Bottom,
        Self::BottomRight,
    ];
}

impl std::fmt::Display for OverlayAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TopLeft => write!(f, "Top Left"),
            Self::Top => write!(f, "Top"),
            Self::TopRight => write!(f, "Top Right"),
            Self::Left => write!(f, "Left"),
            Self::Center => write!(f, "Center"),
            Self::Right => write!(f, "Right"),
            Self::BottomLeft => write!(f, "Bottom Left"),
            Self::Bottom => write!(f, "Bottom"),
            Self::BottomRight => write!(f, "Bottom Right"),
        }
    }
}

impl From<OverlayAnchor> for widgets::anchor::Anchor {
    fn from(anchor: OverlayAnchor) -> Self {
        match anchor {
            OverlayAnchor::TopLeft => Self::TopLeft,
            OverlayAnchor::Top => Self::Top,
            OverlayAnchor::TopRight => Self::TopRight,
            OverlayAnchor::Left => Self::Left,
            OverlayAnchor::Center => Self::Center,
            OverlayAnchor::Right => Self::Right,
            OverlayAnchor::BottomLeft => Self::BottomLeft,
            OverlayAnchor::Bottom => Self::Bottom,
            OverlayAnchor::BottomRight => Self::BottomRight,
        }
    }
}

/// The path to an `Anchor` variant, for generated code.
pub fn overlay_anchor_code(anchor: OverlayAnchor) -> &'static str {
    match anchor {
        OverlayAnchor::TopLeft => "widgets::anchor::Anchor::TopLeft",
        OverlayAnchor::Top => "widgets::anchor::Anchor::Top",
        OverlayAnchor::TopRight => "widgets::anchor::Anchor::TopRight",
        OverlayAnchor::Left => "widgets::anchor::Anchor::Left",
        OverlayAnchor::Center => "widgets::anchor::Anchor::Center",
        OverlayAnchor::Right => "widgets::anchor::Anchor::Right",
        OverlayAnchor::BottomLeft => "widgets::anchor::Anchor::BottomLeft",
        OverlayAnchor::Bottom => "widgets::anchor::Anchor::Bottom",
        OverlayAnchor::BottomRight => "widgets::anchor::Anchor::BottomRight",
    }
}

/// A named starting point for an overlay widget.
///
/// The widgets crate ships constructors that are the plain widget plus a
/// handful of settings — `dialog()` is `layer()` with a backdrop, `modal()`
/// adds a title bar you can drag and resize by. Picking one here applies that
/// same batch of settings, so a widget can start from the useful shape rather
/// than being assembled a property at a time. Nothing is locked afterwards:
/// every value the preset set stays editable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverlayPreset {
    /// `widgets::layer::layer` — a bare surface, centred, no backdrop.
    LayerPlain,
    /// `widgets::layer::dialog` — centred and dimmed, no chrome.
    LayerDialog,
    /// `widgets::layer::modal` — a dialog with a title bar, draggable and
    /// resizable.
    LayerModal,
    /// `widgets::layer::sheet` — slides out of an edge and spans it.
    LayerSheet,
    /// `widgets::popover::popover` — opens on a press.
    PopoverPress,
    /// `widgets::popover::hover_popover` — opens on hover, and the cursor can
    /// travel into the surface.
    PopoverHover,
}

impl OverlayPreset {
    /// The presets offered for a widget type, or none if it has only one shape.
    pub fn all_for(widget_type: WidgetType) -> &'static [Self] {
        match widget_type {
            WidgetType::Layer => &[
                Self::LayerPlain,
                Self::LayerDialog,
                Self::LayerModal,
                Self::LayerSheet,
            ],
            WidgetType::Popover => &[Self::PopoverPress, Self::PopoverHover],
            _ => &[],
        }
    }
}

impl std::fmt::Display for OverlayPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LayerPlain => write!(f, "layer()"),
            Self::LayerDialog => write!(f, "dialog()"),
            Self::LayerModal => write!(f, "modal()"),
            Self::LayerSheet => write!(f, "sheet()"),
            Self::PopoverPress => write!(f, "popover()"),
            Self::PopoverHover => write!(f, "hover_popover()"),
        }
    }
}

// ─── DataTable ───────────────────────────────────────────────────────────────
// The `advanced-table` data grid, as distinct from `WidgetType::Table` (which
// is iced's own `table`: a static layout with no sorting, selection or
// scrolling of its own).
//
// The widget's own rule throughout is that the *host* owns the state and the
// widget reports what it should become — a selection, a sort, a column move.
// So every one of these that is turned on adds a field to the generated app
// and a message that writes it.

/// How wide a `DataTable` column is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DataTableSizing {
    /// Exact width. Never grows, never shrinks, never takes slack.
    Fixed,
    /// The widest cell in the column, clamped between a floor and a ceiling.
    #[default]
    Fit,
    /// Fits its content first, then takes a weighted share of what is left.
    ///
    /// The minimum is a shrink floor, not a starting width: a `Fill` column
    /// still has to fit what is in it.
    Fill,
}

impl DataTableSizing {
    pub const ALL: &'static [Self] = &[Self::Fixed, Self::Fit, Self::Fill];
}

impl std::fmt::Display for DataTableSizing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fixed => write!(f, "Fixed"),
            Self::Fit => write!(f, "Fit"),
            Self::Fill => write!(f, "Fill"),
        }
    }
}

/// What a `DataTable` does when its columns do not fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DataTableOverflow {
    /// Let the content run past the viewport and scroll it.
    ///
    /// The widget's own default, and deliberately so: shrinking every column
    /// at once truncates text everywhere and reads as broken, where a
    /// horizontal scroll reads as intentional.
    #[default]
    Scroll,
    /// Shrink the columns that can shrink, in proportion to the headroom each
    /// has above its own minimum.
    Shrink,
}

impl DataTableOverflow {
    pub const ALL: &'static [Self] = &[Self::Scroll, Self::Shrink];
}

impl std::fmt::Display for DataTableOverflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scroll => write!(f, "Scroll"),
            Self::Shrink => write!(f, "Shrink columns"),
        }
    }
}

/// Whether a `DataTable` axis shows a scrollbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DataTableScrollPolicy {
    /// Present once the content overflows.
    #[default]
    Auto,
    /// Never shown; the content is clipped.
    Hidden,
}

impl DataTableScrollPolicy {
    pub const ALL: &'static [Self] = &[Self::Auto, Self::Hidden];
}

impl std::fmt::Display for DataTableScrollPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::Hidden => write!(f, "Hidden"),
        }
    }
}

/// How much of a `DataTable` can be selected at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DataTableSelection {
    /// Not selectable. Clicks pass through.
    #[default]
    None,
    /// One at a time.
    Single,
    /// Ctrl/Cmd toggles, Shift extends from the anchor.
    Multiple,
}

impl DataTableSelection {
    pub const ALL: &'static [Self] = &[Self::None, Self::Single, Self::Multiple];

    /// The path to the widget's own mode, for generated code.
    pub fn code(&self) -> &'static str {
        match self {
            Self::None => "advanced_table::SelectionMode::None",
            Self::Single => "advanced_table::SelectionMode::Single",
            Self::Multiple => "advanced_table::SelectionMode::Multiple",
        }
    }

    pub fn is_on(&self) -> bool {
        !matches!(self, Self::None)
    }
}

impl std::fmt::Display for DataTableSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "Off"),
            Self::Single => write!(f, "Single"),
            Self::Multiple => write!(f, "Multiple"),
        }
    }
}

/// One leaf column of a `DataTable`: a field of its struct, and how that
/// field's column behaves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataTableColumn {
    /// The struct field this column shows.
    pub field: String,
    /// Header text. The field name is used when this is blank.
    pub header: String,
    pub sizing: DataTableSizing,
    /// Used by `Fixed`.
    pub width: f32,
    /// Used by `Fill`: its share of the leftover space.
    pub weight: u16,
    /// Shrink floor for `Fit` and `Fill`.
    pub min_width: f32,
    /// Ceiling for `Fit`.
    pub max_width: f32,
    pub align: ContainerAlignX,
    /// Draws a sort control in the header and lets it be clicked.
    pub sortable: bool,
    /// Index into the table's groups, when this column sits under one.
    ///
    /// Columns sharing a group have to be adjacent: a group whose columns are
    /// not contiguous has no rectangle to draw its label in.
    pub group: Option<usize>,
    /// Pins the column against the leading edge so it stays put while the rest
    /// scrolls sideways.
    ///
    /// Only honoured on an unbroken run starting at the first column —
    /// freezing is positional, and a frozen island with scrolling columns on
    /// both sides has nowhere coherent to live. A grouped column takes its
    /// stickiness from the group.
    pub sticky: bool,
}

impl Default for DataTableColumn {
    fn default() -> Self {
        Self {
            field: String::new(),
            header: String::new(),
            sizing: DataTableSizing::Fit,
            width: 120.0,
            weight: 1,
            // The widget's own floor for a `Fill` column.
            min_width: 48.0,
            max_width: 0.0,
            align: ContainerAlignX::Left,
            sortable: false,
            group: None,
            sticky: false,
        }
    }
}

/// A header cell spanning the columns that name it.
///
/// One level of grouping: two header rows, groups above leaves. The widget
/// nests arbitrarily, but a deeper tree needs a tree editor rather than a
/// property grid.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DataTableGroup {
    pub header: String,
    /// Pins the whole group, and with it every column beneath — which is what
    /// keeps a group from being split by the frozen boundary.
    pub sticky: bool,
}

impl From<DataTableOverflow> for advanced_table::Overflow {
    fn from(overflow: DataTableOverflow) -> Self {
        match overflow {
            DataTableOverflow::Scroll => Self::Scroll,
            DataTableOverflow::Shrink => Self::Shrink,
        }
    }
}

impl From<DataTableScrollPolicy> for advanced_table::Policy {
    fn from(policy: DataTableScrollPolicy) -> Self {
        match policy {
            DataTableScrollPolicy::Auto => Self::Auto,
            DataTableScrollPolicy::Hidden => Self::Hidden,
        }
    }
}
