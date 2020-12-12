use mongodb::{Client, bson::doc};

pub async fn update(client: &Client, dbname: &str, collection: &str) -> ()
{
    let coll = client.database(collection).collection(collection);

    coll.update_one(doc! {"name": "rust"}, doc! {"updated": true}, None).await;
}
