use ratcn::Theme;

use crate::data::ThemeChoice;

pub fn theme_for(choice: ThemeChoice) -> Theme {
    match choice {
        ThemeChoice::Catppuccin => Theme::catppuccin(),
        ThemeChoice::Nord => Theme::nord(),
        ThemeChoice::TokyoNight => Theme::tokyo_night(),
        ThemeChoice::Gruvbox => Theme::gruvbox(),
        ThemeChoice::Terminal => Theme::terminal(),
        ThemeChoice::Dark => Theme::default_dark(),
    }
}
