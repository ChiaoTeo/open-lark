use crate::message::MessageTrait;

pub trait LarkBot {
    /// 发送原始消息
    fn send_raw_message(
        &self,
        body: impl serde::Serialize + Send,
    ) -> impl Send;

    /// 发送消息结构体
    fn send_message(
        &self,
        message: impl MessageTrait + Send,
    ) -> impl Send;
}

pub mod custom_bot;
