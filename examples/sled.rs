#[cfg(feature = "sled")]
use keyv::{adapter::sled::SledStoreBuilder, Keyv};

#[cfg(feature = "sled")]
#[tokio::main]
async fn main() {
    let store = SledStoreBuilder::new()
        .db_name("~/.db/keyv-sled-test")
        .build()
        .await
        .unwrap();

    let keyv = Keyv::try_new(store).await.unwrap();

    keyv.set("number", 42).await.unwrap();
    keyv.set("number", 10).await.unwrap();
    keyv.set("array", vec!["hola", "test"]).await.unwrap();
    keyv.set("string", "life long").await.unwrap();

    let number: i32 = keyv
        .get("number")
        .await
        .unwrap()
        .map(|v| serde_json::from_value(v).unwrap())
        .unwrap();
    println!("number: {}", number);

    let string: String = keyv
        .get("string")
        .await
        .unwrap()
        .map(|v| serde_json::from_value(v).unwrap())
        .unwrap();
    println!("string: {}", string);

    let array: Vec<String> = keyv
        .get("array")
        .await
        .unwrap()
        .map(|v| serde_json::from_value(v).unwrap())
        .unwrap();
    println!("array: {:?}", array);
}

#[cfg(not(feature = "sled"))]
fn main() {
    println!("This example requires the 'sled' feature to be enabled.");
    println!("Please run the command as follows:");
    println!("cargo run --example sled --features sled");
}
