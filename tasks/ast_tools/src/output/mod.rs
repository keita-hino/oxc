use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use proc_macro2::TokenStream;

mod javascript;
mod rust;
use javascript::print_javascript;
use rust::print_rust;

/// An output from codegen.
///
/// Can be either Rust or Javascript.
pub enum Output {
    Rust { path: String, tokens: TokenStream },
    Javascript { path: String, code: String },
}

impl Output {
    pub fn output(self, generator_path: &str) -> RawOutput {
        let (path, code) = match self {
            Self::Rust { path, tokens } => {
                let code = print_rust(&tokens, generator_path);
                (path, code)
            }
            Self::Javascript { path, code } => {
                let code = print_javascript(&code, generator_path);
                (path, code)
            }
        };
        RawOutput { path, content: code.into_bytes() }
    }
}

/// A raw output from codegen.
#[derive(Debug)]
pub struct RawOutput {
    pub path: String,
    pub content: Vec<u8>,
}

impl RawOutput {
    /// Write output to file
    pub fn write_to_file(&self) -> io::Result<()> {
        write_all_to(&self.content, &self.path)
    }
}

/// Get path for an output.
pub fn output_path(krate: &str, path: &str) -> String {
    format!("{krate}/src/generated/{path}")
}

/// Write data to file.
pub fn write_all_to<P: AsRef<Path>>(data: &[u8], path: P) -> io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    file.write_all(data)?;
    Ok(())
}
