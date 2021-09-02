#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::error::Error;
use std::time::Instant;
use tauri::State;
use users_service::users_client::UsersClient;
use users_service::AllUsersRequest;

pub mod users_service {
  tonic::include_proto!("users");
}

#[derive(Serialize, Deserialize)]
struct User {
  first_name: String,
  last_name: String,
  username: String,
  gender: String,
  age: u32,
  email: String,
  tel: String,
}

#[derive(Serialize, Deserialize)]
struct UsersResult {
  go_time: u64,
  rust_time: u64,
  users: Vec<User>,
}

// process_users takes a vector of users from Tonic an creates a new vector
// of users that can be serialized.
fn process_users(message: users_service::AllUsersReply) -> UsersResult {
  let mut user_list: Vec<User> = Vec::new();
  for user in message.users.iter() {
    user_list.push(User {
      first_name: user.first_name.clone(),
      last_name: user.last_name.clone(),
      username: user.username.clone(),
      gender: user.gender.clone(),
      age: user.age.clone(),
      email: user.email.clone(),
      tel: user.tel.clone(),
    })
  }
  return UsersResult {
    go_time: message.go_time,
    rust_time: 0,
    users: user_list,
  };
}

#[tauri::command]
async fn my_custom_command(
  client: State<
    '_,
    tokio::sync::Mutex<users_service::users_client::UsersClient<tonic::transport::Channel>>,
  >,
) -> Result<UsersResult, String> {
  let start = Instant::now();
  let request = tonic::Request::new(AllUsersRequest { start: 1, end: 200 });
  let response = client
    .lock()
    .await
    .all_users(request)
    .await
    .map_err(|err| err.to_string())?;
  // println!("Time: {}", start.elapsed().as_micros());
  let message = response.into_inner();
  let mut new_users = process_users(message);
  new_users.rust_time = start.elapsed().as_micros().try_into().unwrap();
  Ok(new_users)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let client = tokio::sync::Mutex::new(UsersClient::connect("http://[::1]:50051").await?);
  tauri::Builder::default()
    .manage(client)
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  Ok(())
}
