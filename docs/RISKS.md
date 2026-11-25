# Risk register
| Risk | Mitigation |
|---|---|
| Data loss | Atomic save, recovery, conflict detection, fault injection; release blocker |
| GUI immaturity | Comparative spike, accessibility and IME gate |
| Scope creep | Milestone gates, source-first UX, defer AI/plugins |
| Name conflict | Search EUIPO/WIPO/USPTO, package registries, stores and domains; legal review |
| License incompatibility | Inventory SPDX, review transitive deps and borrowed code before distribution |
| AI provenance/copyright | Review and record origin; avoid unverified copying |
| Low funding | Keep costs bounded; no revenue assumptions in roadmap |
| Platform fragmentation | CI matrix plus real hardware testing; advertise verified support only |
| Malicious files | Offline safe rendering, bounded parsing, fuzzing |
| Maintainer burnout | Small releases, documented decisions, no support SLA |
