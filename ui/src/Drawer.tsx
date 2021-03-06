import { Collapse, createStyles, List, ListItem, ListItemIcon, ListItemText, makeStyles, Theme } from "@material-ui/core";
import { PropsWithChildren, ReactElement } from "react";
import HomeIcon from '@material-ui/icons/Home';
import { HideNav } from "./utils/ResponsiveFrameView";
import { useSwitch } from "./utils/use-switch";
import { Link } from "react-router-dom";
import { AppDbContext } from "./contexts/app-db";

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

  return (
    <List component="div">
      <NestedListItem defaultExpand text="项目" icon={<HomeIcon/>}>
        <AppDbContext.Consumer>
          {({appDb, setCurrent}) => appDb?.syncConfigs.map(syncConfig => (
            <ListItem 
              key={syncConfig.name}
              component={Link}
              className={classes.nested} 
              button={true} 
              to={`/project/${syncConfig.name}`}
              onClick={() => {
                setCurrent(syncConfig);
                hideNav();
              }}
            >
              <ListItemText primary={syncConfig.name}/>
            </ListItem>
          ))}
        </AppDbContext.Consumer>
      </NestedListItem>
    </List>
  );
}
