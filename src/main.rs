use anyhow::Result;
use serde_json::{to_string, Value};
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let data = fs::read_to_string("src/luma_events.json").expect("Unable to read file");
    let v: Value = serde_json::from_str(data.as_str())?;
    for obj in v["objects"].as_array() {
        for x in obj {
            for items in x["items"].as_array() {
                for item in items {
                    println!("({}, {})", item["title"], item["link"]);
                }
            }
        }
    }
    Ok(())
}
