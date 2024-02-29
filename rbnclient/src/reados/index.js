
import { invoke } from "@tauri-apps/api/tauri";

export async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let value = await invoke("greet", { name: name.value });
  return value
}

export async function get_user_name() {
  let value = await invoke("get_user_name");
  return value
}