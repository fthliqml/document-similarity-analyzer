# Document Similarity Analyzer

Backend service berbasis Rust untuk menganalisis kesamaan dokumen teks menggunakan **TF-IDF** dan **Cosine Similarity** dengan parallel processing.

## ✨ Fitur

- 🚀 **High Performance** - Parallel processing dengan Rayon
- 📊 **TF-IDF Vectorization** - Term Frequency-Inverse Document Frequency
- 📐 **Cosine Similarity** - Mengukur kesamaan antar dokumen
- 🔄 **REST API** - Endpoint sederhana dengan Axum
- ✅ **Pure Functional** - Core functions tanpa side effects

## 🛠️ Tech Stack

- **Rust** (Edition 2021)
- **Axum** 0.7 - HTTP Framework
- **Tokio** - Async Runtime
- **Rayon** 1.8 - Parallel Processing
- **Serde** - Serialization

## 📦 Instalasi

```bash
# Clone repository
git clone <repository-url>
cd document-similarity-analyzer

# Build project
cargo build --release

# Run server
cargo run --release
```

Server akan berjalan di `http://localhost:3000`

## 🔌 API Endpoints

### Health Check

```http
GET /health
```

**Response:**

```
OK
```

### Analyze Documents

```http
POST /analyze
Content-Type: application/json
```

**Request Body:**

```json
{
  "documents": [
    "The quick brown fox jumps over the lazy dog",
    "A quick brown dog outpaces a lazy fox",
    "Hello world this is a test"
  ]
}
```

**Response:**

```json
{
  "similarity_matrix": [
    [1.0, 0.456, 0.123],
    [0.456, 1.0, 0.089],
    [0.123, 0.089, 1.0]
  ],
  "index": ["doc0", "doc1", "doc2"]
}
```

## 📝 Contoh Penggunaan

### Menggunakan cURL

```bash
# Health check
curl http://localhost:3000/health

# Analyze documents
curl -X POST http://localhost:3000/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "documents": [
      "machine learning is fascinating",
      "deep learning neural networks",
      "cooking recipes and food"
    ]
  }'
```

### Menggunakan PowerShell

```powershell
# Health check
Invoke-RestMethod -Uri "http://localhost:3000/health" -Method GET

# Analyze documents
$body = @{
    documents = @(
        "machine learning is fascinating",
        "deep learning neural networks",
        "cooking recipes and food"
    )
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:3000/analyze" -Method POST -Body $body -ContentType "application/json"
```

## ⚙️ Konfigurasi

| Environment Variable | Default | Deskripsi                                   |
| -------------------- | ------- | ------------------------------------------- |
| `PORT`               | `3000`  | Port server                                 |
| `RUST_LOG`           | `info`  | Log level (trace, debug, info, warn, error) |

```bash
# Contoh menjalankan dengan custom port
PORT=8080 cargo run
```

## 📊 Batasan Input

| Parameter                    | Nilai  |
| ---------------------------- | ------ |
| Minimum dokumen              | 2      |
| Maximum dokumen              | 100    |
| Maximum karakter per dokumen | 50,000 |

## 🧪 Testing

```bash
# Run semua tests
cargo test

# Run unit tests saja
cargo test --lib

# Run integration tests
cargo test --test integration

# Run dengan output
cargo test -- --nocapture
```

## 📈 Benchmarks

```bash
# Run benchmarks
cargo bench
```

## 🏗️ Arsitektur

```
src/
├── api/
│   ├── error.rs      # Error handling
│   ├── handlers.rs   # Request handlers
│   ├── mod.rs
│   └── server.rs     # Server setup
├── core/
│   ├── idf.rs        # Inverse Document Frequency
│   ├── matrix.rs     # Similarity matrix
│   ├── mod.rs
│   ├── normalize.rs  # Text normalization
│   ├── pipeline.rs   # Processing pipeline
│   ├── similarity.rs # Cosine similarity
│   ├── tf.rs         # Term Frequency
│   ├── tokenize.rs   # Tokenization
│   └── vectorize.rs  # TF-IDF vectorization
├── models/
│   ├── document.rs   # Data structures
│   ├── mod.rs
│   ├── request.rs    # API request
│   └── response.rs   # API response
├── lib.rs
└── main.rs
```

## 📐 Algoritma

### TF-IDF (Term Frequency-Inverse Document Frequency)

1. **Term Frequency (TF)**: Frekuensi kata dalam dokumen

   ```
   TF(t,d) = count(t in d) / total_words(d)
   ```

2. **Inverse Document Frequency (IDF)**: Pentingnya kata di seluruh corpus

   ```
   IDF(t) = log((N + 1) / (df(t) + 1)) + 1
   ```

   (Smoothed IDF untuk menghindari division by zero)

3. **TF-IDF Score**:
   ```
   TF-IDF(t,d) = TF(t,d) × IDF(t)
   ```

### Cosine Similarity

Mengukur sudut antara dua vektor TF-IDF:

```
similarity(A,B) = (A · B) / (||A|| × ||B||)
```

- **1.0** = Dokumen identik
- **0.0** = Dokumen tidak memiliki kata yang sama
- **0.0 - 1.0** = Tingkat kesamaan

## 📄 License

MIT License

## 👤 Author

Document Similarity Analyzer - Built with ❤️ and Rust
