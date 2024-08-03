use bauxite::minecraft::vanilla::VanillaVersionBuilder;
use tracing::debug;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().pretty())
        .with(EnvFilter::from_default_env())
        .init();

    tracing::info!("Hello, world!");

    let vanilla_version = VanillaVersionBuilder::new("1.7.10").build().await.unwrap();
    //let latest_version = VanillaVersionBuilder::latest().build().await.unwrap();

    debug!("{:?}", vanilla_version);

    let instance = bauxite::InstanceBuilder::new(vanilla_version)
        .with_output_dir("tmp")
        .build();

    let updater = bauxite::Updater::new(instance);
    updater.update().await.unwrap();

    /*let auth_info = bauxite::AuthInfo::from_token("access_token", "username");

    let launcher = bauxite::LauncherBuilder::new(instance)
        .with_java_path("java")
        .with_max_memory(bauxite::Memory::Gigabytes(2))
        .with_auth_info(auth_info)
        .build();
    launcher.launch(auth_info).await.unwrap();*/
}
