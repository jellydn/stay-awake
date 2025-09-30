import js from '@eslint/js'
import typescript from '@typescript-eslint/eslint-plugin'
import typescriptParser from '@typescript-eslint/parser'

export default [
  {
    // Global ignores - these files are completely excluded from linting
    ignores: [
      'dist/**',
      'node_modules/**', 
      'src-tauri/target/**',
      'src-tauri/gen/**',
      '*.min.js',
      '*.bundle.js',
      '.vscode/**',
      '.cursor/**',
      'bun.lock',
      'package-lock.json',
      '**/*.vue' // Skip Vue files for now - handled by vue-tsc
    ]
  },
  js.configs.recommended,
  {
    files: ['**/*.{js,jsx,ts,tsx}'],
    plugins: {
      '@typescript-eslint': typescript
    },
    languageOptions: {
      parser: typescriptParser,
      parserOptions: {
        ecmaVersion: 2022,
        sourceType: 'module'
      },
      globals: {
        // Browser globals
        console: 'readonly',
        // Node.js globals (for config files)
        __dirname: 'readonly',
        __filename: 'readonly',
        process: 'readonly',
        // Tauri globals
        __TAURI__: 'readonly'
      }
    },
    rules: {
      // TypeScript rules
      '@typescript-eslint/no-unused-vars': 'warn',
      '@typescript-eslint/no-explicit-any': 'warn',
      
      // General rules
      'no-console': 'off', // Allow console for system tray app
      'no-debugger': 'warn',
      'prefer-const': 'warn',
      'no-unused-vars': 'off', // Use TypeScript version instead
      'no-undef': 'error'
    }
  }
]