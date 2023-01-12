module.exports = {
  root: true,
  parser: '@typescript-eslint/parser',
  extends: ['eslint:recommended','plugin:svelte/recommended', 'plugin:@typescript-eslint/recommended', 'plugin:@typescript-eslint/recommended-requiring-type-checking', 'prettier'],
  plugins: [, '@typescript-eslint'],
  overrides: [{
    files: ["*.svelte"],
    parser: "svelte-eslint-parser",
    // Parse the `<script>` in `.svelte` as TypeScript by adding the following configuration.
    parserOptions: {
      parser: "@typescript-eslint/parser",
    },
  }],
  parserOptions: {
    sourceType: 'module',
    ecmaVersion: 2020,
    project: ['./tsconfig.json'],
    extraFileExtensions: ['.svelte']
  },
  rules: {
    "@typescript-eslint/explicit-function-return-type": [
      "warn",
      {
        "allowExpressions": true, // Allow arrow functions to return an expression
      }
    ],
    '@typescript-eslint/no-explicit-any': 'error',
    '@typescript-eslint/explicit-member-accessibility': 'off',
    '@typescript-eslint/no-var-requires': 'off',
    '@typescript-eslint/camelcase': 'off',
    '@typescript-eslint/no-non-null-assertion': 'off',
    '@typescript-eslint/no-unused-vars': [
      'error',
      { argsIgnorePattern: '^_', varsIgnorePattern: '^_' },
    ],
  },
  env: {
    browser: true,
    es2017: true,
    node: true,
  },
}