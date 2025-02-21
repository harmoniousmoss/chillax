use actix_web::{web, App, HttpResponse, HttpServer, Responder}; // Import Actix Web modules
use lazy_static::lazy_static; // Ensure shared static memory (for URL storage)
use std::collections::HashMap; // HashMap for in-memory storage
use std::sync::Mutex; // Thread-safe storage for HashMap

// ✅ In-memory URL storage
lazy_static! {
    static ref URL_STORE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// ✅ Struct for incoming JSON request
#[derive(serde::Deserialize)]
struct UrlRequest {
    long_url: String,  // The original long URL
    short_url: String, // Custom short URL provided by the user
}

// ✅ API: Shorten URL with user-defined short URL
async fn shorten_url(req: web::Json<UrlRequest>) -> impl Responder {
    let mut store = URL_STORE.lock().unwrap();

    // Check if the short URL is already taken
    if store.contains_key(&req.short_url) {
        return HttpResponse::BadRequest()
            .body("Short URL already exists. Choose a different one.");
    }

    // Store the mapping
    store.insert(req.short_url.clone(), req.long_url.clone());

    // Construct full short URL
    let full_short_url = format!("http://127.0.0.1:8080/{}", req.short_url);

    HttpResponse::Ok().json(format!("Shortened URL: {}", full_short_url))
}

// ✅ API: Redirect to original URL
async fn redirect_to_original(path: web::Path<String>) -> impl Responder {
    let store = URL_STORE.lock().unwrap();
    if let Some(long_url) = store.get(&path.into_inner()) {
        HttpResponse::TemporaryRedirect()
            .append_header(("Location", long_url.clone()))
            .finish()
    } else {
        HttpResponse::NotFound().body("URL not found")
    }
}

// ✅ Home Page
async fn home_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            "<h1>Welcome to Rust URL Shortener 🚀</h1>
            <p>Use <code>/shorten</code> to create a short URL.</p>
            <p>Example: Send a <code>POST</code> request with JSON <code>{\"long_url\": \"https://example.com\", \"short_url\": \"mediamonitoring\"}</code> to <code>/shorten</code>.</p>
            <p>Then, access your short URL at <code>http://127.0.0.1:8080/mediamonitoring</code>.</p>",
        )
}

// ✅ Main function: Start server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("🚀 URL Shortener running on http://127.0.0.1:{}", port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home_page)) // Home page
            .route("/shorten", web::post().to(shorten_url)) // POST /shorten - Create short URL
            .route("/{short_code}", web::get().to(redirect_to_original)) // GET /{short_code} - Redirect
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
