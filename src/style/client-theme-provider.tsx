'use client';

import React from 'react';
import { createTheme, ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';

const palette = {
  text1: '#ffffff',
  text2: '#cfd8dc',
  color1: '#02fcf3',
  color2: '#a9e4cf',
  color3: '#cae0c8',
  color4: '#deddc4',
  color5: '#e8e7d2',
  backgroundColor1: '#0b1113',
  backgroundColor2: '#0f1517',
};

const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: palette.color1,
      contrastText: '#000',
    },
    secondary: {
      main: palette.color2,
      contrastText: '#000',
    },
    background: {
      default: palette.backgroundColor1,
      paper: palette.backgroundColor2,
    },
    text: {
      primary: palette.text1,
      secondary: palette.text2,
    },
    success: {
      main: palette.color3,
    },
  },
  components: {
    MuiButton: {
      defaultProps: {
        disableElevation: true,
      },
    },
  },
});

export default function ClientThemeProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      {children}
    </ThemeProvider>
  );
}
