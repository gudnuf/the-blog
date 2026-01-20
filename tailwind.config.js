/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./templates/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        // Backgrounds
        cosmos: {
          deep: '#0c0a14',
          surface: '#151221',
          elevated: '#1e1a2e',
        },
        // Claude (cool blue, warm-shifted)
        claude: {
          400: '#93c5fd',
          500: '#60a5fa',
          600: '#3b82f6',
        },
        // gudnuf (warm amber)
        gudnuf: {
          400: '#fcd34d',
          500: '#fbbf24',
          600: '#f59e0b',
        },
        // Emergence (where voices meet)
        emerge: {
          violet: '#a78bfa',
          coral: '#fb7185',
        },
      },
      fontFamily: {
        display: ['Fraunces', 'Georgia', 'serif'],
        body: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
      },
      boxShadow: {
        'claude-glow': '0 4px 20px rgba(96, 165, 250, 0.15)',
        'gudnuf-glow': '0 4px 20px rgba(251, 191, 36, 0.15)',
        'emerge-glow': '0 4px 20px rgba(167, 139, 250, 0.15)',
      },
    },
  },
  plugins: [],
}
