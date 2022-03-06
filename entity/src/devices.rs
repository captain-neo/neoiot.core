//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "devices")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub account_id: String,
    pub schema_id: String,
    pub name: String,
    pub label_version: i64,
    pub is_active: bool,
    pub is_online: bool,
    pub mqtt_username: String,
    pub mqtt_password: String,
    pub is_super_device: bool,
    #[sea_orm(column_type = "Custom(\"jsonb\".to_owned())")]
    pub acl_pubs: Json,
    #[sea_orm(column_type = "Custom(\"jsonb\".to_owned())")]
    pub acl_subs: Json,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::accounts::Entity",
        from = "Column::AccountId",
        to = "super::accounts::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Accounts,
    #[sea_orm(
        belongs_to = "super::schemas::Entity",
        from = "Column::SchemaId",
        to = "super::schemas::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Schemas,
    #[sea_orm(has_many = "super::labels_device_relation::Entity")]
    LabelsDeviceRelation,
    #[sea_orm(has_many = "super::command_request_logs::Entity")]
    CommandRequestLogs,
    #[sea_orm(has_many = "super::device_connections::Entity")]
    DeviceConnections,
}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Accounts.def()
    }
}

impl Related<super::schemas::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Schemas.def()
    }
}

impl Related<super::labels_device_relation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LabelsDeviceRelation.def()
    }
}

impl Related<super::command_request_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CommandRequestLogs.def()
    }
}

impl Related<super::device_connections::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DeviceConnections.def()
    }
}

impl Related<super::labels::Entity> for Entity {
    fn to() -> RelationDef {
        super::labels_device_relation::Relation::Labels.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::labels_device_relation::Relation::Devices.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
