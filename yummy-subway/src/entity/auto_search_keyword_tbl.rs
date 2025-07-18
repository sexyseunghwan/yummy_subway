//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.13

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "auto_search_keyword_tbl")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub keyword: String,
    pub keyword_weight: i32,
    pub reg_dt: DateTime,
    pub chg_dt: Option<DateTime>,
    pub reg_id: String,
    pub chg_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
