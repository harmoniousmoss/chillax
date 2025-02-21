use actix_web::{App, HttpResponse, HttpServer, Responder, web}; // Import Actix Web modules
use lazy_static::lazy_static; // Ensure shared static memory (for URL storage)
use rand::Rng; // Import random number generator
use std::collections::HashMap; // HashMap for in-memory storage
use std::sync::Mutex; // Thread-safe storage for HashMap

// ✅ In-memory URL storage with Mutex for thread safety
lazy_static! {
    static ref URL_STORE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// ✅ Struct to handle incoming JSON requests
#[derive(serde::Deserialize)]
struct UrlRequest {
    long_url: String, // The long URL to be shortened
}

// ✅ Function to generate a random 6-character short code
fn generate_short_code() -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..6)
        .map(|_| {
            charset
                .chars()
                .nth(rng.gen_range(0..charset.len()))
                .unwrap()
        })
        .collect()
}

// ✅ API endpoint: Shorten a URL
async fn shorten_url(req: web::Json<UrlRequest>) -> impl Responder {
    let mut store = URL_STORE.lock().unwrap(); // Lock HashMap for writing
    let short_code = generate_short_code(); // Generate short code
    store.insert(short_code.clone(), req.long_url.clone()); // Store mapping
    HttpResponse::Ok().json(format!(
        "Shortened URL: http://127.0.0.1:8080/{}",
        short_code
    ))
}

// ✅ API endpoint: Redirect to the original URL
async fn redirect_to_original(path: web::Path<String>) -> impl Responder {
    let store = URL_STORE.lock().unwrap(); // Lock HashMap for reading
    if let Some(long_url) = store.get(&path.into_inner()) {
        HttpResponse::TemporaryRedirect()
            .append_header(("Location", long_url.clone())) // Redirect to long URL
            .finish()
    } else {
        HttpResponse::NotFound().body("URL not found") // Error if short code is invalid
    }
}

// ✅ New API endpoint: Serve a simple homepage at "/"
async fn home_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8") // ✅ Ensure proper UTF-8 encoding
        .body(
            "<h1>Welcome to Rust URL Shortener 🚀</h1>
            <p>Use <code>/shorten</code> to create a short URL.</p>
            <p>Example: Send a <code>POST</code> request with JSON <code>{\"long_url\": \"https://example.com\"}</code> to <code>/shorten</code>.</p>
            <p>Then, access your short URL at <code>http://127.0.0.1:8080/{short_code}</code>.</p>",
        )
}

// ✅ Main function: Start the Actix Web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("🚀 URL Shortener running on http://127.0.0.1:{}", port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home_page)) // New home route!
            .route("/shorten", web::post().to(shorten_url)) // POST /shorten - Create short URL
            .route("/{short_code}", web::get().to(redirect_to_original)) // GET /{short_code} - Redirect
    })
    .bind(("127.0.0.1", port))? // Bind to local host and port
    .run()
    .await
}
