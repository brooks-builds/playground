use crate::schema::{pets, species};
use diesel::{dsl, pg::Pg, prelude::*};

#[derive(Debug, Identifiable, Queryable, Selectable)]
#[diesel(table_name = species)]
#[diesel(check_for_backend(Pg))]
pub struct Species {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Identifiable, Queryable, Selectable, Associations)]
#[diesel(check_for_backend(Pg))]
#[belongs_to(Species)]
pub struct Pet {
    pub id: i32,
    pub name: String,
    pub species_id: i32,
}
