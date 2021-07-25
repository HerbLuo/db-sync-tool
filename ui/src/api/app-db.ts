import { defaultConfiguration } from "../constants/default-configuration";
import { AppDb } from "../types/app-db";
import { logger } from "../utils/logger";
import { baseUrl, http } from "./common";

async function get(): Promise<AppDb> {
  const response = await fetch(`${baseUrl}/api/setting`);
  const configuration = await response.json();
  if (!configuration.current || response.status !== 200) {
    logger.warn("configuration.get: 状态不为200, ", configuration);
    await save(defaultConfiguration);
  }
  return configuration;
}

async function save(configuration: AppDb): Promise<void> {
  return http.post(`${baseUrl}/api/setting`, configuration);
}

export const appDbApi = {
  get,
  save,
};
