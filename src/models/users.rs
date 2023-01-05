use chrono::prelude::*;
use entity::users::{Entity as Users, self};
use migration::{DbErr};
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
        let filter = users::Column::DiscordId;
        match Users::find()
            .filter(filter.eq(id))
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
    async fn create(discord_id: &i64, user_name: &String, admin: &bool, premium: &bool, db: &DatabaseConnection) -> Result<users::ActiveModel, DbErr> {
        Self::save(Self::new(discord_id, user_name, admin, premium), db).await
    }
    fn new(discord_id: &i64, user_name: &String, admin: &bool, premium: &bool) -> users::ActiveModel {
        users::ActiveModel {
            id: ActiveValue::NotSet,
            admin: ActiveValue::Set(admin.clone()),
            created_at: ActiveValue::Set(Utc::now().date_naive()),
            discord_id: ActiveValue::Set(discord_id.clone()),
            premium: ActiveValue::Set(premium.clone()),
            user_name: ActiveValue::Set(user_name.clone()),
        }
    }

    async fn save(user: users::ActiveModel, db: &DatabaseConnection) -> Result<users::ActiveModel, DbErr> {
        user.save(db).await
    }
}

impl Create for users::ActiveModel{}