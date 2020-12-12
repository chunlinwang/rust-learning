use mongodb::{bson::doc, Client};
use tokio::task;
use chrono::{DateTime, Duration, Utc};

pub async fn insert_many_async(client: &Client, dbname: &str, collection: &str) -> ()
{
    let coll = client.database(dbname).collection(collection);

    for i in 0..5 {
        let coll_ref = coll.clone();

        task::spawn(async move {
            // Perform operations with `coll_ref`. For example:
            coll_ref.insert_one(doc! { "name": i }, None).await;
        });
    }
}

pub async fn insert(client: &Client, dbname: &str, collection: &str) -> ()
{
    let coll = client.database(dbname).collection(collection);

    let now: i64 = Utc::now().timestamp();
    coll.insert_one(doc! {"name": "rust", "date": now}, None).await;
}