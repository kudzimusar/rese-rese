# ADR-0004 — Evidence-based village geography
**Status:** Binding product safety and geographic principle · **Date:** 2026-10-28

## Decision
Canonical place IDs are independent of names and coordinates. A village can exist without a GPS point, polygon or postcode. Store names/aliases, administrative relationships, spatial assertions, source provenance, confidence, validity intervals and independent verification separately.

## Consequences
Preserve duplicate names, conflicting ward claims, incomplete maps and uncertain road access. AI inference, H3 cells, Plus Codes and satellite detections are evidence or indexes, **not** proof of a village, boundary, address or safe transport route. Public projections hide private homes, personal movement and sensitive sites. Community mapping requires review and consent.
