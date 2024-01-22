const languageCacheKey = "language";

export const getLanguageCache = () => {
  const language = localStorage.getItem(languageCacheKey);
  return language;
};
export const setLanguageCache = language => {
  localStorage.setItem(languageCacheKey, language);
}
