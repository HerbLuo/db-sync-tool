import { createStyles, Divider, Grid, makeStyles, TextField, Theme } from "@material-ui/core";
import React, { memo } from "react";
import { useParams } from "react-router";
import { CurrentSyncConfigContext } from "../../contexts/current-sync-config";

interface LinkParams {
  fragment: string;
}

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      flexGrow: 1,
    },
    box: {
      display: "flex",
      padding: "10vh 10vh 0 10vh",
      justifyContent: "center",
      alignItems: "center",
    },
    code: {

    }
  }),
);

function ProjectPage() {
  const classes = useStyles();
  const { fragment } = useParams<LinkParams>();
  console.log(fragment);

  return (
    <CurrentSyncConfigContext.Consumer>
      {([syncConfig]) => (
        <Grid container className={classes.root} alignItems="center">
          <Grid className={classes.box} item xs={12} sm={6}>
            <TextField
              className={classes.code}
              fullWidth
              label="JSON配置"
              multiline
              rows={22}
              value={JSON.stringify(syncConfig, undefined, 2)}
              variant="outlined"
            />
            {/* <Divider orientation="vertical" flexItem /> */}
          </Grid>
          <Grid item className={classes.box} xs={12} sm={6}>
            2
          </Grid>
        </Grid>
      )}
    </CurrentSyncConfigContext.Consumer>
  );
}

export default ProjectPage;
