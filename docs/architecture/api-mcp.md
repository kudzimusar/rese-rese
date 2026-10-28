# REST, MCP and events contract

## Authority boundary
Client or assistant -> Rust API gateway -> scoped domain validation -> TrailBase authorized transaction. TrailBase's development/admin MCP is **not** the customer MCP interface.

## Proposed resource families (design only)
- `/v1/places`, `/v1/places/resolve`, `/v1/geo/assertions`, `/v1/geo/review`
- `/v1/occupations`, `/v1/skills`, `/v1/services`
- `/v1/providers`, `/v1/offers`, `/v1/work-requests`
- `/v1/communities`, `/v1/groups`, `/v1/memberships`
- `/v1/quotes`, `/v1/orders`, `/v1/deliveries`
- `/v1/ai/interpret`, `/v1/ai/proposals`, `/v1/ai/action-confirmations`

## Contract properties
JSON Schema/typed Serde payloads, authenticated subjects, scope/record-level permissions, input canonical IDs, version checks, idempotency keys, explicit source/uncertainty, audit event, output redaction, request quotas and stable error codes. All proposed endpoints remain non-deployed.

## MCP tool groups
Public read tools: `search_skills`, `resolve_place`, `find_services`; authenticated read: `list_my_offers`, `list_my_orders`; draft mutations: `prepare_offer`, `suggest_place_assertion`; confirmed mutations: `publish_offer`, `submit_quote`, `update_delivery`. No model tool may directly update verified identity, legal status, payment, canonical map merges or official boundaries.

## Events
`offer.published`, `place.assertion_submitted`, `place.assertion_reviewed`, `group.membership_updated`, `order.accepted`, `delivery.handoff_recorded`. Consumers are idempotent; analytics are projections, not separate writable authority.

See the [AI spec](../source/RR-AI-CORE-01.original.md) and [GEO spec](../source/RR-GEO-AI-01.original.md) for contract examples and threat analysis.
