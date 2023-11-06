// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc 3.19.4
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `proto_message.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:ProtoUuid)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ProtoUuid {
    // message fields
    // @@protoc_insertion_point(field:ProtoUuid.value)
    pub value: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:ProtoUuid.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ProtoUuid {
    fn default() -> &'a ProtoUuid {
        <ProtoUuid as ::protobuf::Message>::default_instance()
    }
}

impl ProtoUuid {
    pub fn new() -> ProtoUuid {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "value",
            |m: &ProtoUuid| { &m.value },
            |m: &mut ProtoUuid| { &mut m.value },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ProtoUuid>(
            "ProtoUuid",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ProtoUuid {
    const NAME: &'static str = "ProtoUuid";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.value = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.value.is_empty() {
            os.write_bytes(1, &self.value)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ProtoUuid {
        ProtoUuid::new()
    }

    fn clear(&mut self) {
        self.value.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ProtoUuid {
        static instance: ProtoUuid = ProtoUuid {
            value: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ProtoUuid {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ProtoUuid").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ProtoUuid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProtoUuid {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:ProtoGroup)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ProtoGroup {
    // message fields
    // @@protoc_insertion_point(field:ProtoGroup.id)
    pub id: ::protobuf::MessageField<ProtoUuid>,
    // @@protoc_insertion_point(field:ProtoGroup.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:ProtoGroup.members)
    pub members: ::std::vec::Vec<ProtoUuid>,
    // special fields
    // @@protoc_insertion_point(special_field:ProtoGroup.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ProtoGroup {
    fn default() -> &'a ProtoGroup {
        <ProtoGroup as ::protobuf::Message>::default_instance()
    }
}

impl ProtoGroup {
    pub fn new() -> ProtoGroup {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ProtoUuid>(
            "id",
            |m: &ProtoGroup| { &m.id },
            |m: &mut ProtoGroup| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &ProtoGroup| { &m.name },
            |m: &mut ProtoGroup| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "members",
            |m: &ProtoGroup| { &m.members },
            |m: &mut ProtoGroup| { &mut m.members },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ProtoGroup>(
            "ProtoGroup",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ProtoGroup {
    const NAME: &'static str = "ProtoGroup";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.id)?;
                },
                18 => {
                    self.name = is.read_string()?;
                },
                26 => {
                    self.members.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.id.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        for v in &self.members {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ProtoGroup {
        ProtoGroup::new()
    }

    fn clear(&mut self) {
        self.id.clear();
        self.name.clear();
        self.members.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ProtoGroup {
        static instance: ProtoGroup = ProtoGroup {
            id: ::protobuf::MessageField::none(),
            name: ::std::string::String::new(),
            members: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ProtoGroup {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ProtoGroup").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ProtoGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProtoGroup {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:ProtoMessage)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ProtoMessage {
    // message fields
    // @@protoc_insertion_point(field:ProtoMessage.content)
    pub content: ::std::vec::Vec<u8>,
    // message oneof groups
    pub recipient: ::std::option::Option<proto_message::Recipient>,
    // special fields
    // @@protoc_insertion_point(special_field:ProtoMessage.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ProtoMessage {
    fn default() -> &'a ProtoMessage {
        <ProtoMessage as ::protobuf::Message>::default_instance()
    }
}

impl ProtoMessage {
    pub fn new() -> ProtoMessage {
        ::std::default::Default::default()
    }

    // .ProtoUuid user = 1;

    pub fn user(&self) -> &ProtoUuid {
        match self.recipient {
            ::std::option::Option::Some(proto_message::Recipient::User(ref v)) => v,
            _ => <ProtoUuid as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_user(&mut self) {
        self.recipient = ::std::option::Option::None;
    }

    pub fn has_user(&self) -> bool {
        match self.recipient {
            ::std::option::Option::Some(proto_message::Recipient::User(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_user(&mut self, v: ProtoUuid) {
        self.recipient = ::std::option::Option::Some(proto_message::Recipient::User(v))
    }

    // Mutable pointer to the field.
    pub fn mut_user(&mut self) -> &mut ProtoUuid {
        if let ::std::option::Option::Some(proto_message::Recipient::User(_)) = self.recipient {
        } else {
            self.recipient = ::std::option::Option::Some(proto_message::Recipient::User(ProtoUuid::new()));
        }
        match self.recipient {
            ::std::option::Option::Some(proto_message::Recipient::User(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_user(&mut self) -> ProtoUuid {
        if self.has_user() {
            match self.recipient.take() {
                ::std::option::Option::Some(proto_message::Recipient::User(v)) => v,
                _ => panic!(),
            }
        } else {
            ProtoUuid::new()
        }
    }

    // .ProtoGroup group = 2;

    pub fn group(&self) -> &ProtoGroup {
        match self.recipient {
            ::std::option::Option::Some(proto_message::Recipient::Group(ref v)) => v,
            _ => <ProtoGroup as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_group(&mut self) {
        self.recipient = ::std::option::Option::None;
    }

    pub fn has_group(&self) -> bool {
        match self.recipient {
            ::std::option::Option::Some(proto_message::Recipient::Group(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ProtoGroup) {
        self.recipient = ::std::option::Option::Some(proto_message::Recipient::Group(v))
    }

    // Mutable pointer to the field.
    pub fn mut_group(&mut self) -> &mut ProtoGroup {
        if let ::std::option::Option::Some(proto_message::Recipient::Group(_)) = self.recipient {
        } else {
            self.recipient = ::std::option::Option::Some(proto_message::Recipient::Group(ProtoGroup::new()));
        }
        match self.recipient {
            ::std::option::Option::Some(proto_message::Recipient::Group(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_group(&mut self) -> ProtoGroup {
        if self.has_group() {
            match self.recipient.take() {
                ::std::option::Option::Some(proto_message::Recipient::Group(v)) => v,
                _ => panic!(),
            }
        } else {
            ProtoGroup::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, ProtoUuid>(
            "user",
            ProtoMessage::has_user,
            ProtoMessage::user,
            ProtoMessage::mut_user,
            ProtoMessage::set_user,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, ProtoGroup>(
            "group",
            ProtoMessage::has_group,
            ProtoMessage::group,
            ProtoMessage::mut_group,
            ProtoMessage::set_group,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "content",
            |m: &ProtoMessage| { &m.content },
            |m: &mut ProtoMessage| { &mut m.content },
        ));
        oneofs.push(proto_message::Recipient::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ProtoMessage>(
            "ProtoMessage",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ProtoMessage {
    const NAME: &'static str = "ProtoMessage";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.recipient = ::std::option::Option::Some(proto_message::Recipient::User(is.read_message()?));
                },
                18 => {
                    self.recipient = ::std::option::Option::Some(proto_message::Recipient::Group(is.read_message()?));
                },
                26 => {
                    self.content = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.content);
        }
        if let ::std::option::Option::Some(ref v) = self.recipient {
            match v {
                &proto_message::Recipient::User(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &proto_message::Recipient::Group(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.content.is_empty() {
            os.write_bytes(3, &self.content)?;
        }
        if let ::std::option::Option::Some(ref v) = self.recipient {
            match v {
                &proto_message::Recipient::User(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &proto_message::Recipient::Group(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
            };
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ProtoMessage {
        ProtoMessage::new()
    }

    fn clear(&mut self) {
        self.recipient = ::std::option::Option::None;
        self.recipient = ::std::option::Option::None;
        self.content.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ProtoMessage {
        static instance: ProtoMessage = ProtoMessage {
            content: ::std::vec::Vec::new(),
            recipient: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ProtoMessage {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ProtoMessage").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ProtoMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProtoMessage {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ProtoMessage`
pub mod proto_message {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:ProtoMessage.recipient)
    pub enum Recipient {
        // @@protoc_insertion_point(oneof_field:ProtoMessage.user)
        User(super::ProtoUuid),
        // @@protoc_insertion_point(oneof_field:ProtoMessage.group)
        Group(super::ProtoGroup),
    }

    impl ::protobuf::Oneof for Recipient {
    }

    impl ::protobuf::OneofFull for Recipient {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::ProtoMessage as ::protobuf::MessageFull>::descriptor().oneof_by_name("recipient").unwrap()).clone()
        }
    }

    impl Recipient {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Recipient>("recipient")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13proto_message.proto\"!\n\tProtoUuid\x12\x14\n\x05value\x18\x01\x20\
    \x01(\x0cR\x05value\"b\n\nProtoGroup\x12\x1a\n\x02id\x18\x01\x20\x01(\
    \x0b2\n.ProtoUuidR\x02id\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\
    \x12$\n\x07members\x18\x03\x20\x03(\x0b2\n.ProtoUuidR\x07members\"|\n\
    \x0cProtoMessage\x12\x20\n\x04user\x18\x01\x20\x01(\x0b2\n.ProtoUuidH\0R\
    \x04user\x12#\n\x05group\x18\x02\x20\x01(\x0b2\x0b.ProtoGroupH\0R\x05gro\
    up\x12\x18\n\x07content\x18\x03\x20\x01(\x0cR\x07contentB\x0b\n\trecipie\
    ntb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(ProtoUuid::generated_message_descriptor_data());
            messages.push(ProtoGroup::generated_message_descriptor_data());
            messages.push(ProtoMessage::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
