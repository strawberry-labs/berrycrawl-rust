pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentConfigDto {
    /// Model routing mode. "default" uses the fast default model; "smart" uses the higher-latency reasoning model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<AgentConfigDtoMode>,
    /// Explicit AI model to use. Omit for the configured default, or set "smart" to use the configured smart model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl AgentConfigDto {
    pub fn builder() -> AgentConfigDtoBuilder {
        <AgentConfigDtoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentConfigDtoBuilder {
    mode: Option<AgentConfigDtoMode>,
    model: Option<String>,
}

impl AgentConfigDtoBuilder {
    pub fn mode(mut self, value: AgentConfigDtoMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentConfigDto`].
    pub fn build(self) -> Result<AgentConfigDto, BuildError> {
        Ok(AgentConfigDto {
            mode: self.mode,
            model: self.model,
        })
    }
}
