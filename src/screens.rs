use ratcn::runtime::DeclareCtx;

use crate::app::{AppMsg, AppState};

mod layout;

pub mod result;
pub mod typing;

type Ctx<'a> = DeclareCtx<'a, AppState, AppMsg>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurrentScreen {
    #[default]
    Typing,
    Result,
}
