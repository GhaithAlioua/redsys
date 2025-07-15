/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        // Dark theme color palette
        dark: {
          bg: "#1e1e1e",
          sidebar: "#252526",
          statusbar: "#007acc",
          editor: "#1e1e1e",
          panel: "#252526",
          activitybar: "#333333",
          titlebar: "#3c3c3c",
          border: "#474747",
          selection: "#264f78",
          highlight: "#2a2d2e",
          text: "#cccccc",
          textMuted: "#969696",
          link: "#3794ff",
          button: "#0e639c",
          buttonHover: "#1177bb",
          input: "#3c3c3c",
          inputBorder: "#474747",
          inputFocus: "#007acc",
        },
      },
    },
  },
  plugins: [],
};
