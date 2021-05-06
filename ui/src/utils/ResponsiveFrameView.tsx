import { 
  AppBar,
  CssBaseline,
  Divider,
  Drawer,
  Hidden,
  IconButton,
  makeStyles,
  Theme,
  Toolbar,
  Typography,
  useTheme,
} from "@material-ui/core";
import MenuIcon from '@material-ui/icons/Menu';
import React from "react";
import { areDesktop } from "./device";

const drawerWidth = 240;

const useStyles = makeStyles((theme: Theme) => ({
  appBar: {
    marginLeft: drawerWidth,
    [theme.breakpoints.up('md')]: {
      width: `calc(100% - ${drawerWidth}px)`,
    },
  },
  content: {
    width: "100%",
  },
  drawer: {
    [theme.breakpoints.up('md')]: {
      flexShrink: 0,
      width: drawerWidth,
    },
  },
  drawerPaper: {
    width: drawerWidth,
  },
  menuButton: {
    marginRight: theme.spacing(2),
    [theme.breakpoints.up('md')]: {
      display: 'none',
    },
  },
  root: {
    display: 'flex',
  },
  titleBar: {
    alignItems: "center",
    display: "flex",
    justifyContent: "center",
  },
  toolbar: theme.mixins.toolbar,
}));

export type HideNav = () => void;

interface Props {
  children: React.ReactNode;
  drawer: (hideNav: HideNav) => JSX.Element;
  title: string | JSX.Element;
}

function ResponsiveFrameView(props: Props) {
  const classes = useStyles();
  const theme = useTheme();
  const [mobileOpen, setMobileOpen] = React.useState(false);

  function handleDrawerToggle() {
    setMobileOpen(!mobileOpen);
  }

  const drawer = (
    <div>
      <div className={`${classes.toolbar} ${classes.titleBar}`}>
        <Typography component="h2" noWrap={true} >
          {props.title}
        </Typography>
      </div>
      <Divider />
      {props.drawer(areDesktop() ? () => 0 : handleDrawerToggle)}
    </div>
  );

  return (
    <div className={classes.root}>
      <CssBaseline />
      <AppBar position="fixed" className={classes.appBar}>
        <Toolbar>
          <IconButton
            color="inherit"
            aria-label="open drawer"
            edge="start"
            onClick={handleDrawerToggle}
            className={classes.menuButton}
          >
            <MenuIcon />
          </IconButton>
          <Hidden mdUp implementation="css">
            <Typography variant="h1" noWrap>
              {props.title}
            </Typography>
          </Hidden>
        </Toolbar>
      </AppBar>
      <nav className={classes.drawer} aria-label="mailbox folders">
        {/* The implementation can be swapped with js to avoid SEO duplication of links. */}
        <Hidden mdUp implementation="css">
          <Drawer
            variant="temporary"
            anchor={theme.direction === 'rtl' ? 'right' : 'left'}
            open={mobileOpen}
            onClose={handleDrawerToggle}
            classes={{ paper: classes.drawerPaper, }}
            ModalProps={{ keepMounted: true, }} // Better open performance on mobile.
          >
            {drawer}
          </Drawer>
        </Hidden>
        <Hidden smDown implementation="css">
          <Drawer
            classes={{ paper: classes.drawerPaper, }}
            variant="permanent"
            open={true}
          >
            {drawer}
          </Drawer>
        </Hidden>
      </nav>
      <main className={classes.content}>
        <div className={classes.toolbar} />
        {props.children}
      </main>
    </div>
  );
}

export default ResponsiveFrameView;
