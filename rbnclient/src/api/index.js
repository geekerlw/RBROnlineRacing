import { http, httpJson } from "../utils/request";

export const login = (data) => {
  return httpJson("/api/user/login", {
    method: "POST",
    responseType: 2, // 1 to Json, 2 to Text, 3 to Binary
    data: {
      name: data.name,
      passwd: data.passwd,
    },
  });
};

export const createRace = (data) => {
  return httpJson("/api/race/create", {
    method: "POST",
    responseType: 2, // 1 to Json, 2 to Text, 3 to Binary
    data: data
  });
};

export const getVersion = (data) => {
  return httpJson("/api/version", {
    method: "GET",
    responseType: 2,
    data: {}
  })
}

export const getRaceList = (data) => {
  return httpJson("/api/race/list", {
    method: "GET",
    responseType: 2,
    data: {}
  })
}

export const getRaceInfo = (data) => {
  return httpJson("/api/race/info", {
    method: "GET",
    responseType: 2,
    data: data
  });
}