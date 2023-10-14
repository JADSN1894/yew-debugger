module.exports = {
  content: [
    "./index.html",
    "./wasm/**/*.rs"
  ],
  daisyui: {
    themes: ["light", "dark"],
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("daisyui")
  ],
};
