// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use blocks::{
    tipset::tipset_json::{self, TipsetJson, TipsetJsonRef},
    Tipset,
};
use clock::ChainEpoch;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::sync::Arc;
use std::time::SystemTime;

/// Current state of the ChainSyncer using the BlockSync protocol.
#[derive(PartialEq, Debug, Clone, Copy, Deserialize)]
pub enum SyncStage {
    /// Syncing headers from the heaviest tipset to genesis.
    #[serde(rename(deserialize = "header sync"))]
    Headers,
    /// Persisting headers on chain from heaviest to genesis.
    #[serde(rename(deserialize = "persisting headers"))]
    PersistHeaders,
    /// Syncing messages and performing state transitions.
    #[serde(rename(deserialize = "message sync"))]
    Messages,
    /// ChainSync completed and is following chain.
    #[serde(rename(deserialize = "complete"))]
    Complete,
    /// Error has occured while syncing.
    #[serde(rename(deserialize = "error"))]
    Error,
}

impl Default for SyncStage {
    fn default() -> Self {
        Self::Headers
    }
}

impl fmt::Display for SyncStage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SyncStage::Headers => write!(f, "header sync"),
            SyncStage::PersistHeaders => write!(f, "persisting headers"),
            SyncStage::Messages => write!(f, "message sync"),
            SyncStage::Complete => write!(f, "complete"),
            SyncStage::Error => write!(f, "error"),
        }
    }
}

impl Serialize for SyncStage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

/// State of a given sync.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SyncState {
    pub base: Option<Arc<Tipset>>,
    pub target: Option<Arc<Tipset>>,

    stage: SyncStage,
    pub epoch: ChainEpoch,

    pub start: Option<SystemTime>,
    pub end: Option<SystemTime>,
    pub message: String,
}

impl SyncState {
    /// Initializes the syncing state with base and target tipsets and sets start time.
    pub fn init(&mut self, base: Arc<Tipset>, target: Arc<Tipset>) {
        *self = Self {
            target: Some(target),
            base: Some(base),
            start: Some(SystemTime::now()),
            ..Default::default()
        }
    }

    pub fn stage(&self) -> SyncStage {
        self.stage
    }

    /// Sets the sync stage for the syncing state. If setting to complete, sets end timer to now.
    pub fn set_stage(&mut self, stage: SyncStage) {
        if let SyncStage::Complete = stage {
            self.end = Some(SystemTime::now());
        }
        self.stage = stage;
    }

    /// Sets epoch of the sync.
    pub fn set_epoch(&mut self, epoch: ChainEpoch) {
        self.epoch = epoch;
    }

    /// Sets error for the sync.
    pub fn error(&mut self, err: String) {
        self.message = err;
        self.stage = SyncStage::Error;
        self.end = Some(SystemTime::now());
    }
}

impl Serialize for SyncState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct SyncStateJson<'a> {
            base: Option<TipsetJsonRef<'a>>,
            target: Option<TipsetJsonRef<'a>>,

            stage: SyncStage,
            epoch: ChainEpoch,

            start: &'a Option<SystemTime>,
            end: &'a Option<SystemTime>,
            message: &'a str,
        }

        SyncStateJson {
            base: self.base.as_ref().map(|ts| TipsetJsonRef(ts.as_ref())),
            target: self.target.as_ref().map(|ts| TipsetJsonRef(ts.as_ref())),
            stage: self.stage,
            epoch: self.epoch,
            start: &self.start,
            end: &self.end,
            message: &self.message,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SyncState {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct SyncStateJson {
            base: Option<TipsetJson>,
            target: Option<TipsetJson>,
            stage: SyncStage,
            epoch: ChainEpoch,
            start: Option<SystemTime>,
            end: Option<SystemTime>,
            message: String,
        }
        let SyncStateJson {
            base,
            target,
            stage,
            epoch,
            start,
            end,
            message,
        } = Deserialize::deserialize(deserializer)?;
        Ok(Self {
            base: base.map(|b| Arc::new(b.0)),
            target: target.map(|b| Arc::new(b.0)),
            stage,
            epoch,
            start,
            end,
            message,
        })
    }
}
