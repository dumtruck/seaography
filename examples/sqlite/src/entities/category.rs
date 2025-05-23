use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_id: i16,
    pub name: String,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::film_category::Entity")]
    FilmCategory,
}

impl Related<super::film_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmCategory.def()
    }
}

impl Related<super::film::Entity> for Entity {
    fn to() -> RelationDef {
        super::film_category::Relation::Film.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::film_category::Relation::Category.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::film_category::Entity")]
    FilmCategory,
    #[sea_orm(entity = "super::film::Entity")]
    Film,
}
