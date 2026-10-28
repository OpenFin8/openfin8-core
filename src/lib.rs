// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Bank8:OpenCore
// SPDX-License-Identifier: Apache-2.0

//! # bank8-open-core
//!
//! The jurisdiction-agnostic open-banking kernel embedded by every
//! `bank8-open-*` adapter (`bank8-open-us`, `bank8-open-ca`,
//! `bank8-open-uk`, and the adapters to follow). See `DESIGN.md`
//! (OBARCH-1) for the full design; this crate ships the library
//! surface - the four adapter traits, the consent ledger, the
//! recipient registry, the CSDF engine, the notification worker,
//! the admin page framework, and the parameterized SQL.
//!
//! This is a LIBRARY crate: no process runs on it. Each adapter
//! binary embeds it, supplies its wire profile (the
//! [`OpenBankingProfile`] descriptor, the [`ConsentCeremony`]
//! implementation, the provider router, the consent screens), and
//! owns its boot, TLS, and database runtime.
//!
//! ## Module map (skeleton - modules land with OBARCH-1)
//!
//! - [`traits`] - the adapter contract (`OpenBankingProfile`,
//!   `ConsentCeremony`, `ProviderWire`, `ConsentScreens`).
//! - [`consent`] - the provider grant ledger + lifecycle.
//! - [`recipient`] - the recipient registry (two-ledger model).
//! - [`csdf`] - data-minimization engine over profile vocabularies.
//! - [`notify`] - HMAC-signed notification delivery worker.
//! - [`admin`] - shared RFW admin pages.
//! - [`audit`] - data-call audit discipline.
//! - [`contract`] - route-contract validation from profile data.
//! - [`sql`] - schema-parameterized migrations.
//! - [`server`] - the shared router composition + middleware.

pub mod traits;

/// Crate version (the SDK pin tag follows `make tag` - RS §7.1).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
