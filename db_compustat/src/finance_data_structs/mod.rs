pub mod compustat;
use crate::Mem;
use compustat::CompQtrly;
use csv::ReaderBuilder;
use futures::future::join_all;
use std::fs::File;
use std::path::Path;
use surrealdb::opt::auth::Root;
use tokio::task;
use crate::DB;
use std::io::Error as IoError;
use surrealdb::Error as SurrealError;
use csv::Error as CsvError;

// Define an enum that will handle different error types
#[derive(Debug)]
pub enum AppError {
    Surreal(SurrealError),
    Csv(CsvError),
    IoError(String),
}


// Implement From for surrealdb::Error
impl From<SurrealError> for AppError {
    fn from(err: SurrealError) -> Self {
        AppError::Surreal(err)
    }
}

// Implement From for csv::Error
impl From<CsvError> for AppError {
    fn from(err: CsvError) -> Self {
        AppError::Csv(err)
    }
}

// Implement From for std::io::Error
impl From<IoError> for AppError {
    fn from(err: IoError) -> Self {
        AppError::IoError(err.to_string())
    }
}

pub async fn read_compustat(
    begin_path: String,
    file_name: String,
) -> Result<Vec<CompQtrly>, AppError> {
    println!("{:?}", "hello");
    let path = Path::new(&begin_path);
    let new_path = path.join(file_name);
    println!("{:?}", new_path);
    let file = File::open(new_path)?;

    let mut main_vec: Vec<CompQtrly> = Vec::new();
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    for results in rdr.deserialize() {
        let record: CompQtrly = results?;
        main_vec.push(record);
    }
    Ok(main_vec)
}

pub async fn create_compustat(
    data_vec: Vec<CompQtrly>,
    _username: &str,
    _password: &str,
) {

    //DB.connect::<Mem>(()).await.unwrap();
    // Sign into the database
    /*let _ = DB.signin(Root {
        username,
        password,
    })
    .await.unwrap();*/
    // Select namespace and database to use
    DB.use_ns("compustat").use_db("quarterly").await.unwrap();

    // Set the batch size
    let batch_size = 20;

    // Vectorized iteration over chunks of the data_vec
    let chunked_futures: Vec<_> = data_vec
        .chunks(batch_size)  // Split the data into chunks of `batch_size`
        .map(|chunk| {
            // Move the tokio::spawn outside and create a task for each chunk
            let db_clone = &DB; // Clone the db for each task
            // Move the chunk into the async block so the data is owned by the task
            let chunk = chunk.to_vec(); // Convert the slice to an owned Vec
            task::spawn(async move {
                // For each record in the chunk, create a DB entry (but now inside the tokio::spawn)
                for record in chunk {
                    if let Err(e) = db_clone.create::<Option<CompQtrly>>("company_qtr_yr")
                        .content(record.clone())  // Clone the record to satisfy the borrow checker
                        .await
                    {
                        eprintln!("SurrealDB error: {:?}", e);  // Handle SurrealDB error
                    }
                }
            })
        })
        .collect(); // Collect the spawned tasks for each chunk

    // Await all chunks (i.e., all the tokio tasks)
    let results = join_all(chunked_futures).await;

    // Handle any task errors
    for result in results {
        if let Err(e) = result {
            eprintln!("Tokio task failed: {:?}", e); // Handle any task failures
        }
    } // Return the database connection
}