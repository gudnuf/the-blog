/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./templates/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        // Warmer dark backgrounds
        cosmos: {
          deep: '#0f0d13',
          surface: '#17141f',
          elevated: '#201c2a',
        },
        // Single accent color (violet)
        accent: {
          DEFAULT: '#a78bfa',
          light: '#c4b5fd',
          muted: '#7c6ba3',
        },
      },
      fontFamily: {
        display: ['Fraunces', 'Georgia', 'serif'],
        body: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
      },
    },
  },
  plugins: [],
}
