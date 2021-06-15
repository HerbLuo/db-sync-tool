import { Collapse, createStyles, List, ListItem, ListItemIcon, ListItemText, makeStyles, Theme } from "@material-ui/core";
import { PropsWithChildren, ReactElement } from "react";
import HomeIcon from '@material-ui/icons/Home';
import { configurationApi } from "./api/configuration";
import { HideNav } from "./utils/ResponsiveFrameView";
import { usePromise } from "./utils/use-async";
import { useSwitch } from "./utils/use-switch";
import { Link } from "react-router-dom";
import { CurrentSyncConfigContext } from "./contexts/current-sync-config";

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
  const [configuration] = usePromise(configurationApi.get());

  return (
    <List component="div">
      <NestedListItem defaultExpand text="项目" icon={<HomeIcon/>}>
        <CurrentSyncConfigContext.Consumer>
          {([, setContext]) => configuration?.projects.map(project => (
            <ListItem 
              key={project.name}
              component={Link}
              className={classes.nested} 
              button={true} 
              to={`/project/${project.name}`}
              onClick={() => {
                setContext(project.syncs[0]);
                hideNav();
              }}
            >
              <ListItemText primary={project.name}/>
            </ListItem>
          )) }
        </CurrentSyncConfigContext.Consumer>
      </NestedListItem>
    </List>
  );
}
