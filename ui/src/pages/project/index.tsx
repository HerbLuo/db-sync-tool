import React, { memo } from "react";
import { useParams } from "react-router";
import { CurrentSyncConfigContext } from "../../contexts/current-sync-config";
import { SyncConfigEditor } from "./SyncConfigEditor";

interface LinkParams {
  fragment: string;
}

function ProjectPage() {
  const { fragment } = useParams<LinkParams>();
  console.log(fragment);

  return (
    <CurrentSyncConfigContext.Consumer>
      {([syncConfig, setSyncConfig]) => 
        syncConfig && (<SyncConfigEditor config={syncConfig} setConfig={setSyncConfig}/>)
      }
    </CurrentSyncConfigContext.Consumer>
  );
}

export default memo(ProjectPage);
