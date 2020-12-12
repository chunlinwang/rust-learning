use mongodb::{bson::doc, Client};
use mongodb::results::DeleteResult;

pub async fn delete(client: &Client, dbname: &str, coll: &str) -> mongodb::error::Result<DeleteResult>
{
    let db = client.database(dbname);

    let coll = db.collection(coll);

    coll.delete_one(doc! { "name": 0}, None).await
}