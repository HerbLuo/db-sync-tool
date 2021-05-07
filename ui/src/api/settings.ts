import { Project } from "../types/project";
import { SyncConfig } from "../types/sync-config";

type Projects = Project[];

const defProject: Project = {
  name: "默认",
  syncs: [],
};

const KEY = "projects";

export async function defSync(index: number): Promise<SyncConfig> {
  return {
    name: "同步" + index,
    mode: "drop-create",
    tables: [],
    from: {
      hostname: "",
      username: "",
      db: "",
      port: 0,
      password: "",
    },
    to: {
      hostname: "",
      username: "",
      db: "",
      port: 0,
      password: "",
    },
  };
}

export async function getAll(): Promise<Projects | null> {
  const projects = localStorage.getItem(KEY);
  return projects 
    ? JSON.parse(projects) as Projects 
    : defSync(1).then(sync => [{...defProject, syncs: [sync] }]) ;
}

export async function saveAll(projects: Projects): Promise<void> {
  localStorage.setItem(KEY, JSON.stringify(projects));
}
