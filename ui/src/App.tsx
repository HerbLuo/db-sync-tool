import { 
  Collapse,
  createMuiTheme,
  CssBaseline,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  MuiThemeProvider,
  useMediaQuery,
} from "@material-ui/core";
import BlurOnIcon from '@material-ui/icons/BlurOn';
import ChildCareIcon from '@material-ui/icons/ChildCare';
import GroupIcon from '@material-ui/icons/Group';
import HomeIcon from '@material-ui/icons/Home';
import SchoolIcon from '@material-ui/icons/School';
import TextFormatIcon from '@material-ui/icons/TextFormat';
import React, { PropsWithChildren, ReactElement, useMemo } from "react";
import { Switch } from "react-router";
import {
  BrowserRouter, Link, Route,
} from "react-router-dom";
import Home from "./pages/home";
import AsyncPage from "./utils/AsyncPage";
import ResponsiveFrameView, { Hide } from "./utils/ResponsiveFrameView";
import { createTheme } from "./theme";
import { useSwitch } from "./utils/use-switch";

const createListItem = (text: string, to: string, icon: React.ReactElement, onClick: () => void) => (
  <ListItem button={true} component={Link} to={to} onClick={onClick}>
    <ListItemIcon>{icon}</ListItemIcon>
    <ListItemText primary={text}/>
  </ListItem>
);

function NestedListItem(props: PropsWithChildren<{text: string, icon: ReactElement}>) {
  const { text, icon } = props;
  const [open, toggleState] = useSwitch();
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

const drawer = (hide: Hide) => (
  <List component="div">
    <NestedListItem text="项目" icon={<HomeIcon/>}>


    </NestedListItem>
    {/* {createListItem("词频管理", "/word-score", <BlurOnIcon/>, hide)}
    {createListItem("用户管理", "/user", <GroupIcon/>, hide)}
    {createListItem("词形管理", "/word-exchanges", <TextFormatIcon/>, hide)}
    {createListItem("简单词管理", "/simple-words", <ChildCareIcon/>, hide)}
    {createListItem("专业词汇管理", "/professional-words", <SchoolIcon/>, hide)} */}
  </List>
);

const LoginPage = (props: any) => (
  <div>
    <CssBaseline />
    <AsyncPage {...props} match={{...props.match, params: {page: "login"}}}/>;
  </div>
);

const hours = new Date().getHours();
const areNight = hours > 19 || hours < 6;

export default function App() {
  const prefersDarkMode = useMediaQuery('(prefers-color-scheme: dark)', {noSsr: true})
    || areNight;
    // || true;

  const theme = useMemo(() => {
    return createMuiTheme(createTheme(prefersDarkMode ? "dark" : "light"));
  }, [prefersDarkMode]);

  return (
    <MuiThemeProvider theme={theme}>
      <BrowserRouter>
        <Switch>
          <Route exact={true} path="/login" component={LoginPage}/>
          <ResponsiveFrameView
            drawer={drawer}
            title="ZzDbSync"
          >
            <Switch>
              <Route path="/:page" component={AsyncPage}/>
              <Route component={Home}/>
            </Switch>
          </ResponsiveFrameView>
        </Switch>
      </BrowserRouter>
    </MuiThemeProvider>
  );
}
