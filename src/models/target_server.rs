use entity::target_server::{Entity as TargetServer, self};
use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait, ActiveValue, ActiveModelTrait, QueryFilter, ColumnTrait, ModelTrait};

#[async_trait::async_trait]
pub trait Find {
    async fn find_by_id(id: i32, db: &DatabaseConnection) -> Option<entity::target_server::Model> {
        match TargetServer::find_by_id(id).one(db).await {
            Ok(u) => u,
            Err(e) => {
                panic!("unable to connect to db: {}", e);
            }
        }
    }

    async fn find_by_user_model(user: &entity::users::Model, db: &DatabaseConnection) -> Option<entity::target_server::Model> {
        match user
            .find_related(TargetServer)
            .one(db)
            .await {
                Ok(u) => u,
                Err(e) => {
                    panic!("unable to connect to db: {}", e);
                }
            }
    }

    async fn find_by_user_id(id: i32, db: &DatabaseConnection) -> Option<entity::target_server::Model> {
        match TargetServer::find()
            .filter(target_server::Column::UserId.eq(id))
            .one(db)
            .await {
                Ok(u) => u,
                Err(e) => {
                    panic!("unable to connect to db: {}", e);
                }
            }
    }

    async fn find_by_discord_id(id: i32, db: &DatabaseConnection) -> Option<entity::target_server::Model> {
        match TargetServer::find()
            .filter(target_server::Column::UserId.eq(id))
            .one(db)
            .await {
                Ok(u) => u,
                Err(e) => {
                    panic!("unable to connect to db: {}", e);
                }
            }
    }
}

impl Find for target_server::ActiveModel{}

#[async_trait::async_trait]
pub trait Create {
    async fn create(
        user_id: &i32,
        server_address: &String,
        server_port: &i32,
        auth: &bool,
        auth_key: &Option<i32>,
        db: &DatabaseConnection) -> Result<target_server::ActiveModel, DbErr> {
        Self::save(Self::new(user_id, server_address, server_port, auth, auth_key), db).await
    }
    fn new(user_id: &i32, server_address: &String, server_port: &i32, auth: &bool, auth_key: &Option<i32>) -> target_server::ActiveModel {
        let target_server = target_server::ActiveModel {
            user_id: ActiveValue::Set(user_id.clone()),
            address: ActiveValue::Set(server_address.clone()),
            port: ActiveValue::Set(server_port.clone()),
            auth: ActiveValue::Set(auth.clone()),
            auth_key: ActiveValue::Set(auth_key.clone()),
            ..Default::default()
        };
        target_server
    }
    async fn save(target_server: target_server::ActiveModel, db: &DatabaseConnection) -> Result<target_server::ActiveModel, DbErr> {
        target_server.save(db).await
    }
}

impl Create for target_server::ActiveModel{}