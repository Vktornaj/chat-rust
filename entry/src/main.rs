#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = entry::rocket().await
        .ignite().await?
        .launch().await?;
    Ok(())
}