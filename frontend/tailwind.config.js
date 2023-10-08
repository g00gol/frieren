/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        "playfair-display": ["var(--font-playfair-display)"],
        quicksand: ["var(--font-quicksand)"],
      },
    },
    fontSize: {
      sm: "0.750rem",
      base: " 1rem",
      xl: "1.333rem",
      "2xl": "1.777rem",
      "3xl": "2.369rem",
      "4xl": "3.158rem",
      "5xl": "4.210rem",
    },
    colors: {
      "mage-silver": "#E4DFE6",
      "just-black": "#000000",
      "sky-blue": "#7AC9E8",
      "deep-blue": "#0B2B37",
      "night-blue": "#05141A",
    },
  },
  plugins: [],
};
