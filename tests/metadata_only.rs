use rust_embed::{EmbeddedFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "examples/public/"]
#[metadata_only = true]
struct Asset;

#[test]
fn file_is_empty() {
  let index_file: EmbeddedFile = Asset::get("index.html").expect("index.html exists");
  assert_eq!(index_file.data.len(), 0);
}
