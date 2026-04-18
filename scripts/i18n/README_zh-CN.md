# 🇨🇳 中文 (zh-CN) 本土化

对智能体 YAML Frontmatter 中的 `name` 和 `description` 字段进行简体中文本地化。这使得对于中文用户，智能体名称在 Copilot Chat 的智能体选择器中更加易读。

## 文件说明

| 文件 | 描述 |
|------|-------------|
| `agent-names-zh.json` | 英文智能体名称 → 中文翻译的映射表（包含 130 多个条目） |
| `localize-agents-zh.ps1` | 读取 JSON 并更新已安装智能体文件的 PowerShell 脚本 |

## 使用方法

在使用 `install.sh --tool copilot` 安装智能体后：

```powershell
# 将智能体名称本地化为中文
powershell -ExecutionPolicy Bypass -File scripts/i18n/localize-agents-zh.ps1
```

默认情况下，该脚本处理：
- `%USERPROFILE%\.github\agents\`
- `%USERPROFILE%\.copilot\agents\`

如有需要，可传递自定义路径：

```powershell
powershell -File scripts/i18n/localize-agents-zh.ps1 -TargetDirs @("C:\自定义\路径\agents")
```

## 工作原理

1. 读取 `agent-names-zh.json`（UTF-8 编码）以获取翻译映射表。
2. 对于目标目录中的每个 `.md` 文件：
   - 从 YAML Frontmatter 中提取 `name:` 字段。
   - 查找对应的中文翻译。
   - 替换 `name:` 和 `description:` 字段。
   - 以 UTF-8 编码写回文件。

## 效果示例

处理前：
```yaml
---
name: Security Engineer
description: Threat modeling, secure code review, security architecture
---
```

处理后：
```yaml
---
name: 安全工程师
description: 威胁建模、安全代码审查与应用安全架构专家
---
```

## 注意事项

- 脚本仅修改**已安装的副本**（位于 `~/.github/agents/` 中），不修改源文件。
- 每次运行 `install.sh` 更新后需重新运行该脚本（因为 `install.sh` 会用英文原件覆盖）。
- JSON 文件是翻译的唯一事实来源 —— 添加新智能体请更新该文件。
- 脚本本身是纯 ASCII 编码（以避免 PowerShell 编码问题）；所有中文文本均存放于 JSON 中。
