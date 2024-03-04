
import { invoke } from "@tauri-apps/api/tauri";
const parse = (s) => {
  try {
    return JSON.parse(s);
  } catch (e) {
    return s;
  }
}

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

// configs will store in windows appdata directory, here provide api to load and save config by key - value.
export async function load_store_config(key) {
  let pair = await invoke("load_store_config", {key: key});
  return pair; // this will return like {"key": "value"}.
}

export async function save_store_config(keypair) {
  await invoke("save_store_config", {keypair: keypair});
}

export async function save_all_store_config() {
  await invoke("save_all_store_config"); // no need arg, flush and store all configs.
}

export async function load_game_user_name() {
  let name = await invoke("load_game_user_name");
  return parse(name);  // return like {"user": xxx}.
}

export async function load_game_stage_options() {
  let options = await invoke("load_game_state_options");
  return options;
}

export async function load_game_stage_wetness_options() {
  let options = await invoke("load_game_stage_wetness_options");
  return options;
}

export async function load_game_stage_weather_options() {
  let options = await invoke("load_game_stage_weather_options");
  return options;
}

export async function load_game_stage_skytype_options(stage_id) {
  let options = await invoke("load_game_stage_skytype_options", {stage_id: stage_id});
  return options;
}

export async function load_game_car_options() {
  let options = await invoke("load_game_car_options");
  return options;
}

export async function load_game_car_damage_options() {
  let options = await invoke("load_game_car_damage_options");
  return options;
}

export async function load_game_car_tyre_options() {
  let options = await invoke("load_game_car_tyre_options");
  return options;
}

export async function load_game_car_setup_options(path) {
  let options = await invoke("load_game_car_setup_options", {path: path});
  return options;
}


/*
 * Game flow control functions.
 */