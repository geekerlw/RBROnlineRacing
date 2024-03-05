
import { invoke } from "@tauri-apps/api/tauri";
const parse = (s) => {
  try {
    return JSON.parse(s);
  } catch (e) {
    return s;
  }
}

/*
 * Common functions, like config store and game config load etc.
 */

// return game rsf user name string.
export async function load_game_user_name() {
  let name = await invoke("load_game_user_name");
  return name;
}

// return game stage infos. to many key-values, you can see it at game->rbr.rs-> RBRStageData.
export async function load_game_stage_options() {
  let options = await invoke("load_game_state_options");
  return options;
}

// return wetness options, like [{"id": 0, "value": "xxx"}], id is use to create race.
export async function load_game_stage_wetness_options() {
  let options = await invoke("load_game_stage_wetness_options");
  return options;
}

// return weather options, like [{"id": 0, "value": "xxx"}], id is use to create race.
export async function load_game_stage_weather_options() {
  let options = await invoke("load_game_stage_weather_options");
  return options;
}

// return skytype options, like [{"id": 0, "value": "xxx"}], id is use to create race.
// get the stage_id from stage_options api, it's the "id" value.
export async function load_game_stage_skytype_options(stage_id) {
  let options = await invoke("load_game_stage_skytype_options", {stage_id: stage_id});
  return options;
}

// return game stage infos. to many key-values, you can see it at game->rbr.rs-> RBRCarData.
export async function load_game_car_options() {
  let options = await invoke("load_game_car_options");
  return options;
}

// return damage options, like [{"id": 0, "value": "xxx"}], id is use to create race.
export async function load_game_car_damage_options() {
  let options = await invoke("load_game_car_damage_options");
  return options;
}

// return car tyre options, like [{"id": 0, "value": "xxx"}], id is use to create race.
export async function load_game_car_tyre_options() {
  let options = await invoke("load_game_car_tyre_options");
  return options;
}

// return car setup options, like [{"id": 0, "value": "xxx"}], id is use to create race.
export async function load_game_car_setup_options(path) {
  let options = await invoke("load_game_car_setup_options", {path: path});
  return options;
}