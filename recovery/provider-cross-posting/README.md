# Recovered provider-cross-posting client SDKs

This directory preserves the exact credential-clean client SDK delta recovered from the 55-day ChatGPT work-reconciliation corpus. The original archive used top-level `go/`, `typescript/`, `rust/`, `dart/`, and `openapi/` paths; current `main` has since standardized SDKs beneath `clients/`, so the recovered snapshot is namespaced here to avoid overwriting newer implementations.

Source branch: `agent/provider-cross-posting`
Original archived commit: `bfe2445a88d81c03d04844c7108b1dbf0ab087da`
Excluded source space: `dancing-dragons`

Review this snapshot against the canonical `clients/` fleet and port missing behavior semantically.
