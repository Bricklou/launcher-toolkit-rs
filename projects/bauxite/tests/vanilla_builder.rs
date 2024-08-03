use bauxite::minecraft::{
    vanilla::{fetch_version_json, VanillaVersionBuilder},
    version::MinecraftVersion,
};
use tokio::task::JoinSet;
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
    for version in manifest.versions.clone() {
        joinset.spawn(async move {
            let res = VanillaVersionBuilder::new(&version.id.clone())
                .build()
                .await;

            (version.id, res)
        });
    }

    let total = joinset.len();
    let mut counter = 0;
    while let Some(res) = joinset.join_next().await {
        counter += 1;

        let (id, res) = res.unwrap();

        if let Err(e) = &res {
            println!("Error for version {}: {:?}", id, e);
        }

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
