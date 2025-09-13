use sqlx::{PgPool, Pool, Postgres};

pub struct CountryRepository {
    pool: PgPool,
}

impl CountryRepository {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        CountryRepository { pool: pool.clone() }
    }
}
pub trait CountryRepositoryExt {}
