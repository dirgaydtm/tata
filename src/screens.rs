use ratcn::runtime::DeclareCtx;

use crate::app::{AppMsg, AppState};

pub mod typing;

type Ctx<'a> = DeclareCtx<'a, AppState, AppMsg>;
