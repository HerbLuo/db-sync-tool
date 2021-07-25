import { AppDb } from "../types/app-db";
import { generate16LengthId } from "../utils/id";

export const defaultConfiguration: AppDb = {
  databaseAddresses: [],
  current: {
    id: generate16LengthId(),
    name: "未命名配置",
    mode: "drop-create",
    tables: "*",
    from: "",
    to: "",
  },
  syncConfigs: [],
}
