# Rust Crawler 🚀

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**Rust Crawler** is a fast and efficient web crawler written in Rust. It can explore websites, save page content to a PostgreSQL database, and queue links for further crawling.  

This project is **fully open-source** and ready to be deployed as a background worker on platforms like Render.

---

## Features

- Asynchronous crawling using `tokio`  
- Respect `robots.txt` rules and `crawl-delay`  
- Per-domain scheduling (rate limiting)  
- HTML link parsing (`scraper`)  
- Storing pages and links in PostgreSQL  
- Multi-worker deployment (Docker, Render, etc.)  
- Fully free and open-source (MIT License)  

---

## Requirements

- Rust 1.75+  
- PostgreSQL  
- Fedora / Ubuntu / Debian (with `openssl-devel` or `libssl-dev`)  
- Cargo and `pkg-config`  

---

## Local Installation & Running

1. Clone the repository:

```bash
git clone https://github.com/your-username/rust-crawler.git
cd rust-crawler
````

2. Create a PostgreSQL database and tables:

```sql
CREATE DATABASE crawler;
\c crawler

CREATE TABLE queue (
    url TEXT PRIMARY KEY
);

CREATE TABLE pages (
    url TEXT PRIMARY KEY,
    content TEXT
);

CREATE TABLE links (
    from_url TEXT,
    to_url TEXT,
    PRIMARY KEY (from_url, to_url)
);
```

3. Set the database environment variable:

```bash
export DATABASE_URL="postgres://user:password@localhost/crawler"
```

4. Build and run the crawler:

```bash
cargo build --release
./target/release/crawler
```

---

## Usage

* Add URLs to the queue table to start crawling:

```sql
INSERT INTO queue (url) VALUES ('https://example.com');
```

* The crawler will fetch pages, respect `robots.txt`, and save all links to the database.

---

## Deployment

* Docker is supported. Build and run:

```bash
docker build -t rust-crawler .
docker run -e DATABASE_URL=$DATABASE_URL rust-crawler
```

* Multi-worker deployment is ready for Render using the included `render.yaml`.

---

## License

This project is licensed under the **MIT License** — see [LICENSE](LICENSE) for details.

---

You can now **copy this whole block** and save it as `README.md`.  

If you want, I can make a **full ready-to-run Bash script** that creates the Rust crawler project, adds this README, LICENSE, PostgreSQL schema, and builds everything automatically. Do you want me to do that?
