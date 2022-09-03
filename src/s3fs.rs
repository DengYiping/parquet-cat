use aws_config::meta::region::RegionProviderChain;
use aws_config::profile::{ProfileFileCredentialsProvider, ProfileFileRegionProvider};
use aws_config::provider_config::ProviderConfig;
use aws_types::credentials::ProvideCredentials;
use aws_types::region::Region;

use object_store::aws::{AmazonS3, AmazonS3Builder};

pub async fn create_s3fs(bucket_name: &str, profile: Option<String>) -> AmazonS3 {
    let profile_region_provider = match &profile {
        Some(profile_name) => ProfileFileRegionProvider::builder()
            .profile_name(profile_name)
            .build(),
        None => ProfileFileRegionProvider::builder().build(),
    };
    let region = RegionProviderChain::first_try(profile_region_provider)
        .or_else(Region::new("us-east-1"))
        .region()
        .await
        .expect("Cannot parse region!");


    let creds_provider = match &profile {
        Some(profile_name) => ProfileFileCredentialsProvider::builder()
            .configure(&ProviderConfig::default().with_region(Some(region.clone())))
            .profile_name(profile_name)
            .build(),
        None => ProfileFileCredentialsProvider::builder()
            .configure(&ProviderConfig::default().with_region(Some(region.clone())))
            .build(),
    };


    let creds = creds_provider
        .provide_credentials()
        .await
        .expect("Cannot parse AWS credentials");
    let aws_builder = AmazonS3Builder::new()
        .with_region(region.as_ref())
        .with_bucket_name(bucket_name)
        .with_access_key_id(creds.access_key_id())
        .with_secret_access_key(creds.secret_access_key());
    match creds.session_token() {
        Some(session_token) => aws_builder.with_token(session_token).build().unwrap(),
        None => aws_builder.build().unwrap(),
    }
}
