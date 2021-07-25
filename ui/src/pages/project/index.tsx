import React, { memo } from "react";
import { SyncConfigEditor } from "./SyncConfigEditor";
import { AppDbContext } from "../../contexts/app-db";

function ProjectPage() {
  return (
    <AppDbContext.Consumer>
      {({appDb, setCurrent}) => 
        appDb && (<SyncConfigEditor config={appDb.current} setConfig={setCurrent}/>)
      }
    </AppDbContext.Consumer>
  );
}

export default memo(ProjectPage);
