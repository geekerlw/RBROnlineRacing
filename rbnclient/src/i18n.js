import { createI18n } from "vue-i18n";
import { getLocalLanguage } from './store/index.js';

const i18n = createI18n({
  legacy: false, // you must specify 'legacy: false' option
  locale: getLocalLanguage() || 'zh', // set locale
  messages: {
    // set locale messages
    // for the details, see the "Getting Started" section
    zh: {
      hello: "欢迎来到RBN-BattleNet",
      lobbyTitle: "对战大厅",
      zh: "中文",
      en: "英文",
    },
    en: {
      hello: "Welcome to RBN-BattleNet",
      lobbyTitle: "BattleNet-Lobby",
      zh: "Chinese",
      en: "English",
    },
  },
});

export default i18n;
