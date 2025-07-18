//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.13

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "store_location_info_tbl")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub seq: i32,
    #[sea_orm(column_type = "Decimal(Some((20, 16)))", nullable)]
    pub lat: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((20, 16)))", nullable)]
    pub lng: Option<Decimal>,
    pub reg_id: String,
    pub chg_id: Option<String>,
    pub address: Option<String>,
    pub reg_dt: DateTimeUtc,
    pub chg_dt: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
