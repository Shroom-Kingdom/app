module.exports = {
  parser: '@typescript-eslint/parser',
  extends: [
    'eslint:recommended',
    'plugin:import/recommended',
    'plugin:import/typescript',
    'plugin:prettier/recommended',
    'plugin:react/recommended',
    'plugin:@typescript-eslint/eslint-recommended',
    'plugin:@typescript-eslint/recommended',
    'prettier/@typescript-eslint'
  ],
  plugins: ['svelte3', '@typescript-eslint', 'prettier', 'react'],
  parserOptions: {
    ecmaVersion: 2020,
    sourceType: 'module',
    ecmaFeatures: {
      jsx: true,
      arrowFunctions: true
    }
  },
  overrides: [{ files: ['*.svelte'], processor: 'svelte3/svelte3' }],
  env: {
    browser: true,
    es6: true,
    amd: true,
    node: true
  },
  settings: {
    react: {
      version: 'detect'
    },
    'svelte3/typescript': () => require('typescript')
  },
  rules: {
    'react/prop-types': 'off',
    'import/order': [
      'error',
      {
        groups: [['builtin', 'external'], 'parent', ['sibling', 'index']],
        'newlines-between': 'always',
        alphabetize: {
          order: 'asc'
        }
      }
    ]
  }
};
