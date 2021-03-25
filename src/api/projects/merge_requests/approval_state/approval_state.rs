// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for approval state of a merge request.
#[derive(Debug, Builder)]
pub struct MergeRequestApprovalState<'a> {
    /// The project to query for approval state.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The internal ID of the merge request.
    merge_request: u64,
}

impl<'a> MergeRequestApprovalState<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> MergeRequestApprovalStateBuilder<'a> {
        MergeRequestApprovalStateBuilder::default()
    }
}

impl<'a> Endpoint for MergeRequestApprovalState<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/merge_requests/{}/approval_state",
            self.project, self.merge_request,
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::merge_requests::approval_state::MergeRequestApprovalState;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_merge_request_are_needed() {
        let err = MergeRequestApprovalState::builder().build().unwrap_err();
        assert_eq!(err, "`project` must be initialized");
    }

    #[test]
    fn project_is_needed() {
        let err = MergeRequestApprovalState::builder()
            .merge_request(1)
            .build()
            .unwrap_err();
        assert_eq!(err, "`project` must be initialized");
    }

    #[test]
    fn merge_request_is_needed() {
        let err = MergeRequestApprovalState::builder()
            .project(1)
            .build()
            .unwrap_err();
        assert_eq!(err, "`merge_request` must be initialized");
    }

    #[test]
    fn project_and_merge_request_are_sufficient() {
        MergeRequestApprovalState::builder()
            .project(1)
            .merge_request(1)
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/merge_requests/1/approval_state")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = MergeRequestApprovalState::builder()
            .project("simple/project")
            .merge_request(1)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}