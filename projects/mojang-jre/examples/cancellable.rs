use mojang_jre::jre::VersionType;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().pretty())
        .with(EnvFilter::from_default_env())
        .init();

    let mut jre = mojang_jre::MojangJre::new(VersionType::Legacy, "./tmp", None);

    let task = tokio::task::spawn(async move {
        println!("Hello from a task!");
        jre.download().await.expect("Failed to download JRE");
    });

    tokio::select! {
        _ = task => {},
        _ = tokio::signal::ctrl_c() => {
            println!("Timeout reached");
        }
    }
}
