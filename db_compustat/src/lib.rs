pub mod finance_data_structs;
use std::sync::LazyLock;
use surrealdb::engine::local::Db;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

pub static DB: LazyLock<Surreal<Db>> = LazyLock::new(Surreal::init);
