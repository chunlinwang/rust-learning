use dotenv::dotenv;
use std::env::var;

pub fn get_mongo_db_url() -> String {
    dotenv().ok();
    let url: String = var("MONGODB_URL").expect("MONGODB URL is not set");
    let username: String = var("MONGODB_USERNAME").expect("MONGODB_USERNAME is not set");
    let password: String = var("MONGODB_PASSWORD").expect("MONGODB_PASSWORD is not set");
    let cluster_url: String = var("MONGODB_CLUSTER_URL").expect("MONGODB_CLUSTER_URL is not set");
    let dbname: String = var("MONGODB_DBNAME").expect("MONGODB_DBNAME is not set");

    url.replace("<username>", &username)
        .replace("<password>", &password)
        .replace("<cluster-url>", &cluster_url)
        .replace("<dbname>", &dbname)
}