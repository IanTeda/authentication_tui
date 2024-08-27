
#[derive(Debug, Clone, PartialEq, Eq, strum::Display, serde::Serialize, serde::Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,
}