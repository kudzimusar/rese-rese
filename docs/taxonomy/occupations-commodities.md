# Occupations, capabilities, services, goods and commerce

## Distinct canonical concepts
`BrowseCategory` (navigation), `Occupation` (kind of work), `Skill` (capability), `ServiceDefinition` (customer task), `ProviderCapability` (claimed/evidenced ability), `MarketplaceOffer` (actual offer), `WorkRequest` (demand), `Commodity` (goods), `Unit` (quantities), `Rental` (asset use), and `TransportService` (logistics).

## Twenty-four browse families (initial)
1. Construction & Building
2. Repairs & Maintenance
3. Automotive & Machinery
4. Solar, Energy & Water
5. Agriculture & Farming
6. Transport & Logistics
7. Domestic & Household
8. Care & Personal Assistance
9. Beauty & Grooming
10. Fashion & Tailoring
11. Food & Catering
12. Education & Tutoring
13. Technology & Digital
14. Creative & Media
15. Business & Administration
16. Accounting & Finance
17. Legal & Professional
18. Health & Wellness
19. Events & Entertainment
20. Property & Real Estate
21. Security & Safety
22. Animal & Veterinary
23. Tourism & Hospitality
24. General Labour & Odd Jobs

Include safely authorized mining-related services, highlands/remote work, animal husbandry, crop production, local manufacturing, logistics, diaspora procurement and public infrastructure in appropriate service/product facets. Do not expose prohibited or regulated services without proper controls.

## Invariants
One person may offer many services; one service can require many skills; provider claims do not equal verified qualifications. Services, physical goods, labour, rentals, transport, procurement and employment are **different offer types** under a single canonical marketplace family. Prices and local units remain explicit—never silently turn "20 crates" into kilograms or infer an amount from a photo. Use ISCO/ESCO as external references, not automatic internal IDs.

See [master taxonomy](../source/RR-MASTER-01.original.md).
