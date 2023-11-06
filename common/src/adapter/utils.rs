use axum::extract::ws::Message;
use uuid::Uuid;

use crate::domain::{
    models::message::Message as MessageDomain, 
    types::{error::ErrorMsg, recipient::Recipient, id::Id, group::Group}
};
use super::protos_schemas::proto_message::{
    ProtoMessage, 
    proto_message::Recipient as ProtoRecipient, ProtoUuid, ProtoGroup
};


impl TryFrom<MessageDomain> for Message {
    type Error = String;

    fn try_from(value: MessageDomain) -> Result<Self, Self::Error> {
        // serialize value
        todo!()
    }
}

impl TryFrom<Message> for MessageDomain {
    type Error = String;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<ProtoUuid> for Id {
    type Error = ErrorMsg;

    fn try_from(proto_uuid: ProtoUuid) -> Result<Self, Self::Error> {
        let bytes = proto_uuid.value;
        let uuid = Uuid::from_slice(&bytes).map_err(|_| ErrorMsg("Error converting bytes to uuid".to_string()))?;
        match uuid.try_into() {
            Ok(id) => Ok(id),
            Err(_) => Err(ErrorMsg("Error converting uuid to id".to_string())),
        }
    }
}

impl TryFrom<ProtoGroup> for Group {
    type Error = ErrorMsg;

    fn try_from(proto_group: ProtoGroup) -> Result<Self, Self::Error> {
        let id = if let Some(res) = proto_group.id.0 {
            res
        } else {
            return Err(ErrorMsg("Error converting bytes to uuid".to_string()));
        };
        let id = *id;
        let id: Id = id.try_into()?;
        let members = proto_group.members
            .into_iter()
            .map(|member| member.try_into())
            .collect::<Result<Vec<Id>, ErrorMsg>>()?;

        Ok(Self { id, name: proto_group.name, members })
    }
}

impl TryFrom<ProtoRecipient> for Recipient {
    type Error = ErrorMsg;

    fn try_from(proto_recipient: ProtoRecipient) -> Result<Self, Self::Error> {
        match proto_recipient {
            ProtoRecipient::User(user_id) => {
                match user_id.try_into() {
                    Ok(id) => Ok(Recipient::User(id)),
                    Err(_) => Err(ErrorMsg("Error converting uuid to id".to_string())),
                }
            },
            ProtoRecipient::Group(group_id) => {
                match group_id.try_into() {
                    Ok(id) => Ok(Recipient::Group(id)),
                    Err(_) => Err(ErrorMsg("Error converting uuid to id".to_string())),
                }
            },
        }
    }
}
