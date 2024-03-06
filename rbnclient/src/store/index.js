import { defineStore } from "pinia";
import { getLanguageCache, setLanguageCache } from "./localCache.js";

export const getLocalLanguage = () => {
  const language = getLanguageCache();
  if (language) {
    return language;
  }
  const lang = navigator.language;
  if (lang === "zh-CN") {
    return "zh";
  }
  return "en";
};

export const setLocalLanguage = (language) => {
  setLanguageCache(language)
};

export const useGlobalStore = defineStore({
  id: "global",
  state: () => ({
    language: getLocalLanguage(),
    token: '',
    name: '', //username
  }),
  actions: {
    setLanguage(language) {
      this.language = language;
      setLocalLanguage(language);
    },
    logined(token, name) {
      this.token = token;
      this.name = name;
    }
  },
});


import {
  load_game_stage_options,
  load_game_car_options,
  load_game_car_damage_options,
  load_game_stage_weather_options,
  load_game_stage_wetness_options,
} from "../reados/index.js";

export const useGameConfig = defineStore({
  id: 'gameConfig',
  state: () => ({
    stageListOptions: [],
    damageListOptions: [],
    weatherListOptions: [],
    wetnessListOptions: [],
    carListOptions: [],
  }),
  actions: {
    loadGameConfig () {
      load_game_stage_options().then((res) => {
        this.stageListOptions = JSON.parse(res);
      });
      load_game_car_options().then((res) => {
        this.carListOptions = JSON.parse(res);
      });
      load_game_car_damage_options().then((res) => {
        this.damageListOptions = JSON.parse(res);  
      });
      load_game_stage_weather_options().then((res) => {
        this.weatherListOptions = JSON.parse(res);
      });
      load_game_stage_wetness_options().then((res) => {
        this.wetnessListOptions = JSON.parse(res);
      });
    }
  }
})


