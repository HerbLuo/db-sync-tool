import { Collapse, createStyles, List, ListItem, ListItemIcon, ListItemText, makeStyles, Theme } from "@material-ui/core";
import { PropsWithChildren, ReactElement } from "react";
import HomeIcon from '@material-ui/icons/Home';
import { getAll } from "./api/settings";
import { HideNav } from "./utils/ResponsiveFrameView";
import { usePromise } from "./utils/use-async";
import { useSwitch } from "./utils/use-switch";

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    nested: {
      paddingLeft: theme.spacing(6),
    },
  }),
);

interface NestedListItemProps {
  text: string;
  icon: ReactElement;
  defaultExpand?: boolean;
}

function NestedListItem(props: PropsWithChildren<NestedListItemProps>) {
  const { text, icon } = props;
  const [open, toggleState] = useSwitch(props.defaultExpand);
  return (
    <>
      <ListItem button={true} onClick={toggleState}>
        <ListItemIcon>{icon}</ListItemIcon>
        <ListItemText primary={text}/>
      </ListItem>
      <Collapse in={open} timeout="auto" unmountOnExit>
        {props.children}
      </Collapse>
    </>
  );
}

export function Drawer(prop: { hideNav: HideNav }) {
  const { hideNav } = prop;
  const classes = useStyles();
  const [projects] = usePromise(getAll());

  return (
    <List component="div">
      <NestedListItem defaultExpand text="项目" icon={<HomeIcon/>}>
        { (projects || []).map(project => (
          <ListItem className={classes.nested} button={true} onClick={hideNav}>
            <ListItemText primary={project.name}/>
          </ListItem>
        )) }
      </NestedListItem>
    </List>
  );
}
