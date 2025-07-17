use crate::state::AppContext;

pub mod create;

pub trait CommandHandler {
    type Output;
    type Error;

    fn handle(self, ctx: &AppContext) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}
