# 🔬 Document Similarity Analyzer - Algorithm Flow

Dokumentasi lengkap alur algoritma sentence-level similarity dari upload file sampai hasil API response, dengan mapping ke source code spesifik.

---

## 📊 Pipeline Overview

```
┌──────────────────────────────────────────────────────────────┐
│  1. FILE UPLOAD & EXTRACTION                                 │
│  PDF/DOCX/TXT → Extract Text → Raw String                   │
│  📁 src/api/file_upload.rs, src/extraction/*.rs             │
└──────────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────────┐
│  2. SENTENCE SPLITTING                                       │
│  Raw Text → Regex [.!?](?:\s+|$) → Array of Sentences      │
│  📁 src/sentence/mod.rs                                      │
└──────────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────────┐
│  3. NORMALIZATION                                            │
│  Sentences → Lowercase + Remove Punctuation                 │
│  📁 src/core/normalize.rs                                    │
└──────────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────────┐
│  4. TOKENIZATION                                             │
│  Normalized Text → Split Whitespace → Array of Words        │
│  📁 src/core/tokenize.rs                                     │
└──────────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────────┐
│  5. TF CALCULATION (Per Sentence)                            │
│  Words → Count Frequency → HashMap<Word, TF>                │
│  📁 src/core/tf.rs                                           │
└──────────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────────┐
│  6. IDF CALCULATION (Global)                                 │
│  All TF Maps → Document Frequency → HashMap<Word, IDF>      │
│  📁 src/core/idf.rs                                          │
└──────────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────────┐
│  7. TF-IDF VECTORIZATION                                     │
│  TF × IDF → TF-IDF Vector per Sentence                     │
│  📁 src/core/vectorize.rs (via sentence_pipeline.rs)        │
└──────────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────────┐
│  8. COSINE SIMILARITY                                        │
│  Vector A · Vector B → Similarity Score (0.0 - 1.0)        │
│  📁 src/core/similarity.rs (via sentence_pipeline.rs)       │
└──────────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────────┐
│  9. THRESHOLD FILTERING                                      │
│  Keep Only: Similarity ≥ 0.70                              │
│  📁 src/core/sentence_pipeline.rs                           │
└──────────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────────┐
│  10. GLOBAL SIMILARITY                                       │
│  Average All Matches → Global Score per Doc Pair            │
│  📁 src/core/sentence_pipeline.rs                           │
└──────────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────────┐
│  11. JSON RESPONSE                                           │
│  Matches + Global Similarity + Metadata                     │
│  📁 src/models/sentence_analysis.rs                         │
└──────────────────────────────────────────────────────────────┘
```

---

## 🔍 Step-by-Step Details

### **Step 1: File Upload & Extraction**

**Files:**

- `src/api/file_upload.rs` - Multipart upload handler
- `src/extraction/pdf.rs` - PDF extraction
- `src/extraction/docx.rs` - DOCX extraction
- `src/extraction/txt.rs` - TXT extraction

**Input:**

```http
POST /api/analyze
Content-Type: multipart/form-data

files[]: paper.pdf
files[]: reference.docx
threshold: 0.7
```

**Process:**

```rust
// Validate files (max 5, min 2, max 10MB/file, max 50MB total)
// Extract text based on file type
match file_type {
    PDF => extract_pdf(&bytes),   // pdf-extract library
    DOCX => extract_docx(&bytes), // docx-rs library
    TXT => String::from_utf8(&bytes)
}
```

**Output:**

```rust
Vec<(String, String)> // [(filename, raw_text), ...]
```

**Example:**

```rust
[
    ("paper.pdf", "AI is powerful. ML enables analytics."),
    ("reference.docx", "Machine learning is amazing. ML enables analytics.")
]
```

---

### **Step 2: Sentence Splitting**

**File:** `src/sentence/mod.rs`

**Input:**

```rust
"AI is powerful. ML enables analytics. DL uses neural networks."
```

**Process:**

```rust
// Regex: [.!?](?:\s+|$)
// Split on: . ! ? followed by whitespace or end-of-string
split_sentences(text) -> Vec<String>
```

**Output:**

```rust
[
    "AI is powerful.",
    "ML enables analytics.",
    "DL uses neural networks."
]
```

---

### **Step 3: Normalization**

**File:** `src/core/normalize.rs`

**Input:**

```rust
"Machine Learning is POWERFUL!"
```

**Process:**

```rust
text.to_lowercase()           // "machine learning is powerful!"
    .remove_punctuation()     // "machine learning is powerful"
    .trim()                   // "machine learning is powerful"
```

**Output:**

```rust
"machine learning is powerful"
```

---

### **Step 4: Tokenization**

**File:** `src/core/tokenize.rs`

**Input:**

```rust
"machine learning is powerful"
```

**Process:**

```rust
text.split_whitespace()
    .map(|s| s.to_string())
    .collect()
```

**Output:**

```rust
["machine", "learning", "is", "powerful"]
```

---

### **Step 5: TF Calculation (Per Sentence)**

**File:** `src/core/tf.rs`

**Input:**

```rust
["ml", "enables", "analytics"]
```

**Formula:**

```
TF(word) = word_count / total_words_in_sentence
```

**Calculation:**

```rust
Sentence: ["ml", "enables", "analytics"]
Total: 3 words

TF:
- "ml": 1/3 = 0.333
- "enables": 1/3 = 0.333
- "analytics": 1/3 = 0.333
```

**Output:**

```rust
HashMap<String, f32> {
    "ml": 0.333,
    "enables": 0.333,
    "analytics": 0.333
}
```

---

### **Step 6: IDF Calculation (Global)**

**File:** `src/core/idf.rs`

**Input:**

```rust
// All TF maps from all sentences in all documents
Vec<HashMap<String, f32>>
```

**Formula (Smoothed):**

```
IDF(word) = log((N + 1) / (df + 1)) + 1

Where:
- N = total number of sentences
- df = number of sentences containing the word
```

**Example:**

Given **6 total sentences** from 2 documents:

| Word        | df (sentences containing) | Calculation         | IDF  |
| ----------- | ------------------------- | ------------------- | ---- |
| "ml"        | 2                         | ln((6+1)/(2+1)) + 1 | 1.85 |
| "analytics" | 2                         | ln((6+1)/(2+1)) + 1 | 1.85 |
| "ai"        | 1                         | ln((6+1)/(1+1)) + 1 | 2.25 |

**Output:**

```rust
HashMap<String, f32> {
    "ml": 1.85,
    "analytics": 1.85,
    "ai": 2.25,
    // ... all unique words
}
```

---

### **Step 7: TF-IDF Vectorization**

**File:** `src/core/sentence_pipeline.rs` (using `compute_tfidf_vector()`)

**Input:**

```rust
tf: {"ml": 0.333, "enables": 0.333, "analytics": 0.333}
idf: {"ml": 1.85, "enables": 1.85, "analytics": 1.85, ...}
```

**Process:**

```rust
for word in vocabulary {
    tfidf[word] = tf.get(word) * idf.get(word)
}
```

**Example:**

For sentence "ml enables analytics":

| Word      | TF    | IDF  | TF-IDF    |
| --------- | ----- | ---- | --------- |
| analytics | 0.333 | 1.85 | **0.616** |
| enables   | 0.333 | 1.85 | **0.616** |
| ml        | 0.333 | 1.85 | **0.616** |

**Output:**

```rust
HashMap<String, f32> {
    "ml": 0.616,
    "enables": 0.616,
    "analytics": 0.616
}
```

---

### **Step 8: Cosine Similarity**

**File:** `src/core/similarity.rs`

**Input:**

```rust
vector_a: {"ml": 0.616, "enables": 0.616, "analytics": 0.616}
vector_b: {"ml": 0.616, "enables": 0.616, "analytics": 0.616}
```

**Formula:**

```
cosine_similarity = (A · B) / (||A|| × ||B||)

Where:
- A · B = dot product (sum of element-wise multiplication)
- ||A|| = magnitude = sqrt(sum of squared values)
```

**Calculation:**

```rust
// Dot Product
= 0.616² + 0.616² + 0.616²
= 1.137

// Magnitude A
= sqrt(0.616² + 0.616² + 0.616²)
= 1.066

// Magnitude B
= 1.066 (same as A)

// Cosine Similarity
= 1.137 / (1.066 × 1.066)
= 1.00 (identical!)
```

**Output:**

```rust
f32: 1.00 // Range: 0.0 (different) to 1.0 (identical)
```

---

### **Step 9: Threshold Filtering**

**File:** `src/core/sentence_pipeline.rs` - `compute_sentence_matches()`

**Input:**

```rust
all_similarities: [
    (doc0_sent0, doc1_sent0, 0.45),  // ❌ below threshold
    (doc0_sent1, doc1_sent1, 1.00),  // ✅ above threshold
    (doc0_sent2, doc1_sent2, 0.87),  // ✅ above threshold
]
threshold: 0.70
```

**Process:**

```rust
similarities
    .into_iter()
    .filter(|(_, _, sim)| *sim >= threshold)
    .collect()
```

**Output:**

```rust
Vec<SentenceMatch> [
    SentenceMatch {
        source_doc: "paper.pdf",
        source_sentence_index: 1,
        source_sentence: "ml enables analytics",
        target_doc: "reference.docx",
        target_sentence_index: 1,
        target_sentence: "ml enables analytics",
        similarity: 1.00
    },
    SentenceMatch {
        source_doc: "paper.pdf",
        source_sentence_index: 2,
        target_doc: "reference.docx",
        target_sentence_index: 3,
        similarity: 0.87
    }
]
```

---

### **Step 10: Global Similarity**

**File:** `src/core/sentence_pipeline.rs` - `compute_global_similarities()`

**Input:**

```rust
matches: Vec<SentenceMatch> // from Step 9
```

**Process:**

```rust
// 1. Group by document pair
// 2. Calculate average similarity for each pair
let avg = similarities.iter().sum() / count
```

**Example:**

```rust
Matches between paper.pdf ↔ reference.docx:
- Sentence 1 vs 1: 1.00
- Sentence 2 vs 3: 0.87

Global Similarity = (1.00 + 0.87) / 2 = 0.935
```

**Output:**

```rust
Vec<GlobalSimilarity> [
    GlobalSimilarity {
        docA: "paper.pdf",
        docB: "reference.docx",
        score: 0.935
    }
]
```

---

### **Step 11: JSON Response**

**File:** `src/models/sentence_analysis.rs`

**Input:**

```rust
matches: Vec<SentenceMatch>
global_similarities: Vec<GlobalSimilarity>
metadata: AnalysisMetadata
```

**Output:**

```json
{
  "metadata": {
    "documents_count": 2,
    "total_sentences": 6,
    "processing_time_ms": 4,
    "threshold": 0.7
  },
  "matches": [
    {
      "source_doc": "paper.pdf",
      "source_sentence_index": 1,
      "source_sentence": "ml enables analytics",
      "target_doc": "reference.docx",
      "target_sentence_index": 1,
      "target_sentence": "ml enables analytics",
      "similarity": 1.0
    }
  ],
  "global_similarity": [
    {
      "docA": "paper.pdf",
      "docB": "reference.docx",
      "score": 0.935
    }
  ]
}
```

---

## 📂 File Structure

```
src/
├── api/
│   ├── file_upload.rs       # Step 1: Upload handler & orchestrator
│   ├── server.rs            # Axum server setup
│   └── error.rs             # Error types
│
├── extraction/
│   ├── pdf.rs               # Step 1: PDF → text
│   ├── docx.rs              # Step 1: DOCX → text
│   └── txt.rs               # Step 1: TXT → text
│
├── sentence/
│   └── mod.rs               # Step 2: Text → sentences
│
├── core/
│   ├── normalize.rs         # Step 3: Preprocessing
│   ├── tokenize.rs          # Step 4: Text → words
│   ├── tf.rs                # Step 5: Term Frequency
│   ├── idf.rs               # Step 6: IDF calculation
│   ├── vectorize.rs         # Step 7: TF-IDF vector (helper)
│   ├── similarity.rs        # Step 8: Cosine similarity
│   └── sentence_pipeline.rs # Steps 7, 9, 10: Main orchestrator
│
└── models/
    └── sentence_analysis.rs # Step 11: Response models
```

---

## 🔄 Complete Data Flow Example

**Input:** 2 files

```
paper.pdf: "AI is powerful. ML enables analytics."
reference.docx: "ML is amazing. ML enables analytics."
```

**Transformation:**

```
Step 1: Extract
├─> [("paper.pdf", "AI is powerful. ML enables analytics."),
     ("reference.docx", "ML is amazing. ML enables analytics.")]

Step 2: Split
├─> [[["AI is powerful", "ML enables analytics"],
      ["ML is amazing", "ML enables analytics"]]

Step 3: Normalize
├─> [["ai is powerful", "ml enables analytics"],
     ["ml is amazing", "ml enables analytics"]]

Step 4: Tokenize
├─> [[["ai","is","powerful"], ["ml","enables","analytics"]],
     [["ml","is","amazing"], ["ml","enables","analytics"]]]

Step 5: TF (per sentence)
├─> [[{"ai":0.33,"is":0.33,"powerful":0.33},
      {"ml":0.33,"enables":0.33,"analytics":0.33}],
     [{"ml":0.33,"is":0.33,"amazing":0.33},
      {"ml":0.33,"enables":0.33,"analytics":0.33}]]

Step 6: IDF (4 sentences total)
├─> {"ai":1.91, "is":1.22, "powerful":1.91, "ml":1.22,
     "enables":1.22, "analytics":1.22, "amazing":1.91}

Step 7: TF-IDF Vectors
├─> paper[0]: {"ai":0.631, "is":0.403, "powerful":0.631}
    paper[1]: {"ml":0.403, "enables":0.403, "analytics":0.403}
    ref[0]:   {"ml":0.403, "is":0.403, "amazing":0.631}
    ref[1]:   {"ml":0.403, "enables":0.403, "analytics":0.403}

Step 8: Cosine Similarity (cross-doc only)
├─> paper[0] ↔ ref[0]: 0.24
    paper[0] ↔ ref[1]: 0.0
    paper[1] ↔ ref[0]: 0.0
    paper[1] ↔ ref[1]: 1.00 ✅

Step 9: Filter (≥0.70)
├─> [Match(paper[1], ref[1], 1.00)]

Step 10: Global
├─> paper ↔ reference: 1.00

Step 11: Response
└─> {
      "matches": [{"source_doc":"paper.pdf","similarity":1.00,...}],
      "global_similarity": [{"docA":"paper.pdf","score":1.00,...}]
    }
```

---

## 🎯 Key Algorithm Features

### **Sentence-Level TF-IDF**

- TF calculated **per sentence** (not per document)
- More granular detection: knows **which sentence** is plagiarized

### **Global IDF**

- IDF computed from **all sentences** across **all documents**
- Rare words get higher importance

### **Cosine Similarity**

- Measures **angle** between vectors (not distance)
- Range: 0.0 (orthogonal/different) to 1.0 (parallel/identical)
- Works with sparse vectors (many zeros)

### **Threshold Filtering**

- Default: 0.70 (configurable via API)
- Reduces false positives
- Only stores significant matches

### **Parallel Processing (Rayon)**

- Preprocessing (normalize + tokenize)
- TF calculation
- Similarity computation
- Speeds up processing for large documents

---

## 💡 Advantages Over Word-Level System

| Aspect          | Word-Level (Old)                | Sentence-Level (New)                                     |
| --------------- | ------------------------------- | -------------------------------------------------------- |
| **Granularity** | "Doc A is 37% similar to Doc B" | "Sentence 14 in Doc A matches sentence 9 in Doc B (91%)" |
| **Frontend**    | Cannot highlight specific text  | Can highlight exact sentences                            |
| **Detection**   | Document-level only             | Sentence-level precision                                 |
| **Use Case**    | General similarity score        | Plagiarism detection with evidence                       |
| **Response**    | Similarity matrix               | Matched sentences + global score                         |

---

## 🚀 Testing

Run the complete pipeline:

```bash
cargo test
# Result: 83 tests passed

cargo run --release
# Server: http://0.0.0.0:3000

curl -X POST http://localhost:3000/api/analyze \
  -F "files[]=@test_doc1.txt" \
  -F "files[]=@test_doc2.txt" \
  -F "threshold=0.7"
```

---

**Last Updated:** December 7, 2025
