use anyhow::Result;
use serde::Deserialize;
use std::fmt::Debug;
use std::fs;

#[derive(Debug, Deserialize)]
struct Event {
    title: String,
    link: Option<String>,
    image: Option<String>,
    summary: Option<String>,
    date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Object {
    items: Vec<Event>,
}

#[derive(Debug, Deserialize)]
struct Events {
    objects: Vec<Object>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let data = fs::read_to_string("src/luma_events.json").expect("Unable to read file");

    let events: Events = serde_json::from_str(data.as_str())?;

    if let Some(data) = events.objects.first() {
        for event in data.items.iter() {
            if let Some(url) = &event.link {
                println!("----------");
                println!("[{}]({})", event.title, url);
                println!(
                    "{}\n{}",
                    event.date.clone().unwrap_or(String::from("N/A")),
                    event.summary.clone().unwrap_or(String::from("N/A"))
                );
            }
        }
    }
    Ok(())
}
