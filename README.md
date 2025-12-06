# Document Similarity Analyzer

🔍 **Sentence-Level Document Similarity Analysis using TF-IDF and Cosine Similarity**

Backend service berbasis Rust untuk menganalisis kesamaan dokumen pada level kalimat menggunakan **TF-IDF** dan **Cosine Similarity** dengan parallel processing. Mendukung format PDF, DOCX, dan TXT.

---

## ✨ Fitur

- 📄 **Multi-Format Support** - PDF, DOCX, dan TXT
- 🎯 **Analisis Level Kalimat** - Deteksi similarity per kalimat yang presisi
- 🚀 **High Performance** - Parallel processing dengan Rayon
- 📊 **TF-IDF Vectorization** - Global IDF untuk akurasi maksimal
- 📐 **Cosine Similarity** - Cross-document comparison
- 🔧 **Configurable Threshold** - Atur sensitivity (0.0-1.0)
- 🔄 **REST API** - Endpoint sederhana dengan Axum
- ✅ **Production-Ready** - 83 tests passing

---

## 🛠️ Tech Stack

- **Rust** (Edition 2021)
- **Axum** 0.7 - HTTP Framework
- **Tokio** - Async Runtime
- **Rayon** 1.8 - Parallel Processing
- **pdf-extract** 0.7 - PDF text extraction
- **docx-rs** 0.4 - DOCX parsing
- **Serde** - Serialization

---

## 📦 Instalasi

```bash
# Clone repository
git clone https://github.com/fthliqml/document-similarity-analyzer.git
cd document-similarity-analyzer

# Build project
cargo build --release

# Run server
cargo run --release
```

Server akan berjalan di `http://0.0.0.0:3000`

---

## 🚀 Quick Start

### Health Check

```bash
curl http://localhost:3000/health
```

### Analisis Dokumen

**Basic (threshold default 0.70):**

```bash
curl -X POST http://localhost:3000/api/analyze \
  -F "files=@document1.pdf" \
  -F "files=@document2.txt"
```

**Dengan Custom Threshold:**

```bash
curl -X POST http://localhost:3000/api/analyze \
  -F "files=@thesis.pdf" \
  -F "files=@reference.docx" \
  -F "files=@paper.txt" \
  -F "threshold=0.85"
```

### Response Example

```json
{
  "metadata": {
    "documents_count": 3,
    "total_sentences": 264,
    "processing_time_ms": 175,
    "threshold": 0.7
  },
  "matches": [
    {
      "source_doc": "thesis.pdf",
      "source_sentence_index": 14,
      "target_doc": "reference.docx",
      "target_sentence_index": 9,
      "similarity": 0.9143
    }
  ],
  "global_similarity": [
    {
      "docA": "thesis.pdf",
      "docB": "reference.docx",
      "score": 0.7834
    }
  ]
}
```

---

## 📋 Requirements & Limits

| Constraint        | Value          |
| ----------------- | -------------- |
| Minimum files     | 2              |
| Maximum files     | 5              |
| Max file size     | 10 MB          |
| Max total size    | 50 MB          |
| Supported formats | PDF, DOCX, TXT |
| Threshold range   | 0.0 - 1.0      |

---

## 📚 Dokumentasi Lengkap

Untuk dokumentasi API lengkap, contoh kode, dan error handling:

👉 **[API_DOCUMENTATION.md](./API_DOCUMENTATION.md)**

---

## 🏗️ Arsitektur

```
Upload Files → Extract Text (PDF/DOCX/TXT)
           ↓
Split into Sentences (regex: [.!?]\s+)
           ↓
Preprocessing (normalize + tokenize)
           ↓
TF-IDF Vectors (sentence-level, global IDF)
           ↓
Cosine Similarity (cross-document only)
           ↓
Filter by Threshold + Global Scoring
           ↓
JSON Response
```

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run dengan output
cargo test -- --nocapture

# Run integration test
cargo test --test manual_workflow_test
```

**Test Coverage:**

- ✅ 83 Unit Tests (extraction, sentence, TF-IDF, models)
- ✅ Integration Tests (end-to-end workflow)
- ✅ All tests passing

---

## 📊 Performance

Waktu proses tipikal (bervariasi tergantung ukuran dokumen):

- **2 dokumen, ~50 kalimat**: 45-80ms
- **3 dokumen, ~150 kalimat**: 120-200ms
- **5 dokumen, ~300 kalimat**: 250-500ms

**Optimizations:**

- ✅ Rayon parallel processing
- ✅ Efficient HashMap TF-IDF
- ✅ Zero-copy iterators
- ✅ Release mode optimizations

---

## 🎯 Use Cases

- **Deteksi Plagiarisme**: Identifikasi konten yang disalin
- **Analisis Similarity**: Temukan bagian serupa di paper penelitian
- **Perbandingan Dokumen**: Bandingkan versi atau dokumen terkait
- **Riset Akademik**: Analisis pola kutipan dan overlap konten
- **Content Moderation**: Deteksi submission duplikat

---

## 🤝 Contributing

Kontribusi sangat diterima! Silakan buat Pull Request.

1. Fork repository
2. Buat feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit perubahan (`git commit -m 'Add some AmazingFeature'`)
4. Push ke branch (`git push origin feature/AmazingFeature`)
5. Buat Pull Request

---

## 📝 License

MIT License

---

## 🔗 Links

- **Repository**: https://github.com/fthliqml/document-similarity-analyzer
- **Issues**: https://github.com/fthliqml/document-similarity-analyzer/issues
- **API Docs**: [API_DOCUMENTATION.md](./API_DOCUMENTATION.md)

---

**Built with ❤️ using Rust**
