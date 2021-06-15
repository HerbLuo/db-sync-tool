import { defaultConfiguration } from "../constants/default-configuration";
import { Configuration } from "../types/configuration";
import { logger } from "../utils/logger";
import { baseUrl, http } from "./common";

async function get(): Promise<Configuration> {
  const response = await fetch(`${baseUrl}/api/setting`);
  const configuration = await response.json();
  if (!configuration.ok || response.status !== 200) {
    logger.info(configuration);
    await save(defaultConfiguration);
  }
  return configuration;
}

async function save(configuration: Configuration): Promise<void> {
  return http.post(`${baseUrl}/api/setting`, configuration);
}

export const configurationApi = {
  get,
  save,
};
