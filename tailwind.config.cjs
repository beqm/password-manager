/** @type {import('tailwindcss').Config}*/
const config = {
  content: ["./src/**/*.{html,js,svelte,ts}"],

  theme: {
    backgroundColor: {
      'light': '#fafafa',
      'dark': '#090a10',

      'primary-100': '#e0e0eb',
      'primary-200': '#c1c2d7',
      'primary-300': '#a2a3c3',
      'primary-400': '#8385af',
      'primary-500': '#63669c',
      'primary-600': '#50527c',
      'primary-700': '#3c3d5d',
      'primary-800': '#28293e',
      'primary-900': '#14141f',
      'primary-1000': '#0f0f18',

      'secondary-100': '#eff0f5',
      'secondary-200': '#d0d1e1',
      'secondary-300': '#b1b3cd',
      'secondary-400': '#9294b9',
      'secondary-500': '#7376a5',
      'secondary-600': '#5a5c8c',
      'secondary-700': '#46486d',
      'secondary-800': '#32334e',
      'secondary-900': '#1e1f2f',
      'secondary-1000': '#0a0a10',
    },
    textColor: {
      'dark': '#f2f3f8',
      'light': '#050505',
      'hover': '#c1c2d7',
    },
    borderColor: {
      'primary-100': '#e0e0eb',
      'primary-200': '#c1c2d7',
      'primary-300': '#a2a3c3',
      'primary-400': '#8385af',
      'primary-500': '#63669c',
      'primary-600': '#50527c',
      'primary-700': '#3c3d5d',
      'primary-800': '#28293e',
      'primary-900': '#14141f',
      'primary-1000': '#000000',
    },
    extend: {}
  },

  plugins: []
};

module.exports = config;