use walkdir::WalkDir;

pub async fn extract_audio_files<F>(
  dir: &str,
  mut callback: F,
) -> Result<(), Box<dyn std::error::Error>>
where
  F: FnMut(&str),
{
  for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
    if let Some(ext) = entry.path().extension() {
      if ext == "wav" || ext == "mp3" {
        let path = entry.path().display().to_string();
        callback(&path);
      }
    }
  }
  Ok(())
}
