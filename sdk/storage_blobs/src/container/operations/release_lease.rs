use crate::prelude::*;
use azure_core::Method;
use azure_core::{headers::*, prelude::*, RequestId};
use time::OffsetDateTime;

operation! {
    ReleaseLease,
    client: ContainerLeaseClient,
    ?if_modified_since: IfModifiedSinceCondition
}

impl ReleaseLeaseBuilder {
    pub fn into_future(mut self) -> ReleaseLease {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("restype", "container");
            url.query_pairs_mut().append_pair("comp", "lease");

            let mut headers = Headers::new();
            headers.insert(LEASE_ACTION, "release");
            headers.add(self.client.lease_id());
            headers.add(self.if_modified_since);

            let mut request = self
                .client
                .finalize_request(url, Method::Put, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            ReleaseLeaseResponse::from_headers(response.headers())
        })
    }
}

azure_storage::response_from_headers!(ReleaseLeaseResponse ,
    etag_from_headers => etag: String,
    last_modified_from_headers => last_modified: OffsetDateTime,
    request_id_from_headers => request_id: RequestId,
    date_from_headers => date: OffsetDateTime
);
