/** @type {import('tailwindcss').Config} */
export default {
  // content: ["./app/**/*.{html,js,css,rs}"],
  content: [
    "index.html",
    "./app/**/*.rs"
  ],
  daisyui: {
    themes: ["light", "dark"],
  },
  theme: {
    extend: {},
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("daisyui")
  ],
}

