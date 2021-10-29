// This file is generated by rust-protobuf 2.25.2. Do not edit
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
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `controlclient.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct DeleteCacheRequest {
    // message fields
    pub cache_name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DeleteCacheRequest {
    fn default() -> &'a DeleteCacheRequest {
        <DeleteCacheRequest as ::protobuf::Message>::default_instance()
    }
}

impl DeleteCacheRequest {
    pub fn new() -> DeleteCacheRequest {
        ::std::default::Default::default()
    }

    // string cache_name = 1;


    pub fn get_cache_name(&self) -> &str {
        &self.cache_name
    }
    pub fn clear_cache_name(&mut self) {
        self.cache_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_cache_name(&mut self, v: ::std::string::String) {
        self.cache_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cache_name(&mut self) -> &mut ::std::string::String {
        &mut self.cache_name
    }

    // Take field
    pub fn take_cache_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cache_name, ::std::string::String::new())
    }
}

impl ::protobuf::Message for DeleteCacheRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cache_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.cache_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.cache_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.cache_name.is_empty() {
            os.write_string(1, &self.cache_name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> DeleteCacheRequest {
        DeleteCacheRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cache_name",
                |m: &DeleteCacheRequest| { &m.cache_name },
                |m: &mut DeleteCacheRequest| { &mut m.cache_name },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DeleteCacheRequest>(
                "DeleteCacheRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DeleteCacheRequest {
        static instance: ::protobuf::rt::LazyV2<DeleteCacheRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DeleteCacheRequest::new)
    }
}

impl ::protobuf::Clear for DeleteCacheRequest {
    fn clear(&mut self) {
        self.cache_name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteCacheRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteCacheRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteCacheResponse {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DeleteCacheResponse {
    fn default() -> &'a DeleteCacheResponse {
        <DeleteCacheResponse as ::protobuf::Message>::default_instance()
    }
}

impl DeleteCacheResponse {
    pub fn new() -> DeleteCacheResponse {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for DeleteCacheResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> DeleteCacheResponse {
        DeleteCacheResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DeleteCacheResponse>(
                "DeleteCacheResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DeleteCacheResponse {
        static instance: ::protobuf::rt::LazyV2<DeleteCacheResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DeleteCacheResponse::new)
    }
}

impl ::protobuf::Clear for DeleteCacheResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteCacheResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteCacheResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateCacheRequest {
    // message fields
    pub cache_name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CreateCacheRequest {
    fn default() -> &'a CreateCacheRequest {
        <CreateCacheRequest as ::protobuf::Message>::default_instance()
    }
}

impl CreateCacheRequest {
    pub fn new() -> CreateCacheRequest {
        ::std::default::Default::default()
    }

    // string cache_name = 1;


    pub fn get_cache_name(&self) -> &str {
        &self.cache_name
    }
    pub fn clear_cache_name(&mut self) {
        self.cache_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_cache_name(&mut self, v: ::std::string::String) {
        self.cache_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cache_name(&mut self) -> &mut ::std::string::String {
        &mut self.cache_name
    }

    // Take field
    pub fn take_cache_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cache_name, ::std::string::String::new())
    }
}

impl ::protobuf::Message for CreateCacheRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cache_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.cache_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.cache_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.cache_name.is_empty() {
            os.write_string(1, &self.cache_name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CreateCacheRequest {
        CreateCacheRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cache_name",
                |m: &CreateCacheRequest| { &m.cache_name },
                |m: &mut CreateCacheRequest| { &mut m.cache_name },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CreateCacheRequest>(
                "CreateCacheRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CreateCacheRequest {
        static instance: ::protobuf::rt::LazyV2<CreateCacheRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CreateCacheRequest::new)
    }
}

impl ::protobuf::Clear for CreateCacheRequest {
    fn clear(&mut self) {
        self.cache_name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateCacheRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateCacheRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateCacheResponse {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CreateCacheResponse {
    fn default() -> &'a CreateCacheResponse {
        <CreateCacheResponse as ::protobuf::Message>::default_instance()
    }
}

impl CreateCacheResponse {
    pub fn new() -> CreateCacheResponse {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for CreateCacheResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CreateCacheResponse {
        CreateCacheResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CreateCacheResponse>(
                "CreateCacheResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CreateCacheResponse {
        static instance: ::protobuf::rt::LazyV2<CreateCacheResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CreateCacheResponse::new)
    }
}

impl ::protobuf::Clear for CreateCacheResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateCacheResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateCacheResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13controlclient.proto\x12\x0econtrol_client\"3\n\x12DeleteCacheReque\
    st\x12\x1d\n\ncache_name\x18\x01\x20\x01(\tR\tcacheName\"\x15\n\x13Delet\
    eCacheResponse\"3\n\x12CreateCacheRequest\x12\x1d\n\ncache_name\x18\x01\
    \x20\x01(\tR\tcacheName\"\x15\n\x13CreateCacheResponse2\xc0\x01\n\nScsCo\
    ntrol\x12X\n\x0bCreateCache\x12\".control_client.CreateCacheRequest\x1a#\
    .control_client.CreateCacheResponse\"\0\x12X\n\x0bDeleteCache\x12\".cont\
    rol_client.DeleteCacheRequest\x1a#.control_client.DeleteCacheResponse\"\
    \0B\x17\n\x13grpc.control_clientP\x01J\x9a\x03\n\x06\x12\x04\0\0\x18\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x08\x12\x03\x02\0\"\n\t\n\
    \x02\x08\n\x12\x03\x02\0\"\n\x08\n\x01\x08\x12\x03\x03\0,\n\t\n\x02\x08\
    \x01\x12\x03\x03\0,\n\x08\n\x01\x02\x12\x03\x05\0\x17\n\n\n\x02\x06\0\
    \x12\x04\x07\0\n\x01\n\n\n\x03\x06\0\x01\x12\x03\x07\x08\x12\n\x0b\n\x04\
    \x06\0\x02\0\x12\x03\x08\x02G\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x08\
    \x06\x11\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x08\x13%\n\x0c\n\x05\x06\0\
    \x02\0\x03\x12\x03\x080C\n\x0b\n\x04\x06\0\x02\x01\x12\x03\t\x02G\n\x0c\
    \n\x05\x06\0\x02\x01\x01\x12\x03\t\x06\x11\n\x0c\n\x05\x06\0\x02\x01\x02\
    \x12\x03\t\x13%\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\t0C\n\n\n\x02\x04\
    \0\x12\x04\x0c\0\x0e\x01\n\n\n\x03\x04\0\x01\x12\x03\x0c\x08\x1a\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\r\x02\x18\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\r\
    \x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\r\t\x13\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\r\x16\x17\n\n\n\x02\x04\x01\x12\x04\x10\0\x11\x01\n\n\
    \n\x03\x04\x01\x01\x12\x03\x10\x08\x1b\n\n\n\x02\x04\x02\x12\x04\x13\0\
    \x15\x01\n\n\n\x03\x04\x02\x01\x12\x03\x13\x08\x1a\n\x0b\n\x04\x04\x02\
    \x02\0\x12\x03\x14\x02\x18\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x14\x02\
    \x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x14\t\x13\n\x0c\n\x05\x04\x02\
    \x02\0\x03\x12\x03\x14\x16\x17\n\n\n\x02\x04\x03\x12\x04\x17\0\x18\x01\n\
    \n\n\x03\x04\x03\x01\x12\x03\x17\x08\x1bb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}