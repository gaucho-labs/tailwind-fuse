/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs", "./input.css"],
  },
  theme: {
    extend: {
      fontFamily: {
        'opensans': ['Open Sans', 'sans-serif'],
        'robotomono': ['Roboto Mono', 'monospace'],
      }
    },
  },
  plugins: [],
  darkMode: 'class',
}

