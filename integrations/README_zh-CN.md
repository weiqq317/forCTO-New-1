# 🔌 集成 (Integrations)

本目录包含了 The Agency 对各主流 AI 编程工具的集成方案及转换后的格式。

## 支持的工具

- **[Claude Code](#claude-code)** — `.md` 智能体，可直接使用本仓库
- **[GitHub Copilot](#github-copilot)** — `.md` 智能体，可直接使用本仓库
- **[Antigravity](#antigravity)** — 在 `antigravity/` 下为每个智能体生成 `SKILL.md`
- **[Gemini CLI](#gemini-cli)** — 在 `gemini-cli/` 下生成扩展程序及 `SKILL.md` 文件
- **[OpenCode](#opencode)** — 在 `opencode/` 下生成 `.md` 智能体文件
- **[OpenClaw](#openclaw)** — 生成包含 `SOUL.md` + `AGENTS.md` + `IDENTITY.md` 的工作区
- **[Cursor](#cursor)** — 在 `cursor/` 下生成 `.mdc` 规则文件
- **[Aider](#aider)** — 在 `aider/` 下生成单一的 `CONVENTIONS.md`
- **[Windsurf](#windsurf)** — 在 `windsurf/` 下生成单一的 `.windsurfrules`
- **[Kimi Code](#kimi-code)** — 在 `kimi/` 下生成 YAML 格式的智能体规范
- **[Qwen Code](#qwen-code)** — 在 `.qwen/agents/` 下生成项目作用域的 `.md` 子智能体

## 快速安装

```bash
# 自动为所有检测到的工具进行安装
./scripts/install.sh

# 安装特定的个人目录下工具
./scripts/install.sh --tool antigravity
./scripts/install.sh --tool copilot
./scripts/install.sh --tool openclaw
./scripts/install.sh --tool claude-code

# Gemini CLI 在全新克隆后需要先生成集成文件
./scripts/convert.sh --tool gemini-cli
./scripts/install.sh --tool gemini-cli

# Qwen Code 在全新克隆后也需要先生成子智能体文件
./scripts/convert.sh --tool qwen
./scripts/install.sh --tool qwen
```

如果你安装了 OpenClaw 且网关已在运行，请在安装后重启网关：

```bash
openclaw gateway restart
```

对于 OpenCode, Cursor, Aider, Windsurf 和 Qwen Code 等项目作用域工具，请在你的目标项目根目录下运行安装程序，详见下文各工具的特定章节。

## 重新生成集成文件

如果你添加或修改了智能体，请重新生成所有集成文件：

```bash
./scripts/convert.sh
```

---

## Claude Code

The Agency 最初是为 Claude Code 设计的。智能体无需转换即可原生运行。

```bash
cp -r <分类目录>/*.md ~/.claude/agents/
# 或一次性安装全部：
./scripts/install.sh --tool claude-code
```

详见 [claude-code/README.md](claude-code/README.md)。

---

## GitHub Copilot

The Agency 也原生支持 GitHub Copilot。智能体可直接复制到 `~/.github/agents/` 和 `~/.copilot/agents/`，无需转换。

```bash
./scripts/install.sh --tool copilot
```

详见 [github-copilot/README.md](github-copilot/README.md)。

---

## Antigravity

技能将安装至 `~/.gemini/antigravity/skills/`。每个智能体都将成为一个独立的技能，并冠以 `agency-` 前缀以避免命名冲突。

```bash
./scripts/install.sh --tool antigravity
```

详见 [antigravity/README.md](antigravity/README.md)。

---

## Gemini CLI

智能体被打包为一个包含多个技能文件的 Gemini CLI 扩展。该扩展安装至 `~/.gemini/extensions/agency-agents/`。由于 Gemini 的 Manifest 文件和技能文件夹是生成的产物，在从全新克隆的项目安装前，请运行 `./scripts/convert.sh --tool gemini-cli`。

```bash
./scripts/convert.sh --tool gemini-cli
./scripts/install.sh --tool gemini-cli
```

详见 [gemini-cli/README.md](gemini-cli/README.md)。

---

## OpenCode

每个智能体将成为 `.opencode/agents/` 下的一个项目作用域 `.md` 文件。

```bash
cd /你的项目目录 && /路径/至/agency-agents/scripts/install.sh --tool opencode
```

详见 [opencode/README.md](opencode/README.md)。

---

## OpenClaw

每个智能体将成为一个包含 `SOUL.md`、`AGENTS.md` 和 `IDENTITY.md` 的 OpenClaw 工作区。

在安装前，请先生成 OpenClaw 工作区：

```bash
./scripts/convert.sh --tool openclaw
```

然后进行安装：

```bash
./scripts/install.sh --tool openclaw
```

详见 [openclaw/README.md](openclaw/README.md)。

---

## Cursor

每个智能体将成为一个 `.mdc` 规则文件。规则是项目作用域的 —— 请在你的项目根目录下运行安装程序。

```bash
cd /你的项目目录 && /路径/至/agency-agents/scripts/install.sh --tool cursor
```

详见 [cursor/README.md](cursor/README.md)。

---

## Aider

所有智能体将被合并为一个单一的 `CONVENTIONS.md` 文件。当该文件存在于你的项目根目录时，Aider 会自动读取。

```bash
cd /你的项目目录 && /路径/至/agency-agents/scripts/install.sh --tool aider
```

详见 [aider/README.md](aider/README.md)。

---

## Windsurf

所有智能体将被合并为项目根目录下的单一 `.windsurfrules` 文件。

```bash
cd /你的项目目录 && /路径/至/agency-agents/scripts/install.sh --tool windsurf
```

详见 [windsurf/README.md](windsurf/README.md)。

---

## Kimi Code

每个智能体将被转换为 Kimi Code CLI 智能体规范（YAML 格式，配有独立的系统提示词文件）。智能体将安装至 `~/.config/kimi/agents/`。

由于 Kimi 智能体文件是由 Markdown 源码生成的，在全新克隆后安装前，请先运行 `./scripts/convert.sh --tool kimi`。

```bash
./scripts/convert.sh --tool kimi
./scripts/install.sh --tool kimi
```

### 用法

安装后，通过 `--agent-file` 标志使用智能体：

```bash
kimi --agent-file ~/.config/kimi/agents/frontend-developer/agent.yaml
```

或在特定项目中使用：

```bash
cd /你的项目目录
kimi --agent-file ~/.config/kimi/agents/frontend-developer/agent.yaml \
     --work-dir /你的项目目录
```

详见 [kimi/README.md](kimi/README.md)。

---

## Qwen Code

每个智能体将成为 `.qwen/agents/` 下的一个项目作用域 `.md` 子智能体文件。

在全新克隆的项目中，请先生成 Qwen 文件：

```bash
./scripts/convert.sh --tool qwen
```

然后从你的项目根目录进行安装：

```bash
cd /你的项目目录 && /路径/至/agency-agents/scripts/install.sh --tool qwen
```

详见 [qwen/README.md](qwen/README.md)。
