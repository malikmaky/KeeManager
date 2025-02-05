module.exports = {
  content: [
    "./src/**/*.{html,svelte,ts}",
    "./node_modules/flowbite-svelte/**/*.svelte",
  ],
  theme: {
    extend: {
    },
  },
  plugins: [
    require('flowbite/plugin'),
    require('autoprefixer'),
    function ({ addUtilities }) {
      addUtilities({
        '.app-region-drag': {
          '-webkit-app-region': 'drag',
        },
        '.app-region-no-drag': {
          '-webkit-app-region': 'no-drag',
        },
      });
    },
  ],
}


