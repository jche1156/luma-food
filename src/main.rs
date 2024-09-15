use anyhow::Result;
use dotenv::dotenv;
use scraper::{Html, Selector};
use serde::Deserialize;
use std::fmt::Debug;
use std::fs;

use langchain_rust::{
    agent::OpenAiToolAgent,
    chain::{Chain, LLMChainBuilder},
    fmt_message, fmt_placeholder, fmt_template,
    language_models::llm::LLM,
    llm::openai::{OpenAI, OpenAIConfig, OpenAIModel},
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::messages::Message,
    template_fstring,
};

#[derive(Debug, Deserialize)]
struct Event {
    title: String,
    link: Option<String>,
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
    // let data = fs::read_to_string("src/luma_events.json").expect("Unable to read file");
    dotenv().ok();

    // let events: Events = serde_json::from_str(data.as_str())?;

    // if let Some(data) = events.objects.first() {
    // for event in data.items.iter() {
    // if let Some(url) = &event.link {
    // scrape_event(url).await.unwrap();
    // }
    // }
    // }
    let target_url = "https://lu.ma/59isasf3";
    scrape_event(target_url).await.unwrap();
    Ok(())
}

async fn report_on_food(text: String) -> Result<()> {
    check_for_food(text).await;
    Ok(())
}

async fn scrape_event(url: &str) -> Result<String> {
    let html = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse(".event-about-card").unwrap();

    let event_info = document.select(&selector).next().unwrap();
    let text = event_info.text().collect::<Vec<_>>().join("\n");

    report_on_food(text).await?;
    Ok(String::from(url))
}

async fn check_for_food(text: String) {
    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set.");
    let open_ai = OpenAI::default()
        .with_config(OpenAIConfig::default().with_api_key(openai_api_key))
        .with_model(OpenAIModel::Gpt4oMini.to_string());

    let prompt = format!("You are a starving college student named Smallnumbers. Your mission in life is to help identify free food at tech events. You will be given event descriptions, and your job is to determine whether or not food is likely to be served. If there is food, mention what type it'll be. Give the reasoning in a 'comment' blurb inside the object you'll return.
--------
    Event: 
    Join us for the Dreamforce After Party! Dive into the world of Generative AI with an exclusive gathering that combines delicious sushi, themed beverages, and stimulating conversations on the transformative impact of AI technologies on corporations. This event will feature engaging discussions with industry leaders from AWS and Observea providing unique insights into the latest tools and technologies shaping the future of AI.

    Agenda

    5:00 Doors Open & Networking

    6:00 Welcoming Remarks

    6:15 Short Demo/Panel Discussions

    Panel Conversation: How GenAI will change how we work

    Panelists: Shannon Brownlee (Valence Vibrations), Vamsi Pandari/Chris Leggett (Observea), Shaun VanWeelden (ex ScaleAI, OpenAI)

    Lightning Presenters:

    Observea (Vamsi Pandari)

    Open Babylon (Yurii Filipchuk)

    SylphAI & AdalFlow Demo (Li Yin)

    6:30 Dinner & Drinks

    9:00 Event Ends

    10:00 Doors Close

    Space

    Step into the enchanting Roka Akor in San Francisco for the Dreamforce Gen AI After-Party, hosted by Observea and AWS. This exclusive venue combines modern sophistication with a warm, inviting atmosphere, making it a prime location for networking and insightful discussions on AI integration. Located in the historic Jackson Square, Roka Akor offers a stunning backdrop with its contemporary, chef-driven menu featuring prime steak, sushi, and seafood—all prepared on a signature robata grill.

    Throughout the evening, enjoy a selection of exquisite dinner and bite-sized treats provided by your hosts, Observea and AWS, ensuring a delightful culinary experience.

    This is your chance to network with the best in the industry and discuss future AI innovations. Spots are limited and exclusive to approved attendees—no +1s. If you were not pre-approved, unfortunately, you will not be admitted to the event.
--------
    Answer: {{'has_food': true, 'food_type': 'steak, sushi, seafood', 'contributions_required': false, 'name': 'Dreamforce Gen AI After-Party', 'comment': 'Sushi, beverages, and steaks/seafood at Roka Akor in San Francisco mentioned explicitly.'}}
--------
    Event:

    {}
--------
    Answer: {{'has_free_food': ", text);
    let resp = open_ai.invoke(prompt.as_str()).await.unwrap();
    println!("{{'has_food': {}", resp);
}
