/** @type {import('tailwindcss').Config} */
    module.exports = {
      prefix: "tw-",
      content: {
        relative: true,
        files: ["*.html", "./src/**/*.rs"],
      },
      theme: {
        extend: {},
      },
      plugins: [],
    }
    