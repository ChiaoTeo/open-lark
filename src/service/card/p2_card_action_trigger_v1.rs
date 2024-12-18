use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::event::dispatcher::EventHandler;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct P2CardActionTriggerV1 {
    pub schema: String,
    pub header: Header,
    pub event: Event,
}

pub struct P2CardActionTriggerV1ProcessorImpl<F>
where
    F: Fn(P2CardActionTriggerV1) + 'static,
{
    f: F,
}

impl<F> P2CardActionTriggerV1ProcessorImpl<F>
where
    F: Fn(P2CardActionTriggerV1) + 'static,
{
    pub fn new(f: F) -> Self {
        P2CardActionTriggerV1ProcessorImpl { f }
    }
}

impl<F> EventHandler for P2CardActionTriggerV1ProcessorImpl<F>
where
    F: Fn(P2CardActionTriggerV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let message: P2CardActionTriggerV1 = serde_json::from_slice(payload)?;
        (self.f)(message);
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    #[serde(rename = "event_id")]
    pub event_id: String,
    pub token: String,
    #[serde(rename = "create_time")]
    pub create_time: String,
    #[serde(rename = "event_type")]
    pub event_type: String,
    #[serde(rename = "tenant_key")]
    pub tenant_key: String,
    #[serde(rename = "app_id")]
    pub app_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub operator: Operator,
    pub token: String,
    pub action: Action,
    pub host: String,
    pub context: Context,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator {
    #[serde(rename = "tenant_key")]
    pub tenant_key: String,
    #[serde(rename = "open_id")]
    pub open_id: String,
    #[serde(rename = "union_id")]
    pub union_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum Action {
    #[serde(rename = "interactive_container")]
    InteractiveContainer(InteractiveContainer),
    #[serde(rename = "button")]
    Button(ButtonAction),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonAction {
    pub timezone: String,
    #[serde(rename = "form_value")]
    pub form_value: Option<HashMap<String, String>>,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct InteractiveContainer {
    pub value: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    #[serde(rename = "open_message_id")]
    pub open_message_id: String,
    #[serde(rename = "open_chat_id")]
    pub open_chat_id: String,
}
