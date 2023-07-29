#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = launcher::rocket().await
        .ignite().await?
        .launch().await?;
    Ok(())
}