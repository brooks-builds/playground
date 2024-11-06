use crate::models::{Pet, Species};
use diesel::prelude::*;
use eyre::{Context, Result};

pub fn get_all_pets(db: &mut PgConnection) -> Result<Vec<(Pet, Species)>> {
    use crate::schema::pets::dsl::pets;
    use crate::schema::species::dsl::species;

    pets.inner_join(species)
        .select((Pet::as_select(), Species::as_select()))
        .load(db)
        .context("getting all pets with their species")
}
