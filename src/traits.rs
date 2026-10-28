// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bank8:OpenCore
// SPDX-License-Identifier: Apache-2.0

//! The adapter contract (OBARCH-1 §2): every country adapter
//! implements these four traits; core supplies the axum router, the
//! middleware chokepoints, the consumers, and the workers that call
//! through them.
//!
//! Skeleton - the full trait bodies, `CoreState`, and the
//! shared routers land with the OBARCH-1 implementation. The
//! signatures here are the design contract from DESIGN.md §2.

/// The wire-flavor descriptor - what an adapter IS.
#[allow(unused_variables)]
pub trait OpenBankingProfile: Send + Sync + 'static {
    /// e.g. "us", "ca", "uk" - key for config, admin pages, metrics.
    fn jurisdiction(&self) -> &'static str;
    /// The standard's own identity, e.g. "FDX 6.5", "OBIE 3.1.11".
    fn standard(&self) -> StandardDescriptor;
    /// Route prefix the provider surface mounts under,
    /// e.g. "/fdx/v6" or "/open-banking/v3.1.11".
    fn provider_prefix(&self) -> String;
    /// Version string announced on capability/discovery surfaces.
    fn announced_version(&self) -> String;
    /// Cluster vocabulary for this standard (CSDF input).
    fn data_clusters(&self) -> &'static [DataClusterDef];
    /// Scope→cluster mapping the ceremony uses.
    fn scope_to_clusters(&self, scope: &str) -> Vec<DataCluster>;
    /// Route-contract table for request validation + status-code
    /// semantics (consumed by core::contract).
    fn route_contract(&self) -> &'static [RouteDef];
    /// Envelope codec for this standard's response wrapping.
    fn envelope(&self) -> EnvelopeStyle;
    /// Spec publication policy - what docs/openapi.json may carry
    /// (OBARCH-2: US/CA must exclude the standard surface).
    fn spec_publication(&self) -> SpecPublicationPolicy;
}

/// The standard identity an adapter serves.
#[derive(Debug, Clone)]
pub struct StandardDescriptor {
    /// e.g. "FDX", "OBIE".
    pub name: &'static str,
    /// e.g. "6.5", "3.1.11".
    pub version: String,
    /// Whether the standard's own spec may be published (MIT,
    /// CC BY) or must stay customer-supplied (FDX license).
    pub license: StandardLicense,
    /// The resource families this deployment serves.
    pub families: &'static [&'static str],
}

/// How the standard's specification may be published.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardLicense {
    /// FDX (US/CA): licensed, distributed exclusively by FDX.
    CustomerSupplied,
    /// OBL (UK): MIT Open Licence - vendor with the notice.
    VendoredMitObl,
}

/// What the adapter's published `docs/openapi.json` may carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecPublicationPolicy {
    /// Published spec carries ONLY bank8-owned routes (`/v1/*`);
    /// the standard surface is never committed; the customer
    /// supplies the licensed spec under gitignored `specs/`.
    CustomerSupplied,
    /// The standard's paths MAY be published with the upstream
    /// copyright notice preserved (MIT-licensed specs).
    VendoredMitObl,
}

/// One data-cluster vocabulary entry (profile data, not an enum -
/// OBARCH-1 §4: UK permissions and FDX clusters are vocabularies
/// over the same filter engine).
#[derive(Debug, Clone)]
pub struct DataClusterDef {
    /// e.g. "ACCOUNTS", "ACCOUNTS_BASIC" (OB).
    pub name: &'static str,
    /// Human label for admin/consent screens.
    pub label: &'static str,
}

/// A granted cluster (validated against the profile vocabulary).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataCluster(pub String);

/// One route in the adapter's contract table: method, pattern,
/// body requirements. Replaces today's hardcoded `routes` array
/// in validation.rs; the UK extension adds a role segment.
#[derive(Debug, Clone)]
pub struct RouteDef {
    /// HTTP method ("GET", "POST", ...).
    pub method: &'static str,
    /// Pattern with `{placeholder}` segments, version-templated
    /// from the configured prefix at profile build time.
    pub pattern: &'static str,
    /// Whether a JSON body is required.
    pub requires_body: bool,
    /// Required top-level body fields.
    pub required_fields: &'static [&'static str],
}

/// The response envelope style the adapter's codec renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeStyle {
    /// FDX: flat JSON.
    Flat,
    /// UK OB: `{Data, Risk, Links, Meta}` wrapping.
    ObDataLinksMeta,
}

/// The OAuth ceremony the adapter's AS integration speaks
/// (OBARCH-1 §2). Implemented against `sentin8::auth::client`;
/// Auth keeps its own profile flags per jurisdiction (DPoP vs
/// hybrid+JWS vs CIBA).
///
/// Skeleton - the async trait bodies and `CeremonyCtx` land with
/// the OBARCH-1 implementation.
#[allow(unused_variables)]
pub trait ConsentCeremony: Send + Sync + 'static {
    /// Register/lookup the recipient's OAuth client at the
    /// provider's AS (RFC 7591 DCR; UK adds software-statement
    /// assertions).
    fn ensure_client(&self) -> ClientRegistration;
    /// Build the authorization redirect (PAR where the profile
    /// has it; UK hybrid carries intent-id binding in the request
    /// object).
    fn start_authorization(&self) -> AuthorizationRedirect;
    /// Exchange code for tokens (PKCE always; + DPoP proof for
    /// US/CA, + detached JWS where mandated).
    fn exchange_code(&self) -> DelegatedTokens;
    /// Refresh path (sender-constrained per profile).
    fn refresh(&self) -> DelegatedTokens;
}

/// Marker types for the ceremony skeleton - see DESIGN.md §2 for
/// the full shapes (they carry secret references, never plaintext
/// tokens).
pub struct ClientRegistration;
pub struct AuthorizationRedirect;
pub struct DelegatedTokens;

/// The wire handlers - the standard's resource surface. Core mounts
/// them behind the consent middleware.
pub trait ProviderWire: Send + Sync + 'static {
    /// Placeholder for the provider sub-router; the full signature
    /// takes `&CoreState` and returns `axum::Router<CoreState>`.
    fn provider_router_marker(&self) {}
}

/// Hosted consent screen parameters - the RFW page flavor
/// (locale-aware from day one: the Canadian profile requires
/// bilingual EN/FR wording).
pub trait ConsentScreens: Send + Sync + 'static {
    /// Scope display wording for the authorize screen.
    fn scope_labels(&self, scope: &str, locale: &str) -> String;
}