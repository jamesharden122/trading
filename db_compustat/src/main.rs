use surrealdb::engine::remote::ws::Wss;
use db_compustat::DB;
use surrealdb::engine::local::Mem;
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::Client;
use db_compustat::finance_data_structs::*;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to the server
    DB.connect::<Mem>(()).await?;
    let begin_path = String::from("./");
    let file_name = String::from("compustat_quarterly.csv");
    let data = read_compustat(begin_path, file_name).await.unwrap();
    _ = create_compustat(data, "root", "root").await;
    // Signin as a namespace, database, or root user
    //db.signin(Root { username: "root", password: "root",}).await?;
    // Select a specific namespace / databas
    //db.use_ns("test").use_db("test").await?;
    // Create a new person with a random id
    println!("{:?}", "Done");
    Ok(())
}
