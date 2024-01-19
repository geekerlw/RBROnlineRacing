import { defineStore } from 'pinia'

export const getLocalLanguage = () => {
    const language = localStorage.getItem('language');
    if (language) {
        return language;
    }
    const lang = navigator.language;
    if (lang === 'zh-CN') {
        return 'zh';
    }
    return 'en';
}

export const setLocalLanguage = (language) => {
  localStorage.setItem('language', language);
}


export const useGlobalStore = defineStore({
  id: 'global',
  state: () => ({
    language: getLocalLanguage(),
  }),
  actions: {
    setLanguage(language) {
      this.language = language;
      setLocalLanguage(language);
    }
  }
})
