
#[tokio::test]
pub async fn async_server_communication() {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dir = dir.parent().unwrap();
    let server_process = std::process::Command::new("cargo")
        .args(["run", "--bin", "comet-server", "--", "run", "--port", "8000"])
        .current_dir(dir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .stdin(std::process::Stdio::null())
        .spawn();
    assert!(server_process.is_ok());

    // Wait for server to start
    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    // Test identity request with timeout
    let result = tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        async {
            loop {
                if let Ok(response) = reqwest::get("http://localhost:8000/").await {
                    break response;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    )    .await;    assert!(
        result.is_ok(),
        "Identity request timed out after 10 seconds"
    );

    let response = result.unwrap();
    let ident = response.text().await.unwrap();
    let ident: Result<libcomet::request::repo::IdentityRequest,_> = serde_json::from_str(&ident);
    assert!(ident.is_ok());
    
    server_process.unwrap().kill().unwrap();
}
#[test]
fn verify_server_ident() {
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dir = dir.parent().unwrap();
    let test_dir = dir.join("tests/spec/server_ident.json");
    let ident = std::fs::read_to_string(test_dir).unwrap();
    let ident: Result<libcomet::request::repo::IdentityRequest,_> = serde_json::from_str(&ident);
    assert!(ident.is_ok());
}