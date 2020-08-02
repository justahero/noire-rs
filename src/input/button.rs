use super::Button;

/// Stores the state of a button
#[derive(Copy, Clone, Debug)]
pub enum ButtonState {
    /// Button is pressed
    Press,
    /// Button is released
    Release,
}

/// Argument struct for a Button
#[derive(Copy, Clone, Debug)]
pub struct ButtonArgs {
    /// the press state of a button
    pub state: ButtonState,
    /// the Button that changed state
    pub button: Button,
}
