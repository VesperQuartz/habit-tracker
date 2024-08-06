/** @type {import('tailwindcss').Config} */
import { fontFamily as _fontFamily } from "tailwindcss/defaultTheme";
export const mode = "all";
export const content = ["./src/**/*.{rs,html,css}", "./dist/**/*.html"];
export const theme = {
  extend: {
    fontFamily: {
      sans: ['"Poppins"', ..._fontFamily.sans],
    },
  },
};
export const plugins = [];
