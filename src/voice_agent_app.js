/*
  Voice Application for Fonoster VoiceServer
  - Answers call, plays a quick greeting
  - Starts a Stream (OUT) to the Bridge Service so that 8kHz PCM audio flows to it
*/

const VoiceServer = require("@fonoster/voice").default;
const dotenv = require("dotenv");
dotenv.config();

const BRIDGE_PUBLIC_HOST = process.env.BRIDGE_PUBLIC_HOST || "127.0.0.1";
const BRIDGE_WS_PORT = process.env.BRIDGE_WS_PORT || 9092;
const VOICE_SERVER_PORT = parseInt(process.env.VOICE_SERVER_PORT || "50061", 10);

const BRIDGE_ENDPOINT = `ws://${BRIDGE_PUBLIC_HOST}:${BRIDGE_WS_PORT}`;

new VoiceServer().listen(async (req, voice) => {
  const { sessionRef } = req;

  try {
    await voice.answer();
    await voice.say("您好，已为您接入智能客服，请讲话。");

    // Start streaming OUT to the Bridge Service (AudioSocket)
    await voice.stream({
      direction: "OUT",
      endpoint: BRIDGE_ENDPOINT
    });

    // Keep the call open; the Bridge Service will orchestrate say()/hangup() via the SDK.
    // You might add timers, silence detection, or other business logic here.
    console.log(`[VoiceApp] Stream started for sessionRef=${sessionRef} -> ${BRIDGE_ENDPOINT}`);
  } catch (err) {
    console.error("[VoiceApp] Error handling call:", err);
    try { await voice.hangup(); } catch (_) {}
  }
}).listen(VOICE_SERVER_PORT);

console.log(`[VoiceApp] Listening on port ${VOICE_SERVER_PORT}. Forwarding audio to ${BRIDGE_ENDPOINT}`);
