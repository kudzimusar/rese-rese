# AI, multilingual language and agent architecture

**Current authority:** [ADR-0005](../decisions/ADR-0005-ai-deterministic-actions.md) and [full AI technical specification](../source/RR-AI-CORE-01.original.md), interpreted with Rust/TrailBase.

## The AI-native interface
One assistant supports text, recorded voice, live speech where feasible, images and authorized tool calls. Primary languages: English, Shona and Zimbabwean Ndebele, followed by independently certified other Zimbabwean languages and dialects. Offline drafts and human/community-agent fallbacks are required.

```text
Speech/Text/Image -> Recognition -> Intent/Entity Resolution -> Canonical Retrieval
-> AI Proposal -> Typed Validation -> User Confirmation as required
-> Rust Authorized Action Gateway -> TrailBase Transaction -> Actual Result
```

## Requirements
- Local/self-hostable inference with individual **model weight licence** and evaluation checks; no mandatory paid AI API.
- ASR and TTS must be measured separately, including place names, quantities, monetary values, code-switching and low-quality microphones.
- Preserve uncertainty and raw/proposed transcription distinctions; never invent prices, quantity conversions, location precision or credentials.
- Models have no direct unrestricted database access or autonomous funds/identity/official-place authority.
- MCP tools are scoped, least-privilege, typed and auditable. Protect against prompt injection, replay, privilege escalation and leaks.
- Version models, prompts, retrieval data, tool schemas and language evaluation sets.
- Obtain consent for training data; do not repurpose private calls/voice recordings automatically. Protect children and sensitive location information.

## Release gates
Representative language evaluations, adversarial tests, unauthorized-action proofs, offline draft preservation, pricing/unit extraction confirmation and actual action outcome checks. Do not claim Shona/Ndebele support from an untested general multilingual model.
