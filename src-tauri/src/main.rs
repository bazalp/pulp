#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::path::Path;

use prismaMainClient::{directory, file, PrismaClient};
use prisma_client_rust::prisma_errors::query_engine::UniqueKeyViolation;
use tauri::Manager;
use utils::audio_utils::extract_audio_files;
use walkdir::WalkDir;

mod prismaMainClient;
mod utils;

pub struct AppState {
  pub prisma_client: PrismaClient,
}

#[tauri::command]
async fn get_all_directories(
  state: tauri::State<'_, AppState>,
) -> Result<Vec<directory::Data>, String> {
  match state
    .prisma_client
    .directory()
    .find_many(vec![])
    .exec()
    .await
  {
    Ok(directory) => return Ok(directory),
    Err(e) => return Err(e.to_string()),
  }
}

#[tauri::command]
async fn create_directory(
  path_dir: String,
  state: tauri::State<'_, AppState>,
) -> Result<directory::Data, String> {
  let parts = path_dir.split("/").collect::<Vec<&str>>();
  let last_part = parts[parts.len() - 1];
  match state
    .prisma_client
    .directory()
    .create(path_dir.to_string(), last_part.to_string(), true, vec![])
    .exec()
    .await
  {
    Ok(directory) => {
      scan_directoy((*directory.path).to_string(), state).await;
      return Ok(directory);
    }
    Err(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
      return Err(last_part.to_string() + " already exists")
    }
    Err(_error) => return Err("An error occurred".to_string()),
  }
}

#[tauri::command]
async fn delete_directory(
  path_dir: String,
  state: tauri::State<'_, AppState>,
) -> Result<directory::Data, String> {
  match state
    .prisma_client
    .directory()
    .delete(directory::path::equals(path_dir))
    .exec()
    .await
  {
    Ok(directory) => return Ok(directory),
    Err(_error) => return Err("An error occurred".to_string()),
  };
}

#[tauri::command]
async fn scan_directoy(
  path_dir: String,
  state: tauri::State<'_, AppState>,
) -> Result<Vec<file::Data>, String> {
  let mut result = Vec::new();
  for path_file in WalkDir::new(path_dir).into_iter().filter_map(|e| e.ok()) {
    if let Some(ext) = path_file.path().extension() {
      if ext == "wav" || ext == "mp3" {
        let path = path_file.path().display().to_string();

        let last_part = match path_file.file_name().to_str() {
          Some(s) => s.to_string(),
          None => return Err(format!("Invalid file name: {:?}", path_file.file_name())),
        };

        match state
          .prisma_client
          .file()
          .create(path, last_part.to_string(), vec![])
          .exec()
          .await
        {
          Ok(file) => result.push(file),
          Err(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
            return Err(last_part.to_string() + " already exists")
          }
          Err(_error) => return Err("An error occurred".to_string()),
        }
      }
    }
  }
  Ok(result)
}

#[tokio::main]
async fn main() {
  let context = tauri::generate_context!();

  tauri::Builder::default()
    .setup(|app| {
      let resolver = app.path_resolver();
      let mut db_dir_path = resolver.app_data_dir().unwrap();
      db_dir_path.push("databases");

      let mut db_file_path = db_dir_path.clone();
      db_file_path.push("Database.db");

      let handle = app.handle();
      tauri::async_runtime::spawn(async move {
        std::fs::create_dir_all(db_dir_path.as_path()).unwrap();

        let prisma_client = prismaMainClient::new_client_with_url(
          ("file:".to_owned() + db_file_path.to_str().unwrap()).as_str(),
        )
        .await
        .unwrap();

        #[cfg(debug_assertions)]
        prisma_client._db_push().await.unwrap();
        #[cfg(not(debug_assertions))]
        prisma_client._migrate_deploy().await.unwrap();

        handle.manage(AppState { prisma_client })
      });
      Ok(())
    })
    // .manage(AppState { prisma_client })
    .invoke_handler(tauri::generate_handler![
      get_all_directories,
      create_directory,
      delete_directory
    ])
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
