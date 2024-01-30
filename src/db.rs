use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::path::PathBuf;

use sea_orm::{
    Database as SeaOrmDatabase,
    DatabaseConnection,
    DbErr,
};
use crate::migration::{Migrator, MigratorTrait};

#[derive(Debug)]
pub enum Error {
    Database(DbErr),
    File(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(ref err) => write!(f, "Database error: {}", err),
            Self::File(ref err) => write!(f, "File error: {}", err)
        }
    }
}

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        Self::Database(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::File(err)
    }
}

async fn get_db_pool(path: &PathBuf) -> Result<DatabaseConnection, Error> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let db_str = format!("sqlite:{}", path.display());
    let pool = SeaOrmDatabase::connect(&db_str).await?;
    Ok(pool)
}

#[derive(Clone)]
pub struct Database {
    pool: DatabaseConnection,
}

impl Database {
    pub async fn new(db_path: &PathBuf) -> Result<Self, Error> {
        get_db_pool(db_path).await.map(|pool| Self { pool })
    }

    pub async fn apply_migrations(&self) -> Result<(), Error> {
        Ok(Migrator::up(&self.pool, None).await?)
    }
}
