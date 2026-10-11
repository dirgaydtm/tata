pub const SETTINGS_TABS: usize = 6;

#[derive(Default)]
pub struct SettingsView {
    pub query: String,
    pub selection: usize,
    pub tab: usize,
}
