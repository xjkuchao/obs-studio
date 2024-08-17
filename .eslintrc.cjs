module.exports = {
    root: true, // 表示当前目录即为根目录，ESLint 规则将被限制到该目录下
    env: { browser: true, es2020: true, node: true } /* 解析器 */,
    parser: 'vue-eslint-parser', // 指定ESLint解析器
    parserOptions: {
        project: './tsconfig.json', // tsconfig.json的路径
        ecmaVersion: 'latest',
        sourceType: 'module',
        parser: '@typescript-eslint/parser',
        extraFileExtensions: ['.json', '.vue'],
    },
    settings: {
        // 识别 @ # alias
        'import/resolver': {
            alias: {
                map: [
                    ['@', './src'],
                    ['#', './types'],
                ],
                extensions: ['.ts', '.js', '.vue', '.json'],
            },
        },
    } /* ESLint 中基础配置需要继承的配置 */,
    extends: [
        'eslint:recommended',
        'plugin:vue/vue3-essential',
        'plugin:@typescript-eslint/recommended-type-checked', // @typescript-eslint @v6
        'plugin:@typescript-eslint/stylistic-type-checked', // @typescript-eslint @v6
        // 'plugin:@typescript-eslint/recommended',                          // @typescript-eslint @v5
        // 'plugin:@typescript-eslint/recommended-requiring-type-checking',  // @typescript-eslint @v5
        'plugin:import/errors',
        'plugin:import/warnings',
        'plugin:import/typescript',
        'prettier', // 增加 prettier 相关的校验规则
        'plugin:prettier/recommended', // 开启 Prettier 插件推荐的规则
    ] /* ESLint文件所依赖的插件 */,
    plugins: ['@typescript-eslint', 'prettier', 'vue', 'import', 'unused-imports'],
    /**
     * 定义规则
     * "off" 或 0 - 关闭规则
     * "warn" 或 1 - 开启规则，使用警告级别的错误：warn (不会导致程序退出)
     * "error" 或 2 - 开启规则，使用错误级别的错误：error (当被触发的时候，程序会退出)
     */
    rules: {
        'no-console': 'off',
        'no-unused-vars': 'off',
        'no-case-declarations': 'off',
        'no-use-before-define': 'off',
        'no-param-reassign': 'off',
        'space-before-function-paren': 'off',
        'class-methods-use-this': 'off',

        'vue/html-indent': ['error', 4], //在<template>中强制一致缩进
        'vue/singleline-html-element-content-newline': 'off', //要求在单行元素的内容之前和之后有一个换行符
        'vue/max-attributes-per-line': 'off', //执行每行的最大属性数(被 prettier 最大单行控制了暂off)
        'vue/multi-word-component-names': 'off', //要求组件名称始终为多字
        'vue/html-self-closing': 'off', //执行自我封闭式

        'import/first': 'warn',
        'import/newline-after-import': 'warn',
        'import/no-duplicates': 'warn',
        'import/no-unresolved': 'off',
        'import/no-extraneous-dependencies': 'off',
        'import/prefer-default-export': 'off',
        'import/order': [
            'warn',
            {
                groups: [
                    'builtin', // Node.js内置模块
                    'external', // 第三方模块
                    'internal', // 应用程序内部的模块
                    'parent', // 父级目录中导入的模块
                    ['sibling', 'index'], // 具有相同或更高目录的兄弟模块
                    'object',
                    'type',
                ],
                pathGroups: [
                    {
                        pattern: '@/**',
                        group: 'internal',
                    },
                    {
                        pattern: '#/**',
                        group: 'type',
                    },
                    {
                        pattern: '*.{scss,css,less,styl,stylus}',
                        group: 'parent',
                    },
                    {
                        pattern: '*.{js,jsx,ts,tsx}',
                        group: 'sibling',
                    },
                ],
                'newlines-between': 'always', // 在组之间插入空行
                pathGroupsExcludedImportTypes: ['sibling', 'index'],
                warnOnUnassignedImports: true,
                alphabetize: { order: 'asc', caseInsensitive: true }, // 对于每个组，按字母表顺序排序。
            },
        ],

        '@typescript-eslint/no-unused-vars': [
            'warn',
            {
                argsIgnorePattern: '^_',
                varsIgnorePattern: '^_',
            },
        ],
        '@typescript-eslint/no-unused-expressions': 'off',
        '@typescript-eslint/no-unsafe-assignment': 'off',
        '@typescript-eslint/no-unsafe-argument': 'off',
        '@typescript-eslint/no-unsafe-return': 'off',
        '@typescript-eslint/no-unsafe-call': 'off',
        '@typescript-eslint/no-unsafe-member-access': 'off',
        '@typescript-eslint/no-unsafe-enum-comparison': 'off',
        '@typescript-eslint/ban-ts-ignore': 'off',
        '@typescript-eslint/ban-ts-comment': 'off',
        '@typescript-eslint/ban-types': 'off',
        '@typescript-eslint/explicit-function-return-type': 'off',
        '@typescript-eslint/no-explicit-any': 'off',
        '@typescript-eslint/no-var-requires': 'off',
        '@typescript-eslint/no-empty-function': 'off',
        '@typescript-eslint/no-use-before-define': 'off',
        '@typescript-eslint/no-non-null-assertion': 'off',
        '@typescript-eslint/no-shadow': 'off',
        '@typescript-eslint/no-floating-promises': 'off',
        '@typescript-eslint/explicit-module-boundary-types': 'off',
    },
    globals: {
        //可以定义全局中的变量的权限（只读，可读可写）
        defineProps: 'readonly',
        defineEmits: 'readonly',
        defineExpose: 'readonly',
        withDefaults: 'readonly',
        uni: 'readonly',
    },
    ignorePatterns: [
        // # 忽略目录
        '/dist',
        '/public',
        '/src/public',
        '/src/static',
        '/node_modules',
        // # 忽略文件
        '**/*-min.js',
        '**/*.min.js',
        '**/*-min.css',
        '**/*.min.css',
        '**/*.tsbuildinfo',
        '**/*.config.js',
        '**/*.config.ts',
        '/src/manifest.json',
    ],
};
