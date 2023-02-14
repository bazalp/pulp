#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use prismaMainClient::{directory, PrismaClient};
use tauri::Manager;

mod prismaMainClient;

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
async fn create_directories(
  paths_dir: Vec<String>,
  state: tauri::State<'_, AppState>,
) -> Result<Vec<directory::Data>, String> {
  let mut result = Vec::new();
  for path_dir in &paths_dir {
    let parts = path_dir.split("/").collect::<Vec<&str>>();
    let last_part = parts[parts.len() - 1];
    match state
      .prisma_client
      .directory()
      .create(path_dir.to_string(), last_part.to_string(), true, vec![])
      .exec()
      .await
    {
      Ok(directory) => result.push(directory),
      Err(e) => return Err(e.to_string()),
    };
  }

  Ok(result)
}

#[tauri::command]
async fn delete_directories(
  paths_dir: Vec<String>,
  state: tauri::State<'_, AppState>,
) -> Result<Vec<directory::Data>, String> {
  let mut result = Vec::new();
  for path_dir in &paths_dir {
    match state
      .prisma_client
      .directory()
      .delete(directory::path::equals(path_dir.to_string()))
      .exec()
      .await
    {
      Ok(directory) => result.push(directory),
      Err(e) => return Err(e.to_string()),
    };
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
      create_directories,
      get_all_directories,
      delete_directories
    ])
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
