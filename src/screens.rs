use ratcn::runtime::DeclareCtx;

use crate::app::{AppMsg, AppState};

mod layout;

pub mod history;
pub mod result;
pub mod settings;
pub mod typing;

type Ctx<'a> = DeclareCtx<'a, AppState, AppMsg>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurrentScreen {
    #[default]
    Typing,
    Result,
    Settings,
    History,
}
