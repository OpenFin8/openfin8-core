// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the OpenFin8:Core
// SPDX-License-Identifier: Apache-2.0

//! The runtime jurisdiction profile (OBARCH-1 §2, OBARCH-4).
//!
//! Every identity string the shared implementation needs at runtime -
//! env-var prefix, event family, Shield targets, scope vocabulary, wire
//! prefix - derives from ONE `Profile` value supplied by the adapter at
//! boot. The US and CA adapters are the same code with different
//! `Profile` data; the UK adapter later implements the same contract
//! with a different wire.
//!
//! This is the *runtime* half of the adapter contract: the type-level
//! traits in `traits.rs` remain the compile-time shape for future
//! wire families (OB, CDR, XS2A). The FDX wire family is uniform
//! enough that data-only parameterization covers US + CA exactly -
//! one code path, zero forks.

use std::env;

/// The jurisdiction profile: everything identity-shaped the shared
/// kernel derives at runtime.
#[derive(Debug, Clone)]
pub struct Profile {
    /// ISO-ish jurisdiction key: "us", "ca" (future: "uk", "br", "eu", "au").
    pub jurisdiction: &'static str,
    /// The service type: "openfin8:us", "openfin8:ca".
    pub service_type: &'static str,
    /// The binary + repo name: "openfin8-us", "openfin8-ca".
    pub binary: &'static str,
    /// The env-var prefix: "OPENFIN8_US", "OPENFIN8_CA".
    pub env_prefix: &'static str,
    /// The event family (Bus): "openfin8-us", "openfin8-ca" - the producer
    /// family of `bank8:{family}:{resource}:{operation}` events.
    pub family: &'static str,
    /// The Bus stream family default (`{env_prefix}_STREAM_FAMILY`
    /// override still wins).
    pub stream_family: &'static str,
    /// The DB schema this jurisdiction owns: "openfin8_us", "openfin8_ca".
    pub schema: &'static str,
    /// The Shield scope vocabulary prefix: "openfin8:us",
    /// "openfin8:ca" - `format!("{prefix}:accountbasic:read")` etc.
    pub scope_prefix: &'static str,
    /// The FDX version this deployment serves ("6.5") - version-
    /// configurable per environment (OBARCH-2/3).
    pub fdx_version: String,
}

impl Profile {
    /// Read a profile-scoped env var: `{env_prefix}_{suffix}`.
    pub fn env(&self, suffix: &str) -> Option<String> {
        env::var(format!("{}_{}", self.env_prefix, suffix)).ok()
    }

    /// The FDX major segment ("6.5" -> "6") - route prefix + path shape.
    pub fn fdx_major(&self) -> String {
        self.fdx_version
            .split('.')
            .next()
            .unwrap_or("6")
            .to_string()
    }

    /// The provider route prefix: "/fdx/v6" for version 6.x.
    pub fn provider_prefix(&self) -> String {
        format!("/fdx/v{}", self.fdx_major())
    }

    /// A Shield evaluation target: `{service_type}:{resource}:{action}`.
    pub fn target(&self, resource_action: &str) -> String {
        format!("{}:{}", self.service_type, resource_action)
    }

    /// An OAuth scope string: `{scope_prefix}:{suffix}`.
    pub fn scope(&self, suffix: &str) -> String {
        format!("{}:{}", self.scope_prefix, suffix)
    }

    /// An event type: `bank8:{family}:{resource}:{operation}`.
    pub fn event(&self, resource_operation: &str) -> String {
        format!("bank8:{}:{}", self.family, resource_operation)
    }

    /// The outbox family literal for `insert_outbox_row_pool`.
    pub fn outbox_family(&self) -> &str {
        self.family
    }

    /// The workload short name for Bus/TLS workload identities
    /// ("fdx"-equivalent in the old service: "openfin8-us"/"openfin8-ca").
    pub fn workload(&self) -> &'static str {
        // The stream family IS the workload short name in this
        // platform's DCR/TLS registry (fdx used "fdx" for both).
        self.stream_family
    }

    /// The `sys.services` service flag (Sentin8:Sys enablement) and
    /// the auth consent-provider `feature` key.
    pub fn service_flag(&self) -> &'static str {
        self.stream_family
    }

    /// Build the Canadian profile (FDX/CDX - OBARCH-4).
    pub fn canada() -> Self {
        Self {
            jurisdiction: "ca",
            service_type: "openfin8:ca",
            binary: "openfin8-ca",
            env_prefix: "OPENFIN8_CA",
            family: "openfin8-ca",
            stream_family: "openfin8-ca",
            schema: "openfin8_ca",
            scope_prefix: "openfin8:ca",
            fdx_version: env::var("OPENFIN8_CA_VERSION").unwrap_or_else(|_| "6.5".into()),
        }
    }

    /// Build the US profile (FDX - OBARCH-3).
    pub fn united_states() -> Self {
        Self {
            jurisdiction: "us",
            service_type: "openfin8:us",
            binary: "openfin8-us",
            env_prefix: "OPENFIN8_US",
            family: "openfin8-us",
            stream_family: "openfin8-us",
            schema: "openfin8_us",
            scope_prefix: "openfin8:us",
            fdx_version: env::var("OPENFIN8_US_VERSION").unwrap_or_else(|_| "6.5".into()),
        }
    }

    /// Build the European profile (Berlin Group openFinance V2 -
    /// OBARCH-7). The XS2A wire is adapter-owned (a different wire
    /// family from FDX); this profile carries the jurisdiction
    /// identity strings the shared machinery needs: schema
    /// `openfin8_eu`, scopes `openfin8:eu:*`, event family
    /// `openfin8-eu`. The `fdx_version` field is unused on the XS2A
    /// path (the served openFinance version is the adapter's own
    /// `OPENFIN8_EU_VERSION`); it stays at the neutral default.
    pub fn european_union() -> Self {
        Self {
            jurisdiction: "eu",
            service_type: "openfin8:eu",
            binary: "openfin8-eu",
            env_prefix: "OPENFIN8_EU",
            family: "openfin8-eu",
            stream_family: "openfin8-eu",
            schema: "openfin8_eu",
            scope_prefix: "openfin8:eu",
            fdx_version: env::var("OPENFIN8_EU_VERSION").unwrap_or_else(|_| "2.4".into()),
        }
    }

    /// Build the UK profile (OBL Read/Write API - OBARCH-5). The OB
    /// wire is adapter-owned (the first non-FDX wire family): this
    /// profile carries the jurisdiction identity strings the shared
    /// machinery needs - schema `openfin8_uk`, scopes `openfin8:uk:*`,
    /// event family `openfin8-uk`. The `fdx_version` field carries the
    /// served OB version (the adapter's own `OPENFIN8_UK_VERSION`,
    /// default `3.1.11`; `4.0.1` configurable) - the OB path shape is
    /// `v{maj}.{min}`, unlike FDX's `v{major}` (UK DESIGN §2).
    pub fn united_kingdom() -> Self {
        Self {
            jurisdiction: "uk",
            service_type: "openfin8:uk",
            binary: "openfin8-uk",
            env_prefix: "OPENFIN8_UK",
            family: "openfin8-uk",
            stream_family: "openfin8-uk",
            schema: "openfin8_uk",
            scope_prefix: "openfin8:uk",
            fdx_version: env::var("OPENFIN8_UK_VERSION").unwrap_or_else(|_| "3.1.11".into()),
        }
    }
}
