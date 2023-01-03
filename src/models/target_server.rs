use entity::target_server::{Entity as TargetServer, self};
use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait, ActiveValue, ActiveModelTrait};

// Find by primary key
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
}

#[async_trait::async_trait]
pub trait Create {
    async fn create(user_id: &i32, address: &String, port: &i32, auth: &bool, auth_key: &Option<i32>, db: &DatabaseConnection) -> Result<target_server::ActiveModel, DbErr> {
        Self::save(Self::new(user_id, address, port, auth, auth_key), db).await
    }
    fn new(user_id: &i32, address: &String, port: &i32, auth: &bool, auth_key: &Option<i32>) -> target_server::ActiveModel {
        target_server::ActiveModel {
            id: ActiveValue::NotSet,
            user_id: ActiveValue::Set(user_id.clone()),
            address: ActiveValue::Set(address.clone()),
            port: ActiveValue::Set(port.clone()),
            auth: ActiveValue::Set(auth.clone()),
            auth_key: ActiveValue::Set(auth_key.clone()),
        }
    }
    async fn save(target_server: target_server::ActiveModel, db: &DatabaseConnection) -> Result<target_server::ActiveModel, DbErr> {
        target_server.save(db).await
    }
}

impl Create for target_server::ActiveModel{}