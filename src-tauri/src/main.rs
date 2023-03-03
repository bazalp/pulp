#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{path::Path, process::Command};

use prismaMainClient::{directory, file, PrismaClient};
use prisma_client_rust::prisma_errors::query_engine::UniqueKeyViolation;
use tauri::Manager;
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
    .file()
    .delete_many(vec![file::directory_path::equals(path_dir.to_string())])
    .exec()
    .await
  {
    Ok(_file_deleted_cout) => {
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
    Err(_error) => return Err("An error occurred".to_string()),
  };
}

#[tauri::command]
async fn scan_directory(
  path_dir: String,
  state: tauri::State<'_, AppState>,
) -> Result<Vec<file::Data>, String> {
  let path_dir_string = path_dir.to_string();
  let walk_dir = match WalkDir::new(&path_dir)
    .into_iter()
    .filter_map(|e| e.ok())
    .collect::<Vec<_>>()
  {
    v if !v.is_empty() => v,
    _ => return Err(format!("No file found in directory: {}", path_dir)),
  };
  let mut result = Vec::with_capacity(walk_dir.len());
  for path_file in walk_dir {
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
          .create(
            path,
            last_part.to_string(),
            directory::path::equals(path_dir_string.clone()),
            vec![],
          )
          .exec()
          .await
        {
          Ok(file) => result.push(file),
          Err(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
            return Err(format!("File already exists: {}", last_part))
          }
          Err(error) => {
            return Err(format!(
              "Error creating file '{}': {}",
              last_part,
              error.to_string()
            ))
          }
        }
      }
    }
  }
  if result.is_empty() {
    return Err(format!("No audio files found in directory: {}", path_dir));
  }
  Ok(result)
}

#[tauri::command]
async fn analyze_file(
  app_handle: tauri::AppHandle,
  path_file: String,
  state: tauri::State<'_, AppState>,
) -> Result<file::Data, String> {
  // Créer le répertoire temporaire s'il n'existe pas encore
  let resolver = app_handle.path_resolver();
  let temp_dir_path = resolver.app_data_dir().unwrap().join("temp");
  std::fs::create_dir_all(&temp_dir_path)
    .map_err(|err| format!("Failed to create temp directory: {}", err))?;

  let path = Path::new(&path_file);
  let file_name = path
    .file_name()
    .and_then(|name| name.to_str())
    .map(|name_str| format!("{}.json", name_str))
    .ok_or_else(|| "Invalid file name".to_string())?;

  // Exécuter la commande externe pour extraire les informations musicales
  let path_resource_music_extractor = resolver
    .resolve_resource("resources/streaming_extractor_music_osx-i686")
    .expect("failed to resolve resource");
  Command::new(path_resource_music_extractor)
    .arg(&path_file)
    .arg(&temp_dir_path.join(&file_name))
    .output()
    .map_err(|err| format!("Failed to execute process: {}", err))?;

  // Mettre à jour l'entrée de fichier dans la base de données
  state
    .prisma_client
    .file()
    .update(file::path::equals(path_file.clone()), vec![])
    .exec()
    .await
    .map_err(|err| format!("Failed to update file entry in database: {}", err))
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
      delete_directory,
      scan_directory
    ])
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
