# ADR-0003 — Free APIs and open technology
**Status:** Binding Owner constraint · **Date:** 2026-10-28

## Decision
No mandatory paid or pay-per-call API for maps, geocoding, AI, speech, messaging, data or commerce. Prefer commercially usable open datasets, self-hosted APIs, locally runnable inference and open protocols. Public zero-cost free tiers are **not** assumed unlimited or production-safe.

## Compliance
Every dependency must record source URL, exact licence, commercial-use rights, cost category, limits, version, data attribution, hosting costs and fallback. Block unlicensed downloads or noncommercial-only weights in commercial production. Public OSM tile, Nominatim and Overpass endpoints are not the scalable production backbone; use rights-cleared extracts and owned/self-hosted infrastructure.

## Caveat
Free API access does **not** mean zero hardware, bandwidth, storage, GPU or operational cost.
