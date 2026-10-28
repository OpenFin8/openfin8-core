// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bank8:OpenCore
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
    /// The service type: "bank8:open-us", "bank8:open-ca".
    pub service_type: &'static str,
    /// The binary + repo name: "bank8-open-us", "bank8-open-ca".
    pub binary: &'static str,
    /// The env-var prefix: "BANK8_OPEN_US", "BANK8_OPEN_CA".
    pub env_prefix: &'static str,
    /// The event family (Bus): "open-us", "open-ca" - the producer
    /// family of `bank8:{family}:{resource}:{operation}` events.
    pub family: &'static str,
    /// The Bus stream family default (`{env_prefix}_STREAM_FAMILY`
    /// override still wins).
    pub stream_family: &'static str,
    /// The DB schema this jurisdiction owns: "open_us", "open_ca".
    pub schema: &'static str,
    /// The Shield scope vocabulary prefix: "bank8:open-us",
    /// "bank8:open-ca" - `format!("{prefix}:accountbasic:read")` etc.
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
    /// ("fdx"-equivalent in the old service: "open-us"/"open-ca").
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
            service_type: "bank8:open-ca",
            binary: "bank8-open-ca",
            env_prefix: "BANK8_OPEN_CA",
            family: "open-ca",
            stream_family: "open-ca",
            schema: "open_ca",
            scope_prefix: "bank8:open-ca",
            fdx_version: env::var("BANK8_OPEN_CA_VERSION").unwrap_or_else(|_| "6.5".into()),
        }
    }

    /// Build the US profile (FDX - OBARCH-3).
    pub fn united_states() -> Self {
        Self {
            jurisdiction: "us",
            service_type: "bank8:open-us",
            binary: "bank8-open-us",
            env_prefix: "BANK8_OPEN_US",
            family: "open-us",
            stream_family: "open-us",
            schema: "open_us",
            scope_prefix: "bank8:open-us",
            fdx_version: env::var("BANK8_OPEN_US_VERSION").unwrap_or_else(|_| "6.5".into()),
        }
    }
}
