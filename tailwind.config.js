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
          "500": "#E26928",
          "600": "#D84528",
          "700": "#CE2029"
        }
      }
    },
  },
  plugins: [],
}