use crate::{db, parser, scheduler::Scheduler, robots, config};
use tokio_postgres::NoTls;
use reqwest::Client;
use rand::seq::SliceRandom;
use url::Url;
use std::time::Duration;

pub async fn start() {
    let (db, conn) = tokio_postgres::connect(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL not set"),
        NoTls
    ).await.unwrap();

    tokio::spawn(conn);

    let scheduler = Scheduler::new();

    loop {
        if let Some(url) = db::get_next_url(&db).await {
            let agent = config::AGENTS.choose(&mut rand::thread_rng()).unwrap();

            if !robots::allowed(&url, agent) {
                continue;
            }

            let domain = Url::parse(&url).ok().and_then(|u| u.domain().map(|d| d.to_string()));

            if let Some(domain) = domain {
                if !scheduler.allow(&domain) {
                    continue;
                }
            }

            let client = Client::builder()
                .timeout(Duration::from_secs(10))
                .user_agent(*agent)
                .build()
                .unwrap();

            println!("🌍 Crawling {}", url);

            if let Ok(res) = client.get(&url).send().await {
                if let Ok(body) = res.text().await {
                    db::save_page(&db, &url, &body).await;
                    let links = parser::extract_links(&url, &body);
                    for link in links {
                        db::save_link(&db, &url, &link).await;
                        db::add_url(&db, &link).await;
                    }
                }
            }
        } else {
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
}
