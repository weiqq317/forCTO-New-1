# Fonoster ↔ Coze 实时语音桥接服务（含 8kHz ↔ 24kHz 重采样）
本项目提供一个参考实现，用于在 Fonoster（基于 Asterisk 的语音平台）与扣子（Coze）实时语音服务之间建立桥接。核心目标是构建一个稳定、低延迟的 8kHz ↔ 24kHz 跨平台语音交互管道，并在上行方向实现 8kHz（电话级）到 24kHz（广播级）的实时重采样。

该实现包含两部分：
- Voice Application（运行在 Fonoster VoiceServer 上，负责接听来电并启动音频流）
- Bridge Service（独立服务，接收 Fonoster 8kHz 音频，重采样为 24kHz 并通过 WebSocket 转发至 Coze，同时将 Coze 的流式文本通过 Fonoster SDK 回放给来电方）

注意：本项目为“参考实现”，旨在配合自托管的 Fonoster 环境与 Coze 文档要求进行快速集成验证。生产部署请结合企业网络与安全策略审慎评估。

## 一、Fonoster 自托管与网络配置（摘要）
- 使用 Docker / Docker Compose 自托管 Fonoster。
- 在内部测试环境中，将以下环境变量设置为宿主机的实际可路由 IP：
  - ROUTR_EXTERNAL_ADDR（SIP 信令外网地址公告）
  - ROUTR_RTPENGINE_HOST（RTP 媒体监听/公告地址）
  - ASTERISK_SIPPROXY_HOST（Asterisk 用于代理 SIP/媒体的地址）
- 参考 Fonoster 官方仓库的 .env、compose.yaml、config/* 模板配置。
- 启动：`docker-compose up -d`
- 验证：用 SIP 软电话注册至 Routr（默认 5060/UDP），拨打内部号码，验证基础呼叫可达。

## 二、协议与桥接要点
- Fonoster AudioSocket（Stream 动词）
  - 方向：目前仅支持 OUT（从呼叫通道 → 外部 WebSocket）
  - 编码：16-bit PCM（SLIN），采样率 8000 Hz，单声道
  - 常见块：320 字节 ≈ 20ms（160 样本 × 2 字节）
- Coze 实时语音 WebSocket
  - 协议：WSS，全双工
  - 采样率：建议 24000 Hz（默认）
  - 音频：PCM/G711/OPUS 等，建议 24kHz PCM 获最佳效果
- 核心桥接：
  - 上行（Fonoster → Coze）：8kHz PCM 必须实时上采样至 24kHz PCM。320B/20ms → 960B/20ms。
  - 下行（Coze → Fonoster）：由于 Fonoster Stream 不支持 IN 方向，采用“文本控制 + Fonoster SDK TTS/Say”策略。即：Bridge Service 读 Coze 的流式文本 token，通过 Fonoster SDK 的 `say()` 对来电方播放（TTS 由 Fonoster 核心或第三方集成完成）。

## 三、项目结构
- `src/voice_agent_app.js`
  - 在 Fonoster VoiceServer 上运行，接听来电并调用 `voice.stream({ direction: "OUT", endpoint })` 将 8kHz 音频推送给 Bridge Service。
- `src/bridge_service.js`
  - 独立运行，监听来自 Fonoster 的 AudioSocket 连接；对 8kHz PCM 进行 x3 上采样为 24kHz PCM；将 24kHz PCM 以二进制帧推送给 Coze WebSocket。
  - 默认使用简化的线性插值上采样（8k → 24k）。如安装可选依赖 `@purinton/resampler`，则自动切换为其高质量重采样实现。
  - 接收 Coze 返回的流式文本 token，调用 Fonoster SDK 的 `say()` 接口面向当前会话播放 TTS。
- `.env.example`
  - 示例环境变量（Coze 访问参数、Bridge/Voice 服务端口、Fonoster SDK 鉴权等）。

## 四、快速开始
1) 准备运行环境
   - Node.js ≥ 16
   - Fonoster 自托管环境（VoiceServer 可达）
2) 安装依赖
   - 本仓库仅提供参考代码与 package.json，实际运行前请执行：
     - `npm install`
   - 可选：安装高质量重采样器 `@purinton/resampler`（未安装则自动回退至线性插值）：
     - `npm install @purinton/resampler`
3) 配置环境变量
   - 复制 `.env.example` 为 `.env` 并补充：
     - `COZE_BOT_ID`、`COZE_ACCESS_TOKEN`
     - `BRIDGE_WS_PORT`（默认 9092）、`VOICE_SERVER_PORT`（默认 50061）
     - `BRIDGE_PUBLIC_HOST`（Fonoster 可访问到的 Bridge Service 地址，例如 192.168.1.10）
     - Fonoster SDK 的访问凭证（示例变量见 `.env.example`）
4) 启动服务
   - 启动 Bridge Service：`npm run start:bridge`
   - 启动 Voice App：`npm run start:voice`
   - 使用 SIP 软电话呼入，观察 Console 日志与双向实时交互。

## 五、延迟优化建议（摘要）
- 采用流式 LLM 输出（token 级），Bridge Service 逐 token 调用 `say()`，避免整句等待。
- 严控单次回复 token 上限（例如 150-200），缩短 LLM 思考与合成时间。
- 如需更好音质/更低延迟，可替换为高性能的原生/向量化 DSP 重采样实现。

## 六、重要说明
- 本参考实现未包含对 Coze WebSocket 全量协议的严谨状态机实现，仅示意了一个典型流式处理流程。实际生产需根据 Coze 官方文档完善初始握手、心跳、事件处理、重连、鉴权失败处理等逻辑。
- 如果 Coze 返回 24kHz 音频 URL 供播放，请在应用层通过 `voice.play()` 触发播放。Asterisk/Fonoster 会在播放前进行必要的格式转换（如从 24kHz 转 8kHz）。
- 本项目不包含 CI/CD 配置与单元测试，建议在接入企业环境时补充。

## 参考
- Fonoster 官方文档与示例
- Coze 实时音频/对话 WebSocket 文档
