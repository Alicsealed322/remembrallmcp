//! Ingest tool parameter structs and implementation helpers.
//!
//! Covers: remembrall_ingest_github, remembrall_ingest_docs.
//! The `#[tool]` wrapper methods live in `lib.rs` (required by `#[tool_router]`).
//!
//! All ingestion logic lives in `remembrall_core::ingest`. The functions here
//! are thin wrappers that extract params, delegate to core, and convert the
//! returned `IngestResult` into an MCP `CallToolResult` JSON response.

use std::sync::Arc;

use rmcp::{ErrorData as McpError, model::*, schemars};
use serde_json::json;

use remembrall_core::{embed::Embedder, ingest, memory::store::MemoryStore};

// ---------------------------------------------------------------------------
// Parameter structs
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct IngestGithubParams {
    #[schemars(description = "GitHub repo in owner/repo format (e.g. 'owner/repo')")]
    pub repo: String,
    #[schemars(description = "Maximum number of recent merged PRs to ingest (default 50, max 200)")]
    pub limit: Option<u32>,
    #[schemars(description = "Project name to tag memories with (defaults to the repo name)")]
    pub project: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct IngestDocsParams {
    #[schemars(description = "Path to project directory to scan for markdown files")]
    pub path: String,
    #[schemars(description = "Project name to tag memories with")]
    pub project: Option<String>,
}

// ---------------------------------------------------------------------------
// Thin wrappers - delegate all logic to remembrall_core::ingest
// ---------------------------------------------------------------------------

pub async fn ingest_github_impl(
    memory: &Arc<MemoryStore>,
    embedder: &Arc<dyn Embedder>,
    params: IngestGithubParams,
) -> Result<CallToolResult, McpError> {
    let IngestGithubParams { repo, limit, project } = params;

    // Derive the effective project name here so we can include it in the
    // response even though core also derives it internally.
    let effective_project = project.clone().unwrap_or_else(|| {
        repo.split('/').last().unwrap_or("unknown").to_string()
    });

    let result = ingest::ingest_github_prs(
        &repo,
        limit.map(|l| l as i32),
        project.as_deref(),
        memory.as_ref(),
        Arc::clone(embedder),
    )
    .await
    .map_err(|e| McpError::internal_error(e.to_string(), None))?;

    let text = json!({
        "repo": repo,
        "project": effective_project,
        "memories_stored": result.memories_stored,
        "memories_skipped": result.memories_skipped,
        "errors": result.errors,
    })
    .to_string();

    Ok(CallToolResult::success(vec![Content::text(text)]))
}

pub async fn ingest_docs_impl(
    memory: &Arc<MemoryStore>,
    embedder: &Arc<dyn Embedder>,
    params: IngestDocsParams,
) -> Result<CallToolResult, McpError> {
    let IngestDocsParams { path, project } = params;

    // Derive the effective project name here so we can include it in the
    // response even though core also derives it internally.
    let effective_project = project.clone().unwrap_or_else(|| {
        std::path::Path::new(&path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string()
    });

    let result = ingest::ingest_docs(
        &path,
        project.as_deref(),
        memory.as_ref(),
        Arc::clone(embedder),
    )
    .await
    .map_err(|e| McpError::internal_error(e.to_string(), None))?;

    let text = json!({
        "path": path,
        "project": effective_project,
        "memories_stored": result.memories_stored,
        "memories_skipped": result.memories_skipped,
        "errors": result.errors,
    })
    .to_string();

    Ok(CallToolResult::success(vec![Content::text(text)]))
}
