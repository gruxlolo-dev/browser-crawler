use tokio_postgres::Client;

pub async fn get_next_url(db: &Client) -> Option<String> {
    let row = db.query_opt(
        "DELETE FROM queue WHERE url = (
            SELECT url FROM queue LIMIT 1
        ) RETURNING url",
        &[]
    ).await.ok()?;

    row.map(|r| r.get::<usize, String>(0))
}

pub async fn add_url(db: &Client, url: &str) {
    let _ = db.execute(
        "INSERT INTO queue (url) VALUES ($1) ON CONFLICT DO NOTHING",
        &[&url],
    ).await;
}

pub async fn save_page(db: &Client, url: &str, body: &str) {
    let _ = db.execute(
        "INSERT INTO pages (url, content) VALUES ($1,$2) ON CONFLICT DO NOTHING",
        &[&url, &body],
    ).await;
}

pub async fn save_link(db: &Client, from: &str, to: &str) {
    let _ = db.execute(
        "INSERT INTO links (from_url, to_url) VALUES ($1,$2) ON CONFLICT DO NOTHING",
        &[&from, &to],
    ).await;
}
