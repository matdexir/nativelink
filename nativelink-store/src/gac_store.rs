// Copyright 2025 The NativeLink Authors. All rights reserved.
//
// Licensed under the Functional Source License, Version 1.1, Apache 2.0 Future License (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    See LICENSE file for details
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use futures::stream::FuturesUnordered;
use futures::{StreamExt, TryStreamExt};
use nativelink_config::stores::ExperimentalGacSpec;
use nativelink_error::{Code, Error, ResultExt, make_err};
use nativelink_metric::{
    MetricFieldData, MetricKind, MetricPublishKnownKindData, MetricsComponent,
};
use nativelink_util::buf_channel::{DropCloserReadHalf, DropCloserWriteHalf};
use nativelink_util::health_utils::{HealthStatusIndicator, default_health_status_indicator};
use nativelink_util::store_trait::{
    RemoveItemCallback, StoreDriver, StoreKey, UploadSizeInfo,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::error;

const GAC_TWIRP_PATH: &str = "/twirp/github.actions.results.api.v1.CacheService";
const GAC_API_VERSION: &str = "nativelink-v1";
const KEY_PREFIX: &str = "nativelink";

#[derive(Serialize)]
struct TwirpCreateCacheEntryRequest {
    key: String,
    version: String,
}

#[derive(Deserialize)]
struct TwirpCreateCacheEntryResponse {
    ok: bool,
    #[serde(default)]
    signed_upload_url: String,
}

#[derive(Serialize)]
struct TwirpFinalizeCacheEntryUploadRequest {
    key: String,
    version: String,
    #[serde(rename = "sizeBytes")]
    size_bytes: String,
}

#[derive(Deserialize)]
#[expect(dead_code)]
struct TwirpFinalizeCacheEntryUploadResponse {
    ok: bool,
    #[serde(default)]
    entry_id: String,
}

#[derive(Serialize)]
struct TwirpGetCacheEntryDownloadURLRequest {
    key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_keys: Option<Vec<String>>,
    version: String,
}

#[derive(Deserialize)]
#[expect(dead_code)]
struct TwirpGetCacheEntryDownloadURLResponse {
    ok: bool,
    #[serde(default)]
    signed_download_url: String,
    #[serde(default)]
    matched_key: String,
}

/// Store implementation backed by GitHub Actions Cache V2 (Twirp) API.
///
/// Works only when running inside a GitHub Actions runner context,
/// requiring `ACTIONS_RESULTS_URL` and `ACTIONS_RUNTIME_TOKEN` environment
/// variables (or explicit config overrides).
#[derive(Debug)]
pub struct ExperimentalGacStore {
    client: Client,
    base_url: String,
    token: String,
    version: String,
}

impl ExperimentalGacStore {
    pub async fn new(spec: &ExperimentalGacSpec) -> Result<Arc<Self>, Error> {
        let base_url = spec
            .base_url
            .clone()
            .or_else(|| std::env::var("ACTIONS_RESULTS_URL").ok())
            .ok_or_else(|| {
                make_err!(
                    Code::FailedPrecondition,
                    "GAC store requires ACTIONS_RESULTS_URL env var or base_url config"
                )
            })?;

        let token = spec
            .token
            .clone()
            .or_else(|| std::env::var("ACTIONS_RUNTIME_TOKEN").ok())
            .ok_or_else(|| {
                make_err!(
                    Code::FailedPrecondition,
                    "GAC store requires ACTIONS_RUNTIME_TOKEN env var or token config"
                )
            })?;

        let client = Client::builder()
            .build()
            .map_err(|e| make_err!(Code::Internal, "Failed to create HTTP client: {e}"))?;

        Ok(Arc::new(Self {
            client,
            base_url,
            token,
            version: spec
                .cache_version
                .clone()
                .unwrap_or_else(|| GAC_API_VERSION.to_string()),
        }))
    }

    fn gac_key(key: &StoreKey<'_>) -> String {
        match key {
            StoreKey::Str(s) => format!("{KEY_PREFIX}/{s}"),
            StoreKey::Digest(d) => format!("{KEY_PREFIX}/{d}"),
        }
    }

    fn key_size(key: &StoreKey<'_>) -> u64 {
        match key {
            StoreKey::Digest(d) => d.size_bytes(),
            StoreKey::Str(_) => 0,
        }
    }

    fn twirp_url(&self, method: &str) -> String {
        format!(
            "{}{GAC_TWIRP_PATH}/{method}",
            self.base_url.trim_end_matches('/')
        )
    }

    async fn create_cache_entry(
        &self,
        key: &str,
    ) -> Result<String, Error> {
        let body = TwirpCreateCacheEntryRequest {
            key: key.to_string(),
            version: self.version.clone(),
        };

        let response = self
            .client
            .post(self.twirp_url("CreateCacheEntry"))
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&body)
            .send()
            .await
            .err_tip(|| "Failed to send CreateCacheEntry request")?;

        let status = response.status();
        let twirp_response: TwirpCreateCacheEntryResponse = response
            .json()
            .await
            .err_tip(|| "Failed to parse CreateCacheEntry response")?;

        if !twirp_response.ok {
            return Err(make_err!(
                Code::AlreadyExists,
                "Cache entry already exists for key: {key} (status: {status})"
            ));
        }

        Ok(twirp_response.signed_upload_url)
    }

    async fn finalize_cache_entry(
        &self,
        key: &str,
        size_bytes: u64,
    ) -> Result<(), Error> {
        let body = TwirpFinalizeCacheEntryUploadRequest {
            key: key.to_string(),
            version: self.version.clone(),
            size_bytes: size_bytes.to_string(),
        };

        let response = self
            .client
            .post(self.twirp_url("FinalizeCacheEntryUpload"))
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&body)
            .send()
            .await
            .err_tip(|| "Failed to send FinalizeCacheEntryUpload request")?;

        let twirp_response: TwirpFinalizeCacheEntryUploadResponse = response
            .json()
            .await
            .err_tip(|| "Failed to parse FinalizeCacheEntryUpload response")?;

        if !twirp_response.ok {
            return Err(make_err!(
                Code::Internal,
                "Failed to finalize cache entry for key: {key}"
            ));
        }

        Ok(())
    }

    async fn get_download_url(
        &self,
        key: &str,
    ) -> Result<Option<String>, Error> {
        let body = TwirpGetCacheEntryDownloadURLRequest {
            key: key.to_string(),
            restore_keys: Some(vec![format!("{KEY_PREFIX}/")]),
            version: self.version.clone(),
        };

        let response = self
            .client
            .post(self.twirp_url("GetCacheEntryDownloadURL"))
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&body)
            .send()
            .await
            .err_tip(|| "Failed to send GetCacheEntryDownloadURL request")?;

        let twirp_response: TwirpGetCacheEntryDownloadURLResponse = response
            .json()
            .await
            .err_tip(|| "Failed to parse GetCacheEntryDownloadURL response")?;

        if twirp_response.ok && !twirp_response.signed_download_url.is_empty() {
            Ok(Some(twirp_response.signed_download_url))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl StoreDriver for ExperimentalGacStore {
    async fn post_init(self: Arc<Self>) -> Result<(), Error> {
        // Env vars are already validated in `new`, but if the spec provided
        // explicit values they may be invalid at runtime. Do a quick health
        // check by trying to look up a non-existent key.
        let result = self.get_download_url("__nativelink_init_check__").await;
        // Any response (including 404) means the connection works.
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("GAC store post_init connectivity check failed: {e}");
                Err(e)
            }
        }
    }

    async fn has_with_results(
        self: Pin<&Self>,
        keys: &[StoreKey<'_>],
        results: &mut [Option<u64>],
    ) -> Result<(), Error> {
        keys.iter()
            .zip(results.iter_mut())
            .map(|(key, result)| async {
                let gac_key = Self::gac_key(key);
                match self.get_download_url(&gac_key).await {
                    Ok(Some(_)) => *result = Some(Self::key_size(key)),
                    Ok(None) => *result = None,
                    Err(e) => {
                        error!(
                            "GAC store has_with_results failed for key: {gac_key} - {e:?}"
                        );
                        *result = None;
                    }
                }
                Ok::<_, Error>(())
            })
            .collect::<FuturesUnordered<_>>()
            .try_for_each(|()| async { Ok(()) })
            .await
    }

    async fn update(
        self: Pin<&Self>,
        key: StoreKey<'_>,
        mut reader: DropCloserReadHalf,
        _size_info: UploadSizeInfo,
    ) -> Result<u64, Error> {
        let gac_key = Self::gac_key(&key);

        let data = reader
            .consume(None)
            .await
            .err_tip(|| format!("Failed to read data for key: {gac_key}"))?;

        let data_len = data.len() as u64;

        let signed_upload_url = self
            .create_cache_entry(&gac_key)
            .await
            .err_tip(|| format!("Failed to create cache entry for key: {gac_key}"))?;

        let upload_response = self
            .client
            .put(&signed_upload_url)
            .header("Content-Type", "application/octet-stream")
            .body(data.to_vec())
            .send()
            .await
            .err_tip(|| format!("Failed to upload data for key: {gac_key}"))?;

        let upload_status = upload_response.status();
        if !upload_status.is_success() {
            return Err(make_err!(
                Code::Internal,
                "Failed to upload data to signed URL for key: {gac_key} (status: {upload_status})"
            ));
        }

        self.finalize_cache_entry(&gac_key, data_len)
            .await
            .err_tip(|| format!("Failed to finalize cache entry for key: {gac_key}"))?;

        Ok(data_len)
    }

    async fn get_part(
        self: Pin<&Self>,
        key: StoreKey<'_>,
        writer: &mut DropCloserWriteHalf,
        offset: u64,
        length: Option<u64>,
    ) -> Result<(), Error> {
        let gac_key = Self::gac_key(&key);

        let download_url = self
            .get_download_url(&gac_key)
            .await
            .err_tip(|| format!("Failed to get download URL for key: {gac_key}"))?
            .ok_or_else(|| make_err!(Code::NotFound, "Cache entry not found for key: {gac_key}"))?;

        let mut request = self.client.get(&download_url);

        if let Some(len) = length {
            let range_end = offset.saturating_add(len).saturating_sub(1);
            request = request.header("Range", format!("bytes={offset}-{range_end}"));
        } else if offset > 0 {
            request = request.header("Range", format!("bytes={offset}-"));
        }

        let response = request
            .send()
            .await
            .err_tip(|| format!("Failed to download data for key: {gac_key}"))?;

        let status = response.status();
        if !status.is_success() {
            return Err(make_err!(
                Code::Internal,
                "Failed to download from signed URL for key: {gac_key} (status: {status})"
            ));
        }

        let mut stream = response.bytes_stream();
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.err_tip(|| {
                format!("Failed to read download stream for key: {gac_key}")
            })?;
            writer
                .send(chunk)
                .await
                .err_tip(|| format!("Failed to write chunk for key: {gac_key}"))?;
        }

        Ok(())
    }

    fn inner_store(&self, _key: Option<StoreKey>) -> &dyn StoreDriver {
        self
    }

    fn as_any<'a>(&'a self) -> &'a (dyn core::any::Any + Sync + Send + 'static) {
        self
    }

    fn as_any_arc(self: Arc<Self>) -> Arc<dyn core::any::Any + Sync + Send + 'static> {
        self
    }

    fn register_remove_callback(
        self: Arc<Self>,
        _callback: Arc<dyn RemoveItemCallback>,
    ) -> Result<(), Error> {
        Ok(())
    }
}

impl MetricsComponent for ExperimentalGacStore {
    fn publish(
        &self,
        _kind: MetricKind,
        _field_metadata: MetricFieldData,
    ) -> Result<MetricPublishKnownKindData, nativelink_metric::Error> {
        Ok(MetricPublishKnownKindData::Component)
    }
}

default_health_status_indicator!(ExperimentalGacStore);
