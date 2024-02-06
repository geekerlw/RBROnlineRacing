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
