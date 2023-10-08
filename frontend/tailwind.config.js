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
