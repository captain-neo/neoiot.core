//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub is_superuser: bool,
    pub last_login_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::schemas::Entity")]
    Schemas,
    #[sea_orm(has_many = "super::devices::Entity")]
    Devices,
    #[sea_orm(has_many = "super::labels::Entity")]
    Labels,
}

impl Related<super::schemas::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Schemas.def()
    }
}

impl Related<super::devices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Devices.def()
    }
}

impl Related<super::labels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Labels.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
