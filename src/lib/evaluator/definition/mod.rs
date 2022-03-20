pub mod def;
pub mod math;
pub mod misc;
mod prelude;

use crate::lib::evaluator::{EvalResult, EvalState};
use crate::lib::token::Token;

use std::sync::Arc;

type Handler = Arc<Box<dyn Fn(&mut EvalState, Token) -> EvalResult + Send + Sync>>;

#[derive(Clone)]
pub struct Definition {
    pub signature: Token,
    handler: Handler,
}

impl Definition {
    pub fn new(signature: Token, handler: Handler) -> Self {
        Definition { signature, handler }
    }

    pub fn run(&self, state: &mut EvalState, token: Token) -> EvalResult {
        (*self.handler)(state, token)
    }
}

pub trait Module {
    fn load() -> Vec<Definition>;
}

#[macro_export]
macro_rules! handler {
    ($f:expr) => {
        Arc::new(Box::new($f))
    };
}
