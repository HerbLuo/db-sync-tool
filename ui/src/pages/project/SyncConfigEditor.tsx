import { createStyles, Grid, makeStyles, TextField, Theme } from "@material-ui/core";
import React, { ChangeEventHandler } from "react";
import { useState } from "react";
import { ClientAddr, SyncConfig } from "../../types/app-db";
import { formatAddr } from "../../utils/client-addr-str-to-obj";
import { TypedKey } from "../../utils/ts-typed-key";
import { ZzTextField } from "../../utils/ZzTextField";
import { ClientAddrConfig } from "./DbConfig";

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      flexGrow: 1,
    },
    editorBox: {
      display: "flex",
      padding: "10vh 10vh 0 10vh",
    },
    componentGroup: {
      padding: "10vh 10vh 0 6vh",
      '&>*': {
        marginBottom: "10px"
      },
    },
    code: {
      '& textarea': {
        fontSize: "14px",
        fontFamily: "JetBrainsMono, monospace",
      }
    }
  }),
);

interface Props {
  config: SyncConfig;
  setConfig: (config: SyncConfig) => void;
}

export function SyncConfigEditor(props: Props) {
  const { config, setConfig } = props;
  const classes = useStyles();

  const stringifyConfig = (config: SyncConfig) => JSON.stringify(config, undefined, 2);
  const [validConfig, setConfigValid] = useState(true);
  const [configInStr, setConfigInStr] = useState(stringifyConfig(config));

  const onEditorChange: ChangeEventHandler<HTMLTextAreaElement> = (e) => {
    const v = e.target.value;
    setConfigInStr(v);
    try {
      setConfig(JSON.parse(v));
      setConfigValid(true);
    } catch(e) {
      setConfigValid(false);
    }
  }
  const onEditorBlur = () => {
    setConfigInStr(stringifyConfig(config))
  };
  const onAddrChange = (key: TypedKey<SyncConfig, ClientAddr>) => (addr: ClientAddr) => {
    const newConfig = {
      ...config,
      [key]: addr,
    };
    setConfig(newConfig);
    setConfigValid(true);
    setConfigInStr(stringifyConfig(newConfig));
  }
  // const onConfigChange = (key: keyof SyncConfig) => (e: ChangeEvent<HTMLInputElement>) => {

  // }
  
  return (
    <Grid container className={classes.root} alignItems="stretch">
      <Grid item className={classes.editorBox} xs={12} sm={6}>
        <TextField
          className={classes.code}
          fullWidth
          label="JSON配置"
          multiline
          error={!validConfig}
          rows={22}
          value={configInStr}
          onBlur={onEditorBlur}
          onChange={onEditorChange}
          variant="outlined"
        />
      </Grid>
      <Grid item className={classes.componentGroup} xs={12} sm={6}>
        <ZzTextField label="模式" wrap={true} rowGrip={5} value={config.mode} readOnly/>
        <ClientAddrConfig label="源数据库" addr={formatAddr(config.from)} onChange={onAddrChange("from")}/>
        <ZzTextField label="表" wrap={true} rowGrip={5} value={config.mode} readOnly/>
        <ClientAddrConfig label="目标数据库" addr={formatAddr(config.to)} onChange={onAddrChange("to")}/>
      </Grid>
    </Grid>
  );
}
