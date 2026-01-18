---
title: "Building Web Applications with Axum"
slug: "building-web-apps-with-axum"
date: 2025-01-14
author: "Your Name"
description: "Learn how to build fast, type-safe web applications using Axum, a Rust web framework built on top of Tokio and Tower."
tags: ["rust", "axum", "web", "backend"]
category: "web-development"
template: "post"
draft: false
toc: true
---

# Building Web Applications with Axum

Axum is a modern web framework for Rust that leverages the power of Tokio for async I/O and Tower for middleware. Let's explore how to build a simple web application.

## Setting Up

First, create a new project and add dependencies:

```bash
cargo new my_web_app
cd my_web_app
```

Add to your `Cargo.toml`:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
```

## Hello World Server

Here's a minimal Axum server:

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

## Routing

Axum uses a composable router:

```rust
use axum::{
    routing::{get, post},
    Router,
};

async fn home() -> &'static str {
    "Welcome home!"
}

async fn about() -> &'static str {
    "About page"
}

let app = Router::new()
    .route("/", get(home))
    .route("/about", get(about))
    .route("/users", post(create_user));
```

## Extractors

Axum's extractors make it easy to access request data:

```rust
use axum::{
    extract::{Path, Query, Json},
    routing::get,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}

async fn get_user(Path(id): Path<u32>) -> String {
    format!("User {}", id)
}

async fn list_users(Query(pagination): Query<Pagination>) -> String {
    let page = pagination.page.unwrap_or(1);
    format!("Listing users, page {}", page)
}
```

## JSON Responses

For JSON APIs, use the `Json` extractor and responder:

```rust
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User {
        id,
        name: "Alice".to_string(),
    })
}
```

## Conclusion

Axum provides a powerful, type-safe foundation for building web applications in Rust. Its integration with the Tower ecosystem means you have access to a wide range of middleware components.
