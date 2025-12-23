#[derive(Debug, Clone)]
pub enum StyleChoice {
    Default,
    Custom(String), // Name/ID of custom style
    
    // For buttons, we can keep the predefined ones
    ButtonPrimary,
    ButtonSecondary,
    ButtonSuccess,
    ButtonDanger,
    ButtonText,
}