//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.13

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_auth_tbl")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_no: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub login_channel: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub token_id: String,
    pub reg_dt: DateTime,
    pub reg_id: String,
    pub chg_dt: Option<DateTime>,
    pub chg_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
