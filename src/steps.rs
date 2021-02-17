use sentry::{Breadcrumb, Level};
use serde_json::value::Value;
use std::collections::btree_map::BTreeMap;

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

pub enum StepType {
    Default,
    Error,
    Debug,
    Info,
    Http,
}
