mod setup;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db = setup::get_db_connection().await.expect("DB connection failed");

    rocket::build()
        .manage(db)
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}
