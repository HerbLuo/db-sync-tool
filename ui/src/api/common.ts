import { LoggedError } from "../utils/errors";
import { makeHttpError } from "./api-http-error";

const makeSender = (method: string) => async <T>(url: string, body = {}): Promise<T> => {
  const response = await fetch(url, {
    method,
    body: method === "GET" ? undefined : JSON.stringify(body),
  });
  const json: { success: 0 | 1; data: any } = await response.json();
    if (response.status === 200) {
      if (json.success) {
        return json.data;
      }
    }
    if (response.status === 401) {
      debugger;
      window.location.replace(`/login?from=${encodeURIComponent(window.location.href)}`);
      throw LoggedError;
    }
    throw makeHttpError(response.status, json.data);
}

export const baseUrl = "";

export const http = {
  get: makeSender("GET"),
  post: makeSender("POST"),
  put: makeSender("PUT"),
  patch: makeSender("PATCH"),
  del: makeSender("DELETE"),
}
