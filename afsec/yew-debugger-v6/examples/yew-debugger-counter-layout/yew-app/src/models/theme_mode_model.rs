use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

impl From<ThemeMode> for bool {
    fn from(value: ThemeMode) -> Self {
        match value {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
        }
    }
}

impl From<ThemeMode> for &str {
    fn from(value: ThemeMode) -> Self {
        match value {
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
        }
    }
}
