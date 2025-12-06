//! Integration test for sentence-level file upload API

use reqwest::multipart::{Form, Part};
use std::fs;

#[tokio::test]
async fn test_analyze_files_endpoint() {
    // Create test files
    let doc1 = "Artificial intelligence is transforming the world. Machine learning is powerful.";
    let doc2 = "Artificial intelligence is transforming the world. Deep learning is amazing.";
    
    fs::write("test_doc1.txt", doc1).unwrap();
    fs::write("test_doc2.txt", doc2).unwrap();
    
    // Wait for server to be ready (assume it's running)
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // Create multipart form
    let form = Form::new()
        .part("files", Part::bytes(doc1.as_bytes().to_vec())
            .file_name("test_doc1.txt")
            .mime_str("text/plain").unwrap())
        .part("files", Part::bytes(doc2.as_bytes().to_vec())
            .file_name("test_doc2.txt")
            .mime_str("text/plain").unwrap());
    
    // Send request
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/api/analyze")
        .multipart(form)
        .send()
        .await;
    
    // Check response
    match response {
        Ok(res) => {
            println!("✅ Status: {}", res.status());
            
            let body = res.text().await.unwrap();
            println!("📊 Response:\n{}", body);
            
            // Parse JSON
            let json: serde_json::Value = serde_json::from_str(&body).unwrap();
            
            // Validate structure
            assert!(json.get("metadata").is_some());
            assert!(json.get("matches").is_some());
            assert!(json.get("global_similarity").is_some());
            
            let metadata = json.get("metadata").unwrap();
            assert_eq!(metadata.get("documents_count").unwrap().as_u64().unwrap(), 2);
            assert!(metadata.get("total_sentences").unwrap().as_u64().unwrap() > 0);
            
            println!("✅ All assertions passed!");
        }
        Err(e) => {
            println!("❌ Request failed: {}", e);
            panic!("API test failed");
        }
    }
    
    // Cleanup
    let _ = fs::remove_file("test_doc1.txt");
    let _ = fs::remove_file("test_doc2.txt");
}
