// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::fmt;

use derive_builder::Builder;

use crate::query_common::NameOrId;
use crate::query_prelude::*;

/// The type of a pipeline variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineVariableType {
    /// An environment variable.
    ///
    /// The value of the variable is available as the value of the named environment variable.
    EnvVar,
    /// A file variable.
    ///
    /// The value of the variable is available in a file given by the value of the named
    /// environment variable.
    File,
}

impl Default for PipelineVariableType {
    fn default() -> Self {
        PipelineVariableType::EnvVar
    }
}

impl PipelineVariableType {
    /// The variable type query parameter.
    fn as_str(self) -> &'static str {
        match self {
            PipelineVariableType::EnvVar => "env_var",
            PipelineVariableType::File => "file",
        }
    }
}

impl fmt::Display for PipelineVariableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A pipeline variable.
#[derive(Debug, Builder, Clone)]
pub struct PipelineVariable<'a> {
    /// The name of the pipeline variable.
    #[builder(setter(into))]
    pub key: Cow<'a, str>,
    /// The value of the pipeline variable.
    #[builder(setter(into))]
    pub value: Cow<'a, str>,
    /// The way the variable should be exposed to pipeline jobs.
    #[builder(default)]
    pub variable_type: PipelineVariableType,
}

impl<'a> PipelineVariable<'a> {
    /// Create a builder for the pipeline variable.
    pub fn builder() -> PipelineVariableBuilder<'a> {
        PipelineVariableBuilder::default()
    }
}

/// Create a new pipeline on a project.
#[derive(Debug, Builder)]
pub struct CreatePipeline<'a> {
    /// The project to create the pipeline within.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// The ref to create the pipeline for.
    #[builder(setter(into))]
    ref_: Cow<'a, str>,

    /// Search for users with a given custom attribute set.
    #[builder(setter(name = "_variables"), default, private)]
    variables: Vec<PipelineVariable<'a>>,
}

impl<'a> CreatePipeline<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreatePipelineBuilder<'a> {
        CreatePipelineBuilder::default()
    }
}

impl<'a> CreatePipelineBuilder<'a> {
    /// Add a variable.
    pub fn variable(&mut self, variable: PipelineVariable<'a>) -> &mut Self {
        self.variables
            .get_or_insert_with(Default::default)
            .push(variable);
        self
    }

    /// Add multiple variables.
    pub fn variables<I, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = V>,
        V: Into<PipelineVariable<'a>>,
    {
        self.variables
            .get_or_insert_with(Default::default)
            .extend(iter.map(Into::into));
        self
    }
}

impl<'a, T> SingleQuery<T> for CreatePipeline<'a>
where
    T: DeserializeOwned,
{
    type FormData = ();

    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> String {
        format!("projects/{}/pipeline", self.project)
    }

    fn add_parameters(&self, mut pairs: Pairs) {
        pairs.append_pair("ref", &self.ref_);

        self.variables.iter().for_each(|variable| {
            pairs.extend_pairs(&[
                ("variables[][key]", variable.key.as_ref()),
                ("variables[][value]", variable.value.as_ref()),
                (
                    "variables[][variable_type]",
                    variable.variable_type.as_str(),
                ),
            ]);
        });
    }

    fn form_data(&self) {}
}

impl<'a, T> Query<T> for CreatePipeline<'a>
where
    T: DeserializeOwned,
{
    fn query(&self, client: &Gitlab) -> Result<T, GitlabError> {
        self.single_query(client)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::pipelines::CreatePipeline;

    #[test]
    fn project_and_ref_are_needed() {
        let err = CreatePipeline::builder().build().unwrap_err();
        assert_eq!(err, "`project` must be initialized");
    }

    #[test]
    fn project_is_needed() {
        let err = CreatePipeline::builder()
            .ref_("testref")
            .build()
            .unwrap_err();
        assert_eq!(err, "`project` must be initialized");
    }

    #[test]
    fn ref_is_needed() {
        let err = CreatePipeline::builder().project(1).build().unwrap_err();
        assert_eq!(err, "`ref_` must be initialized");
    }

    #[test]
    fn project_and_ref_are_sufficient() {
        CreatePipeline::builder()
            .project(1)
            .ref_("testref")
            .build()
            .unwrap();
    }
}