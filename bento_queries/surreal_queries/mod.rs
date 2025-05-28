use crate::listtobin::vectype::VecType;
use crate::TradeMsg;
use fnv::FnvHashMap;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Response, Surreal};
use time::Instant;

#[derive(Deserialize, Serialize, Debug)]
pub struct Hd {
    instrument_id: u32,
    length: u8,
    publisher_id: u32,
    rtype: u32,
    ts_event: u64,
}

async fn create_surrealdb() -> surrealdb::Result<()> {
    todo!()
}

pub async fn upload_to_surreal_db<'a>(
    trade_vec: Vec<TradeMsg>,
    usern: &str,
    passw: &str,
    nm_spc: &str,
    data_b: &str,
    tbl: &'a str,
) -> surrealdb::Result<()> {
    // Connect to the server
    //DB.connect::<Mem>(()).await?;
    // Signin as a namespace, database, or root user
    // Start the timer
    let start = Instant::now();
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.signin(Root {
        username: usern,
        password: passw,
    })
    .await?;
    // Select a specific namespace / database
    db.use_ns(nm_spc).use_db(data_b).await?;
    let iter = trade_vec.into_iter();
    // Use the iterator to iterate over the elements
    // Assuming iter yields items of type Trade
    println!("The Iterator length is {:?}",iter.len());
    for value in iter {
        let db_clone = db.clone();
        let tbl_clone = String::from(tbl);
        tokio::spawn(async move {
            let _: TradeMsg = db_clone
                .create(tbl_clone)
                .content(value)
                .await
                .expect("Failed to perform create")
                .expect("Failed to deserialize TradeMsg");
        })
        .await
        .unwrap();
    }
    // Stop the timer
    let duration = start.elapsed();
    println!("Time elapsed for the loop is: {:?}", duration);
    Ok(())
}

pub async fn query_surr_flex_hashmap(
    nm_space: String,
    dat_base: String,
    table: String,
    variables: Vec<String>,
) -> surrealdb::Result<FnvHashMap<String, VecType>> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns(nm_space).use_db(dat_base).await?;

    // Format the variables to create the SELECT part of the query
    let select_vars = variables.join(", ");
    let query = format!("SELECT {} FROM type::table($table)", select_vars);

    let mut response: Response = db.query(&query).bind(("table", table)).await?;

    // Create a new HashMap with String as the key and VecType as the value
    let mut map: FnvHashMap<String, VecType> = FnvHashMap::default();

    // Helper macro to simplify repetitive extraction and insertion of vectors into the map
    macro_rules! extract_and_insert {
        ($response:expr, $map:expr, $key:expr, $variant:ident) => {
            match $response.take($key) {
                Ok(value) => {
                    $map.insert($key.to_string(), VecType::$variant(value));
                }
                Err(e) => {
                    eprintln!("Failed to extract {}: {}", $key, e);
                    return Err(e.into());
                }
            }
        };
    }

    // Extract individual columns and insert them into the HashMap
    for var in &variables {
        match var.as_str() {
            "action" | "depth" | "flags" | "side" | "length" => {
                extract_and_insert!(response, map, var.as_str(), U8Vec);
            }
            "size" | "instrument_id" | "publisher_id" | "rtype" => {
                extract_and_insert!(response, map, var.as_str(), U32Vec);
            }
            "ts_in_delta" => {
                extract_and_insert!(response, map, var.as_str(), I32Vec);
            }
            "price" | "sequence" => {
                extract_and_insert!(response, map, var.as_str(), I64Vec);
            }
            "ts_recv" | "ts_event" => {
                extract_and_insert!(response, map, var.as_str(), U64Vec);
            }
            _ => {
                eprintln!("Unsupported variable: {}", var);
            }
        }
    }

    Ok(map)
}
