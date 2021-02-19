use sentry::{Breadcrumb, Level};
use serde_json::value::Value;
use std::collections::btree_map::BTreeMap;

/// Represents an step done previous to an event.
/// It gets internally converted to a sentry breadcrumb.
///
/// ```rust
/// let step = Step { 
///   ty: StepType::Error,
///   title: "Bad request".into(),
///   message: "Mike made a bad request".into(),
///   level: LogLevel::Info,
///   body: None,
/// };
///
/// logger::track_step(step);
/// ```
pub struct Step {
    pub ty: StepType,
    pub title: String,
    pub message: String,
    pub level: Level,
    pub body: Option<BTreeMap<String, Value>>,
}

impl Into<Breadcrumb> for Step {
    fn into(self) -> Breadcrumb {
        Breadcrumb {
            category: Some(self.title),
            message: Some(self.message),
            level: self.level,
            ty: match self.ty {
                StepType::Default => "default",
                StepType::Error => "error",
                StepType::Debug => "debug",
                StepType::Info => "info",
                StepType::Http => "http",
            }
            .into(),
            data: if let Some(data) = self.body {
                data
            } else {
                Default::default()
            },
            ..Default::default()
        }
    }
}

/// It helps the step to be better represented visually in sentry.
///
/// ```rust
/// let step = Step { 
///   ty: StepType::Error,
///   ...
/// };
///
/// logger::track_step(step);
/// ```
pub enum StepType {
    Default,
    Error,
    Debug,
    Info,
    Http,
}
