use mongodb::{bson::doc, options::ClientOptions, Client};

mod vars;
mod crud;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let mongo_db_url: String = vars::get_mongo_db_url();

    println!("{}", mongo_db_url);

    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse(mongo_db_url.as_str())
            .await?;

    // Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    crud::read::get_databases(&client).await;
    crud::read::get_collection_of_db(&client, "rust").await;
    crud::create::insert_many_async(&client, "rust", "rust").await;
    crud::create::insert(&client, "rust", "rust").await;
    crud::update::update(&client, "rust", "rust").await;
    crud::read::find_all(&client, "rust", "rust").await;
    crud::read::find_by(&client, "rust", "rust", doc! {"name": "rust"}).await;
    crud::read::find_one_by(&client, "rust", "rust", doc! {"name": "rust"}).await;

    crud::delete::delete(&client, "rust", "rust").await;

    Ok(())
}