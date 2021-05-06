import { 
  createMuiTheme,
  CssBaseline,
  MuiThemeProvider,
  useMediaQuery,
} from "@material-ui/core";
import React, { useMemo } from "react";
import { Switch } from "react-router";
import {
  BrowserRouter, Route,
} from "react-router-dom";
import Home from "./pages/home";
import AsyncPage from "./utils/AsyncPage";
import ResponsiveFrameView from "./utils/ResponsiveFrameView";
import { createTheme } from "./theme";
import { Drawer } from "./Drawer";

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
            drawer={hideNav => <Drawer hideNav={hideNav}/>}
            title={<div className="zz-text-logo">数据库同步</div>}
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
