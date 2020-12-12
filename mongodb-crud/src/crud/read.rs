use mongodb::{bson::{doc, Bson, Document}, Client, Database, Cursor};
use std::fmt::Debug;

pub async fn get_collection_of_db(client: &Client, dbname: &str) -> ()
{
    let db: Database
        = client.database(dbname);

    for collection in db.list_collection_names(None).await {
        println!("Collection: {:?}", collection)
    }
}

pub async fn get_databases(client: &Client)
{
    // List the names of the databases in that cluster
    let collections = client.list_database_names(None, None).await;

    for db_name in Some(collections) {
        println!("{:?}", db_name);
    }
}

pub async fn find_all(client: &Client, dbname: &str, collection: &str)
{
    let coll = client.database(dbname).collection(collection);
    let cursor = coll.find(None, None).await;

    match cursor {
        Ok(cursor) => {
            println!("{:?}", cursor);
        }
        Err(e) => {println!("err {:?}", e)},
    }
}

pub async fn find_by(client: &Client, dbname: &str, collection: &str, doc: Document)
{
    let coll = client.database(dbname).collection(collection);
    for item in coll.find(doc, None).await {
        let i = Some(item);
        println!("{:?}", i.unwrap());
    }
}

pub async fn find_one_by(client: &Client, dbname: &str, collection: &str, doc: Document)
{
    let coll = client.database(dbname).collection(collection);
    let item = coll.find_one(doc, None).await;

    println!("{:?}", item);
}