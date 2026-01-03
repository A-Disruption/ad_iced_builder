use iced::{Alignment, ContentFit, Font, Length};
use iced::mouse::Interaction;
use iced::widget::{text, tooltip, scrollable};

use super::types::*;

// Display implementations
impl std::fmt::Display for WidgetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

impl std::fmt::Display for RowColumnAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
        match self { Orientation::Horizontal => write!(f, "Horizontal"),
                     Orientation::Vertical   => write!(f, "Vertical"), }
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

#[derive(Debug, Clone, Copy, PartialEq,)]
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

#[derive(Debug, Clone, Copy, PartialEq,)]
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


#[derive(Debug, Clone, Copy, PartialEq,)]
pub enum TextWrapping {
    None,
    Word,
    Glyph,
    WordOrGlyph
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

#[derive(Debug, Clone, Copy, PartialEq,)]
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

#[derive(Debug, Clone, Copy, PartialEq,)]
pub enum ContainerAlignX { Left, Center, Right }

#[derive(Debug, Clone, Copy, PartialEq,)]
pub enum ContainerAlignY { Top, Center, Bottom }

#[derive(Debug, Clone, Copy, PartialEq,)]
pub enum RowColumnAlign { Start, Center, End }

#[derive(Debug, Clone, Copy, PartialEq,)]
pub enum ButtonStyleType { Primary, Secondary, Success, Danger, Text, Background, Subtle }

impl ButtonStyleType{
    pub fn all() -> Vec<String> {
        vec![
        "Primary".to_string(), 
        "Secondary".to_string(), 
        "Success".to_string(), 
        "Danger".to_string(), 
        "Text".to_string(), 
        "Background".to_string(), 
        "Subtle".to_string()
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
            _ => None           
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq,)]
pub enum ContainerStyleType { Transparent, Background, RoundedBox, BorderedBox, Dark, Primary, Secondary, Success, Danger, Warning }

impl ContainerStyleType{
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
            _ => None           
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq,)]
pub enum FontType { Default, Monospace }

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub enum Orientation { Horizontal, Vertical }



#[derive(Debug, Clone, Copy, PartialEq,)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub enum DirChoice { Vertical, Horizontal, Both }
impl std::fmt::Display for DirChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self { DirChoice::Vertical => "Vertical", DirChoice::Horizontal => "Horizontal", DirChoice::Both => "Both" })
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
            DirChoice::Vertical   => iced::widget::scrollable::Direction::Vertical(scrollable::Scrollbar::default()),
            DirChoice::Horizontal => iced::widget::scrollable::Direction::Horizontal(scrollable::Scrollbar::default()),
            DirChoice::Both       => iced::widget::scrollable::Direction::Both { 
                vertical: scrollable::Scrollbar::default(), 
                horizontal: scrollable::Scrollbar::default() 
            }
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub enum AnchorChoice { Start, End }
impl std::fmt::Display for AnchorChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self { AnchorChoice::Start => "Start", AnchorChoice::End => "End" })
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
            AnchorChoice::Start   => iced::widget::scrollable::Anchor::Start,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub enum ContentFitChoice { Contain, Cover, Fill, ScaleDown, None }
impl std::fmt::Display for ContentFitChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ContentFitChoice::*;
        write!(f, "{}", match self { Contain=>"Contain", Cover=>"Cover", Fill=>"Fill", ScaleDown=>"ScaleDown", None=>"None" })
    }
}
impl From<ContentFit> for ContentFitChoice {
    fn from(f: ContentFit) -> Self {
        use ContentFit::*;
        match f { Contain=>Self::Contain, Cover=>Self::Cover, Fill=>Self::Fill, ScaleDown=>Self::ScaleDown, None=>Self::None }
    }
}
impl From<ContentFitChoice> for ContentFit {
    fn from(c: ContentFitChoice) -> Self {
        use ContentFitChoice::*;
        match c { Contain=>ContentFit::Contain, Cover=>ContentFit::Cover, Fill=>ContentFit::Fill, ScaleDown=>ContentFit::ScaleDown, None=>ContentFit::None }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub enum TooltipPosition { Top, Bottom, Left, Right, FollowCursor }
impl std::fmt::Display for TooltipPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TooltipPosition::*;
        write!(f, "{}", match self { Top=>"Top", Bottom=>"Bottom", Left=>"Left", Right=>"Right", FollowCursor=>"Follow Cursor" })
    }
}
impl From<TooltipPosition> for tooltip::Position {
    fn from(p: TooltipPosition) -> Self {
        use TooltipPosition::*;
        match p { Top=>tooltip::Position::Top, Bottom=>tooltip::Position::Bottom, Left=>tooltip::Position::Left, Right=>tooltip::Position::Right, FollowCursor=>tooltip::Position::FollowCursor }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerSizingMode {
    Manual,     // User sets width/height separately
    CenterX,    // Use center_x(length)
    CenterY,    // Use center_y(length)
    Center,     // Use center(length)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
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
    Shrink,
    Fixed,
}

impl std::fmt::Display for LengthChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthChoice::Fill => write!(f, "Fill"),
            LengthChoice::FillPortion => write!(f, "FillPortion"),
            LengthChoice::Shrink => write!(f, "Shrink"),
            LengthChoice::Fixed => write!(f, "Fixed"),
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
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub fn length_to_string(length: Length) -> String {
    match length {
        Length::Fill => "Fill".to_string(),
        Length::Shrink => "Shrink".to_string(),
        Length::Fixed(pixels) => format!("{}", pixels),
        Length::FillPortion(p) => format!("FillPortion({p})"),
    }
}

pub fn parse_length(value: &str) -> Length {
    match value.to_lowercase().as_str() {
        "fill" => Length::Fill,
        "shrink" => Length::Shrink,
        _ => {
            if let Ok(pixels) = value.parse::<f32>() {
                Length::Fixed(pixels)
            } else if value.ends_with("px") {
                if let Ok(pixels) = value[..value.len()-2].parse::<f32>() {
                    Length::Fixed(pixels)
                } else {
                    Length::Shrink
                }
            } else {
                Length::Shrink
            }
        }
    }
}