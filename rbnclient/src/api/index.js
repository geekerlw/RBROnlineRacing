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
    data: {
      name: data.name,
      description: data.description,
      start_time: data.start_time,
      end_time: data.end_time,
    },
  });
};
