use uuid::Uuid;

use crate::domain::types::sender_type::Sender;
use crate::domain::types::{
    error::ErrorMsg, 
    recipient::Recipient, 
    id::Id, 
    group::Group,
};
use crate::domain::protos_schemas::proto_package::{
    ProtoUuid, 
    ProtoGroup,
    ProtoPackage,
    ProtoSender,
    ProtoRecipient,
};
use crate::domain::protos_schemas::proto_package::{
    ProtoSender as ProtoSender2,
    proto_recipient::Recipient as ProtoRecipient2,
};


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

impl TryFrom<ProtoRecipient2> for Recipient {
    type Error = ErrorMsg;

    fn try_from(proto_recipient: ProtoRecipient2) -> Result<Self, Self::Error> {
        match proto_recipient {
            ProtoRecipient2::User(user_id) => {
                match user_id.try_into() {
                    Ok(id) => Ok(Recipient::User(id)),
                    Err(_) => Err(ErrorMsg("Error converting uuid to id".to_string())),
                }
            },
            ProtoRecipient2::Group(group_id) => {
                match group_id.try_into() {
                    Ok(id) => Ok(Recipient::Group(id)),
                    Err(_) => Err(ErrorMsg("Error converting uuid to id".to_string())),
                }
            },
        }
    }
}

impl From<Id> for ProtoUuid {
    fn from(id: Id) -> Self {
        let id: Uuid = id.into();
        let id = id.to_bytes_le();
        protobuf::Message::parse_from_bytes(&id).unwrap()
    }
}

impl From<Uuid> for ProtoUuid {
    fn from(uuid: Uuid) -> Self {
        let id = uuid.to_bytes_le();
        protobuf::Message::parse_from_bytes(&id).unwrap()
    }
}

impl From<Group> for ProtoGroup {
    fn from(group: Group) -> Self {
        let id: ProtoUuid = group.id.into();
        let members = group.members
            .into_iter()
            .map(|member| member.into())
            .collect::<Vec<ProtoUuid>>();

        Self { 
            id: protobuf::MessageField(Some(Box::new(id))),
            name: group.name, 
            members, 
            special_fields: protobuf::SpecialFields::default(),
        }
    }
}

impl From<Recipient> for ProtoRecipient2 {
    fn from(recipient: Recipient) -> Self {
        match recipient {
            Recipient::User(user_id) => {
                let user_id: ProtoUuid = user_id.into();
                Self::User(user_id)
            },
            Recipient::Group(group_id) => {
                let group_id: ProtoGroup = group_id.into();
                Self::Group(group_id)
            },
        }
    }
}

impl From<Sender> for ProtoSender2 {
    fn from(sender: Sender) -> Self {
        match sender {
            Sender::User(user_id) => {
                let user_id: ProtoUuid = user_id.into();
                ProtoSender2 {
                    user: protobuf::MessageField(Some(Box::new(user_id))),
                    special_fields: protobuf::SpecialFields::default(),
                }
            },
        }
    }
}

// impl TryFrom<MessageDomain> for ProtoMessage {
//     type Error = String;

//     fn try_from(message: MessageDomain) -> Result<Self, Self::Error> {
//         let id: ProtoUuid = message.id.into();
//         let sender: ProtoSender2 = message.sender.into();
//         let recipient: ProtoRecipient2 = message.recipient.into();
//         let content = message.content;
//         let timestamp = message.timestamp;
//         let special_fields = protobuf::SpecialFields::default();

//         let sender: ProtoSender = ProtoSender {
//             sender: Some(sender),
//             special_fields: protobuf::SpecialFields::default(),
//         };

//         let recipient: ProtoRecipient = ProtoRecipient {
//             recipient: Some(recipient),
//             special_fields: protobuf::SpecialFields::default(),
//         };

//         Ok(Self { 
//             id: protobuf::MessageField(Some(Box::new(id))), 
//             sender: protobuf::MessageField(Some(Box::new(sender))), 
//             recipient: protobuf::MessageField(Some(Box::new(recipient))), 
//             content, 
//             timestamp: timestamp as i64,
//             special_fields 
//         })
//     }
// }

// impl TryFrom<MessageDomain> for Message {
//     type Error = String;

//     fn try_from(message: MessageDomain) -> Result<Self, Self::Error> {
//         let message: ProtoMessage = message.try_into()?;
//         let proto_message: Vec<u8> = protobuf::Message::write_to_bytes(&message)
//             .map_err(|_| "Error encoding message".to_string())?;
//         Ok(Self::Binary(proto_message))
//     }
// }

