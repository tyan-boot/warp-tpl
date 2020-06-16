use crate::errors::AppError;

#[derive(Clone)]
pub struct Db;

#[allow(dead_code)]
impl Db {
    pub fn query(&self) {}
}

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}

impl AppState {
    pub fn create() -> Result<Self, AppError> {
        Ok(AppState { db: Db })
    }
}

macro_rules! with_state {
    ($state:ident) => {{
        let state = $state.clone();
        warp::any().map(move || state.clone())
    }};
}
