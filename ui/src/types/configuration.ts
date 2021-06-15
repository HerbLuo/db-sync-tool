export interface ClientAddr {
  hostname: string;
  username: string;
  db: string;
  port: number;
  password: string;
}

export interface SyncConfig {
  name?: string;
  mode: "drop-create";
  tables: "*" | string[];
  from: string | ClientAddr;
  to: string | ClientAddr;
  buffer_size?: number;
  skip_sync_if_table_not_exist?: boolean;
}

export interface Project {
  name: string;
  def?: boolean;
  syncs: SyncConfig[];
}

export interface Configuration {
  databaseAddresses: ClientAddr[];
  projects: Project[];
}
