export default {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'header-max-length': [2, 'always', 100],
    'type-enum': [
      2,
      'always',
      [
        'feat', // 新功能
        'fix', // Bug 修复
        'docs', // 文档
        'style', // 格式（不影响代码运行的变动）
        'refactor', // 重构
        'test', // 测试
        'chore' // 其他杂项（构建过程、辅助工具等）
      ]
    ]
  }
}
