use bauxite::minecraft::{
    vanilla::{fetch_version_json, VanillaVersion, VanillaVersionBuilder, VanillaVersionError},
    version::MinecraftVersion,
};
use tokio::task::{self, JoinHandle, JoinSet};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::test]
async fn check_all_versions() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Fetch manifest
    let manifest = fetch_version_json().await.unwrap();

    let mut joinset = JoinSet::new();

    // For all versions in the manifest, try to call the builder
    for version in manifest.versions {
        joinset.spawn(async move { VanillaVersionBuilder::new(&version.id).build().await });
    }

    let total = joinset.len();
    let mut counter = 0;
    while let Some(res) = joinset.join_next().await {
        counter += 1;

        let res = res.unwrap();
        assert!(res.is_ok());
        let vanilla_version = res.unwrap();
        println!(
            "Checked {:?} ({}/{})",
            vanilla_version.name(),
            counter,
            total
        );

        // Sleep
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}
