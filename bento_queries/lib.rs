pub mod bento;
pub mod listtobin;
pub mod stochastic_estimation;
pub mod surreal_queries;
use bento::*;
use databento::{
    dbn::{Schema, TradeMsg},
    historical::Client,
};
use fnv::FnvHashMap;
use listtobin::{vectype::*, *};
use std::num::NonZeroU64;
use stochastic_estimation::*;
use surreal_queries::*;
use time::{macros::datetime, OffsetDateTime};

//
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn upload_bento() {
        let client: Client = authent_bento(String::from("db-DvDSjrPnCcQhKCuEyuQaj3e4MhNhk"))
            .await
            .unwrap();
        let data_set: String = String::from("XNAS.ITCH");
        let symbol: String = String::from("SPY");
        let d1: OffsetDateTime = datetime!(2025-05-10 00:00 UTC);
        let d2: OffsetDateTime = datetime!(2025-05-22 00:00 UTC);
        let data_schema: Schema = Schema::Trades;
        let limit_int = NonZeroU64::new(1000).unwrap();
        let trade_vec = hist_req_helper(data_set, symbol, d1, d2, data_schema, limit_int, client)
            .await
            .unwrap();
        println!("{:?}", trade_vec.len());
        upload_to_surreal_db(trade_vec, "root", "root", "equities", "historical", "trade")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn query_surr_matrix() {
        let vars = vec![
            String::from("price"),
            String::from("size"),
            String::from("ts_in_delta"),
            String::from("ts_recv"),
            String::from("side"),
        ];
        let table = String::from("trade");
        let ns = String::from("equities");
        let db = String::from("historical");
        let map: FnvHashMap<String, VecType> =
            query_surr_flex_hashmap(ns, db, table, vars).await.unwrap();
        println!("{:?}", &map);
        let _matrix = MyMatrix::from_hashmap(map.clone());
        iterate_and_match(map).await;
    }
}
