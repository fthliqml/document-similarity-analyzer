# 🧾 Product Requirements Document (PRD)
## Document Similarity Analyzer – Backend (Rust Functional + Parallel)

---

## 1. Goal
Membangun backend service berbasis web yang dapat:
- menerima banyak dokumen teks sekaligus
- menganalisis tingkat kesamaan dokumen satu sama lain
- melakukan komputasi similarity secara paralel pada multi-core CPU
- menerapkan pure functional design & immutable transformation pipeline

---

## 2. Problem Statement
Mahasiswa, peneliti, dan developer sering bekerja dengan banyak dokumen teks. Membandingkan dokumen secara manual sulit & lambat. Sistem ini menyelesaikan:

- deteksi duplikasi
- mendeteksi kemiripan
- clustering teks
- deteksi indikasi plagiarisme

Dengan memberikan similarity matrix antar dokumen.

---

## 3. System Scope
Backend menyediakan:
✔ REST API
✔ menerima file teks / PDF / DOCX (opsional)
✔ normalisasi teks
✔ tokenisasi
✔ term frequency extraction
✔ TF-IDF vectorizing
✔ Cosine similarity
✔ Output berupa JSON hasil similarity

---

## 4. Tech Stack (Backend)
- **Rust**
- **Axum** — HTTP API
- **Tokio** — async runtime
- **Rayon** — parallel CPU processing
- **Serde** — serialize/deserialize
- **thiserror / anyhow** — error model
- (opsional) **PDF parsing crate**

---

## 5. Core Functional Concept
### Immutable Data Model
- setiap dokumen disimpan sebagai immutable
- setiap vector kata hasil transformasi adalah immutable
- tidak ada mutation state global

### Pure Functions (bernyawa FP)
Semua transformasi data berbentuk fungsi pure:

```
fn normalize_text(&str) -> String
fn tokenize(&str) -> Vec<String>
fn compute_tf(&Vec<String>) -> HashMap<String, f32>
fn compute_idf(&Vec<HashMap<String, f32>>) -> HashMap<String, f32>
fn vectorize(tf & HashMap<String,f32>, idf & HashMap<String,f32>) -> Vec<f32>
fn cosine_similarity(&Vec<f32>, &Vec<f32>) -> f32
```

Tidak ada fungsi yang mengubah global state.

---

## 6. Performance Design
### Parallel Execution Strategy
- setiap dokumen → task parallel
- menggunakan rayon `.par_iter()`
- no locking needed
- no mutex hell
- dictionary & IDF immutable → thread-safe by design

#### Example execution pipeline:
```
docs
  |> normalize (parallel)
  |> tokenize (parallel)
  |> compute tf (parallel)
  |> compute tf-idf vector (parallel)
  |> compute similarity matrix (parallel)
```

### Expected Target Performance:
- 50 dokumen dapat dianalisis dalam < 3 detik
- CPU utilization: > 70% across cores

---

## 7. API Design (Backend)

### POST /analyze
Mengirim dokumen dalam bentuk teks.

**Request JSON:**
```json
{
  "documents": [
    "this is sample text",
    "this is another document",
    "lorem ipsum dolor"
  ]
}
```

**Response JSON:**
```json
{
  "similarity_matrix": [
    [1.0, 0.82, 0.01],
    [0.82, 1.0, 0.02],
    [0.01, 0.02, 1.0]
  ],
  "index": ["doc0", "doc1", "doc2"]
}
```

---

## 8. Constraints & Rules
- Backend tidak menyimpan dokumen (stateless by design)
- Input maksimal per request: 100 docs
- belajar pada prinsip FP: tidak ada global mutable state
- memastikan deterministic output (fungsi sama → hasil sama)

---

## 9. Acceptance Criteria

| Requirement | Status |
|---|---|
| API menerima paling tidak 10 dokumen | Wajib |
| Dokumen diproses parallel | Wajib |
| Menghasilkan similarity matrix | Wajib |
| Tidak ada shared mutable state | Wajib |
| CPU usage terlihat di demo | Sangat dianjurkan |
| Fungsi pure digunakan | Wajib |
| Hasil deterministik | Wajib |

---

## 10. Potential Extensions
- cluster dokumen berdasarkan similarity
- integrate stop-word removal
- stemming (kata → akar kata)
- UI visual graph similarity
- PDF / DOCX ingestion
- persistent mode (DB)

---

## 11. Developer Notes (as a backend dev vibe)
- jangan pakai mutex kecuali *terpaksa banget*
- prefer immutable data
- gunakan `Arc<...>` jika diperlukan
- gunakan `.par_iter()` sebisa mungkin
- fungsi harus free of side-effects
- return value > mutate state internal
- error handling elegan dengan `thiserror`

---

## 12. Example Code Snippet Plan

```rust
let normalized_docs: Vec<_> = docs.par_iter()
    .map(|d| normalize_text(d))
    .collect();
```

```rust
let tokenized: Vec<_> = normalized_docs.par_iter()
    .map(|d| tokenize(d))
    .collect();
```

```rust
let tfs: Vec<_> = tokenized.par_iter()
    .map(|words| compute_tf(words))
    .collect();
```

Semua *pure*, tidak ada global state.

---

