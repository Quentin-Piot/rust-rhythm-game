use bevy::log::info;
use std::fs::File;
use std::io;

#[tokio::main(flavor = "current_thread")]
pub async fn get_song() -> File {
    // chaining .await will yield our query result

    info!("Downloading file");

    let resp = reqwest::get("https://storage.googleapis.com/game-storage-qp/territory2.toml")
        .await
        .unwrap();
    let body = resp.text().await.unwrap();

    info!("Creating config");

    let mut out = File::create("config.toml").expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
    File::open("config.toml").unwrap()
}
