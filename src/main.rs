mod arg_parser;
mod s3fs;

use arg_parser::Args;
use datafusion;
use datafusion::datasource::file_format::parquet::ParquetFormat;
use datafusion::datasource::listing::ListingOptions;
use datafusion::prelude::*;
use s3fs::create_s3fs;
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    let args = Args::parse();
    let url = args.url;
    assert_eq!(url.scheme(), "s3");
    let bucket = url.domain().unwrap();
    let profile = args.aws_profile;

    let fs = create_s3fs(&bucket, profile).await;
    let ctx = SessionContext::new();
    ctx.runtime_env()
        .register_object_store("s3", &bucket, Arc::new(fs));
    ctx.register_listing_table(
        "tbl",
        Into::<String>::into(url),
        ListingOptions::new(Arc::new(ParquetFormat::default())),
        None,
    )
    .await?;
    let df = ctx.sql(&args.query).await?;
    df.show().await?;
    Ok(())
}
