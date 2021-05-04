import { get } from "./common";

export function getOne() {
  return get("/api/settings");
}
