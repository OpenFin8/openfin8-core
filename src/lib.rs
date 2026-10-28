// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bank8:OpenCore
// SPDX-License-Identifier: Apache-2.0

//! # bank8-open-core
//!
//! The jurisdiction-agnostic open-banking kernel embedded by every
//! `bank8-open-*` adapter (`bank8-open-us`, `bank8-open-ca`,
//! `bank8-open-uk`, and the adapters to follow). See `DESIGN.md`
//! (OBARCH-1) for the full design.
//!
//! This is a LIBRARY crate: no process runs on it. Each adapter
//! binary embeds it, supplies its jurisdiction [`profile::Profile`]
//! at boot (`Profile::canada()` / `::united_states()`), its
//! registration data, and its published-spec policy - and the SAME
//! code serves both markets. The US and Canadian adapters differ in
//! data (profile + registration), not in code (OBARCH-4: no fork).
//!
//! ## Layout
//!
//! - [`profile`] - the runtime jurisdiction profile: every identity
//!   string (env prefix, family, schema, scopes, wire version)
//!   derives from one value.
//! - [`traits`] - the compile-time adapter contract for future wire
//!   families (OB, CDR, XS2A).
//! - [`config`] - the shared config, profile-scoped env reads.
//! - [`server`] - boot (`build_state` + `routes`) and the provider
//!   sub-router, version-parameterized mounts.
//! - [`handlers`] - the API surface (consents, recipients, provider
//!   data reads, money movement, admin).
//! - [`csdf`] - data minimization over profile scope vocabularies.
//! - [`event_consumer`] / [`event_notifications`] - the Bus
//!   env-init/grant-seed consumers and the HMAC delivery worker.
//! - [`consent_records`] / [`consent_rfw`] / [`money_movement`] /
//!   [`payment_networks`] / [`pip`] / [`recurring`] /
//!   [`statement_download`] / [`tax`] / [`operations`] - the FDX
//!   wire families.
//! - `registration`/`openapi` are adapter-owned: they carry
//!   per-jurisdiction identity and live in each adapter repo.

pub mod profile;
pub mod traits;

pub mod admin_ui;
pub mod auth;
pub mod common;
pub mod config;
pub mod consent_records;
pub mod consent_rfw;
pub mod csdf;
pub mod dtos;
pub mod errors;
pub mod event_consumer;
pub mod event_notifications;
pub mod flow;
pub mod handlers;
pub mod headers;
pub mod middleware;
pub mod money_movement;
pub mod notifications;
pub mod operations;
pub mod payment_networks;
pub mod pip;
pub mod policy;
pub mod recurring;
pub mod server;
pub mod statement_download;
pub mod tax;
pub mod tenant_snapshot;
pub mod tokens;
pub mod validation;

/// Crate version (the SDK pin tag follows `make tag` - RS §7.1).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
