//! Model Context Protocol (MCP) implementation.
//!
//! This module handles communication with external tools via the MCP protocol.
//! It supports connecting to MCP servers, discovering tools, and executing them.

pub mod client;
pub mod config;
pub mod http_client;
pub mod storage;
pub mod types;

pub use client::MCPManager;
pub use types::MCPServerConfig;
pub use types::*;