use std::num::NonZeroU64;
use time::OffsetDateTime;
use databento::{
    dbn::{Schema, TradeMsg},
    historical::{timeseries::GetRangeParams, Client},
    HistoricalClient,
};

pub async fn authent_bento(key: String) -> Result<Client, databento::error::Error> {
    let mut client = HistoricalClient::builder().key(key)?.build()?;
    // Authenticated request
    let datasets = client.metadata().list_datasets(None).await?;
    for dataset in datasets {
        println!("{dataset}");
    }
    Ok(client)
}

pub async fn hist_req_helper(
    dataset: String,
    symbol: String,
    d1: OffsetDateTime,
    d2: OffsetDateTime,
    data_schema: Schema,
    lim_int: NonZeroU64,
    client: Client,
) -> Option<Vec<TradeMsg>> {
    let mut trade_vec: Vec<TradeMsg> = Vec::new();
    let mut decoder = client
        .clone()
        .timeseries()
        .get_range(
            &GetRangeParams::builder()
                .dataset(dataset)
                .date_time_range((d1, d2))
                .symbols(symbol)
                .schema(data_schema)
                .limit(Some(lim_int))
                .build(),
        )
        .await
        .unwrap();
    while let Some(trade) = decoder.decode_record::<TradeMsg>().await.unwrap() {
        //println!("{trade:#?}");
        trade_vec.push(trade.clone());
    }
    Some(trade_vec)
}
