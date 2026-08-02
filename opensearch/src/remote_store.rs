/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

// -----------------------------------------------
// This file is generated, Please do not edit it manually.
// Run the following in the root of the repo to regenerate:
//
// cargo make generate-api
// -----------------------------------------------

#![cfg(feature = "experimental-apis")]
#![doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#![allow(unused_imports)]
use crate::{
    client::OpenSearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        transport::Transport,
        Method,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::{borrow::Cow, time::Duration};
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Remote Store Restore API"]
pub enum RemoteStoreRestoreParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl RemoteStoreRestoreParts {
    #[doc = "Builds a relative URL path to the Remote Store Restore API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RemoteStoreRestoreParts::None => "/_remotestore/_restore".into(),
        }
    }
}
#[doc = "Builder for the [Remote Store Restore API](https://opensearch.org/docs/latest/tuning-your-cluster/availability-and-recovery/remote-store/index/#restoring-from-a-backup)\n\nRestores from remote store."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct RemoteStoreRestore<'a, 'b, B> {
    transport: &'a Transport,
    parts: RemoteStoreRestoreParts,
    body: Option<B>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b, B> RemoteStoreRestore<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [RemoteStoreRestore]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        RemoteStoreRestore {
            transport,
            parts: RemoteStoreRestoreParts::None,
            headers,
            body: None,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_completion: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> RemoteStoreRestore<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        RemoteStoreRestore {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            cluster_manager_timeout: self.cluster_manager_timeout,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            wait_for_completion: self.wait_for_completion,
        }
    }
    #[doc = "Explicit operation timeout for connection to cluster-manager node"]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Should this request wait until the operation has completed before returning"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Remote Store Restore API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion: self.wait_for_completion,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Remote Store Stats API"]
pub enum RemoteStoreStatsParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
    #[doc = "Index and ShardId"]
    IndexShardId(&'b str, &'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> RemoteStoreStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Remote Store Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RemoteStoreStatsParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_index.len());
                p.push_str("/_remotestore/stats/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
            RemoteStoreStatsParts::IndexShardId(index, shard_id) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_shard_id: Cow<str> =
                    percent_encode(shard_id.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(21usize + encoded_index.len() + encoded_shard_id.len());
                p.push_str("/_remotestore/stats/");
                p.push_str(encoded_index.as_ref());
                p.push('/');
                p.push_str(encoded_shard_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Remote Store Stats API](https://opensearch.org/docs/latest/tuning-your-cluster/availability-and-recovery/remote-store/remote-store-stats-api/)\n\nStats for remote store."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct RemoteStoreStats<'a, 'b> {
    transport: &'a Transport,
    parts: RemoteStoreStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> RemoteStoreStats<'a, 'b> {
    #[doc = "Creates a new instance of [RemoteStoreStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: RemoteStoreStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RemoteStoreStats {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Max time each individual stats request should take."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Remote Store Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for RemoteStore APIs"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
pub struct RemoteStore<'a> {
    transport: &'a Transport,
}
#[cfg(feature = "experimental-apis")]
impl<'a> RemoteStore<'a> {
    #[doc = "Creates a new instance of [RemoteStore]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Remote Store Restore API](https://opensearch.org/docs/latest/tuning-your-cluster/availability-and-recovery/remote-store/index/#restoring-from-a-backup)\n\nRestores from remote store."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn restore<'b>(&'a self) -> RemoteStoreRestore<'a, 'b, ()> {
        RemoteStoreRestore::new(self.transport())
    }
    #[doc = "[Remote Store Stats API](https://opensearch.org/docs/latest/tuning-your-cluster/availability-and-recovery/remote-store/remote-store-stats-api/)\n\nStats for remote store."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn stats<'b>(&'a self, parts: RemoteStoreStatsParts<'b>) -> RemoteStoreStats<'a, 'b> {
        RemoteStoreStats::new(self.transport(), parts)
    }
}
#[cfg(feature = "experimental-apis")]
impl OpenSearch {
    #[doc = "Creates a namespace client for RemoteStore APIs"]
    pub fn remote_store(&self) -> RemoteStore {
        RemoteStore::new(self.transport())
    }
}
