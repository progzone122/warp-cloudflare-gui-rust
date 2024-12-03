/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        "primary": {
          700: "#2F2E39",
          800: "#272630",
          900: "#1c1b24",
        },
        "accent": {
          "500": "#c40c51",
          "600": "#A80945",
          "700": "#870636"
        }
      }
    },
  },
  plugins: [],
}