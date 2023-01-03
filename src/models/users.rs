use chrono::prelude::*;
use entity::users::{Entity as Users, self};
use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait, ActiveValue, ActiveModelTrait, QueryFilter, ColumnTrait};

// Find by primary key
#[async_trait::async_trait]
pub trait Find {
    async fn find_by_id(id: i32, db: &DatabaseConnection) -> Option<entity::users::Model> {
        match Users::find_by_id(id).one(db).await {
            Ok(u) => u,
            Err(e) => {
                panic!("unable to connect to db: {}", e);
            }
        }
    }
    async fn find_by_discord_id(id: i64, db: &DatabaseConnection) -> Option<entity::users::Model> {
       match Users::find()
        .filter(users::Column::DiscordId.eq(id))
        .one(db)
        .await {
            Ok(option) => option,
            Err(e) => {
                panic!("unable to connect to db: {}", e);
            }
        }
    }
}

impl Find for users::ActiveModel{}

#[async_trait::async_trait]
pub trait Create {
    async fn create(discord_id: &i64, user_name: &String, db: &DatabaseConnection) -> Result<users::ActiveModel, DbErr> {
        Self::save(Self::new(discord_id, user_name), db).await
    }
    fn new(discord_id: &i64, user_name: &String) -> users::ActiveModel {
        users::ActiveModel {
            id: ActiveValue::NotSet,
            created_at: ActiveValue::Set(Utc::now().date_naive()),
            discord_id: ActiveValue::Set(discord_id.clone()),
            user_name: ActiveValue::Set(user_name.clone()),
        }
    }

    async fn save(user: users::ActiveModel, db: &DatabaseConnection) -> Result<users::ActiveModel, DbErr> {
        user.save(db).await
    }
}

impl Create for users::ActiveModel{}