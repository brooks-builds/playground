use diesel_playground::{connect_to_db, queries::get_all_pets};
use eyre::{Context, Result};

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").context("getting DATABASE_URL environment variable")?;
    let db_connection = &mut connect_to_db(&database_url)?;
    let pets = get_all_pets(db_connection)?;

    println!("Pets");
    println!("id\tname");
    println!("------------");

    for (pet, species) in pets {
        println!("{}\t{}", pet.name, species.name);
    }

    Ok(())
}
