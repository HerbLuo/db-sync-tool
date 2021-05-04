import { colors } from "@material-ui/core";
import { ThemeOptions } from "@material-ui/core/styles/createMuiTheme";

type Mode = "dark" | "light";

const createPalette = (mode: Mode) => ({
  type: mode,
  primary: {
    main: mode === "dark" ? colors.blue[200] : colors.blue[700]
  },
  secondary: {
    main: mode === "dark" ? colors.blue[900] : colors.blue['A400']
  },
  text: {
    primary: mode === "dark" ? colors.blueGrey[50] : colors.blueGrey[900],
  },
  background: {
    default: mode === "dark" ? "#303030" : "#F4F6F8",
  },
  divider: colors.grey[200]
});

export const createTheme = (mode: Mode): ThemeOptions => {
  const palette = createPalette(mode);

  const typography = {
    h1: {
      fontSize: "1.25rem",
      fontWeight: 500,
      lineHeight: 1.6,
      letterSpacing: "0.0075em",
    },
    h2: {
      fontWeight: 500,
      fontSize: '1.2rem',
      lineHeight: '32px'
    },
    h3: {
      fontWeight: 500,
      fontSize: '1.15rem',
      lineHeight: '28px'
    },
    h4: {
      fontWeight: 500,
      fontSize: '1.1rem',
      lineHeight: '24px'
    },
    h5: {
      fontWeight: 500,
      fontSize: '1rem',
      lineHeight: "22px"
    },
    h6: {
      fontWeight: 500,
      fontSize: '0.8rem',
      lineHeight: "20px"
    },
  };

  const MuiButton = {
    contained: {
      boxShadow: '0 1px 1px 0 rgba(0,0,0,0.14), 0 2px 1px -1px rgba(0,0,0,0.12), 0 1px 3px 0 rgba(0,0,0,0.20)'
    }
  };
  const MuiCardHeader = {
    root: {
      padding: "14px",
    },
    action: {
      marginTop: 0,
      marginRight: -4,
    }
  };
  const MuiPaper = {
    elevation1: {
      boxShadow: '0 0 0 1px rgba(63,63,68,0.05), 0 1px 3px 0 rgba(63,63,68,0.15)'
    }
  };
  const MuiTableCell = {
    root: {
      // color: palette.text.primary,
      fontSize: '14px',
      letterSpacing: '-0.05px',
      lineHeight: '21px',
      borderBottom: `1px solid ${palette.divider}`
    }
  };
  const MuiTableHead = {
    root: {
      backgroundColor: mode === "dark" ? "#505050" : colors.grey[50]
    }
  };
  const MuiTableRow = {
    // root: {
    //   '&$selected': {
    //     backgroundColor: palette.background.default
    //   },
    //   '&$hover': {
    //     '&:hover': {
    //       backgroundColor: palette.background.default
    //     }
    //   }
    // }
  };
  const MuiToolbar = {
    root: {
      backgroundColor: mode === "dark" ? "#333" : "#1976d2",
      color: colors.blueGrey[50],
    }
  };

  return {
    palette,
    overrides: {
      MuiButton,
      MuiCardHeader,
      MuiPaper,
      MuiToolbar,
      MuiTableCell,
      MuiTableHead,
      MuiTableRow,
    },
    typography,
  }
};
