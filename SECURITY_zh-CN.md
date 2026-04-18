# 安全策略

## 报告安全漏洞

如果你在本项目中发现安全漏洞，请以负责任的方式报告。请勿为安全漏洞开启公开的 GitHub Issue。请通过 GitHub 的 Security 选项卡提交私密安全建议 (private security advisory)。

## 响应时间线

- 确认接收：48 小时内
- 初步评估：7 天内
- 修复或缓解：视严重程度而定

## 范围

本仓库包含基于 Markdown 的智能体定义，以及用于安装和转换的 Shell 脚本。

### 智能体文件 (.md)
- 非可执行的提示词定义
- 智能体文件中不得存储 API 密钥、密钥或凭据

### Shell 脚本 (scripts/)
- `install.sh`、`convert.sh` 和 `lint-agents.sh` 是可执行文件
- 贡献者在运行脚本前应检查是否存在非预期的行为

## 贡献者最佳实践

- 严禁提交 API 密钥、Token 或凭据
- 严禁在智能体 Markdown 文件中添加可执行代码
- Shell 脚本在合并前必须经过审查
- 发现企图进行提示词注入 (Prompt Injection) 的可疑智能体定义时请及时报告
