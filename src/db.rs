use std::fmt::{Display, Formatter};

use sea_orm::{Database as SeaOrmDatabase, DatabaseConnection, DbErr, EntityTrait, sea_query, Set};

use crate::entity::user;
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

async fn get_db_pool(url: &String) -> Result<DatabaseConnection, Error> {
    let pool = SeaOrmDatabase::connect(url).await?;
    Ok(pool)
}

#[derive(Clone)]
pub struct Database {
    pool: DatabaseConnection,
}

impl Database {
    pub async fn new(url: &String) -> Result<Self, Error> {
        get_db_pool(url).await.map(|pool| Self { pool })
    }

    pub async fn apply_migrations(&self) -> Result<(), Error> {
        Ok(Migrator::up(&self.pool, None).await?)
    }

    pub async fn upsert_user(
        &self,
        telegram_id: u64,
        first_name: String,
        last_name: Option<String>,
        username: Option<String>,
        timezone: Option<String>,
    ) -> Result<(), Error> {
        let new_user = user::ActiveModel {
            telegram_id: Set(telegram_id),
            username: Set(username),
            first_name: Set(first_name),
            last_name: Set(last_name),
            timezone: Set(timezone),
            ..Default::default()
        };

        user::Entity::insert(new_user)
            .on_conflict(
                sea_query::OnConflict::column(user::Column::TelegramId)
                    .update_columns([
                        user::Column::Username,
                        user::Column::FirstName,
                        user::Column::LastName,
                        user::Column::Timezone,
                    ])
                    .to_owned()
            )
            .exec_without_returning(&self.pool)
            .await?;

        Ok(())
    }
}
