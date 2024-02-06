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
