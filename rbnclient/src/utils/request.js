const { fetch } = require("@tauri-apps/api/http");

const server = "";
const baseURL = `${server}/`;

const BODY_TYPE = {
  Form: "Form",
  Json: "Json",
  Text: "Text",
  Bytes: "Bytes",
};

const commonOptions = {
  timeout: 60,
};

const isAbsoluteURL = (url) => {
  return /^([a-z][a-z\d+\-.]*:)?\/\//i.test(url);
};

const combineURLs = (baseURL, relativeURL) => {
  return relativeURL
    ? baseURL.replace(/\/+$/, "") + "/" + relativeURL.replace(/^\/+/, "")
    : baseURL;
};

const buildFullPath = (baseURL, requestedURL) => {
  if (baseURL && !isAbsoluteURL(requestedURL)) {
    return combineURLs(baseURL, requestedURL);
  }
  return requestedURL;
};

const checkStatus = (status, data) => {
  if (status >= 200 && status < 400) {
    console.log(data);
    console.log(typeof data)
    return data;
  }
  return Promise.reject(`Request failed with status ${status}`);
};

const http = (url, options) => {
  if (!options.headers) options.headers = {};
  // todo 可以往 headers 中添加 token 或 cookie 等信息
  options.timeout = options.timeout || 10; // 默认超时时间为 10s
  if (options?.body) {
    if (options.body.type === BODY_TYPE.Form) {
      options.headers["Content-Type"] = "multipart/form-data";
    }
  }
  options = { ...commonOptions, ...options };
  return fetch(buildFullPath(baseURL, url), options)
    .then(({ status, data }) => checkStatus(status, data))
    .catch((err) => {
      console.error(`An error occurred while making an HTTP request: ${err}`);
      return Promise.reject(err);
    });
};

const httpJson = (url, options) => {
  return http(url, {
    ...options,
    body: { type: "Json", payload: options.data },
  });
};

module.exports = { http, httpJson };