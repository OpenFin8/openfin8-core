# OpenFin8:Core - Business Requirements & Functionality

> Audience: product owners, business analysts, and anyone evaluating what
> openfin8-core **does for the business**. For architecture read `README.md`;
> for wire-level detail read `DESIGN.md` (OBARCH-1).

## 1. Business Purpose

openfin8-core is the **open-finance kernel**: the shared implementation
behind every jurisdiction adapter. It is a library crate - no process
runs on it - and it delivers:

- **The consent authority** - the provider's grant ledger and the
  recipient's connection registry (the two-ledger model), so that no
  customer data ever leaves an institution without a live, revocable,
  scope-limited consent.
- **Data minimization as an engine** - the CSDF filter projects every
  response to the granted data clusters before serialization; unknown
  scopes grant nothing (fail-closed).
- **One code path, every market** - the runtime
  [`Profile`](https://github.com/OpenFin8/openfin8-core/blob/main/src/profile.rs)
  carries every jurisdiction identity (service type, scope vocabulary,
  schema, env prefix, standard version), so adding a market is a
  constructor and a thin binary - never a port, never a fork.
- **Signed, durable notifications** - HMAC-signed webhook delivery with
  retry backoff, so recipients reliably learn about consent and
  account events.
- **The audit and evidence trail** - every provider data call is
  audited (`data_call*` actions), feeding the per-market certification
  metrics.

## 2. Business Scope (What the Library Provides)

| # | Capability area | What the business gets |
|---|---|---|
| 1 | Consent ledger | Grant lifecycle (`ACTIVE → REVOKED/EXPIRED`), duration windows (`ONE_TIME`/`PERSISTENT`/`TIME_BOUND` + lookback), revocation records with reason + initiator |
| 2 | Recipient registry | The two-ledger recipient plane: per-provider OAuth client cache (DCR), PKCE verifier + delegated-token custody, connection lifecycle |
| 3 | CSDF minimization | Response projection by granted clusters, scope, product category, and requested fields |
| 4 | Provider catalog | The cross-env provider directory (auto-registered at env-init) recipients discover and connect to |
| 5 | Money-movement initiation | Recipient-initiated transfers/payments, consent-checked here, executed by the bank's Payments service - one executor |
| 6 | Notifications | Subscription registry, HMAC-signed webhook delivery with backoff, polling feed |
| 7 | Admin surfaces | RFW admin pages (overview, provider, recipients, consents, CSDF, settings + the policy studio) rendered in the platform dashboard |
| 8 | Consumers | Bus consumers for env-init (provider self-registration, per-env policy-set seed) and the authorization-server grant event (ledger seed) |
| 9 | Contract validation | The route contract + FAPI header validation (`x-fapi-interaction-id` echo) per standard profile |
| 10 | Events | 16 outbound event types per adapter family (`openfin8:{xx}:{resource}:{operation}`) |
| 11 | Audit & evidence | `data_call*` audit rows + the certification-metrics counters |
| 12 | Health | The health surface the fleet supervisor polls |

**What the kernel does NOT own (business boundary):**

| Out of scope | Owned by |
|---|---|
| Account/transaction data (the data plane) | The composing vertical (bank8:Ledger) - consumed via public APIs only |
| Money-movement execution | The composing vertical (bank8:Payments) |
| Statement bytes at rest | Sentin8:Storage (signed URLs for download) |
| Identity, sign-in, the OAuth ceremony hosting | Sentin8:Auth (the kernel drives it per profile) |
| The jurisdiction identity, registration data, published-spec policy | The adapter binaries (this crate is jurisdiction-agnostic by design) |

## 3. Business Capabilities

### 3.1 The consent lifecycle

| Capability | Business rule |
|---|---|
| Grant capture | Seeded from the authorization server's grant event at code issuance - the grant ledger reflects the ceremony the customer actually completed |
| Enforcement | Every provider route passes the consent middleware on top of row-level security; a route cannot bypass it structurally |
| Revocation | Immediate: a revoked consent stops serving on the next call; terminal - introspection can never revive one |
| Expiry | Duration-window driven, introspection-checked, never cached |

### 3.2 The two-ledger model

The provider's grant ledger and the recipient's consent registry live in
**separate tenants** - no cross-tenant SQL ever. Either institution may
run any technology; the only contract between them is standards-based
OAuth at the provider's authorization server.

### 3.3 Data minimization (CSDF)

Responses are projected to the granted clusters before serialization.
The cluster vocabulary is profile data - FDX clusters and UK OB
permission arrays are two vocabularies over the same filter engine -
so a new market's vocabulary needs no engine change.

### 3.4 The runtime Profile

One struct carries every identity string: service type
(`openfin8:ca`), env prefix (`OPENFIN8_CA_*`), scope vocabulary
(`openfin8:ca:*`), event family, schema, workload names, and the
served standard version. Adapters construct it; everything else
derives.

## 4. Business Events (Outbound)

Per-adapter family `openfin8:{xx}:*`: consent created/updated/revoked,
recipient consent started/refreshed/revoked, transfer created/
cancelled, payment created, recurring payment created, payee created,
PIP party created, payment network revoked, notification subscription
created/deleted, provider profile updated - consumed by recipient-side
reconciliation and the delivery worker.

## 5. Business Rules & Invariants

- No provider route bypasses the consent middleware.
- Revoked consents are terminal.
- No plaintext delegated tokens or webhook secrets in any column -
  secret references only, resolved at call time.
- Unknown scopes grant nothing (fail-closed minimization).
- No cross-tenant SQL between the provider and recipient planes.
- Every tenant-table write carries the tenant context in the same
  transaction (the platform's FORCE-RLS discipline).
- The OBVERT boundary: this crate never imports another vertical's
  database - public API + SDK contracts only.

## 6. Target-User Roles & Use Cases

| Role | Use case |
|---|---|
| Adapter engineer | Embeds the kernel, supplies a Profile + registration; the served surface is complete without writing service code |
| Compliance officer | Reads the consent ledger, revocation records, audit trail, and certification metrics |
| Recipient product owner | Relies on the connection registry and event feed for reconciliation UX |
| Bank operator | Enables a jurisdiction per environment (`sys.services`); version + capability via config, not deploys |

## 7. KPIs / Success Criteria

- **Time-to-new-market**: a new jurisdiction adapter = a Profile
  constructor + registration data (target: days, not quarters).
- **Zero-fork guarantee**: no implementation code differs between
  adapters (auditable: adapters carry only `main.rs` +
  `registration.rs` + `openapi.rs`).
- **Consent enforcement coverage**: 100% of provider routes behind
  the middleware (structural).
- **Certification readiness**: per-market evidence counters derived
  from audited traffic, current at query time.

## 8. Dependencies & Reliance on Other Services

| Dependency | What the kernel gets |
|---|---|
| Sentin8 platform SDK | The shell (boot, TLS, OpenAPI merge), Auth client, Bus, Shield policy engine, RFW admin framework |
| bank8:: SDK contracts | Ledger/Payments typed clients - the cross-vertical data plane (public APIs only) |
| Sentin8:Auth | The OAuth ceremonies; the grant-seed event |
| Sentin8:Bus | Event transport (producer outbox + SSE consumers) |
| Sentin8:Secrets | Token/secret reference resolution |
| Sentin8:Sys | Environment enablement events (env-init) |
| Sentin8:Storage | Statement objects (signed-URL download) |

## 9. Acceptance Criteria

- 91 unit tests green (consent state machines, CSDF filter per
  vocabulary, route contract, revocation model).
- The US and Canadian adapters build against this crate unchanged -
  byte-for-byte the same implementation, different data.
- Every identity string in the served surface derives from the
  Profile (verified during the OBVERT rename: no stragglers).
- The published-spec policy is enforced per adapter by CI drift
  guards.

## 10. Open Business Questions

- Should the certification-metrics rollup grow a daily snapshot table
  + evidence admin page (deferred at scale in the original design)?
- Is a response cache for recipient polling warranted before a
  customer hits FDX poll-guidance degradation (explicit trigger in
  the design)?
- Should the staged-intent expiry sweep (OB's 24h intent retention)
  generalize into the kernel when the UK adapter lands, or stay
  adapter-owned until a second standard needs it?
- Should the recipient plane grow a dashboard-grade connection
  health surface, or does the event feed suffice?

---

**Architecture & Design:** openfin8:core and its underlying concepts were
designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).