use crate::data_structures::*;
use crate::ops::*;
use nalgebra::DMatrix;
#[cfg(feature = "server")]
use surrealdb::engine::remote::ws::Ws;
#[cfg(feature = "server")]
use surrealdb::opt::auth::Root;
#[cfg(feature = "server")]
use surrealdb::{Response, Surreal};
use dioxus::prelude::*;


#[server]
pub async fn query_surr_trademsg_db(
    user_nm: String,
    pass_wrd: String,
    nm_space: String,
    dat_base: String,
    table: String,
) -> Result<MyMatrix, ServerFnError> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.signin(Root {
        username: user_nm.as_str(),
        password: pass_wrd.as_str(),
    })
    .await?;
    db.use_ns(nm_space).use_db(dat_base).await?;
    let query: &str = r#"
    SELECT * FROM type::table($table)
    "#;

    let mut response: Response = db.query(query).bind(("table", table)).await?;
    // Extract individual columns into Vec
    let price: Vec<i64> = response.take("price")?;
    let size: Vec<u32> = response.take("size")?;
    let ts_in_delta: Vec<i32> = response.take("ts_in_delta")?;
    let ts_recv: Vec<u64> = response.take("ts_recv")?;
    let _hd: Vec<Hd> = response.take("hd")?; //ts_event")?;
                                            // Ensure all vectors have the same length
    assert!(price.len() == size.len() && size.len() == ts_in_delta.len());

    let row_count = price.len();
    let col_count = 4;
    // Create a DMatrix with 3 columns from the given vectors
    let data: Vec<f64> = price
        .iter()
        .zip(size.iter())
        .zip(ts_in_delta.iter())
        .zip(ts_recv.iter())
        .flat_map(|(((q, p), s), t)| vec![*q as f64, *p as f64, *s as f64, *t as f64])
        .collect();
    let matrix = DMatrix::from_row_slice(row_count, col_count, &data);
    let mut my_matrix = MyMatrix::from(matrix);
    my_matrix = my_matrix.scale_column(0.000000001, 0).unwrap();
    my_matrix.data = my_matrix.estimate_retuns().unwrap();
    my_matrix.descrips = my_matrix.snapshot().unwrap();
    Ok(my_matrix)
}