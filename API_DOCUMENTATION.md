# Document Similarity Analyzer API Documentation

## Overview

REST API untuk analisis kesamaan dokumen berbasis kalimat menggunakan TF-IDF dan Cosine Similarity. Sistem ini mengekstrak teks dari berbagai format file (PDF, DOCX, TXT), memecah menjadi kalimat, dan menghitung similarity antar dokumen pada level kalimat.

**Base URL:** `http://localhost:3000`

**Version:** 1.0.0

---

## Table of Contents

- [Endpoints](#endpoints)
  - [Health Check](#health-check)
  - [Analyze Documents](#analyze-documents)
- [Request Format](#request-format)
- [Response Format](#response-format)
- [Error Handling](#error-handling)
- [Examples](#examples)
- [Limitations](#limitations)

---

## Endpoints

### Health Check

Check if the API server is running and healthy.

**Endpoint:** `GET /health`

**Response:**

```json
{
  "status": "ok"
}
```

**Status Codes:**

- `200 OK` - Server is healthy

**Example:**

```bash
curl http://localhost:3000/health
```

---

### Analyze Documents

Analyze sentence-level similarity across multiple documents.

**Endpoint:** `POST /api/analyze`

**Content-Type:** `multipart/form-data`

**Parameters:**

| Parameter   | Type   | Required | Description                                   |
| ----------- | ------ | -------- | --------------------------------------------- |
| `files`     | File[] | Yes      | 2-5 document files (PDF, DOCX, or TXT)        |
| `threshold` | Float  | No       | Similarity threshold (0.0-1.0), default: 0.70 |

**File Requirements:**

- **Minimum files:** 2
- **Maximum files:** 5
- **Maximum file size:** 10 MB per file
- **Maximum total size:** 50 MB
- **Supported formats:** PDF (.pdf), Word (.docx), Text (.txt)

**Request Example:**

```bash
curl -X POST http://localhost:3000/api/analyze \
  -F "files=@research_paper.pdf" \
  -F "files=@reference_1.docx" \
  -F "files=@reference_2.txt" \
  -F "threshold=0.75"
```

---

## Response Format

### Success Response

**Status Code:** `200 OK`

**Response Body:**

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
      "source_doc": "research_paper.pdf",
      "source_sentence_index": 14,
      "target_doc": "reference_1.docx",
      "target_sentence_index": 9,
      "similarity": 0.9143
    },
    {
      "source_doc": "research_paper.pdf",
      "source_sentence_index": 23,
      "target_doc": "reference_2.txt",
      "target_sentence_index": 5,
      "similarity": 0.8721
    }
  ],
  "global_similarity": [
    {
      "docA": "research_paper.pdf",
      "docB": "reference_1.docx",
      "score": 0.7834
    },
    {
      "docA": "research_paper.pdf",
      "docB": "reference_2.txt",
      "score": 0.6512
    },
    {
      "docA": "reference_1.docx",
      "docB": "reference_2.txt",
      "score": 0.4231
    }
  ]
}
```

### Response Fields

#### `metadata` Object

| Field                | Type    | Description                                    |
| -------------------- | ------- | ---------------------------------------------- |
| `documents_count`    | Integer | Total number of documents analyzed             |
| `total_sentences`    | Integer | Total number of sentences across all documents |
| `processing_time_ms` | Integer | Processing time in milliseconds                |
| `threshold`          | Float   | Similarity threshold used for filtering        |

#### `matches` Array

Contains sentence pairs that exceed the similarity threshold, sorted by similarity score (descending).

| Field                   | Type    | Description                                     |
| ----------------------- | ------- | ----------------------------------------------- |
| `source_doc`            | String  | Filename of the source document                 |
| `source_sentence_index` | Integer | Zero-based index of sentence in source document |
| `target_doc`            | String  | Filename of the target document                 |
| `target_sentence_index` | Integer | Zero-based index of sentence in target document |
| `similarity`            | Float   | Cosine similarity score (0.0-1.0)               |

#### `global_similarity` Array

Contains overall similarity scores between document pairs, sorted by score (descending).

| Field   | Type   | Description                                                  |
| ------- | ------ | ------------------------------------------------------------ |
| `docA`  | String | Filename of first document                                   |
| `docB`  | String | Filename of second document                                  |
| `score` | Float  | Average similarity score across all sentence pairs (0.0-1.0) |

---

## Error Handling

### Error Response Format

```json
{
  "error": "Error message description"
}
```

### HTTP Status Codes

| Status Code                 | Description                |
| --------------------------- | -------------------------- |
| `200 OK`                    | Request successful         |
| `400 Bad Request`           | Invalid request parameters |
| `413 Payload Too Large`     | File size exceeds limits   |
| `422 Unprocessable Entity`  | File extraction failed     |
| `500 Internal Server Error` | Server error               |

### Common Error Messages

#### File Upload Errors

**Missing Filename:**

```json
{
  "error": "File is missing filename"
}
```

**Status:** `400 Bad Request`

**Too Many Files:**

```json
{
  "error": "Too many files. Maximum allowed: 5"
}
```

**Status:** `400 Bad Request`

**Not Enough Files:**

```json
{
  "error": "Not enough files. Minimum required: 2"
}
```

**Status:** `400 Bad Request`

**File Too Large:**

```json
{
  "error": "File 'document.pdf' exceeds maximum size of 10485760 bytes"
}
```

**Status:** `413 Payload Too Large`

**Total Size Too Large:**

```json
{
  "error": "Total upload size exceeds maximum of 52428800 bytes"
}
```

**Status:** `413 Payload Too Large`

#### File Type Errors

**Unsupported File Type:**

```json
{
  "error": "Unsupported file type: document.xlsx. Allowed: PDF, DOCX, TXT"
}
```

**Status:** `400 Bad Request`

#### Threshold Errors

**Invalid Threshold Value:**

```json
{
  "error": "Invalid threshold value: 'abc'. Must be a number between 0.0 and 1.0"
}
```

**Status:** `400 Bad Request`

**Threshold Out of Range:**

```json
{
  "error": "Threshold 1.5 out of range. Must be between 0.0 and 1.0"
}
```

**Status:** `400 Bad Request`

#### Processing Errors

**Extraction Failed:**

```json
{
  "error": "Failed to extract text from 'document.pdf': Invalid PDF format"
}
```

**Status:** `422 Unprocessable Entity`

**Empty Document:**

```json
{
  "error": "Document 'empty.txt' contains no text or sentences"
}
```

**Status:** `400 Bad Request`

---

## Examples

### Example 1: Basic Analysis (Default Threshold)

**Request:**

```bash
curl -X POST http://localhost:3000/api/analyze \
  -F "files=@document1.txt" \
  -F "files=@document2.txt"
```

**Response:**

```json
{
  "metadata": {
    "documents_count": 2,
    "total_sentences": 8,
    "processing_time_ms": 45,
    "threshold": 0.7
  },
  "matches": [
    {
      "source_doc": "document1.txt",
      "source_sentence_index": 0,
      "target_doc": "document2.txt",
      "target_sentence_index": 0,
      "similarity": 1.0
    }
  ],
  "global_similarity": [
    {
      "docA": "document1.txt",
      "docB": "document2.txt",
      "score": 0.3421
    }
  ]
}
```

---

### Example 2: Custom Threshold

**Request:**

```bash
curl -X POST http://localhost:3000/api/analyze \
  -F "files=@thesis.pdf" \
  -F "files=@reference.docx" \
  -F "files=@paper.txt" \
  -F "threshold=0.85"
```

**Response:**

```json
{
  "metadata": {
    "documents_count": 3,
    "total_sentences": 156,
    "processing_time_ms": 198,
    "threshold": 0.85
  },
  "matches": [
    {
      "source_doc": "thesis.pdf",
      "source_sentence_index": 45,
      "target_doc": "reference.docx",
      "target_sentence_index": 23,
      "similarity": 0.9567
    },
    {
      "source_doc": "thesis.pdf",
      "source_sentence_index": 67,
      "target_doc": "paper.txt",
      "target_sentence_index": 12,
      "similarity": 0.8821
    }
  ],
  "global_similarity": [
    {
      "docA": "thesis.pdf",
      "docB": "reference.docx",
      "score": 0.6234
    },
    {
      "docA": "thesis.pdf",
      "docB": "paper.txt",
      "score": 0.5123
    },
    {
      "docA": "reference.docx",
      "docB": "paper.txt",
      "score": 0.4567
    }
  ]
}
```

---

### Example 3: Using PowerShell (Windows)

**Request:**

```powershell
$boundary = [System.Guid]::NewGuid().ToString()
$LF = "`r`n"

$file1 = [System.IO.File]::ReadAllBytes("document1.txt")
$file2 = [System.IO.File]::ReadAllBytes("document2.txt")

$bodyLines = @(
    "--$boundary",
    'Content-Disposition: form-data; name="files"; filename="document1.txt"',
    'Content-Type: text/plain',
    '',
    [System.Text.Encoding]::UTF8.GetString($file1),
    "--$boundary",
    'Content-Disposition: form-data; name="files"; filename="document2.txt"',
    'Content-Type: text/plain',
    '',
    [System.Text.Encoding]::UTF8.GetString($file2),
    "--$boundary",
    'Content-Disposition: form-data; name="threshold"',
    '',
    '0.75',
    "--$boundary--"
)

$body = $bodyLines -join $LF

Invoke-RestMethod `
    -Uri "http://localhost:3000/api/analyze" `
    -Method Post `
    -ContentType "multipart/form-data; boundary=$boundary" `
    -Body $body
```

---

### Example 4: Using Python

**Request:**

```python
import requests

url = "http://localhost:3000/api/analyze"

files = [
    ('files', ('document1.pdf', open('document1.pdf', 'rb'), 'application/pdf')),
    ('files', ('document2.docx', open('document2.docx', 'rb'), 'application/vnd.openxmlformats-officedocument.wordprocessingml.document')),
]

data = {
    'threshold': '0.80'
}

response = requests.post(url, files=files, data=data)
result = response.json()

print(f"Documents analyzed: {result['metadata']['documents_count']}")
print(f"Matches found: {len(result['matches'])}")

for match in result['matches']:
    print(f"\nSimilarity: {match['similarity']:.4f}")
    print(f"  {match['source_doc']} [sentence {match['source_sentence_index']}]")
    print(f"  {match['target_doc']} [sentence {match['target_sentence_index']}]")
```

---

### Example 5: Using JavaScript/Node.js

**Request:**

```javascript
const FormData = require("form-data");
const fs = require("fs");
const axios = require("axios");

const form = new FormData();
form.append("files", fs.createReadStream("document1.txt"));
form.append("files", fs.createReadStream("document2.txt"));
form.append("threshold", "0.75");

axios
  .post("http://localhost:3000/api/analyze", form, {
    headers: form.getHeaders(),
  })
  .then((response) => {
    const data = response.data;
    console.log(`Documents analyzed: ${data.metadata.documents_count}`);
    console.log(`Total sentences: ${data.metadata.total_sentences}`);
    console.log(`Processing time: ${data.metadata.processing_time_ms}ms`);
    console.log(`Matches found: ${data.matches.length}`);

    data.matches.forEach((match) => {
      console.log(`\nSimilarity: ${match.similarity.toFixed(4)}`);
      console.log(
        `  ${match.source_doc} [sentence ${match.source_sentence_index}]`
      );
      console.log(
        `  ${match.target_doc} [sentence ${match.target_sentence_index}]`
      );
    });
  })
  .catch((error) => {
    console.error("Error:", error.response?.data || error.message);
  });
```

---

## Limitations

### File Constraints

- **Minimum files:** 2 documents required for comparison
- **Maximum files:** 5 documents per request
- **File size limit:** 10 MB per individual file
- **Total size limit:** 50 MB for all files combined
- **Supported formats:** PDF, DOCX, TXT only

### Processing Constraints

- **Sentence splitting:** Uses regex pattern `[.!?]\s+`
- **Cross-document only:** Only compares sentences between different documents (no intra-document comparison)
- **Threshold range:** Must be between 0.0 and 1.0
- **Text extraction:** Requires valid/readable file formats

### Performance Notes

- Processing time increases with:
  - Number of documents
  - Total number of sentences
  - Document size
- Typical processing time: 50-500ms depending on document size
- Uses parallel processing (Rayon) for optimization

---

## Algorithm Details

### TF-IDF Pipeline

1. **Text Extraction:** Extract raw text from PDF/DOCX/TXT files
2. **Sentence Splitting:** Split text into sentences using regex
3. **Normalization:** Lowercase, trim whitespace, remove punctuation
4. **Tokenization:** Split sentences into words
5. **TF Computation:** Calculate Term Frequency for each sentence
6. **IDF Computation:** Calculate Inverse Document Frequency globally across all sentences
7. **TF-IDF Vectors:** Compute TF-IDF vector for each sentence
8. **Similarity Calculation:** Calculate Cosine Similarity between sentence pairs
9. **Filtering:** Keep only matches above threshold
10. **Global Scoring:** Compute average similarity between document pairs

### Similarity Score Interpretation

| Score Range | Interpretation                                            |
| ----------- | --------------------------------------------------------- |
| 0.90 - 1.00 | Very High Similarity (likely identical or near-identical) |
| 0.75 - 0.89 | High Similarity (significant overlap)                     |
| 0.60 - 0.74 | Moderate Similarity (some common content)                 |
| 0.40 - 0.59 | Low Similarity (minor overlap)                            |
| 0.00 - 0.39 | Very Low Similarity (minimal or no overlap)               |

---

## Running the Server

### Development Mode

```bash
cargo run
```

Server will start on `http://0.0.0.0:3000`

### Production Mode

```bash
# Build optimized binary
cargo build --release

# Run the server
./target/release/document-similarity-analyzer
```

### Environment Configuration

The server binds to `0.0.0.0:3000` by default. To change the port, modify the source code or use a reverse proxy.

---

## Support

For issues, feature requests, or contributions:

- **Repository:** https://github.com/fthliqml/document-similarity-analyzer
- **Issue Tracker:** https://github.com/fthliqml/document-similarity-analyzer/issues

---

## License

[Include your license information here]

---

**Last Updated:** December 7, 2025  
**API Version:** 1.0.0
