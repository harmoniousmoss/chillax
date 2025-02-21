# 🚀 Rust URL Shortener

A simple **URL Shortener** built with **Rust** and **Actix Web**, using in-memory storage (`HashMap`).

## 🌟 Features

✅ Shorten a long URL  
✅ Retrieve the original URL using a short code  
✅ Redirect users to the original URL  
✅ In-memory storage (no database required)  
✅ Simple and lightweight API  

## 🔗 API Usage

Endpoint to shorten a URL:  
```
POST /shorten
```

Example Request (Using Curl):
```
curl -X POST -H "Content-Type: application/json" \
     -d '{"long_url": "https://www.rust-lang.org"}' \
     http://127.0.0.1:8080/shorten

```

Example Response:
```
"Shortened URL: http://127.0.0.1:8080/XyZ123"
```


