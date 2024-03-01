
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


/*
 * Common functions, like config store and game config load etc.
 */
export async function load_store_config() {

}

export async function save_store_config() {

}

export async function load_game_user_config() {

}

export async function load_game_stage_options() {

}

export async function load_game_stage_wetness_options() {

}

export async function load_game_stage_weather_options() {

}

export async function load_game_stage_skytype_options() {

}

export async function load_game_car_options() {

}

export async function load_game_car_damage_options() {

}

export async function load_game_car_tyre_options() {

}

export async function load_game_car_setup_options() {

}


/*
 * Game flow control functions.
 */