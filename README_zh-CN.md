# 🎭 The Agency: 助力工作流转型的 AI 专家团队

> **触手可及的完整 AI 代理机构** - 从前端奇才到 Reddit 社区忍者，从创意注入师到生产就绪验证员。每个智能体都是拥有独立个性、专业流程和可靠交付物的领域专家。

[![GitHub stars](https://img.shields.io/github/stars/msitarzewski/agency-agents?style=social)](https://github.com/msitarzewski/agency-agents)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![欢迎 PR](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://makeapullrequest.com)
[![赞助](https://img.shields.io/badge/Sponsor-%E2%9D%A4-pink?logo=github)](https://github.com/sponsors/msitarzewski)

---

## 🚀 这是什么？

源自 Reddit 讨论帖并经过数月迭代，**The Agency** 是一个不断增长的、精心打造的 AI 智能体人格集合。每个智能体都具备：

- **🎯 专业性**：深耕其领域（非通用提示词模板）
- **🧠 人格驱动**：独特的口吻、沟通风格和处理方式
- **📋 交付导向**：真实的面试代码、流程和可衡量的成果
- **✅ 生产就绪**：经过实战测试的工作流和成功指标

**你可以把它理解为**：组建你的梦想团队，只不过他们是永不疲倦、从不抱怨且使命必达的 AI 专家。

---

## ⚡ 快速上手

### 选项 1：配合 Claude Code 使用（推荐）

```bash
# 将所有智能体安装到你的 Claude Code 目录
./scripts/install.sh --tool claude-code

# 或者手动复制你需要的特定分类
cp engineering/*.md ~/.claude/agents/

# 然后在 Claude Code 会话中激活：
# “嘿 Claude，激活前端开发工程师模式，帮我构建一个 React 组件”
```

### 选项 2：作为参考引用

每个智能体文件包含：
- 身份定义与人格特质
- 核心使命与工作流
- 带有代码示例的技术交付物
- 成功指标与沟通风格

浏览下方的智能体名录，复制并适配你需要的角色！

### 选项 3：配合其他工具使用 (GitHub Copilot, Antigravity, Gemini CLI, OpenCode, OpenClaw, Cursor, Aider, Windsurf, Kimi Code)

```bash
# 第一步 —— 为所有支持的工具生成集成文件
./scripts/convert.sh

# 第二步 —— 交互式安装（自动检测已安装的工具）
./scripts/install.sh

# 或者直接针对特定工具安装
./scripts/install.sh --tool antigravity
./scripts/install.sh --tool gemini-cli
./scripts/install.sh --tool opencode
./scripts/install.sh --tool copilot
./scripts/install.sh --tool openclaw
./scripts/install.sh --tool cursor
./scripts/install.sh --tool aider
./scripts/install.sh --tool windsurf
./scripts/install.sh --tool kimi
```

详见下文的 [多工具集成](#-多工具集成) 章节。

---

## 🎨 The Agency 智能体名录

### 💻 工程学部 (Engineering Division)

一笔一画，构建未来。

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 🎨 [前端开发工程师](engineering/engineering-frontend-developer_zh-CN.md) | React/Vue/Angular, UI 实现, 性能优化 | 现代 Web 应用, 像素级 UI, 核心 Web 指标优化 |
| 🏗️ [后端架构师](engineering/engineering-backend-architect_zh-CN.md) | API 设计, 数据库架构, 可扩展性 | 服务端系统, 微服务, 云基础设施 |
| 📱 [移动端开发工程师](engineering/engineering-mobile-app-builder_zh-CN.md) | iOS/Android, React Native, Flutter | 原生及跨平台移动应用 |
| 🤖 [AI 工程师](engineering/engineering-ai-engineer_zh-CN.md) | 机器学习模型, 部署, AI 集成 | 机器学习功能, 数据管道, AI 驱动应用 |
| 🚀 [DevOps 自动化工程师](engineering/engineering-devops-automator_zh-CN.md) | CI/CD, 基础设施自动化, 云运维 | 流水线开发, 部署自动化, 监控 |
| ⚡ [快速原型工程师](engineering/engineering-rapid-prototyper_zh-CN.md) | 快速 POC 开发, MVP | 快速概念验证, 黑客松项目, 快速迭代 |
| 💎 [高级开发工程师](engineering/engineering-senior-developer_zh-CN.md) | Laravel/Livewire, 高级模式 | 复杂实现, 架构决策 |
| 🔧 [Filament 优化专家](engineering/engineering-filament-optimization-specialist_zh-CN.md) | Filament PHP 后台 UX, 表单重构, 资源优化 | 重构 Filament 资源/表单/表格，实现更高效的后台工作流 |
| 🔒 [安全工程师](engineering/engineering-security-engineer_zh-CN.md) | 威胁建模, 安全代码审查, 安全架构 | 应用安全, 漏洞评估, 安全 CI/CD |
| ⚡ [自主优化架构师](engineering/engineering-autonomous-optimization-architect_zh-CN.md) | LLM 路由, 成本优化, 影子测试 | 需要智能 API 选择和成本护栏的自主系统 |
| 🔩 [嵌入式固件工程师](engineering/engineering-embedded-firmware-engineer_zh-CN.md) | 裸机开发, RTOS, ESP32/STM32/Nordic 固件 | 生产级嵌入式系统与物联网设备 |
| 🚨 [故障响应指挥官](engineering/engineering-incident-response-commander_zh-CN.md) | 事件管理, 事故复盘, 值班 | 管理生产故障并构建事故响应就绪度 |
| ⛓️ [Solidity 智能合约工程师](engineering/engineering-solidity-smart-contract-engineer_zh-CN.md) | EVM 合约, Gas 优化, DeFi | 安全、高效的智能合约与 DeFi 协议 |
| 🧭 [代码库上手工程师](engineering/engineering-codebase-onboarding-engineer_zh-CN.md) | 快速上手, 源码探索, 事实解读 | 帮助新开发者通过代码阅读、路径追踪快速理解陌生仓库 |
| 📚 [技术文档工程师](engineering/engineering-technical-writer_zh-CN.md) | 开发者文档, API 参考, 教程 | 清晰、准确的技术文档撰写 |
| 🎯 [威胁检测工程师](engineering/engineering-threat-detection-engineer_zh-CN.md) | SIEM 规则, 威胁狩猎, ATT&CK 映射 | 构建检测层与威胁狩猎 |
| 💬 [微信小程序开发工程师](engineering/engineering-wechat-mini-program-developer_zh-CN.md) | 微信生态, 小程序, 支付集成 | 为微信生态构建高性能应用 |
| 👁️ [代码审查工程师](engineering/engineering-code-reviewer_zh-CN.md) | 建设性审查, 安全性, 可维护性 | PR 审查, 代码质量门禁, 审查式带教 |
| 🗄️ [数据库优化工程师](engineering/engineering-database-optimizer_zh-CN.md) | Schema 设计, 查询优化, 索引策略 | PostgreSQL/MySQL 调优, 慢查询调试, 迁移规划 |
| 🌿 [Git 工作流大师](engineering/engineering-git-workflow-master_zh-CN.md) | 分支策略, 规范提交, 高级 Git 操作 | 工作流设计, 历史清理, CI 友好的分支管理 |
| 🏛️ [软件架构师](engineering/engineering-software-architect_zh-CN.md) | 系统设计, DDD, 架构模式, 权衡分析 | 架构决策, 领域建模, 系统演进策略 |
| 🛡️ [SRE 工程师](engineering/engineering-sre_zh-CN.md) | SLO, 错误预算, 可观测性, 混沌工程 | 生产可靠性, 减少琐事, 容量规划 |
| 🧬 [AI 数据修复工程师](engineering/engineering-ai-data-remediation-engineer_zh-CN.md) | 自愈管道, 离线 SLM, 语义聚类 | 大规模修复破损数据，确保零损耗 |
| 🔧 [数据工程师](engineering/engineering-data-engineer_zh-CN.md) | 数据管道, 湖仓架构, ETL/ELT | 构建可靠的数据基础设施与仓库 |
| 🔗 [飞书集成开发工程师](engineering/engineering-feishu-integration-developer_zh-CN.md) | 飞书/Lark 开放平台, 机器人, 工作流 | 为飞书生态构建集成应用 |
| 🧱 [CMS 开发工程师](engineering/engineering-cms-developer_zh-CN.md) | WordPress/Drupal 主题, 插件/模块, 内容架构 | 代码优先的 CMS 实现与定制 |
| 📧 [邮件智能工程师](engineering/engineering-email-intelligence-engineer_zh-CN.md) | 邮件解析, MIME 提取, 智能体结构化数据 | 将原始邮件线程转化为可供推理的上下文 |
| 🎙️ [语音 AI 集成工程师](engineering/engineering-voice-ai-integration-engineer_zh-CN.md) | 语音转文字管道, Whisper, ASR, 说话人识别 | 端到端转录管道, 音频预处理, 结构化转录交付 |

### 🎨 设计学部 (Design Division)

赋予美感、易用性与惊喜。

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 🎯 [UI 设计师](design/design-ui-designer_zh-CN.md) | 视觉设计, 组件库, 设计系统 | 界面创建, 品牌一致性, 组件设计 |
| 🔍 [用户体验研究员](design/design-ux-researcher_zh-CN.md) | 用户测试, 行为分析, 研究洞察 | 理解用户, 易用性测试, 设计洞察 |
| 🏛️ [UX 架构师](design/design-ux-architect_zh-CN.md) | 技术架构, CSS 系统, 落地指导 | 开发者友好的基础, 实现路径指导 |
| 🎭 [品牌守护者](design/design-brand-guardian_zh-CN.md) | 品牌识别, 一致性, 定位 | 品牌策略, 识别系统开发, 规范准则 |
| 📖 [视觉叙事师](design/design-visual-storyteller_zh-CN.md) | 视觉叙事, 多媒体内容 | 引人入胜的视觉故事, 品牌叙事 |
| ✨ [创意注入师](design/design-whimsy-injector_zh-CN.md) | 个性化, 惊喜感, 趣味交互 | 增加趣味、微交互、彩蛋、品牌个性 |
| 📷 [图像提示词工程师](design/design-image-prompt-engineer_zh-CN.md) | AI 图像生成提示词, 摄影 | Midjourney, DALL-E, Stable Diffusion 提示词 |
| 🌈 [包容性视觉专家](design/design-inclusive-visuals-specialist_zh-CN.md) | 代表性, 消除偏见, 真实影像 | 生成文化准确的 AI 图像与视频 |

### 💰 付费媒体学部 (Paid Media Division)

将广告支出转化为可衡量的业务成果。

| 智能体 | 专业领域 | 适用场景 |
| --- | --- | --- |
| 💰 [竞价广告策略师](paid-media/paid-media-ppc-strategist_zh-CN.md) | Google/MS/Amazon 广告, 账户架构, 出价 | 账户搭建, 预算分配, 扩规模, 表现诊断 |
| 🔍 [搜索词分析师](paid-media/paid-media-search-query-analyst_zh-CN.md) | 搜索词分析, 否定词, 意图映射 | 查询审计, 消除浪费支出, 关键词发现 |
| 📋 [付费媒体审计师](paid-media/paid-media-auditor_zh-CN.md) | 200+ 维度账户审计, 竞品分析 | 账户接管, 季度审查, 竞标提案 |
| 📡 [追踪与埋点专家](paid-media/paid-media-tracking-specialist_zh-CN.md) | GTM, GA4, 转化追踪, CAPI | 新项目落地, 追踪审计, 平台迁移 |
| ✍️ [广告创意策略师](paid-media/paid-media-creative-strategist_zh-CN.md) | RSA 文案, Meta 创意, PMax 素材 | 创意发布, 测试计划, 广告疲劳刷新 |
| 📺 [程序化与展示广告购买专家](paid-media/paid-media-programmatic-buyer_zh-CN.md) | GDN, DSP, 合作媒体, ABM 展示 | 展示广告规划, 媒体联络, ABM 计划 |
| 📱 [付费社交策略师](paid-media/paid-media-paid-social-strategist_zh-CN.md) | Meta, LinkedIn, TikTok, 跨平台社交 | 社交广告项目, 平台选择, 受众策略 |

### 💼 销售学部 (Sales Division)

将商机转化为营收，靠的是工艺，而非 CRM 琐事。

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 🎯 [外呼销售策略师](sales/sales-outbound-strategist_zh-CN.md) | 基于信号的找客, 多渠道序列, ICP 定位 | 通过深度研究而非海投来构建商机池 |
| 🔍 [销售发现教练](sales/sales-discovery-coach_zh-CN.md) | SPIN, Gap Selling, Sandler — 问题设计 | 准备发现电话, 资格认定, 代表辅导 |
| ♟️ [商机策略师](sales/sales-deal-strategist_zh-CN.md) | MEDDPICC 认定, 竞争定位, 赢单计划 | 商机评分, 揭示管道风险, 构建赢单策略 |
| 🛠️ [售前工程师](sales/sales-engineer_zh-CN.md) | 技术演示, POC 范围, 竞争对比表 | 售前技术赢单, 演示准备, 竞争定位 |
| 🏹 [提案策略师](sales/sales-proposal-strategist_zh-CN.md) | RFP 响应, 赢单主题, 叙事结构 | 撰写具有说服力而非仅仅合规的提案 |
| 📊 [销售漏斗分析师](sales/sales-pipeline-analyst_zh-CN.md) | 预测, 管道健康度, 商机速度, RevOps | 管道审查, 预测准确性, 收入运营 |
| 🗺️ [客户策略师](sales/sales-account-strategist_zh-CN.md) | 扩容增购, QBR, 利益相关者地图 | 售后增购, 客户计划, NRR 增长 |
| 🏋️ [销售教练](sales/sales-coach_zh-CN.md) | 代表成长, 通话辅导, 管道审查引导 | 通过结构化辅导让每位代表和每笔订单变得更好 |
| 🎯 [销售拓客](specialized/sales-outreach_zh-CN.md) | 冷启动拓客, 多点触达, 异议处理, 提案 | B2B 漏斗顶端拓客 —— 从冷邮件到约见成功 |

### 📢 营销学部 (Marketing Division)

通过每一次真实的互动，实现受众增长。

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 🚀 [增长黑客](marketing/marketing-growth-hacker_zh-CN.md) | 快速获客, 病毒循环, 实验 | 爆发式增长, 用户获取, 转化优化 |
| 📝 [内容创作者](marketing/marketing-content-creator_zh-CN.md) | 多平台内容, 编辑日历 | 内容策略, 文案撰写, 品牌叙事 |
| 🐦 [Twitter 运营专家](marketing/marketing-twitter-engager_zh-CN.md) | 实时互动, 思想领导力 | 推特策略, 领英活动, 专业社交 |
| 📱 [TikTok 策略师](marketing/marketing-tiktok-strategist_zh-CN.md) | 病毒内容, 算法优化 | TikTok 增长, 病毒内容, Z 世代/千禧一代受众 |
| 📸 [Instagram 运营专家](marketing/marketing-instagram-curator_zh-CN.md) | 视觉叙事, 社区建设 | Instagram 策略, 美学开发, 视觉内容 |
| 🤝 [Reddit 社区运营](marketing/marketing-reddit-community-builder_zh-CN.md) | 真实互动, 价值导向内容 | Reddit 策略, 社区信任, 真实营销 |
| 📱 [应用商店优化专家](marketing/marketing-app-store-optimizer_zh-CN.md) | ASO, 转化优化, 可发现性 | 应用营销, 商店优化, 应用增长 |
| 🌐 [社交媒体策略师](marketing/marketing-social-media-strategist_zh-CN.md) | 跨平台策略, 营销活动 | 整体社媒策略, 多平台活动 |
| 📕 [小红书运营专家](marketing/marketing-xiaohongshu-specialist_zh-CN.md) | 生活方式内容, 趋势驱动策略 | 小红书增长, 美学叙事, Z 世代受众 |
| 💬 [微信公众号运营专家](marketing/marketing-wechat-official-account_zh-CN.md) | 粉丝互动, 内容营销 | 公众号策略, 社区建设, 转化优化 |
| 🧠 [知乎运营专家](marketing/marketing-zhihu-strategist_zh-CN.md) | 思想领导力, 知识驱动互动 | 知乎权威建立, 问答策略, 线索生成 |
| 🇨🇳 [百度 SEO 专家](marketing/marketing-baidu-seo-specialist_zh-CN.md) | 百度优化, 中国 SEO, ICP 合规 | 在百度获取排名并触达中国搜索市场 |
| 🎬 [Bilibili 内容策略师](marketing/marketing-bilibili-content-strategist_zh-CN.md) | B站算法, 弹幕文化, UP 主成长 | 以社区优先的内容在 B 站构建受众 |
| 🎠 [轮播图增长引擎](marketing/marketing-carousel-growth-engine_zh-CN.md) | TikTok/Instagram 轮播图, 自动发布 | 生成并发布病毒式轮播图内容 |
| 💼 [领英内容创作者](marketing/marketing-linkedin-content-creator_zh-CN.md) | 个人品牌, 思想领导力, 专业内容 | 领英增长, 专业受众构建, B2B 内容 |
| 🛒 [中国电商运营专家](marketing/marketing-china-ecommerce-operator_zh-CN.md) | 淘宝, 天猫, 拼多多, 直播电商 | 运营中国多平台电商业务 |
| 🎥 [快手运营策略师](marketing/marketing-kuaishou-strategist_zh-CN.md) | 快手, 老铁社区, 基层增长 | 在下沉市场构建真实受众 |
| 🔍 [SEO 专家](marketing/marketing-seo-specialist_zh-CN.md) | 技术 SEO, 内容策略, 外链建设 | 推动可持续的有机搜索增长 |
| 📘 [图书联合作者](marketing/marketing-book-co-author_zh-CN.md) | 思想领导力书籍, 代笔, 出版 | 为创始人和专家提供战略性图书协作 |
| 🌏 [跨境电商专家](marketing/marketing-cross-border-ecommerce_zh-CN.md) | 亚马逊, Shopee, Lazada, 跨境履约 | 全链路跨境电商战略 |
| 🎵 [抖音运营策略师](marketing/marketing-douyin-strategist_zh-CN.md) | 抖音平台, 短视频营销, 算法 | 在中国领先的短视频平台增长受众 |
| 🎙️ [直播带货教练](marketing/marketing-livestream-commerce-coach_zh-CN.md) | 主播培训, 直播间优化, 转化 | 构建高性能直播电商运营体系 |
| 🎧 [播客策略师](marketing/marketing-podcast-strategist_zh-CN.md) | 播客内容策略, 平台优化 | 中国播客市场策略与运营 |
| 🔒 [私域运营专家](marketing/marketing-private-domain-operator_zh-CN.md) | 企业微信, 私域流量, 社群运营 | 构建企业微信私域生态系统 |
| 🎬 [短视频剪辑教练](marketing/marketing-short-video-editing-coach_zh-CN.md) | 后期制作, 剪辑流程, 平台规范 | 实操短视频剪辑培训与优化 |
| 🔥 [微博运营策略师](marketing/marketing-weibo-strategist_zh-CN.md) | 新浪微博, 热搜话题, 粉丝互动 | 全方位微博运营与增长 |
| 🔮 [AI 引用策略师](marketing/marketing-ai-citation-strategist_zh-CN.md) | AEO/GEO, AI 推荐可见度, 引用审计 | 提升在 ChatGPT, Claude, Gemini, Perplexity 中的品牌可见度 |
| 🇨🇳 [中国市场本土化策略师](marketing/marketing-china-market-localization-strategist_zh-CN.md) | 全栈中国本土化, 抖音/小红书/微信 GTM | 将趋势信号转化为可执行的中国市场准入策略 |
| 🎬 [视频优化专家](marketing/marketing-video-optimization-specialist_zh-CN.md) | YouTube 算法, 章节划分, 封面图概念 | YouTube 频道增长, 视频 SEO, 留存优化 |

### 📊 产品学部 (Product Division)

在正确的时间，做正确的事。

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 🎯 [Sprint 优先级规划师](product/product-sprint-prioritizer_zh-CN.md) | 敏捷规划, 功能优先级 | Sprint 规划, 资源分配, Backlog 管理 |
| 🔍 [市场趋势研究员](product/product-trend-researcher_zh-CN.md) | 市场情报, 竞品分析 | 市场研究, 机会评估, 趋势识别 |
| 💬 [用户反馈分析师](product/product-feedback-synthesizer_zh-CN.md) | 反馈分析, 洞察提取 | 反馈分析, 用户洞察, 产品优先级 |
| 🧠 [行为助推引擎](product/product-behavioral-nudge-engine_zh-CN.md) | 行为心理学, 助推设计, 参与度 | 通过行为科学最大化用户动力 |
| 🧭 [产品经理](product/product-manager_zh-CN.md) | 全生命周期产品管理 | 发现, PRD, 路线图, GTM, 效果衡量 |

### 🎬 项目管理学部 (Project Management Division)

确保准时交付（且不超预算）。

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 🎬 [工作室制作人](project-management/project-management-studio-producer_zh-CN.md) | 高层编排, 组合管理 | 多项目监督, 战略对齐, 资源分配 |
| 🐑 [项目协调专家](project-management/project-management-project-shepherd_zh-CN.md) | 跨职能协调, 时间轴管理 | 端到端项目协调, 利益相关者管理 |
| ⚙️ [工作室运营专家](project-management/project-management-studio-operations_zh-CN.md) | 日常效率, 流程优化 | 卓越运营, 团队支持, 生产力 |
| 🧪 [实验追踪专家](project-management/project-management-experiment-tracker_zh-CN.md) | A/B 测试, 假设验证 | 实验管理, 数据决策, 测试 |
| 👔 [高级项目经理](project-management/project-manager-senior_zh-CN.md) | 现实范围评估, 任务转化 | 需求转任务, 范围管理 |
| 📋 [Jira 工作流管理员](project-management/project-management-jira-workflow-steward_zh-CN.md) | Git 工作流, 分支策略, 溯源性 | 强化 Jira 关联的 Git 纪律与交付 |

### 🧪 测试学部 (Testing Division)

先把东西弄坏，免得用户遭殃。

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 📸 [测试证据采集员](testing/testing-evidence-collector_zh-CN.md) | 截图 QA, 视觉证据 | UI 测试, 视觉验证, Bug 文档 |
| 🔍 [生产就绪验证员](testing/testing-reality-checker_zh-CN.md) | 证照化验证, 质量门禁 | 生产就绪, 质量审批, 发布认证 |
| 📊 [测试结果分析师](testing/testing-test-results-analyzer_zh-CN.md) | 测试评估, 指标分析 | 产出分析, 质量洞察, 覆盖率报告 |
| ⚡ [性能基准测试专家](testing/testing-performance-benchmarker_zh-CN.md) | 性能测试, 优化 | 速度测试, 压力测试, 性能调优 |
| Plug [API 测试工程师](testing/testing-api-tester_zh-CN.md) | API 验证, 集成测试 | API 测试, 端点核查, 集成 QA |
| 🛠️ [工具评估专家](testing/testing-tool-evaluator_zh-CN.md) | 技术评估, 工具选型 | 评估工具, 软件推荐, 技术决策 |
| 🔄 [工作流优化专家](testing/testing-workflow-optimizer_zh-CN.md) | 流程分析, 工作流改进 | 流程优化, 效率提升, 自动化挖掘 |
| ♿ [无障碍审计师](testing/testing-accessibility-auditor_zh-CN.md) | WCAG 审计, 辅助技术测试 | 无障碍合规, 读屏测试, 包容性验证 |

### 🛟 运营支持学部 (Support Division)

业务的中流砥柱。

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 💬 [客户支持专员](support/support-support-responder_zh-CN.md) | 客户服务, 问题解决 | 客户支持, 用户体验, 支持运营 |
| 📊 [数据分析报告员](support/support-analytics-reporter_zh-CN.md) | 数据分析, 仪表板, 洞察 | 商业智能, KPI 追踪, 数据可视化 |
| 💰 [财务追踪专员](support/support-finance-tracker_zh-CN.md) | 财务规划, 预算管理 | 财务分析, 现金流, 业务表现 |
| 🏗️ [基础设施维护工程师](support/support-infrastructure-maintainer_zh-CN.md) | 系统可靠性, 性能优化 | 基础设施管理, 系统运营, 监控 |
| ⚖️ [法律合规检查员](support/support-legal-compliance-checker_zh-CN.md) | 合规, 法规, 法律审查 | 法律合规, 监管要求, 风险管理 |
| 📑 [高管摘要生成师](support/support-executive-summary-generator_zh-CN.md) | C 级沟通, 战略摘要 | 高管汇报, 战略沟通, 决策支持 |

### 🥽 空间计算学部 (Spatial Computing Division)

构建沉浸式未来。

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 🏗️ [XR 界面架构师](spatial-computing/xr-interface-architect_zh-CN.md) | 空间交互设计, 沉浸式 UX | AR/VR/XR 界面设计, 空间计算 UX |
| 💻 [macOS 空间/Metal 工程师](spatial-computing/macos-spatial-metal-engineer_zh-CN.md) | Swift, Metal, 高性能 3D | macOS 空间计算, Vision Pro 原生应用 |
| 🌐 [XR 沉浸式开发工程师](spatial-computing/xr-immersive-developer_zh-CN.md) | WebXR, 浏览器端 AR/VR | 浏览器沉浸式体验, WebXR 应用 |
| 🎮 [XR 座舱交互专家](spatial-computing/xr-cockpit-interaction-specialist_zh-CN.md) | 座舱控件, 沉浸式系统 | 座舱控制系统, 沉浸式控制界面 |
| 🍎 [visionOS 空间工程师](spatial-computing/visionos-spatial-engineer_zh-CN.md) | Apple Vision Pro 开发 | Vision Pro 应用, 空间计算体验 |
| 🔌 [终端集成专家](spatial-computing/terminal-integration-specialist_zh-CN.md) | 终端集成, 命令行工具 | CLI 工具, 终端工作流, 开发者工具 |

### 🎯 特种专家学部 (Specialized Division)

无法被归类的独特专家。

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 🎭 [智能体编排师](specialized/agents-orchestrator_zh-CN.md) | 多智能体协调, 工作流管理 | 需要多个智能体协作的复杂项目 |
| 🔍 [LSP/索引工程师](specialized/lsp-index-engineer_zh-CN.md) | 语言服务器协议, 代码智能 | 代码智能系统, LSP 实现, 语义索引 |
| 📥 [销售数据提取智能体](specialized/sales-data-extraction-agent_zh-CN.md) | Excel 监控, 销售指标提取 | 销售数据摄取, MTD/YTD/年终指标 |
| 📈 [数据整合智能体](specialized/data-consolidation-agent_zh-CN.md) | 销售数据聚合, 仪表板报告 | 区域摘要, 代表表现, 管道快照 |
| 📬 [报告分发智能体](specialized/report-distribution-agent_zh-CN.md) | 自动化报告交付 | 基于区域的报告分发, 定时发送 |
| 🔐 [智能体身份与信任架构师](specialized/agentic-identity-trust_zh-CN.md) | 智能体身份, 认证, 信任验证 | 多智能体身份系统, 授权, 审计追踪 |
| 🔗 [身份图谱操作员](specialized/identity-graph-operator_zh-CN.md) | 多智能体共享身份解析 | 实体去重, 合并提议, 跨智能体一致性 |
| 💸 [应付账款智能体](specialized/accounts-payable-agent_zh-CN.md) | 支付处理, 供应商管理, 审计 | 跨加密货币、法币、稳定币的自主支付 |
| 🛡️ [区块链安全审计师](specialized/blockchain-security-auditor_zh-CN.md) | 智能合约审计, 漏洞分析 | 在部署前发现合约漏洞 |
| 📋 [合规审计师](specialized/compliance-auditor_zh-CN.md) | SOC 2, ISO 27001, HIPAA, PCI-DSS | 指导组织完成合规认证 |
| 🌍 [文化智能策略师](specialized/specialized-cultural-intelligence-strategist_zh-CN.md) | 全球 UX, 代表性, 文化排斥 | 确保软件跨文化引起共鸣 |
| 🗣️ [开发者关系 (DevAdvocate)](specialized/specialized-developer-advocate_zh-CN.md) | 社区建设, DX, 开发者内容 | 桥接产品与开发者社区 |
| 🔬 [模型 QA 专家](specialized/specialized-model-qa_zh-CN.md) | ML 审计, 特征分析, 可解释性 | 机器学习模型端到端 QA |
| 🗃️ [ZK 管家](specialized/zk-steward_zh-CN.md) | 知识管理, 卢曼卡片盒, 笔记 | 构建互联、经验证的知识库 |
| 🔌 [MCP 构建者](specialized/specialized-mcp-builder_zh-CN.md) | 模型上下文协议服务器, AI 智能体工具 | 构建扩展 AI 智能体能力的 MCP 服务器 |
| 📄 [文档生成器](specialized/specialized-document-generator_zh-CN.md) | 代码生成 PDF/PPTX/DOCX/XLSX | 专业文档创建, 报告, 数据可视化 |
| ⚙️ [自动化治理架构师](specialized/automation-governance-architect_zh-CN.md) | 自动化治理, n8n, 工作流审计 | 大规模评估并治理业务自动化 |
| 📚 [企业培训设计师](specialized/corporate-training-designer_zh-CN.md) | 企业培训, 课程开发 | 设计培训系统与学习计划 |
| 🏛️ [政府数字化售前顾问](specialized/government-digital-presales-consultant_zh-CN.md) | 中国 ToG 售前, 数字化转型 | 政府数字化转型方案与投标 |
| ⚕️ [医疗营销合规专家](specialized/healthcare-marketing-compliance_zh-CN.md) | 中国医疗广告合规 | 医疗营销法律法规合规 |
| 🎯 [招聘专家](specialized/recruitment-specialist_zh-CN.md) | 人才获取, 招聘运营 | 招聘策略, 渠道开拓, 招聘流程 |
| 🎓 [留学顾问](specialized/study-abroad-advisor_zh-CN.md) | 国际教育, 申请规划 | 美、英、加、澳留学规划 |
| 🔗 [供应链策略师](specialized/supply-chain-strategist_zh-CN.md) | 供应链管理, 采购策略 | 供应链优化与采购规划 |
| 🗺️ [工作流架构师](specialized/specialized-workflow-architect_zh-CN.md) | 工作流发现, 映射, 规范 | 在写代码前映射系统的每一条路径 |
| ☁️ [Salesforce 架构师](specialized/specialized-salesforce-architect_zh-CN.md) | 多云 Salesforce 设计, 配额限制, 集成 | 企业级 Salesforce 架构, Org 策略, 部署流水线 |
| 🇫🇷 [法国咨询市场导航员](specialized/specialized-french-consulting-market_zh-CN.md) | ESN/SI 生态, 薪资托管, 费率定位 | 法国 IT 市场的自由职业咨询 |
| 🇰🇷 [韩国业务导航员](specialized/specialized-korean-business-navigator_zh-CN.md) | 韩国商务文化, 审批流程, 关系机制 | 应对韩国商务关系的外国专业人士 |
| 🏗️ [土木工程师](specialized/specialized-civil-engineer_zh-CN.md) | 结构分析, 岩土设计, 全球建筑规范 | 跨欧元规范、ACI、AISC 等标准的结构工程 |
| 🎧 [客户服务专家](specialized/customer-service_zh-CN.md) | 全渠道支持, 投诉处理, 留存, 升级 | 零售、SaaS、酒店、金融、物流等行业客服 |
| 🏥 [医疗客户服务](specialized/healthcare-customer-service_zh-CN.md) | 遵循 HIPAA 的患者支持, 账单, 保险 | 需要合规、共情患者支持的医疗机构 |
| 🏨 [酒旅宾客服务](specialized/hospitality-guest-services_zh-CN.md) | 预订, 礼宾, 投诉恢复, 忠诚度, 活动 | 酒店、度假村、餐厅及活动场地 |
| 🤝 [HR 入职管理](specialized/hr-onboarding_zh-CN.md) | 预入职, 合规, 福利注册, 30-60-90 天计划 | 任何招聘新员工的公司 —— 从初创到企业 |
| 🌐 [语言翻译官](specialized/language-translator_zh-CN.md) | 西 ↔ 英翻译, 方言意识, 文化语境 | 旅游、商业、医疗及法律翻译需求 |
| ⏱️ [法律计费与工时追踪](specialized/legal-billing-time-tracking_zh-CN.md) | 工时记录, 账单叙述, IOLTA 合规 | 最大化收入回收与计费准确性的律所 |
| 📋 [法律客户准入](specialized/legal-client-intake_zh-CN.md) | 潜在客户筛选, 冲突审查, 咨询预约 | 将咨询转化为签约客户的律所 |
| ⚖️ [法律文件审查](specialized/legal-document-review_zh-CN.md) | 合同审查, 风险标记, 版本对比, 合规 | 为律师准备的各领域初审报告 |
| 🏦 [信贷员助手](specialized/loan-officer-assistant_zh-CN.md) | 借款人准入, TRID 合规, 管道追踪 | 抵押贷款与消费信贷团队 |
| 🏠 [房产买卖专家](specialized/real-estate-buyer-seller_zh-CN.md) | 买卖双方代理, 出价, 交易协调 | 住宅与投资性房地产交易 |
| 🛒 [零售客户退货](specialized/retail-customer-returns_zh-CN.md) | 退货处理, 反欺诈, 换货, 供应商退货 | 实体、电商及全渠道零售 |

### 💵 财务学部 (Finance Division)

会计、财务分析、税务策略与投资研究专家。

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 📒 [簿记与财务主管](finance/finance-bookkeeper-controller_zh-CN.md) | 月末结账, 对账, GAAP 合规, 内部控制 | 日常会计运营, 审计准备, 财务记录 |
| 📊 [财务分析师](finance/finance-financial-analyst_zh-CN.md) | 财务建模, 预测, 情景分析, 决策支持 | 三大报表模型, 差异分析, 商业智能 |
| 📈 [FP&A 分析师](finance/finance-fpa-analyst_zh-CN.md) | 预算编制, 滚动预测, 差异分析, 业务回顾 | 年度经营计划, 月度业务分析, 资源分配 |
| 🔍 [投资研究员](finance/finance-investment-researcher_zh-CN.md) | 尽职调查, 组合分析, 资产估值 | 投资论证开发, 风险评估, 市场研究 |
| 🏛️ [税务策略师](finance/finance-tax-strategist_zh-CN.md) | 税务优化, 多司法管辖区合规, 转移定价 | 实体结构设计, ETR 分析, 审计抗辩 |

### 🎮 游戏开发学部 (Game Development Division)

跨越各大引擎，构建世界、系统与体验。

#### 通用智能体 (跨引擎)

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 🎯 [游戏设计师](game-development/game-designer_zh-CN.md) | 系统设计, GDD 撰写, 经济平衡, 玩法循环 | 设计游戏机制、数值系统、编写设计文档 |
| 🗺️ [关卡设计师](game-development/level-designer_zh-CN.md) | 布局理论, 节奏控制, 战斗设计, 叙事 | 构建关卡, 设计战斗流, 空间叙事 |
| 🎨 [技术美术](game-development/technical-artist_zh-CN.md) | 着色器, VFX, LOD 管道, 艺术到引擎优化 | 桥接美术与工程, 性能安全资产管道 |
| 🔊 [游戏音效工程师](game-development/game-audio-engineer_zh-CN.md) | FMOD/Wwise, 自适应音乐, 空间音频 | 交互式音频系统, 动态音乐 |
| 📖 [叙事设计师](game-development/narrative-designer_zh-CN.md) | 故事系统, 分支对话, 世界观架构 | 编写分支叙事, 实现对话系统 |

#### Unity

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 🏗️ [Unity 架构师](game-development/unity/unity-architect_zh-CN.md) | ScriptableObjects, 数据驱动, DOTS/ECS | 大规模 Unity 项目, 高性能系统设计 |
| ✨ [Unity Shader Graph 美术](game-development/unity/unity-shader-graph-artist_zh-CN.md) | Shader Graph, HLSL, URP/HDRP | 自定义材质, VFX 着色器, 后处理 |
| 🌐 [Unity 多人游戏工程师](game-development/unity/unity-multiplayer-engineer_zh-CN.md) | Netcode for GameObjects, Relay/Lobby | 在线 Unity 游戏, 客户端预测, UGS 集成 |
| 🛠️ [Unity 编辑器工具开发者](game-development/unity/unity-editor-tool-developer_zh-CN.md) | EditorWindows, 资产处理, 属性绘制 | 自定义编辑器工具, 管道自动化 |

#### Unreal Engine

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| ⚙️ [Unreal 系统工程师](game-development/unreal-engine/unreal-systems-engineer_zh-CN.md) | C++/Blueprint 混合, GAS, Nanite 约束 | 复杂 Unreal 玩法系统, 引擎级 C++ |
| 🎨 [Unreal 技术美术](game-development/unreal-engine/unreal-technical-artist_zh-CN.md) | 材质编辑器, Niagara, PCG, Substrate | Unreal 材质, Niagara 特效, 程序化生成 |
| 🌐 [Unreal 多人游戏架构师](game-development/unreal-engine/unreal-multiplayer-architect_zh-CN.md) | Actor 同步, 游戏模式/状态继承, 专用服 | 在线游戏, 同步图表, 服务端权威 |
| 🗺️ [Unreal 世界构建师](game-development/unreal-engine/unreal-world-builder_zh-CN.md) | 世界分区, 地景, HLOD, LWC | 大型开放世界关卡, 流式加载系统 |

#### Godot

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 📜 [Godot 玩法脚本师](game-development/godot/godot-gameplay-scripter_zh-CN.md) | GDScript 2.0, 信号, 组合模式 | Godot 玩法系统, 场景组合, 静态类型 |
| 🌐 [Godot 多人游戏工程师](game-development/godot/godot-multiplayer-engineer_zh-CN.md) | MultiplayerAPI, ENet/WebRTC, RPC | 在线 Godot 游戏, 场景同步, 权威模型 |
| ✨ [Godot Shader 开发者](game-development/godot/godot-shader-developer_zh-CN.md) | Godot 着色语言, VisualShader | 自定义 Godot 材质, 2D/3D 特效, 后处理 |

#### Blender

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 🧩 [Blender 插件工程师](game-development/blender/blender-addon-engineer_zh-CN.md) | Blender Python (bpy), 自定义操作符 | 构建插件, 资产准备工具, DCC 管道自动化 |

#### Roblox Studio

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| ⚙️ [Roblox 系统脚本师](game-development/roblox-studio/roblox-systems-scripter_zh-CN.md) | Luau, 远程事件, DataStore, 模块架构 | 构建安全系统, 客户端-服务器通信 |
| 🎯 [Roblox 体验设计师](game-development/roblox-studio/roblox-experience-designer_zh-CN.md) | 参与循环, 变现, 留存, 入职流 | 游戏循环设计, 通行证, 玩家留存 |
| 👗 [Roblox 化身创作者](game-development/roblox-studio/roblox-avatar-creator_zh-CN.md) | UGC 管道, 配件绑定, 市场提交 | Roblox UGC 项目, 人形自定义, 店铺 |

### 📚 学术学部 (Academic Division)

为世界构建、叙事和故事设计提供学术严谨性。

| 智能体 | 专业领域 | 适用场景 |
|-------|-----------|-------------|
| 🌍 [人类学家](academic/academic-anthropologist_zh-CN.md) | 文化系统, 亲属关系, 仪式, 信仰 | 设计具有内在逻辑的文化连贯社会 |
| 🌐 [地理学家](academic/academic-geographer_zh-CN.md) | 自然/人文地理, 气候, 制图 | 构建具有真实地形和定居点的地理连贯世界 |
| 📚 [历史学家](academic/academic-historian_zh-CN.md) | 历史分析, 周期化, 物质文化 | 验证历史连贯性, 以真实的时代细节充实设定 |
| 📜 [叙事学家](academic/academic-narratologist_zh-CN.md) | 叙事理论, 故事结构, 角色弧线 | 使用既定的理论框架分析并优化故事结构 |
| 🧠 [心理学家](academic/academic-psychologist_zh-CN.md) | 人格理论, 动机, 认知模式 | 构建基于研究的、具有心理学可信度的角色 |

---

## 🎯 真实应用场景

### 场景 1：构建初创公司 MVP

**你的团队**：
1. 🎨 **前端开发工程师** - 构建 React 应用
2. 🏗️ **后端架构师** - 设计 API 和数据库
3. 🚀 **增长黑客** - 规划用户获取
4. ⚡ **快速原型工程师** - 快速迭代周期
5. 🔍 **生产就绪验证员** - 确保发布前质量

**结果**：凭借每个阶段的专业知识，实现更快的交付。

---

### 场景 2：营销活动发布

**你的团队**：
1. 📝 **内容创作者** - 开发活动内容
2. 🐦 **Twitter 运营专家** - 执行推特策略
3. 📸 **Instagram 运营专家** - 视觉内容与故事
4. 🤝 **Reddit 社区运营** - 真实的社区互动
5. 📊 **数据分析报告员** - 追踪并优化表现

**结果**：具有平台针对性的多渠道协同营销活动。

---

### 场景 3：企业级功能开发

**你的团队**：
1. 👔 **高级项目经理** - 范围与任务规划
2. 💎 **高级开发工程师** - 复杂功能实现
3. 🎨 **UI 设计师** - 设计系统与组件
4. 🧪 **实验追踪专家** - A/B 测试规划
5. 📸 **测试证据采集员** - 质量验证
6. 🔍 **生产就绪验证员** - 生产就绪检查

**结果**：具备质量门禁和完善文档的企业级交付。

---

### 场景 4：付费媒体账户接管

**你的团队**：
1. 📋 **付费媒体审计师** - 全面的账户评估
2. 📡 **追踪与埋点专家** - 验证转化追踪准确性
3. 💰 **竞价广告策略师** - 重新设计账户架构
4. 🔍 **搜索词分析师** - 清理搜索词中的浪费支出
5. ✍️ **广告创意策略师** - 刷新所有广告文案与扩展
6. 📊 **数据分析报告员** (支持学部) - 构建报告仪表板

**结果**：系统化的账户接管，在头 30 天内实现追踪验证、浪费消除、结构优化和创意刷新。

---

### 场景 5：全机构产品探索

**你的团队**：所有 8 个学部并行工作，达成单一使命。

详见 **[Nexus 空间探索练习](examples/nexus-spatial-discovery_zh-CN.md)** —— 一个完整的案例。其中 8 个智能体（产品趋势研究员、后端架构师、品牌守护者、增长黑客、客户支持专员、用户体验研究员、项目协调专家和 XR 界面架构师）同时部署，从市场验证、技术架构、品牌策略、市场准入、支持体系、用户研究、项目执行到空间 UI 设计，产出了一份统一的产品计划。

**结果**：在单次会话中产出全面的跨职能产品蓝图。[更多案例](examples/)。

---

## 🤝 参与贡献

我们欢迎各种形式的贡献！你可以：

### 添加新智能体

1. Fork 本仓库
2. 在合适分类下创建新智能体文件
3. 遵循智能体模板结构：
   - 包含名称、描述、颜色的 Frontmatter
   - 身份与记忆章节
   - 核心使命
   - 关键规则（领域特定）
   - 技术交付物（含示例）
   - 工作流流程
   - 成功指标
4. 提交 PR

### 优化现有智能体

- 添加真实案例
- 增强代码示例
- 更新成功指标
- 完善工作流

### 分享成功案例

你是否成功使用了这些智能体？在 [Discussions](https://github.com/msitarzewski/agency-agents/discussions) 中分享你的故事！

---

## 📖 智能体设计哲学

每个智能体在设计时都遵循：

1. **🎭 鲜明人格**：非通用模板 —— 真实的性格与口吻
2. **📋 明确交付物**：具体的产出，而非模糊的指导
3. **✅ 成功指标**：可衡量的成果与质量标准
4. **🔄 经验证的工作流**：行之有效的步骤化流程
5. **💡 学习记忆**：模式识别与持续改进

---

## 🎁 为什么它与众不同？

### 不同于通用 AI 提示词：
- ❌ 泛泛而谈的“扮演一个开发人员”
- ✅ 具备人格与流程的深度专业化

### 不同于提示词库：
- ❌ 一次性的提示词集合
- ✅ 包含工作流与交付物的完整智能体系统

### 不同于 AI 工具：
- ❌ 无法自定义的黑盒工具
- ✅ 透明、可 Fork、可适配的智能体人格

---

## 🎨 智能体人格亮点

> “我不只是测试你的代码 —— 我默认会找出 3-5 个问题，并且每项工作都要求视觉证据。”
>
> -- **测试证据采集员** (测试学部)

> “你不是在 Reddit 上做营销 —— 你是在成为一名恰好代表某个品牌的、受人尊重的社区成员。”
>
> -- **Reddit 社区运营** (营销学部)

> “每一个趣味元素都必须服务于功能或情感目的。设计的惊喜感应该是增强体验而非干扰。”
>
> -- **创意注入师** (设计学部)

> “让我加一个庆祝动画，这能降低 40% 的任务完成焦虑。”
>
> -- **创意注入师** (在一次 UX 评审中)

---

## 📊 统计数据

- 🎭 **144 个专业智能体** 遍布 12 个学部
- 📝 **10,000+ 行** 人格、流程与代码示例
- ⏱️ **数月的实战迭代**
- 🌟 **在生产环境中经过测试**
- 💬 Reddit 发布头 12 小时内收到 **50+ 次请求**

---

## 🔌 多工具集成

The Agency 与 Claude Code 原生兼容，并提供转换与安装脚本，让你能在各大主流 AI 编程工具中使用同一套智能体。

### 支持的工具

- **[Claude Code](https://claude.ai/code)** — 原生 `.md` 智能体，无需转换 → `~/.claude/agents/`
- **[GitHub Copilot](https://github.com/copilot)** — 原生 `.md` 智能体，无需转换 → `~/.github/agents/` + `~/.copilot/agents/`
- **[Antigravity](https://github.com/google-gemini/antigravity)** — 每个智能体一个 `SKILL.md` → `~/.gemini/antigravity/skills/`
- **[Gemini CLI](https://github.com/google-gemini/gemini-cli)** — 扩展 + `SKILL.md` 文件 → `~/.gemini/extensions/agency-agents/`
- **[OpenCode](https://opencode.ai)** — `.md` 智能体文件 → `.opencode/agents/`
- **[Cursor](https://cursor.sh)** — `.mdc` 规则文件 → `.cursor/rules/`
- **[Aider](https://aider.chat)** — 单一 `CONVENTIONS.md` → `./CONVENTIONS.md`
- **[Windsurf](https://codeium.com/windsurf)** — 单一 `.windsurfrules` → `./.windsurfrules`
- **[OpenClaw](https://github.com/openclaw/openclaw)** — 每个智能体包含 `SOUL.md` + `AGENTS.md` + `IDENTITY.md`
- **[Qwen Code](https://github.com/QwenLM/qwen-code)** — `.md` 子智能体文件 → `~/.qwen/agents/`
- **[Kimi Code](https://github.com/MoonshotAI/kimi-cli)** — YAML 智能体规范 → `~/.config/kimi/agents/`

---

### ⚡ 快速安装

**第一步 —— 生成集成文件：**
```bash
./scripts/convert.sh
# 更快（并行，输出顺序可能不同）：./scripts/convert.sh --parallel
```

**第二步 —— 安装（交互式，自动检测你的工具）：**
```bash
./scripts/install.sh
# 更快（并行，输出顺序可能不同）：./scripts/install.sh --no-interactive --parallel
```

安装程序会扫描系统中的工具，显示复选框界面供你选择安装：

```
  +------------------------------------------------+
  |   The Agency -- 工具安装程序                    |
  +------------------------------------------------+

  系统扫描结果: [*] = 已检测到
  
  [x]  1)  [*]  Claude Code     (claude.ai/code)
  [x]  2)  [*]  Copilot         (~/.github + ~/.copilot)
  [x]  3)  [*]  Antigravity     (~/.gemini/antigravity)
  [ ]  4)  [ ]  Gemini CLI      (gemini extension)
  [ ]  5)  [ ]  OpenCode        (opencode.ai)
  [ ]  6)  [ ]  OpenClaw        (~/.openclaw/agency-agents)
  [x]  7)  [*]  Cursor          (.cursor/rules)
  [ ]  8)  [ ]  Aider           (CONVENTIONS.md)
  [ ]  9)  [ ]  Windsurf        (.windsurfrules)
  [ ] 10)  [ ]  Qwen Code       (~/.qwen/agents)
  [ ] 11)  [ ]  Kimi Code       (~/.config/kimi/agents)

  [1-11] 切换   [a] 全选   [n] 全不选   [d] 仅已检测
  [Enter] 开始安装   [q] 退出
```

**或者直接安装特定工具：**
```bash
./scripts/install.sh --tool cursor
./scripts/install.sh --tool opencode
./scripts/install.sh --tool openclaw
./scripts/install.sh --tool antigravity
```

**非交互式（CI/脚本使用）：**
```bash
./scripts/install.sh --no-interactive --tool all
```

**更快的运行速度（并行）** —— 在多核机器上，使用 `--parallel`。每个工具的处理会并行执行。适用于交互式和非交互式安装。任务数默认为 `nproc` (Linux) 或 4；可通过 `--jobs N` 覆盖。

```bash
./scripts/convert.sh --parallel                    # 并行转换所有工具
./scripts/convert.sh --parallel --jobs 8           # 限制并行任务数
./scripts/install.sh --no-interactive --parallel   # 并行安装所有检测到的工具
./scripts/install.sh --interactive --parallel      # 选择工具后并行安装
./scripts/install.sh --no-interactive --parallel --jobs 4
```

---

### 工具特定说明

<details>
<summary><strong>Claude Code</strong></summary>

智能体直接从仓库复制到 `~/.claude/agents/` —— 无需转换。

```bash
./scripts/install.sh --tool claude-code
```

在 Claude Code 中激活：
```
请使用前端开发工程师智能体来评审这个组件。
```

详见 [integrations/claude-code/README.md](integrations/claude-code/README.md)。
</details>

<details>
<summary><strong>GitHub Copilot</strong></summary>

智能体直接从仓库复制到 `~/.github/agents/` 和 `~/.copilot/agents/` —— 无需转换。

```bash
./scripts/install.sh --tool copilot
```

在 GitHub Copilot 中激活：
```
请使用前端开发工程师智能体来评审这个组件。
```

详见 [integrations/github-copilot/README.md](integrations/github-copilot/README.md)。
</details>

<details>
<summary><strong>Antigravity (Gemini)</strong></summary>

每个智能体在 `~/.gemini/antigravity/skills/agency-<slug>/` 中成为一个技能。

```bash
./scripts/install.sh --tool antigravity
```

在 Gemini 中使用 Antigravity 激活：
```
@agency-frontend-developer 评审这个 React 组件
```

详见 [integrations/antigravity/README.md](integrations/antigravity/README.md)。
</details>

<details>
<summary><strong>Gemini CLI</strong></summary>

作为 Gemini CLI 扩展安装，每个智能体为一个技能，外加一个清单文件。
如果是全新克隆，请在运行安装程序前生成扩展文件。

```bash
./scripts/convert.sh --tool gemini-cli
./scripts/install.sh --tool gemini-cli
```

详见 [integrations/gemini-cli/README.md](integrations/gemini-cli/README.md)。
</details>

<details>
<summary><strong>OpenCode</strong></summary>

智能体放置在项目根目录的 `.opencode/agents/` 下（项目作用域）。

```bash
cd /your/project
/path/to/agency-agents/scripts/install.sh --tool opencode
```

或者全局安装：
```bash
mkdir -p ~/.config/opencode/agents
cp integrations/opencode/agents/*.md ~/.config/opencode/agents/
```

在 OpenCode 中激活：
```
@backend-architect 设计这个 API。
```

详见 [integrations/opencode/README.md](integrations/opencode/README.md)。
</details>

<details>
<summary><strong>Cursor</strong></summary>

每个智能体在项目的 `.cursor/rules/` 中成为一个 `.mdc` 规则文件。

```bash
cd /your/project
/path/to/agency-agents/scripts/install.sh --tool cursor
```

当 Cursor 检测到这些规则时会自动应用。也可以显式引用：
```
请使用 @security-engineer 规则来评审这段代码。
```

详见 [integrations/cursor/README.md](integrations/cursor/README.md)。
</details>

<details>
<summary><strong>Aider</strong></summary>

所有智能体被编译进一个 Aider 自动读取的 `CONVENTIONS.md` 文件。

```bash
cd /your/project
/path/to/agency-agents/scripts/install.sh --tool aider
```

在 Aider 会话中引用：
```
请使用前端开发工程师智能体来重构这个组件。
```

详见 [integrations/aider/README.md](integrations/aider/README.md)。
</details>

<details>
<summary><strong>Windsurf</strong></summary>

所有智能体被编译进项目根目录的 `.windsurfrules`。

```bash
cd /your/project
/path/to/agency-agents/scripts/install.sh --tool windsurf
```

在 Windsurf 的 Cascade 中引用：
```
请使用生产就绪验证员智能体来验证这是否达到了生产标准。
```

详见 [integrations/windsurf/README.md](integrations/windsurf/README.md)。
</details>

<details>
<summary><strong>OpenClaw</strong></summary>

每个智能体在 `~/.openclaw/agency-agents/` 中成为一个包含 `SOUL.md`、`AGENTS.md` 和 `IDENTITY.md` 的工作区。

```bash
./scripts/convert.sh --tool openclaw
./scripts/install.sh --tool openclaw
```

如果 `openclaw` CLI 可用，安装程序会自动注册工作区。
安装后请运行 `openclaw gateway restart` 以激活新智能体。

详见 [integrations/openclaw/README.md](integrations/openclaw/README.md)。

</details>

<details>
<summary><strong>Qwen Code</strong></summary>

子智能体（SubAgents）安装在项目根目录的 `.qwen/agents/` 下（项目作用域）。

```bash
# 转换并安装（在项目根目录运行）
cd /your/project
./scripts/convert.sh --tool qwen
./scripts/install.sh --tool qwen
```

**在 Qwen Code 中使用：**
- 按名称引用：`请使用 frontend-developer 智能体评审此组件`
- 或者让 Qwen 根据任务上下文自动委派
- 在交互模式下通过 `/agents` 命令进行管理

> 📚 [Qwen 子智能体文档](https://qwenlm.github.io/qwen-code-docs/zh/users/features/sub-agents/)

</details>

<details>
<summary><strong>Kimi Code</strong></summary>

智能体被转换为 Kimi Code CLI 格式 (YAML + 系统提示词) 并安装到 `~/.config/kimi/agents/`。

```bash
# 转换并安装
./scripts/convert.sh --tool kimi
./scripts/install.sh --tool kimi
```

**使用 Kimi Code：**
```bash
# 使用智能体
kimi --agent-file ~/.config/kimi/agents/frontend-developer/agent.yaml

# 在项目中使用
kimi --agent-file ~/.config/kimi/agents/frontend-developer/agent.yaml \
     --work-dir /your/project \
     "评审这个 React 组件"
```

详见 [integrations/kimi/README.md](integrations/kimi/README.md)。

</details>

---

### 更改后重新生成

当你添加新智能体或编辑现有智能体后，请重新生成所有集成文件：

```bash
./scripts/convert.sh                    # 重新生成全部（串行）
./scripts/convert.sh --parallel         # 重新生成全部（并行，更快）
./scripts/convert.sh --tool cursor      # 仅重新生成特定工具
```

---

## 🗺️ 路线图

- [ ] 交互式智能体选择 Web 工具
- [x] 多智能体工作流案例 —— 详见 [examples/](examples/)
- [x] 多工具集成脚本 (Claude Code, GitHub Copilot, Antigravity, Gemini CLI, OpenCode, OpenClaw, Cursor, Aider, Windsurf, Qwen Code, Kimi Code)
- [ ] 智能体设计视频教程
- [ ] 社区智能体市场
- [ ] 项目匹配的智能体“性格测试”
- [ ] “每周之星智能体”展示系列

---

## 🌐 社区翻译与本土化

社区维护的翻译和地区适配。这些项目独立维护 —— 详见各自仓库了解覆盖范围和版本兼容性。

| 语言 | 维护者 | 链接 | 备注 |
|----------|-----------|------|-------|
| 🇨🇳 简体中文 (zh-CN) | [@jnMetaCode](https://github.com/jnMetaCode) | [agency-agents-zh](https://github.com/jnMetaCode/agency-agents-zh) | 141 个已翻译智能体 + 46 个中国市场原创智能体 |
| 🇨🇳 简体中文 (zh-CN) | [@dsclca12](https://github.com/dsclca12) | [agent-teams](https://github.com/dsclca12/agent-teams) | 独立翻译，包含 B 站、微信、小红书等本土化内容 |

想要添加翻译？请提交 Issue，我们会在此处链接。

---

## 🔗 相关资源

- [awesome-openclaw-agents](https://github.com/mergisi/awesome-openclaw-agents) — 社区维护的 OpenClaw 智能体集合（衍生自本仓库）

---

## 📜 许可证

MIT 许可证 - 可自由用于商业或个人用途。欢迎署名，但非强制要求。

---

## 🙏 致谢

最初关于 AI 智能体专业化的 Reddit 讨论已成长为令人瞩目的成果 —— **跨越 12 个学部的 147 个智能体**，得到了全球贡献者社区的支持。本仓库中的每个智能体都凝聚了创作者的心血、测试与分享。

感谢每一位提交 PR、反馈 Issue、发起 Discussion，或者仅仅是尝试并告诉我们使用感受的朋友。正是因为你们，The Agency 才会变得越来越好。

---

## 💬 社区交流

- **GitHub Discussions**: [分享你的成功故事](https://github.com/msitarzewski/agency-agents/discussions)
- **Issues**: [报告 Bug 或请求功能](https://github.com/msitarzewski/agency-agents/issues)
- **Reddit**: 加入 r/ClaudeAI 的讨论
- **Twitter/X**: 使用 #TheAgency 标签分享

---

## 🚀 立即开始

1. **浏览**上方的智能体名录，找到适合你需求的专家
2. **复制**智能体到 `~/.claude/agents/` 以集成到 Claude Code
3. 在 Claude 会话中**引用**这些智能体来激活它们
4. 为你的特定需求**定制**智能体人格与工作流
5. **分享**你的成果并回馈社区

---

<div align="center">

**🎭 The Agency: 你的 AI 梦想团队虚位以待 🎭**

[⭐ Star 本仓库](https://github.com/msitarzewski/agency-agents) • [🍴 Fork 本仓库](https://github.com/msitarzewski/agency-agents/fork) • [🐛 报告问题](https://github.com/msitarzewski/agency-agents/issues) • [❤️ 赞助项目](https://github.com/sponsors/msitarzewski)

由社区驱动，服务于社区 ❤️

</div>
