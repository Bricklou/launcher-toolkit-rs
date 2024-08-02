use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct McVersionManifest {
    /// Minecraft launcher [`Arguments`] field.
    /// This field appear in 1.13 (17w43a) and replace the [`minecraftArguments`] field.
    arguments: Option<Arguments>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Arguments {
    game: Vec<Argument>,
    jvm: Vec<Argument>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Argument {
    /// A simple string argument.
    Simple(String),
    /// A conditional argument.
    /// This argument will be included if the condition is true.
    Conditional {
        /// The condition rules to check
        rules: Vec<ArgumentRule>,
        // The value of the argument
        // Simple string will be converted into vec<string>, because the value can be a string or a vec<string>
        #[serde(serialize_with = "crate::utils::serde::deserialize_string_or_seq_string")]
        value: Vec<String>,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub struct ArgumentRule {
    /// The action to perform.
    action: String,
    /// The feature/OS to check.
    #[serde(alias = "os")]
    features: HashMap<String, bool>,
}
