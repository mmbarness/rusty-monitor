use chrono::prelude::*;
use entity::users::{Entity as Users, self};
use migration::{DbErr};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveValue, ActiveModelTrait, QueryFilter, ColumnTrait};
use crate::models::target_server::Find as FindTargetServer;
use sea_orm::TryIntoModel;
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

    async fn is_user_all_set(discord_id: i64, db: &DatabaseConnection) -> Option<(entity::users::Model, entity::target_server::Model)> {
        let user = match Self::find_by_discord_id(discord_id, db).await {
            Some(u) => u,
            None => return None
        };

        let target_server = match entity::target_server::ActiveModel::find_by_user_model(&user, db).await {
            Some(u) => match u.try_into_model() {
                Ok(m) => m,
                Err(e) => return None
            },
            None => return None
        };
        Some((user, target_server))
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
