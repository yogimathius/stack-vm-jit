/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#eff6ff',
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8',
          900: '#1e3a8a',
        },
        secondary: {
          50: '#f8fafc',
          100: '#f1f5f9',
          200: '#e2e8f0',
          500: '#64748b',
          600: '#475569',
          700: '#334155',
          800: '#1e293b',
          900: '#0f172a',
        }
      },
      fontFamily: {
        mono: ['JetBrains Mono', 'Consolas', 'Monaco', 'monospace'],
      },
      animation: {
        'stack-push': 'stackPush 0.3s ease-out',
        'stack-pop': 'stackPop 0.3s ease-out',
        'instruction-highlight': 'instructionHighlight 0.5s ease-out',
        'memory-access': 'memoryAccess 0.4s ease-out',
      },
      keyframes: {
        stackPush: {
          '0%': { transform: 'translateY(-20px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        stackPop: {
          '0%': { transform: 'translateY(0)', opacity: '1' },
          '100%': { transform: 'translateY(-20px)', opacity: '0' },
        },
        instructionHighlight: {
          '0%': { backgroundColor: 'rgb(59 130 246 / 0.3)' },
          '100%': { backgroundColor: 'transparent' },
        },
        memoryAccess: {
          '0%': { backgroundColor: 'rgb(16 185 129 / 0.3)' },
          '100%': { backgroundColor: 'transparent' },
        },
      },
    },
  },
  plugins: [],
}