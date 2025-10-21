/*
  Bridge Service
  - Listens for Fonoster AudioSocket connections (8kHz PCM, 16-bit, mono)
  - Linearly upsamples x3 to 24kHz PCM
  - Streams the 24kHz PCM to Coze WebSocket as binary frames
  - Consumes Coze streaming text and forwards to the live call via Fonoster SDK say()

  Notes:
  - This is a reference implementation. Adjust Coze handshake, events, and auth according to official docs.
  - For production, add reconnection strategies, heartbeats, backpressure, and strict error handling.
*/

const { AudioSocket } = require("@fonoster/streams");
const WebSocket = require("ws");
const dotenv = require("dotenv");
const SDK = require("@fonoster/sdk");

dotenv.config();

const BRIDGE_WS_PORT = parseInt(process.env.BRIDGE_WS_PORT || "9092", 10);
const COZE_BOT_ID = process.env.COZE_BOT_ID || "YOUR_COZE_BOT_ID";
const COZE_ACCESS_TOKEN = process.env.COZE_ACCESS_TOKEN || "YOUR_COZE_ACCESS_TOKEN";

// Example Coze WS URL (adjust per official docs)
const COZE_WS_URL = `wss://ws.coze.cn/v1/chat?bot_id=${encodeURIComponent(
  COZE_BOT_ID
)}&authorization=Bearer%20${encodeURIComponent(COZE_ACCESS_TOKEN)}`;

// Initialize Fonoster SDK client (ensure env vars are set for auth, e.g., FONOSTER_ENDPOINT, etc.)
// Exact configuration depends on your Fonoster setup.
const voiceSDK = new SDK.Voice();

// Simple linear upsampler 8kHz -> 24kHz (factor 3)
// Input: Buffer of signed 16-bit LE mono PCM
// Output: Buffer of signed 16-bit LE mono PCM at 24kHz (length * 3)
function upsample8kTo24kLinear(int16Buffer) {
  if (!Buffer.isBuffer(int16Buffer)) return Buffer.alloc(0);
  const lenBytes = int16Buffer.length;
  if (lenBytes % 2 !== 0) {
    // trim odd byte
    int16Buffer = int16Buffer.slice(0, lenBytes - 1);
  }

  const samples = int16Buffer.length / 2;
  if (samples === 0) return Buffer.alloc(0);

  const outSamples = samples * 3;
  const out = Buffer.alloc(outSamples * 2);

  // Read helper
  const readS = (idx) => int16Buffer.readInt16LE(idx * 2);
  const writeS = (idx, v) => out.writeInt16LE(Math.max(-32768, Math.min(32767, v | 0)), idx * 2);

  for (let n = 0; n < samples; n++) {
    const s0 = readS(n);
    const s1 = n < samples - 1 ? readS(n + 1) : s0;
    const d = s1 - s0;

    // 3x expansion: s0, s0 + 1/3*d, s0 + 2/3*d
    const base = n * 3;
    writeS(base, s0);
    writeS(base + 1, s0 + d / 3);
    writeS(base + 2, s0 + (2 * d) / 3);
  }

  return out;
}

// Utility queue to buffer upsampled frames until Coze WS becomes OPEN
class FrameQueue {
  constructor() { this.q = []; }
  push(buf) { if (buf && buf.length) this.q.push(buf); }
  drain(sender) {
    while (this.q.length) {
      const chunk = this.q.shift();
      try { sender(chunk); } catch (e) { this.q.unshift(chunk); break; }
    }
  }
  clear() { this.q = []; }
}

const audioSocket = new AudioSocket();

audioSocket.onConnection(async (req, res) => {
  const sessionRef = req.ref;
  console.log(`[Bridge] New AudioSocket connection. sessionRef=${sessionRef}`);

  // Create Coze WebSocket per call/session
  const cozeWs = new WebSocket(COZE_WS_URL);
  const pending = new FrameQueue();
  let cozeOpen = false;

  cozeWs.on("open", () => {
    cozeOpen = true;
    console.log(`[Bridge] Coze connected for sessionRef=${sessionRef}`);

    // Example: send an initial message to declare audio format (adjust per Coze docs)
    try {
      const initMsg = {
        type: "input_audio_format",
        encoding: "pcm_s16le",
        sample_rate: 24000,
        channels: 1
      };
      cozeWs.send(JSON.stringify(initMsg));
    } catch (e) {
      console.warn("[Bridge] Failed to send Coze init message:", e);
    }

    // Flush buffered frames
    pending.drain((chunk) => cozeWs.send(chunk, { binary: true }));
  });

  cozeWs.on("error", (err) => {
    console.error(`[Bridge] Coze WS error (sessionRef=${sessionRef}):`, err);
  });

  cozeWs.on("close", () => {
    cozeOpen = false;
    console.log(`[Bridge] Coze disconnected (sessionRef=${sessionRef}).`);
  });

  // Receive streaming messages from Coze (e.g., partial text tokens)
  cozeWs.on("message", async (message) => {
    try {
      // Messages might be JSON or binary; handle JSON for text events
      if (Buffer.isBuffer(message)) return; // ignore binary (e.g., downstream audio)
      const event = JSON.parse(message.toString("utf8"));

      const et = event.event_type || event.type;
      if ((et === "chat_chunk" || et === "partial" || et === "token") && event.data && event.data.text) {
        const token = String(event.data.text);
        // Stream token to the live call via TTS
        try {
          await voiceSDK.say({ callRef: sessionRef, text: token });
        } catch (e) {
          console.warn(`[Bridge] voiceSDK.say failed (sessionRef=${sessionRef}):`, e.message || e);
        }
      }

      if (et === "dialog_ended" || et === "end") {
        try { await voiceSDK.hangup({ callRef: sessionRef }); } catch (_) {}
      }
    } catch (e) {
      // Non-JSON or parse error; ignore
    }
  });

  // Handle incoming 8kHz audio stream from Fonoster
  res.on("data", (dataBuffer) => {
    // dataBuffer: 8kHz 16-bit PCM (typically 320 bytes per 20ms)
    try {
      const up = upsample8kTo24kLinear(dataBuffer);
      if (!up || up.length === 0) return;

      if (cozeOpen && cozeWs.readyState === WebSocket.OPEN) {
        cozeWs.send(up, { binary: true });
      } else {
        pending.push(up);
      }
    } catch (e) {
      console.warn("[Bridge] Upsample/send error:", e.message || e);
    }
  });

  res.on("end", () => {
    console.log(`[Bridge] AudioSocket ended (sessionRef=${sessionRef})`);
    try { cozeWs.close(); } catch (_) {}
  });

  res.on("error", (err) => {
    console.error(`[Bridge] AudioSocket error (sessionRef=${sessionRef}):`, err);
    try { cozeWs.close(); } catch (_) {}
  });
});

audioSocket.listen(BRIDGE_WS_PORT, () => {
  console.log(`[Bridge] Listening for Fonoster AudioSocket on port ${BRIDGE_WS_PORT}`);
});
