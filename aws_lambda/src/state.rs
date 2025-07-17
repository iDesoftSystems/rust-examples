use sea_orm::DbConn;

#[derive(Clone)]
pub struct AppContext {
    pub conn: DbConn,
}
