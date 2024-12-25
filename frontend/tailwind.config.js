/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        accent: 'var(--accent-color)',
      },
      animation:{
        progress:"5s progress linear infinite"

      },
      keyframes:{
        progress:{
          '0%':{width:'0%'},
          '100%':{width:'100%'}
        }

      }
    },
  },
  plugins: [],
  darkMode: ['class'],
};
