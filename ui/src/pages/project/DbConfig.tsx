import { createStyles, makeStyles, Theme } from "@material-ui/core";
import React from "react";
import { ChangeEvent } from "react";
import { ClientAddr } from "../../types/app-db";
import { ZzTextField } from "../../utils/ZzTextField";

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      padding: "10px",
      border: "1px solid #666",
      display: "grid",
      gridTemplateColumns: "max-content max-content",
      rowGap: "5px"
    },
  }),
);

interface Props {
  label: string;
  addr: ClientAddr;
  onChange: (addr: ClientAddr) => void;
}

export function ClientAddrConfig(props: Props) {
  const { label, addr } = props;
  const classes = useStyles();

  const onChange = (key: keyof ClientAddr) => (e: ChangeEvent<HTMLInputElement>) => {
    props.onChange({
      ...addr,
      [key]: e.target.value
    })
  }

  return (
    <div className={classes.root}>
      <div>{label}</div>
      <div></div>
      <ZzTextField label="地址" value={addr.hostname} onChange={onChange("hostname")}/>
      <ZzTextField label="端口" value={addr.port} onChange={onChange("port")}/>
      <ZzTextField label="用户名" value={addr.username} onChange={onChange("username")}/>
      <ZzTextField label="密码" value={addr.password} onChange={onChange("password")}/>
      <ZzTextField label="数据库" value={addr.db} onChange={onChange("db")}/>
    </div>
  );
}
