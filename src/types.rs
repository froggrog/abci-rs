// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct KVPair {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KVPair {}

impl KVPair {
    pub fn new() -> KVPair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KVPair {
        static mut instance: ::protobuf::lazy::Lazy<KVPair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KVPair,
        };
        unsafe {
            instance.get(KVPair::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for KVPair {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KVPair {
    fn new() -> KVPair {
        KVPair::new()
    }

    fn descriptor_static(_: ::std::option::Option<KVPair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    KVPair::get_key_for_reflect,
                    KVPair::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    KVPair::get_value_for_reflect,
                    KVPair::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KVPair>(
                    "KVPair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KVPair {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KVPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KVPair {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct KI64Pair {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub value: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KI64Pair {}

impl KI64Pair {
    pub fn new() -> KI64Pair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KI64Pair {
        static mut instance: ::protobuf::lazy::Lazy<KI64Pair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KI64Pair,
        };
        unsafe {
            instance.get(KI64Pair::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // int64 value = 2;

    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i64) {
        self.value = v;
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &i64 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut i64 {
        &mut self.value
    }
}

impl ::protobuf::Message for KI64Pair {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.value = tmp;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(2, self.value, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if self.value != 0 {
            os.write_int64(2, self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KI64Pair {
    fn new() -> KI64Pair {
        KI64Pair::new()
    }

    fn descriptor_static(_: ::std::option::Option<KI64Pair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    KI64Pair::get_key_for_reflect,
                    KI64Pair::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "value",
                    KI64Pair::get_value_for_reflect,
                    KI64Pair::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KI64Pair>(
                    "KI64Pair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KI64Pair {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KI64Pair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KI64Pair {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message oneof groups
    value: ::std::option::Option<Request_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

#[derive(Clone,PartialEq)]
pub enum Request_oneof_value {
    echo(RequestEcho),
    flush(RequestFlush),
    info(RequestInfo),
    set_option(RequestSetOption),
    init_chain(RequestInitChain),
    query(RequestQuery),
    begin_block(RequestBeginBlock),
    check_tx(RequestCheckTx),
    deliver_tx(RequestDeliverTx),
    end_block(RequestEndBlock),
    commit(RequestCommit),
}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }

    // .types.RequestEcho echo = 2;

    pub fn clear_echo(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_echo(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::echo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_echo(&mut self, v: RequestEcho) {
        self.value = ::std::option::Option::Some(Request_oneof_value::echo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_echo(&mut self) -> &mut RequestEcho {
        if let ::std::option::Option::Some(Request_oneof_value::echo(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::echo(RequestEcho::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::echo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_echo(&mut self) -> RequestEcho {
        if self.has_echo() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::echo(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestEcho::new()
        }
    }

    pub fn get_echo(&self) -> &RequestEcho {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::echo(ref v)) => v,
            _ => RequestEcho::default_instance(),
        }
    }

    // .types.RequestFlush flush = 3;

    pub fn clear_flush(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_flush(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::flush(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_flush(&mut self, v: RequestFlush) {
        self.value = ::std::option::Option::Some(Request_oneof_value::flush(v))
    }

    // Mutable pointer to the field.
    pub fn mut_flush(&mut self) -> &mut RequestFlush {
        if let ::std::option::Option::Some(Request_oneof_value::flush(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::flush(RequestFlush::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::flush(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_flush(&mut self) -> RequestFlush {
        if self.has_flush() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::flush(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestFlush::new()
        }
    }

    pub fn get_flush(&self) -> &RequestFlush {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::flush(ref v)) => v,
            _ => RequestFlush::default_instance(),
        }
    }

    // .types.RequestInfo info = 4;

    pub fn clear_info(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_info(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::info(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: RequestInfo) {
        self.value = ::std::option::Option::Some(Request_oneof_value::info(v))
    }

    // Mutable pointer to the field.
    pub fn mut_info(&mut self) -> &mut RequestInfo {
        if let ::std::option::Option::Some(Request_oneof_value::info(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::info(RequestInfo::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::info(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_info(&mut self) -> RequestInfo {
        if self.has_info() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::info(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestInfo::new()
        }
    }

    pub fn get_info(&self) -> &RequestInfo {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::info(ref v)) => v,
            _ => RequestInfo::default_instance(),
        }
    }

    // .types.RequestSetOption set_option = 5;

    pub fn clear_set_option(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_set_option(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::set_option(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_option(&mut self, v: RequestSetOption) {
        self.value = ::std::option::Option::Some(Request_oneof_value::set_option(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_option(&mut self) -> &mut RequestSetOption {
        if let ::std::option::Option::Some(Request_oneof_value::set_option(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::set_option(RequestSetOption::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::set_option(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_option(&mut self) -> RequestSetOption {
        if self.has_set_option() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::set_option(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestSetOption::new()
        }
    }

    pub fn get_set_option(&self) -> &RequestSetOption {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::set_option(ref v)) => v,
            _ => RequestSetOption::default_instance(),
        }
    }

    // .types.RequestInitChain init_chain = 6;

    pub fn clear_init_chain(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_init_chain(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::init_chain(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_init_chain(&mut self, v: RequestInitChain) {
        self.value = ::std::option::Option::Some(Request_oneof_value::init_chain(v))
    }

    // Mutable pointer to the field.
    pub fn mut_init_chain(&mut self) -> &mut RequestInitChain {
        if let ::std::option::Option::Some(Request_oneof_value::init_chain(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::init_chain(RequestInitChain::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::init_chain(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_init_chain(&mut self) -> RequestInitChain {
        if self.has_init_chain() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::init_chain(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestInitChain::new()
        }
    }

    pub fn get_init_chain(&self) -> &RequestInitChain {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::init_chain(ref v)) => v,
            _ => RequestInitChain::default_instance(),
        }
    }

    // .types.RequestQuery query = 7;

    pub fn clear_query(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_query(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::query(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: RequestQuery) {
        self.value = ::std::option::Option::Some(Request_oneof_value::query(v))
    }

    // Mutable pointer to the field.
    pub fn mut_query(&mut self) -> &mut RequestQuery {
        if let ::std::option::Option::Some(Request_oneof_value::query(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::query(RequestQuery::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::query(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_query(&mut self) -> RequestQuery {
        if self.has_query() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::query(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestQuery::new()
        }
    }

    pub fn get_query(&self) -> &RequestQuery {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::query(ref v)) => v,
            _ => RequestQuery::default_instance(),
        }
    }

    // .types.RequestBeginBlock begin_block = 8;

    pub fn clear_begin_block(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_begin_block(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::begin_block(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_begin_block(&mut self, v: RequestBeginBlock) {
        self.value = ::std::option::Option::Some(Request_oneof_value::begin_block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_begin_block(&mut self) -> &mut RequestBeginBlock {
        if let ::std::option::Option::Some(Request_oneof_value::begin_block(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::begin_block(RequestBeginBlock::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::begin_block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_begin_block(&mut self) -> RequestBeginBlock {
        if self.has_begin_block() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::begin_block(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestBeginBlock::new()
        }
    }

    pub fn get_begin_block(&self) -> &RequestBeginBlock {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::begin_block(ref v)) => v,
            _ => RequestBeginBlock::default_instance(),
        }
    }

    // .types.RequestCheckTx check_tx = 9;

    pub fn clear_check_tx(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_check_tx(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::check_tx(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_check_tx(&mut self, v: RequestCheckTx) {
        self.value = ::std::option::Option::Some(Request_oneof_value::check_tx(v))
    }

    // Mutable pointer to the field.
    pub fn mut_check_tx(&mut self) -> &mut RequestCheckTx {
        if let ::std::option::Option::Some(Request_oneof_value::check_tx(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::check_tx(RequestCheckTx::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::check_tx(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_check_tx(&mut self) -> RequestCheckTx {
        if self.has_check_tx() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::check_tx(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestCheckTx::new()
        }
    }

    pub fn get_check_tx(&self) -> &RequestCheckTx {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::check_tx(ref v)) => v,
            _ => RequestCheckTx::default_instance(),
        }
    }

    // .types.RequestDeliverTx deliver_tx = 19;

    pub fn clear_deliver_tx(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_deliver_tx(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::deliver_tx(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_deliver_tx(&mut self, v: RequestDeliverTx) {
        self.value = ::std::option::Option::Some(Request_oneof_value::deliver_tx(v))
    }

    // Mutable pointer to the field.
    pub fn mut_deliver_tx(&mut self) -> &mut RequestDeliverTx {
        if let ::std::option::Option::Some(Request_oneof_value::deliver_tx(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::deliver_tx(RequestDeliverTx::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::deliver_tx(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_deliver_tx(&mut self) -> RequestDeliverTx {
        if self.has_deliver_tx() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::deliver_tx(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestDeliverTx::new()
        }
    }

    pub fn get_deliver_tx(&self) -> &RequestDeliverTx {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::deliver_tx(ref v)) => v,
            _ => RequestDeliverTx::default_instance(),
        }
    }

    // .types.RequestEndBlock end_block = 11;

    pub fn clear_end_block(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_end_block(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::end_block(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_block(&mut self, v: RequestEndBlock) {
        self.value = ::std::option::Option::Some(Request_oneof_value::end_block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_end_block(&mut self) -> &mut RequestEndBlock {
        if let ::std::option::Option::Some(Request_oneof_value::end_block(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::end_block(RequestEndBlock::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::end_block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_block(&mut self) -> RequestEndBlock {
        if self.has_end_block() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::end_block(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestEndBlock::new()
        }
    }

    pub fn get_end_block(&self) -> &RequestEndBlock {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::end_block(ref v)) => v,
            _ => RequestEndBlock::default_instance(),
        }
    }

    // .types.RequestCommit commit = 12;

    pub fn clear_commit(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_commit(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::commit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_commit(&mut self, v: RequestCommit) {
        self.value = ::std::option::Option::Some(Request_oneof_value::commit(v))
    }

    // Mutable pointer to the field.
    pub fn mut_commit(&mut self) -> &mut RequestCommit {
        if let ::std::option::Option::Some(Request_oneof_value::commit(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Request_oneof_value::commit(RequestCommit::new()));
        }
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::commit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_commit(&mut self) -> RequestCommit {
        if self.has_commit() {
            match self.value.take() {
                ::std::option::Option::Some(Request_oneof_value::commit(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestCommit::new()
        }
    }

    pub fn get_commit(&self) -> &RequestCommit {
        match self.value {
            ::std::option::Option::Some(Request_oneof_value::commit(ref v)) => v,
            _ => RequestCommit::default_instance(),
        }
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if let Some(Request_oneof_value::echo(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_value::flush(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_value::info(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_value::set_option(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_value::init_chain(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_value::query(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_value::begin_block(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_value::check_tx(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_value::deliver_tx(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_value::end_block(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_value::commit(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Request_oneof_value::echo(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Request_oneof_value::flush(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Request_oneof_value::info(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Request_oneof_value::set_option(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Request_oneof_value::init_chain(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Request_oneof_value::query(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Request_oneof_value::begin_block(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Request_oneof_value::check_tx(is.read_message()?));
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Request_oneof_value::deliver_tx(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Request_oneof_value::end_block(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Request_oneof_value::commit(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Request_oneof_value::echo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::flush(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::info(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::set_option(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::init_chain(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::query(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::begin_block(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::check_tx(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::deliver_tx(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::end_block(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_value::commit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Request_oneof_value::echo(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::flush(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::info(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::set_option(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::init_chain(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::query(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::begin_block(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::check_tx(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::deliver_tx(ref v) => {
                    os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::end_block(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_value::commit(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestEcho>(
                    "echo",
                    Request::has_echo,
                    Request::get_echo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestFlush>(
                    "flush",
                    Request::has_flush,
                    Request::get_flush,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestInfo>(
                    "info",
                    Request::has_info,
                    Request::get_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestSetOption>(
                    "set_option",
                    Request::has_set_option,
                    Request::get_set_option,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestInitChain>(
                    "init_chain",
                    Request::has_init_chain,
                    Request::get_init_chain,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestQuery>(
                    "query",
                    Request::has_query,
                    Request::get_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestBeginBlock>(
                    "begin_block",
                    Request::has_begin_block,
                    Request::get_begin_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestCheckTx>(
                    "check_tx",
                    Request::has_check_tx,
                    Request::get_check_tx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestDeliverTx>(
                    "deliver_tx",
                    Request::has_deliver_tx,
                    Request::get_deliver_tx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestEndBlock>(
                    "end_block",
                    Request::has_end_block,
                    Request::get_end_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestCommit>(
                    "commit",
                    Request::has_commit,
                    Request::get_commit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_echo();
        self.clear_flush();
        self.clear_info();
        self.clear_set_option();
        self.clear_init_chain();
        self.clear_query();
        self.clear_begin_block();
        self.clear_check_tx();
        self.clear_deliver_tx();
        self.clear_end_block();
        self.clear_commit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestEcho {
    // message fields
    pub message: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestEcho {}

impl RequestEcho {
    pub fn new() -> RequestEcho {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestEcho {
        static mut instance: ::protobuf::lazy::Lazy<RequestEcho> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestEcho,
        };
        unsafe {
            instance.get(RequestEcho::new)
        }
    }

    // string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }
}

impl ::protobuf::Message for RequestEcho {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
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
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestEcho {
    fn new() -> RequestEcho {
        RequestEcho::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestEcho>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    RequestEcho::get_message_for_reflect,
                    RequestEcho::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestEcho>(
                    "RequestEcho",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestEcho {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestEcho {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestEcho {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestFlush {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestFlush {}

impl RequestFlush {
    pub fn new() -> RequestFlush {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestFlush {
        static mut instance: ::protobuf::lazy::Lazy<RequestFlush> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestFlush,
        };
        unsafe {
            instance.get(RequestFlush::new)
        }
    }
}

impl ::protobuf::Message for RequestFlush {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestFlush {
    fn new() -> RequestFlush {
        RequestFlush::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestFlush>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestFlush>(
                    "RequestFlush",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestFlush {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestFlush {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestFlush {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestInfo {
    // message fields
    pub version: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestInfo {}

impl RequestInfo {
    pub fn new() -> RequestInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestInfo {
        static mut instance: ::protobuf::lazy::Lazy<RequestInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestInfo,
        };
        unsafe {
            instance.get(RequestInfo::new)
        }
    }

    // string version = 1;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    fn get_version_for_reflect(&self) -> &::std::string::String {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }
}

impl ::protobuf::Message for RequestInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
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
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.version.is_empty() {
            os.write_string(1, &self.version)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestInfo {
    fn new() -> RequestInfo {
        RequestInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    RequestInfo::get_version_for_reflect,
                    RequestInfo::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestInfo>(
                    "RequestInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestInfo {
    fn clear(&mut self) {
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestSetOption {
    // message fields
    pub key: ::std::string::String,
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestSetOption {}

impl RequestSetOption {
    pub fn new() -> RequestSetOption {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestSetOption {
        static mut instance: ::protobuf::lazy::Lazy<RequestSetOption> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestSetOption,
        };
        unsafe {
            instance.get(RequestSetOption::new)
        }
    }

    // string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::string::String {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::string::String {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }
}

impl ::protobuf::Message for RequestSetOption {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_string(2, &self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestSetOption {
    fn new() -> RequestSetOption {
        RequestSetOption::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestSetOption>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    RequestSetOption::get_key_for_reflect,
                    RequestSetOption::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    RequestSetOption::get_value_for_reflect,
                    RequestSetOption::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestSetOption>(
                    "RequestSetOption",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestSetOption {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestSetOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestSetOption {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestInitChain {
    // message fields
    pub validators: ::protobuf::RepeatedField<Validator>,
    pub app_state_bytes: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestInitChain {}

impl RequestInitChain {
    pub fn new() -> RequestInitChain {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestInitChain {
        static mut instance: ::protobuf::lazy::Lazy<RequestInitChain> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestInitChain,
        };
        unsafe {
            instance.get(RequestInitChain::new)
        }
    }

    // repeated .types.Validator validators = 1;

    pub fn clear_validators(&mut self) {
        self.validators.clear();
    }

    // Param is passed by value, moved
    pub fn set_validators(&mut self, v: ::protobuf::RepeatedField<Validator>) {
        self.validators = v;
    }

    // Mutable pointer to the field.
    pub fn mut_validators(&mut self) -> &mut ::protobuf::RepeatedField<Validator> {
        &mut self.validators
    }

    // Take field
    pub fn take_validators(&mut self) -> ::protobuf::RepeatedField<Validator> {
        ::std::mem::replace(&mut self.validators, ::protobuf::RepeatedField::new())
    }

    pub fn get_validators(&self) -> &[Validator] {
        &self.validators
    }

    fn get_validators_for_reflect(&self) -> &::protobuf::RepeatedField<Validator> {
        &self.validators
    }

    fn mut_validators_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Validator> {
        &mut self.validators
    }

    // bytes app_state_bytes = 2;

    pub fn clear_app_state_bytes(&mut self) {
        self.app_state_bytes.clear();
    }

    // Param is passed by value, moved
    pub fn set_app_state_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.app_state_bytes = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_app_state_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.app_state_bytes
    }

    // Take field
    pub fn take_app_state_bytes(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.app_state_bytes, ::std::vec::Vec::new())
    }

    pub fn get_app_state_bytes(&self) -> &[u8] {
        &self.app_state_bytes
    }

    fn get_app_state_bytes_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.app_state_bytes
    }

    fn mut_app_state_bytes_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.app_state_bytes
    }
}

impl ::protobuf::Message for RequestInitChain {
    fn is_initialized(&self) -> bool {
        for v in &self.validators {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.validators)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.app_state_bytes)?;
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
        for value in &self.validators {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.app_state_bytes.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.app_state_bytes);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.validators {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.app_state_bytes.is_empty() {
            os.write_bytes(2, &self.app_state_bytes)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestInitChain {
    fn new() -> RequestInitChain {
        RequestInitChain::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestInitChain>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Validator>>(
                    "validators",
                    RequestInitChain::get_validators_for_reflect,
                    RequestInitChain::mut_validators_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "app_state_bytes",
                    RequestInitChain::get_app_state_bytes_for_reflect,
                    RequestInitChain::mut_app_state_bytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestInitChain>(
                    "RequestInitChain",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestInitChain {
    fn clear(&mut self) {
        self.clear_validators();
        self.clear_app_state_bytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestInitChain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestInitChain {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestQuery {
    // message fields
    pub data: ::std::vec::Vec<u8>,
    pub path: ::std::string::String,
    pub height: i64,
    pub prove: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestQuery {}

impl RequestQuery {
    pub fn new() -> RequestQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestQuery {
        static mut instance: ::protobuf::lazy::Lazy<RequestQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestQuery,
        };
        unsafe {
            instance.get(RequestQuery::new)
        }
    }

    // bytes data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // string path = 2;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.path, ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    fn get_path_for_reflect(&self) -> &::std::string::String {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // int64 height = 3;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i64) {
        self.height = v;
    }

    pub fn get_height(&self) -> i64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &i64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut i64 {
        &mut self.height
    }

    // bool prove = 4;

    pub fn clear_prove(&mut self) {
        self.prove = false;
    }

    // Param is passed by value, moved
    pub fn set_prove(&mut self, v: bool) {
        self.prove = v;
    }

    pub fn get_prove(&self) -> bool {
        self.prove
    }

    fn get_prove_for_reflect(&self) -> &bool {
        &self.prove
    }

    fn mut_prove_for_reflect(&mut self) -> &mut bool {
        &mut self.prove
    }
}

impl ::protobuf::Message for RequestQuery {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.height = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.prove = tmp;
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.data);
        }
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.path);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.prove != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.data.is_empty() {
            os.write_bytes(1, &self.data)?;
        }
        if !self.path.is_empty() {
            os.write_string(2, &self.path)?;
        }
        if self.height != 0 {
            os.write_int64(3, self.height)?;
        }
        if self.prove != false {
            os.write_bool(4, self.prove)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestQuery {
    fn new() -> RequestQuery {
        RequestQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    RequestQuery::get_data_for_reflect,
                    RequestQuery::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    RequestQuery::get_path_for_reflect,
                    RequestQuery::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "height",
                    RequestQuery::get_height_for_reflect,
                    RequestQuery::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "prove",
                    RequestQuery::get_prove_for_reflect,
                    RequestQuery::mut_prove_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestQuery>(
                    "RequestQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestQuery {
    fn clear(&mut self) {
        self.clear_data();
        self.clear_path();
        self.clear_height();
        self.clear_prove();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestBeginBlock {
    // message fields
    pub hash: ::std::vec::Vec<u8>,
    pub header: ::protobuf::SingularPtrField<Header>,
    pub absent_validators: ::std::vec::Vec<i32>,
    pub byzantine_validators: ::protobuf::RepeatedField<Evidence>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestBeginBlock {}

impl RequestBeginBlock {
    pub fn new() -> RequestBeginBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestBeginBlock {
        static mut instance: ::protobuf::lazy::Lazy<RequestBeginBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestBeginBlock,
        };
        unsafe {
            instance.get(RequestBeginBlock::new)
        }
    }

    // bytes hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // .types.Header header = 2;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: Header) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut Header {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> Header {
        self.header.take().unwrap_or_else(|| Header::new())
    }

    pub fn get_header(&self) -> &Header {
        self.header.as_ref().unwrap_or_else(|| Header::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<Header> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Header> {
        &mut self.header
    }

    // repeated int32 absent_validators = 3;

    pub fn clear_absent_validators(&mut self) {
        self.absent_validators.clear();
    }

    // Param is passed by value, moved
    pub fn set_absent_validators(&mut self, v: ::std::vec::Vec<i32>) {
        self.absent_validators = v;
    }

    // Mutable pointer to the field.
    pub fn mut_absent_validators(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.absent_validators
    }

    // Take field
    pub fn take_absent_validators(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.absent_validators, ::std::vec::Vec::new())
    }

    pub fn get_absent_validators(&self) -> &[i32] {
        &self.absent_validators
    }

    fn get_absent_validators_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.absent_validators
    }

    fn mut_absent_validators_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.absent_validators
    }

    // repeated .types.Evidence byzantine_validators = 4;

    pub fn clear_byzantine_validators(&mut self) {
        self.byzantine_validators.clear();
    }

    // Param is passed by value, moved
    pub fn set_byzantine_validators(&mut self, v: ::protobuf::RepeatedField<Evidence>) {
        self.byzantine_validators = v;
    }

    // Mutable pointer to the field.
    pub fn mut_byzantine_validators(&mut self) -> &mut ::protobuf::RepeatedField<Evidence> {
        &mut self.byzantine_validators
    }

    // Take field
    pub fn take_byzantine_validators(&mut self) -> ::protobuf::RepeatedField<Evidence> {
        ::std::mem::replace(&mut self.byzantine_validators, ::protobuf::RepeatedField::new())
    }

    pub fn get_byzantine_validators(&self) -> &[Evidence] {
        &self.byzantine_validators
    }

    fn get_byzantine_validators_for_reflect(&self) -> &::protobuf::RepeatedField<Evidence> {
        &self.byzantine_validators
    }

    fn mut_byzantine_validators_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Evidence> {
        &mut self.byzantine_validators
    }
}

impl ::protobuf::Message for RequestBeginBlock {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.byzantine_validators {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.absent_validators)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.byzantine_validators)?;
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
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.hash);
        }
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.absent_validators {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.byzantine_validators {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.hash.is_empty() {
            os.write_bytes(1, &self.hash)?;
        }
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.absent_validators {
            os.write_int32(3, *v)?;
        };
        for v in &self.byzantine_validators {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestBeginBlock {
    fn new() -> RequestBeginBlock {
        RequestBeginBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestBeginBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    RequestBeginBlock::get_hash_for_reflect,
                    RequestBeginBlock::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Header>>(
                    "header",
                    RequestBeginBlock::get_header_for_reflect,
                    RequestBeginBlock::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "absent_validators",
                    RequestBeginBlock::get_absent_validators_for_reflect,
                    RequestBeginBlock::mut_absent_validators_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Evidence>>(
                    "byzantine_validators",
                    RequestBeginBlock::get_byzantine_validators_for_reflect,
                    RequestBeginBlock::mut_byzantine_validators_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestBeginBlock>(
                    "RequestBeginBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestBeginBlock {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_header();
        self.clear_absent_validators();
        self.clear_byzantine_validators();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestBeginBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestBeginBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestCheckTx {
    // message fields
    pub tx: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestCheckTx {}

impl RequestCheckTx {
    pub fn new() -> RequestCheckTx {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestCheckTx {
        static mut instance: ::protobuf::lazy::Lazy<RequestCheckTx> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestCheckTx,
        };
        unsafe {
            instance.get(RequestCheckTx::new)
        }
    }

    // bytes tx = 1;

    pub fn clear_tx(&mut self) {
        self.tx.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx(&mut self, v: ::std::vec::Vec<u8>) {
        self.tx = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx
    }

    // Take field
    pub fn take_tx(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tx, ::std::vec::Vec::new())
    }

    pub fn get_tx(&self) -> &[u8] {
        &self.tx
    }

    fn get_tx_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.tx
    }

    fn mut_tx_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx
    }
}

impl ::protobuf::Message for RequestCheckTx {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tx)?;
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
        if !self.tx.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.tx);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx.is_empty() {
            os.write_bytes(1, &self.tx)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestCheckTx {
    fn new() -> RequestCheckTx {
        RequestCheckTx::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestCheckTx>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx",
                    RequestCheckTx::get_tx_for_reflect,
                    RequestCheckTx::mut_tx_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestCheckTx>(
                    "RequestCheckTx",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestCheckTx {
    fn clear(&mut self) {
        self.clear_tx();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestCheckTx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestCheckTx {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestDeliverTx {
    // message fields
    pub tx: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestDeliverTx {}

impl RequestDeliverTx {
    pub fn new() -> RequestDeliverTx {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestDeliverTx {
        static mut instance: ::protobuf::lazy::Lazy<RequestDeliverTx> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestDeliverTx,
        };
        unsafe {
            instance.get(RequestDeliverTx::new)
        }
    }

    // bytes tx = 1;

    pub fn clear_tx(&mut self) {
        self.tx.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx(&mut self, v: ::std::vec::Vec<u8>) {
        self.tx = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx
    }

    // Take field
    pub fn take_tx(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tx, ::std::vec::Vec::new())
    }

    pub fn get_tx(&self) -> &[u8] {
        &self.tx
    }

    fn get_tx_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.tx
    }

    fn mut_tx_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx
    }
}

impl ::protobuf::Message for RequestDeliverTx {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tx)?;
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
        if !self.tx.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.tx);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx.is_empty() {
            os.write_bytes(1, &self.tx)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestDeliverTx {
    fn new() -> RequestDeliverTx {
        RequestDeliverTx::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestDeliverTx>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx",
                    RequestDeliverTx::get_tx_for_reflect,
                    RequestDeliverTx::mut_tx_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestDeliverTx>(
                    "RequestDeliverTx",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestDeliverTx {
    fn clear(&mut self) {
        self.clear_tx();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestDeliverTx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestDeliverTx {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestEndBlock {
    // message fields
    pub height: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestEndBlock {}

impl RequestEndBlock {
    pub fn new() -> RequestEndBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestEndBlock {
        static mut instance: ::protobuf::lazy::Lazy<RequestEndBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestEndBlock,
        };
        unsafe {
            instance.get(RequestEndBlock::new)
        }
    }

    // int64 height = 1;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i64) {
        self.height = v;
    }

    pub fn get_height(&self) -> i64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &i64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut i64 {
        &mut self.height
    }
}

impl ::protobuf::Message for RequestEndBlock {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.height = tmp;
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
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(1, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.height != 0 {
            os.write_int64(1, self.height)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestEndBlock {
    fn new() -> RequestEndBlock {
        RequestEndBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestEndBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "height",
                    RequestEndBlock::get_height_for_reflect,
                    RequestEndBlock::mut_height_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestEndBlock>(
                    "RequestEndBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestEndBlock {
    fn clear(&mut self) {
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestEndBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestEndBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestCommit {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestCommit {}

impl RequestCommit {
    pub fn new() -> RequestCommit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestCommit {
        static mut instance: ::protobuf::lazy::Lazy<RequestCommit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestCommit,
        };
        unsafe {
            instance.get(RequestCommit::new)
        }
    }
}

impl ::protobuf::Message for RequestCommit {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestCommit {
    fn new() -> RequestCommit {
        RequestCommit::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestCommit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestCommit>(
                    "RequestCommit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestCommit {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestCommit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestCommit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message oneof groups
    value: ::std::option::Option<Response_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

#[derive(Clone,PartialEq)]
pub enum Response_oneof_value {
    exception(ResponseException),
    echo(ResponseEcho),
    flush(ResponseFlush),
    info(ResponseInfo),
    set_option(ResponseSetOption),
    init_chain(ResponseInitChain),
    query(ResponseQuery),
    begin_block(ResponseBeginBlock),
    check_tx(ResponseCheckTx),
    deliver_tx(ResponseDeliverTx),
    end_block(ResponseEndBlock),
    commit(ResponseCommit),
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }

    // .types.ResponseException exception = 1;

    pub fn clear_exception(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_exception(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::exception(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_exception(&mut self, v: ResponseException) {
        self.value = ::std::option::Option::Some(Response_oneof_value::exception(v))
    }

    // Mutable pointer to the field.
    pub fn mut_exception(&mut self) -> &mut ResponseException {
        if let ::std::option::Option::Some(Response_oneof_value::exception(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::exception(ResponseException::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::exception(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_exception(&mut self) -> ResponseException {
        if self.has_exception() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::exception(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseException::new()
        }
    }

    pub fn get_exception(&self) -> &ResponseException {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::exception(ref v)) => v,
            _ => ResponseException::default_instance(),
        }
    }

    // .types.ResponseEcho echo = 2;

    pub fn clear_echo(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_echo(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::echo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_echo(&mut self, v: ResponseEcho) {
        self.value = ::std::option::Option::Some(Response_oneof_value::echo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_echo(&mut self) -> &mut ResponseEcho {
        if let ::std::option::Option::Some(Response_oneof_value::echo(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::echo(ResponseEcho::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::echo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_echo(&mut self) -> ResponseEcho {
        if self.has_echo() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::echo(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseEcho::new()
        }
    }

    pub fn get_echo(&self) -> &ResponseEcho {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::echo(ref v)) => v,
            _ => ResponseEcho::default_instance(),
        }
    }

    // .types.ResponseFlush flush = 3;

    pub fn clear_flush(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_flush(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::flush(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_flush(&mut self, v: ResponseFlush) {
        self.value = ::std::option::Option::Some(Response_oneof_value::flush(v))
    }

    // Mutable pointer to the field.
    pub fn mut_flush(&mut self) -> &mut ResponseFlush {
        if let ::std::option::Option::Some(Response_oneof_value::flush(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::flush(ResponseFlush::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::flush(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_flush(&mut self) -> ResponseFlush {
        if self.has_flush() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::flush(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseFlush::new()
        }
    }

    pub fn get_flush(&self) -> &ResponseFlush {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::flush(ref v)) => v,
            _ => ResponseFlush::default_instance(),
        }
    }

    // .types.ResponseInfo info = 4;

    pub fn clear_info(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_info(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::info(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ResponseInfo) {
        self.value = ::std::option::Option::Some(Response_oneof_value::info(v))
    }

    // Mutable pointer to the field.
    pub fn mut_info(&mut self) -> &mut ResponseInfo {
        if let ::std::option::Option::Some(Response_oneof_value::info(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::info(ResponseInfo::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::info(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_info(&mut self) -> ResponseInfo {
        if self.has_info() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::info(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseInfo::new()
        }
    }

    pub fn get_info(&self) -> &ResponseInfo {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::info(ref v)) => v,
            _ => ResponseInfo::default_instance(),
        }
    }

    // .types.ResponseSetOption set_option = 5;

    pub fn clear_set_option(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_set_option(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::set_option(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_set_option(&mut self, v: ResponseSetOption) {
        self.value = ::std::option::Option::Some(Response_oneof_value::set_option(v))
    }

    // Mutable pointer to the field.
    pub fn mut_set_option(&mut self) -> &mut ResponseSetOption {
        if let ::std::option::Option::Some(Response_oneof_value::set_option(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::set_option(ResponseSetOption::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::set_option(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_set_option(&mut self) -> ResponseSetOption {
        if self.has_set_option() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::set_option(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseSetOption::new()
        }
    }

    pub fn get_set_option(&self) -> &ResponseSetOption {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::set_option(ref v)) => v,
            _ => ResponseSetOption::default_instance(),
        }
    }

    // .types.ResponseInitChain init_chain = 6;

    pub fn clear_init_chain(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_init_chain(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::init_chain(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_init_chain(&mut self, v: ResponseInitChain) {
        self.value = ::std::option::Option::Some(Response_oneof_value::init_chain(v))
    }

    // Mutable pointer to the field.
    pub fn mut_init_chain(&mut self) -> &mut ResponseInitChain {
        if let ::std::option::Option::Some(Response_oneof_value::init_chain(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::init_chain(ResponseInitChain::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::init_chain(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_init_chain(&mut self) -> ResponseInitChain {
        if self.has_init_chain() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::init_chain(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseInitChain::new()
        }
    }

    pub fn get_init_chain(&self) -> &ResponseInitChain {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::init_chain(ref v)) => v,
            _ => ResponseInitChain::default_instance(),
        }
    }

    // .types.ResponseQuery query = 7;

    pub fn clear_query(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_query(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::query(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: ResponseQuery) {
        self.value = ::std::option::Option::Some(Response_oneof_value::query(v))
    }

    // Mutable pointer to the field.
    pub fn mut_query(&mut self) -> &mut ResponseQuery {
        if let ::std::option::Option::Some(Response_oneof_value::query(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::query(ResponseQuery::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::query(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_query(&mut self) -> ResponseQuery {
        if self.has_query() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::query(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseQuery::new()
        }
    }

    pub fn get_query(&self) -> &ResponseQuery {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::query(ref v)) => v,
            _ => ResponseQuery::default_instance(),
        }
    }

    // .types.ResponseBeginBlock begin_block = 8;

    pub fn clear_begin_block(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_begin_block(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::begin_block(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_begin_block(&mut self, v: ResponseBeginBlock) {
        self.value = ::std::option::Option::Some(Response_oneof_value::begin_block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_begin_block(&mut self) -> &mut ResponseBeginBlock {
        if let ::std::option::Option::Some(Response_oneof_value::begin_block(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::begin_block(ResponseBeginBlock::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::begin_block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_begin_block(&mut self) -> ResponseBeginBlock {
        if self.has_begin_block() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::begin_block(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseBeginBlock::new()
        }
    }

    pub fn get_begin_block(&self) -> &ResponseBeginBlock {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::begin_block(ref v)) => v,
            _ => ResponseBeginBlock::default_instance(),
        }
    }

    // .types.ResponseCheckTx check_tx = 9;

    pub fn clear_check_tx(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_check_tx(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::check_tx(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_check_tx(&mut self, v: ResponseCheckTx) {
        self.value = ::std::option::Option::Some(Response_oneof_value::check_tx(v))
    }

    // Mutable pointer to the field.
    pub fn mut_check_tx(&mut self) -> &mut ResponseCheckTx {
        if let ::std::option::Option::Some(Response_oneof_value::check_tx(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::check_tx(ResponseCheckTx::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::check_tx(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_check_tx(&mut self) -> ResponseCheckTx {
        if self.has_check_tx() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::check_tx(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseCheckTx::new()
        }
    }

    pub fn get_check_tx(&self) -> &ResponseCheckTx {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::check_tx(ref v)) => v,
            _ => ResponseCheckTx::default_instance(),
        }
    }

    // .types.ResponseDeliverTx deliver_tx = 10;

    pub fn clear_deliver_tx(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_deliver_tx(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::deliver_tx(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_deliver_tx(&mut self, v: ResponseDeliverTx) {
        self.value = ::std::option::Option::Some(Response_oneof_value::deliver_tx(v))
    }

    // Mutable pointer to the field.
    pub fn mut_deliver_tx(&mut self) -> &mut ResponseDeliverTx {
        if let ::std::option::Option::Some(Response_oneof_value::deliver_tx(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::deliver_tx(ResponseDeliverTx::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::deliver_tx(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_deliver_tx(&mut self) -> ResponseDeliverTx {
        if self.has_deliver_tx() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::deliver_tx(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseDeliverTx::new()
        }
    }

    pub fn get_deliver_tx(&self) -> &ResponseDeliverTx {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::deliver_tx(ref v)) => v,
            _ => ResponseDeliverTx::default_instance(),
        }
    }

    // .types.ResponseEndBlock end_block = 11;

    pub fn clear_end_block(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_end_block(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::end_block(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_block(&mut self, v: ResponseEndBlock) {
        self.value = ::std::option::Option::Some(Response_oneof_value::end_block(v))
    }

    // Mutable pointer to the field.
    pub fn mut_end_block(&mut self) -> &mut ResponseEndBlock {
        if let ::std::option::Option::Some(Response_oneof_value::end_block(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::end_block(ResponseEndBlock::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::end_block(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_block(&mut self) -> ResponseEndBlock {
        if self.has_end_block() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::end_block(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseEndBlock::new()
        }
    }

    pub fn get_end_block(&self) -> &ResponseEndBlock {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::end_block(ref v)) => v,
            _ => ResponseEndBlock::default_instance(),
        }
    }

    // .types.ResponseCommit commit = 12;

    pub fn clear_commit(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_commit(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::commit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_commit(&mut self, v: ResponseCommit) {
        self.value = ::std::option::Option::Some(Response_oneof_value::commit(v))
    }

    // Mutable pointer to the field.
    pub fn mut_commit(&mut self) -> &mut ResponseCommit {
        if let ::std::option::Option::Some(Response_oneof_value::commit(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(Response_oneof_value::commit(ResponseCommit::new()));
        }
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::commit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_commit(&mut self) -> ResponseCommit {
        if self.has_commit() {
            match self.value.take() {
                ::std::option::Option::Some(Response_oneof_value::commit(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseCommit::new()
        }
    }

    pub fn get_commit(&self) -> &ResponseCommit {
        match self.value {
            ::std::option::Option::Some(Response_oneof_value::commit(ref v)) => v,
            _ => ResponseCommit::default_instance(),
        }
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        if let Some(Response_oneof_value::exception(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_value::echo(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_value::flush(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_value::info(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_value::set_option(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_value::init_chain(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_value::query(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_value::begin_block(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_value::check_tx(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_value::deliver_tx(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_value::end_block(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_value::commit(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Response_oneof_value::exception(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Response_oneof_value::echo(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Response_oneof_value::flush(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Response_oneof_value::info(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Response_oneof_value::set_option(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Response_oneof_value::init_chain(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Response_oneof_value::query(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Response_oneof_value::begin_block(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Response_oneof_value::check_tx(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Response_oneof_value::deliver_tx(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Response_oneof_value::end_block(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(Response_oneof_value::commit(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Response_oneof_value::exception(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::echo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::flush(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::info(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::set_option(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::init_chain(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::query(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::begin_block(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::check_tx(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::deliver_tx(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::end_block(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_value::commit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &Response_oneof_value::exception(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::echo(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::flush(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::info(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::set_option(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::init_chain(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::query(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::begin_block(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::check_tx(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::deliver_tx(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::end_block(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_value::commit(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseException>(
                    "exception",
                    Response::has_exception,
                    Response::get_exception,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseEcho>(
                    "echo",
                    Response::has_echo,
                    Response::get_echo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseFlush>(
                    "flush",
                    Response::has_flush,
                    Response::get_flush,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseInfo>(
                    "info",
                    Response::has_info,
                    Response::get_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseSetOption>(
                    "set_option",
                    Response::has_set_option,
                    Response::get_set_option,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseInitChain>(
                    "init_chain",
                    Response::has_init_chain,
                    Response::get_init_chain,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseQuery>(
                    "query",
                    Response::has_query,
                    Response::get_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseBeginBlock>(
                    "begin_block",
                    Response::has_begin_block,
                    Response::get_begin_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseCheckTx>(
                    "check_tx",
                    Response::has_check_tx,
                    Response::get_check_tx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseDeliverTx>(
                    "deliver_tx",
                    Response::has_deliver_tx,
                    Response::get_deliver_tx,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseEndBlock>(
                    "end_block",
                    Response::has_end_block,
                    Response::get_end_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseCommit>(
                    "commit",
                    Response::has_commit,
                    Response::get_commit,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_exception();
        self.clear_echo();
        self.clear_flush();
        self.clear_info();
        self.clear_set_option();
        self.clear_init_chain();
        self.clear_query();
        self.clear_begin_block();
        self.clear_check_tx();
        self.clear_deliver_tx();
        self.clear_end_block();
        self.clear_commit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseException {
    // message fields
    pub error: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseException {}

impl ResponseException {
    pub fn new() -> ResponseException {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseException {
        static mut instance: ::protobuf::lazy::Lazy<ResponseException> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseException,
        };
        unsafe {
            instance.get(ResponseException::new)
        }
    }

    // string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::std::string::String {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.error
    }
}

impl ::protobuf::Message for ResponseException {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.error)?;
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
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.error.is_empty() {
            os.write_string(1, &self.error)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseException {
    fn new() -> ResponseException {
        ResponseException::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseException>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    ResponseException::get_error_for_reflect,
                    ResponseException::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseException>(
                    "ResponseException",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseException {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseException {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseException {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseEcho {
    // message fields
    pub message: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseEcho {}

impl ResponseEcho {
    pub fn new() -> ResponseEcho {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseEcho {
        static mut instance: ::protobuf::lazy::Lazy<ResponseEcho> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseEcho,
        };
        unsafe {
            instance.get(ResponseEcho::new)
        }
    }

    // string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }
}

impl ::protobuf::Message for ResponseEcho {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
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
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseEcho {
    fn new() -> ResponseEcho {
        ResponseEcho::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseEcho>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    ResponseEcho::get_message_for_reflect,
                    ResponseEcho::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseEcho>(
                    "ResponseEcho",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseEcho {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseEcho {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseEcho {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseFlush {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseFlush {}

impl ResponseFlush {
    pub fn new() -> ResponseFlush {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseFlush {
        static mut instance: ::protobuf::lazy::Lazy<ResponseFlush> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseFlush,
        };
        unsafe {
            instance.get(ResponseFlush::new)
        }
    }
}

impl ::protobuf::Message for ResponseFlush {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseFlush {
    fn new() -> ResponseFlush {
        ResponseFlush::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseFlush>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ResponseFlush>(
                    "ResponseFlush",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseFlush {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseFlush {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseFlush {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseInfo {
    // message fields
    pub data: ::std::string::String,
    pub version: ::std::string::String,
    pub last_block_height: i64,
    pub last_block_app_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseInfo {}

impl ResponseInfo {
    pub fn new() -> ResponseInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseInfo {
        static mut instance: ::protobuf::lazy::Lazy<ResponseInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseInfo,
        };
        unsafe {
            instance.get(ResponseInfo::new)
        }
    }

    // string data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::string::String) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::string::String {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.data, ::std::string::String::new())
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::string::String {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.data
    }

    // string version = 2;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    fn get_version_for_reflect(&self) -> &::std::string::String {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // int64 last_block_height = 3;

    pub fn clear_last_block_height(&mut self) {
        self.last_block_height = 0;
    }

    // Param is passed by value, moved
    pub fn set_last_block_height(&mut self, v: i64) {
        self.last_block_height = v;
    }

    pub fn get_last_block_height(&self) -> i64 {
        self.last_block_height
    }

    fn get_last_block_height_for_reflect(&self) -> &i64 {
        &self.last_block_height
    }

    fn mut_last_block_height_for_reflect(&mut self) -> &mut i64 {
        &mut self.last_block_height
    }

    // bytes last_block_app_hash = 4;

    pub fn clear_last_block_app_hash(&mut self) {
        self.last_block_app_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_last_block_app_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.last_block_app_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_block_app_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.last_block_app_hash
    }

    // Take field
    pub fn take_last_block_app_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.last_block_app_hash, ::std::vec::Vec::new())
    }

    pub fn get_last_block_app_hash(&self) -> &[u8] {
        &self.last_block_app_hash
    }

    fn get_last_block_app_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.last_block_app_hash
    }

    fn mut_last_block_app_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.last_block_app_hash
    }
}

impl ::protobuf::Message for ResponseInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.data)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.last_block_height = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.last_block_app_hash)?;
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.data);
        }
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.version);
        }
        if self.last_block_height != 0 {
            my_size += ::protobuf::rt::value_size(3, self.last_block_height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.last_block_app_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.last_block_app_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.data.is_empty() {
            os.write_string(1, &self.data)?;
        }
        if !self.version.is_empty() {
            os.write_string(2, &self.version)?;
        }
        if self.last_block_height != 0 {
            os.write_int64(3, self.last_block_height)?;
        }
        if !self.last_block_app_hash.is_empty() {
            os.write_bytes(4, &self.last_block_app_hash)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseInfo {
    fn new() -> ResponseInfo {
        ResponseInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data",
                    ResponseInfo::get_data_for_reflect,
                    ResponseInfo::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    ResponseInfo::get_version_for_reflect,
                    ResponseInfo::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "last_block_height",
                    ResponseInfo::get_last_block_height_for_reflect,
                    ResponseInfo::mut_last_block_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "last_block_app_hash",
                    ResponseInfo::get_last_block_app_hash_for_reflect,
                    ResponseInfo::mut_last_block_app_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseInfo>(
                    "ResponseInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseInfo {
    fn clear(&mut self) {
        self.clear_data();
        self.clear_version();
        self.clear_last_block_height();
        self.clear_last_block_app_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseSetOption {
    // message fields
    pub code: u32,
    pub log: ::std::string::String,
    pub info: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseSetOption {}

impl ResponseSetOption {
    pub fn new() -> ResponseSetOption {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseSetOption {
        static mut instance: ::protobuf::lazy::Lazy<ResponseSetOption> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseSetOption,
        };
        unsafe {
            instance.get(ResponseSetOption::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string log = 3;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::std::string::String) {
        self.log = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.log, ::std::string::String::new())
    }

    pub fn get_log(&self) -> &str {
        &self.log
    }

    fn get_log_for_reflect(&self) -> &::std::string::String {
        &self.log
    }

    fn mut_log_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // string info = 4;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ::std::string::String) {
        self.info = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut ::std::string::String {
        &mut self.info
    }

    // Take field
    pub fn take_info(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.info, ::std::string::String::new())
    }

    pub fn get_info(&self) -> &str {
        &self.info
    }

    fn get_info_for_reflect(&self) -> &::std::string::String {
        &self.info
    }

    fn mut_info_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.info
    }
}

impl ::protobuf::Message for ResponseSetOption {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.log)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.info)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.log.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.log);
        }
        if !self.info.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.info);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.log.is_empty() {
            os.write_string(3, &self.log)?;
        }
        if !self.info.is_empty() {
            os.write_string(4, &self.info)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseSetOption {
    fn new() -> ResponseSetOption {
        ResponseSetOption::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseSetOption>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    ResponseSetOption::get_code_for_reflect,
                    ResponseSetOption::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "log",
                    ResponseSetOption::get_log_for_reflect,
                    ResponseSetOption::mut_log_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "info",
                    ResponseSetOption::get_info_for_reflect,
                    ResponseSetOption::mut_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseSetOption>(
                    "ResponseSetOption",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseSetOption {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_log();
        self.clear_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseSetOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseSetOption {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseInitChain {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseInitChain {}

impl ResponseInitChain {
    pub fn new() -> ResponseInitChain {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseInitChain {
        static mut instance: ::protobuf::lazy::Lazy<ResponseInitChain> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseInitChain,
        };
        unsafe {
            instance.get(ResponseInitChain::new)
        }
    }
}

impl ::protobuf::Message for ResponseInitChain {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseInitChain {
    fn new() -> ResponseInitChain {
        ResponseInitChain::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseInitChain>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ResponseInitChain>(
                    "ResponseInitChain",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseInitChain {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseInitChain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseInitChain {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseQuery {
    // message fields
    pub code: u32,
    pub log: ::std::string::String,
    pub info: ::std::string::String,
    pub index: i64,
    pub key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    pub proof: ::std::vec::Vec<u8>,
    pub height: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseQuery {}

impl ResponseQuery {
    pub fn new() -> ResponseQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseQuery {
        static mut instance: ::protobuf::lazy::Lazy<ResponseQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseQuery,
        };
        unsafe {
            instance.get(ResponseQuery::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // string log = 3;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::std::string::String) {
        self.log = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.log, ::std::string::String::new())
    }

    pub fn get_log(&self) -> &str {
        &self.log
    }

    fn get_log_for_reflect(&self) -> &::std::string::String {
        &self.log
    }

    fn mut_log_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // string info = 4;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ::std::string::String) {
        self.info = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut ::std::string::String {
        &mut self.info
    }

    // Take field
    pub fn take_info(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.info, ::std::string::String::new())
    }

    pub fn get_info(&self) -> &str {
        &self.info
    }

    fn get_info_for_reflect(&self) -> &::std::string::String {
        &self.info
    }

    fn mut_info_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.info
    }

    // int64 index = 5;

    pub fn clear_index(&mut self) {
        self.index = 0;
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: i64) {
        self.index = v;
    }

    pub fn get_index(&self) -> i64 {
        self.index
    }

    fn get_index_for_reflect(&self) -> &i64 {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut i64 {
        &mut self.index
    }

    // bytes key = 6;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // bytes value = 7;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // bytes proof = 8;

    pub fn clear_proof(&mut self) {
        self.proof.clear();
    }

    // Param is passed by value, moved
    pub fn set_proof(&mut self, v: ::std::vec::Vec<u8>) {
        self.proof = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_proof(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.proof
    }

    // Take field
    pub fn take_proof(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.proof, ::std::vec::Vec::new())
    }

    pub fn get_proof(&self) -> &[u8] {
        &self.proof
    }

    fn get_proof_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.proof
    }

    fn mut_proof_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.proof
    }

    // int64 height = 9;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i64) {
        self.height = v;
    }

    pub fn get_height(&self) -> i64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &i64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut i64 {
        &mut self.height
    }
}

impl ::protobuf::Message for ResponseQuery {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.log)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.info)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.index = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.proof)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.height = tmp;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.log.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.log);
        }
        if !self.info.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.info);
        }
        if self.index != 0 {
            my_size += ::protobuf::rt::value_size(5, self.index, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.value);
        }
        if !self.proof.is_empty() {
            my_size += ::protobuf::rt::bytes_size(8, &self.proof);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(9, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.log.is_empty() {
            os.write_string(3, &self.log)?;
        }
        if !self.info.is_empty() {
            os.write_string(4, &self.info)?;
        }
        if self.index != 0 {
            os.write_int64(5, self.index)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(6, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(7, &self.value)?;
        }
        if !self.proof.is_empty() {
            os.write_bytes(8, &self.proof)?;
        }
        if self.height != 0 {
            os.write_int64(9, self.height)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseQuery {
    fn new() -> ResponseQuery {
        ResponseQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    ResponseQuery::get_code_for_reflect,
                    ResponseQuery::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "log",
                    ResponseQuery::get_log_for_reflect,
                    ResponseQuery::mut_log_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "info",
                    ResponseQuery::get_info_for_reflect,
                    ResponseQuery::mut_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "index",
                    ResponseQuery::get_index_for_reflect,
                    ResponseQuery::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    ResponseQuery::get_key_for_reflect,
                    ResponseQuery::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    ResponseQuery::get_value_for_reflect,
                    ResponseQuery::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "proof",
                    ResponseQuery::get_proof_for_reflect,
                    ResponseQuery::mut_proof_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "height",
                    ResponseQuery::get_height_for_reflect,
                    ResponseQuery::mut_height_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseQuery>(
                    "ResponseQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseQuery {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_log();
        self.clear_info();
        self.clear_index();
        self.clear_key();
        self.clear_value();
        self.clear_proof();
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseBeginBlock {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseBeginBlock {}

impl ResponseBeginBlock {
    pub fn new() -> ResponseBeginBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseBeginBlock {
        static mut instance: ::protobuf::lazy::Lazy<ResponseBeginBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseBeginBlock,
        };
        unsafe {
            instance.get(ResponseBeginBlock::new)
        }
    }
}

impl ::protobuf::Message for ResponseBeginBlock {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseBeginBlock {
    fn new() -> ResponseBeginBlock {
        ResponseBeginBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseBeginBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ResponseBeginBlock>(
                    "ResponseBeginBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseBeginBlock {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseBeginBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseBeginBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseCheckTx {
    // message fields
    pub code: u32,
    pub data: ::std::vec::Vec<u8>,
    pub log: ::std::string::String,
    pub info: ::std::string::String,
    pub gas_wanted: i64,
    pub gas_used: i64,
    pub tags: ::protobuf::RepeatedField<KVPair>,
    pub fee: ::protobuf::SingularPtrField<KI64Pair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseCheckTx {}

impl ResponseCheckTx {
    pub fn new() -> ResponseCheckTx {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseCheckTx {
        static mut instance: ::protobuf::lazy::Lazy<ResponseCheckTx> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseCheckTx,
        };
        unsafe {
            instance.get(ResponseCheckTx::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // string log = 3;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::std::string::String) {
        self.log = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.log, ::std::string::String::new())
    }

    pub fn get_log(&self) -> &str {
        &self.log
    }

    fn get_log_for_reflect(&self) -> &::std::string::String {
        &self.log
    }

    fn mut_log_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // string info = 4;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ::std::string::String) {
        self.info = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut ::std::string::String {
        &mut self.info
    }

    // Take field
    pub fn take_info(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.info, ::std::string::String::new())
    }

    pub fn get_info(&self) -> &str {
        &self.info
    }

    fn get_info_for_reflect(&self) -> &::std::string::String {
        &self.info
    }

    fn mut_info_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.info
    }

    // int64 gas_wanted = 5;

    pub fn clear_gas_wanted(&mut self) {
        self.gas_wanted = 0;
    }

    // Param is passed by value, moved
    pub fn set_gas_wanted(&mut self, v: i64) {
        self.gas_wanted = v;
    }

    pub fn get_gas_wanted(&self) -> i64 {
        self.gas_wanted
    }

    fn get_gas_wanted_for_reflect(&self) -> &i64 {
        &self.gas_wanted
    }

    fn mut_gas_wanted_for_reflect(&mut self) -> &mut i64 {
        &mut self.gas_wanted
    }

    // int64 gas_used = 6;

    pub fn clear_gas_used(&mut self) {
        self.gas_used = 0;
    }

    // Param is passed by value, moved
    pub fn set_gas_used(&mut self, v: i64) {
        self.gas_used = v;
    }

    pub fn get_gas_used(&self) -> i64 {
        self.gas_used
    }

    fn get_gas_used_for_reflect(&self) -> &i64 {
        &self.gas_used
    }

    fn mut_gas_used_for_reflect(&mut self) -> &mut i64 {
        &mut self.gas_used
    }

    // repeated .types.KVPair tags = 7;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<KVPair>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::protobuf::RepeatedField<KVPair> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<KVPair> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags(&self) -> &[KVPair] {
        &self.tags
    }

    fn get_tags_for_reflect(&self) -> &::protobuf::RepeatedField<KVPair> {
        &self.tags
    }

    fn mut_tags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<KVPair> {
        &mut self.tags
    }

    // .types.KI64Pair fee = 8;

    pub fn clear_fee(&mut self) {
        self.fee.clear();
    }

    pub fn has_fee(&self) -> bool {
        self.fee.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fee(&mut self, v: KI64Pair) {
        self.fee = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fee(&mut self) -> &mut KI64Pair {
        if self.fee.is_none() {
            self.fee.set_default();
        }
        self.fee.as_mut().unwrap()
    }

    // Take field
    pub fn take_fee(&mut self) -> KI64Pair {
        self.fee.take().unwrap_or_else(|| KI64Pair::new())
    }

    pub fn get_fee(&self) -> &KI64Pair {
        self.fee.as_ref().unwrap_or_else(|| KI64Pair::default_instance())
    }

    fn get_fee_for_reflect(&self) -> &::protobuf::SingularPtrField<KI64Pair> {
        &self.fee
    }

    fn mut_fee_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KI64Pair> {
        &mut self.fee
    }
}

impl ::protobuf::Message for ResponseCheckTx {
    fn is_initialized(&self) -> bool {
        for v in &self.tags {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fee {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.log)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.info)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.gas_wanted = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.gas_used = tmp;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tags)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fee)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        if !self.log.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.log);
        }
        if !self.info.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.info);
        }
        if self.gas_wanted != 0 {
            my_size += ::protobuf::rt::value_size(5, self.gas_wanted, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.gas_used != 0 {
            my_size += ::protobuf::rt::value_size(6, self.gas_used, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tags {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.fee.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        }
        if !self.log.is_empty() {
            os.write_string(3, &self.log)?;
        }
        if !self.info.is_empty() {
            os.write_string(4, &self.info)?;
        }
        if self.gas_wanted != 0 {
            os.write_int64(5, self.gas_wanted)?;
        }
        if self.gas_used != 0 {
            os.write_int64(6, self.gas_used)?;
        }
        for v in &self.tags {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.fee.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseCheckTx {
    fn new() -> ResponseCheckTx {
        ResponseCheckTx::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseCheckTx>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    ResponseCheckTx::get_code_for_reflect,
                    ResponseCheckTx::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ResponseCheckTx::get_data_for_reflect,
                    ResponseCheckTx::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "log",
                    ResponseCheckTx::get_log_for_reflect,
                    ResponseCheckTx::mut_log_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "info",
                    ResponseCheckTx::get_info_for_reflect,
                    ResponseCheckTx::mut_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "gas_wanted",
                    ResponseCheckTx::get_gas_wanted_for_reflect,
                    ResponseCheckTx::mut_gas_wanted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "gas_used",
                    ResponseCheckTx::get_gas_used_for_reflect,
                    ResponseCheckTx::mut_gas_used_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KVPair>>(
                    "tags",
                    ResponseCheckTx::get_tags_for_reflect,
                    ResponseCheckTx::mut_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KI64Pair>>(
                    "fee",
                    ResponseCheckTx::get_fee_for_reflect,
                    ResponseCheckTx::mut_fee_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseCheckTx>(
                    "ResponseCheckTx",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseCheckTx {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_data();
        self.clear_log();
        self.clear_info();
        self.clear_gas_wanted();
        self.clear_gas_used();
        self.clear_tags();
        self.clear_fee();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseCheckTx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseCheckTx {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseDeliverTx {
    // message fields
    pub code: u32,
    pub data: ::std::vec::Vec<u8>,
    pub log: ::std::string::String,
    pub info: ::std::string::String,
    pub gas_wanted: i64,
    pub gas_used: i64,
    pub tags: ::protobuf::RepeatedField<KVPair>,
    pub fee: ::protobuf::SingularPtrField<KI64Pair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseDeliverTx {}

impl ResponseDeliverTx {
    pub fn new() -> ResponseDeliverTx {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseDeliverTx {
        static mut instance: ::protobuf::lazy::Lazy<ResponseDeliverTx> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseDeliverTx,
        };
        unsafe {
            instance.get(ResponseDeliverTx::new)
        }
    }

    // uint32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: u32) {
        self.code = v;
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &u32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut u32 {
        &mut self.code
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // string log = 3;

    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    // Param is passed by value, moved
    pub fn set_log(&mut self, v: ::std::string::String) {
        self.log = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // Take field
    pub fn take_log(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.log, ::std::string::String::new())
    }

    pub fn get_log(&self) -> &str {
        &self.log
    }

    fn get_log_for_reflect(&self) -> &::std::string::String {
        &self.log
    }

    fn mut_log_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.log
    }

    // string info = 4;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ::std::string::String) {
        self.info = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut ::std::string::String {
        &mut self.info
    }

    // Take field
    pub fn take_info(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.info, ::std::string::String::new())
    }

    pub fn get_info(&self) -> &str {
        &self.info
    }

    fn get_info_for_reflect(&self) -> &::std::string::String {
        &self.info
    }

    fn mut_info_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.info
    }

    // int64 gas_wanted = 5;

    pub fn clear_gas_wanted(&mut self) {
        self.gas_wanted = 0;
    }

    // Param is passed by value, moved
    pub fn set_gas_wanted(&mut self, v: i64) {
        self.gas_wanted = v;
    }

    pub fn get_gas_wanted(&self) -> i64 {
        self.gas_wanted
    }

    fn get_gas_wanted_for_reflect(&self) -> &i64 {
        &self.gas_wanted
    }

    fn mut_gas_wanted_for_reflect(&mut self) -> &mut i64 {
        &mut self.gas_wanted
    }

    // int64 gas_used = 6;

    pub fn clear_gas_used(&mut self) {
        self.gas_used = 0;
    }

    // Param is passed by value, moved
    pub fn set_gas_used(&mut self, v: i64) {
        self.gas_used = v;
    }

    pub fn get_gas_used(&self) -> i64 {
        self.gas_used
    }

    fn get_gas_used_for_reflect(&self) -> &i64 {
        &self.gas_used
    }

    fn mut_gas_used_for_reflect(&mut self) -> &mut i64 {
        &mut self.gas_used
    }

    // repeated .types.KVPair tags = 7;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<KVPair>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::protobuf::RepeatedField<KVPair> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<KVPair> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags(&self) -> &[KVPair] {
        &self.tags
    }

    fn get_tags_for_reflect(&self) -> &::protobuf::RepeatedField<KVPair> {
        &self.tags
    }

    fn mut_tags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<KVPair> {
        &mut self.tags
    }

    // .types.KI64Pair fee = 8;

    pub fn clear_fee(&mut self) {
        self.fee.clear();
    }

    pub fn has_fee(&self) -> bool {
        self.fee.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fee(&mut self, v: KI64Pair) {
        self.fee = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fee(&mut self) -> &mut KI64Pair {
        if self.fee.is_none() {
            self.fee.set_default();
        }
        self.fee.as_mut().unwrap()
    }

    // Take field
    pub fn take_fee(&mut self) -> KI64Pair {
        self.fee.take().unwrap_or_else(|| KI64Pair::new())
    }

    pub fn get_fee(&self) -> &KI64Pair {
        self.fee.as_ref().unwrap_or_else(|| KI64Pair::default_instance())
    }

    fn get_fee_for_reflect(&self) -> &::protobuf::SingularPtrField<KI64Pair> {
        &self.fee
    }

    fn mut_fee_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KI64Pair> {
        &mut self.fee
    }
}

impl ::protobuf::Message for ResponseDeliverTx {
    fn is_initialized(&self) -> bool {
        for v in &self.tags {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fee {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.log)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.info)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.gas_wanted = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.gas_used = tmp;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tags)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fee)?;
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
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        if !self.log.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.log);
        }
        if !self.info.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.info);
        }
        if self.gas_wanted != 0 {
            my_size += ::protobuf::rt::value_size(5, self.gas_wanted, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.gas_used != 0 {
            my_size += ::protobuf::rt::value_size(6, self.gas_used, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tags {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.fee.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_uint32(1, self.code)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        }
        if !self.log.is_empty() {
            os.write_string(3, &self.log)?;
        }
        if !self.info.is_empty() {
            os.write_string(4, &self.info)?;
        }
        if self.gas_wanted != 0 {
            os.write_int64(5, self.gas_wanted)?;
        }
        if self.gas_used != 0 {
            os.write_int64(6, self.gas_used)?;
        }
        for v in &self.tags {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.fee.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseDeliverTx {
    fn new() -> ResponseDeliverTx {
        ResponseDeliverTx::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseDeliverTx>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code",
                    ResponseDeliverTx::get_code_for_reflect,
                    ResponseDeliverTx::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ResponseDeliverTx::get_data_for_reflect,
                    ResponseDeliverTx::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "log",
                    ResponseDeliverTx::get_log_for_reflect,
                    ResponseDeliverTx::mut_log_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "info",
                    ResponseDeliverTx::get_info_for_reflect,
                    ResponseDeliverTx::mut_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "gas_wanted",
                    ResponseDeliverTx::get_gas_wanted_for_reflect,
                    ResponseDeliverTx::mut_gas_wanted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "gas_used",
                    ResponseDeliverTx::get_gas_used_for_reflect,
                    ResponseDeliverTx::mut_gas_used_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KVPair>>(
                    "tags",
                    ResponseDeliverTx::get_tags_for_reflect,
                    ResponseDeliverTx::mut_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KI64Pair>>(
                    "fee",
                    ResponseDeliverTx::get_fee_for_reflect,
                    ResponseDeliverTx::mut_fee_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseDeliverTx>(
                    "ResponseDeliverTx",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseDeliverTx {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_data();
        self.clear_log();
        self.clear_info();
        self.clear_gas_wanted();
        self.clear_gas_used();
        self.clear_tags();
        self.clear_fee();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseDeliverTx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseDeliverTx {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseEndBlock {
    // message fields
    pub validator_updates: ::protobuf::RepeatedField<Validator>,
    pub consensus_param_updates: ::protobuf::SingularPtrField<ConsensusParams>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseEndBlock {}

impl ResponseEndBlock {
    pub fn new() -> ResponseEndBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseEndBlock {
        static mut instance: ::protobuf::lazy::Lazy<ResponseEndBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseEndBlock,
        };
        unsafe {
            instance.get(ResponseEndBlock::new)
        }
    }

    // repeated .types.Validator validator_updates = 1;

    pub fn clear_validator_updates(&mut self) {
        self.validator_updates.clear();
    }

    // Param is passed by value, moved
    pub fn set_validator_updates(&mut self, v: ::protobuf::RepeatedField<Validator>) {
        self.validator_updates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_validator_updates(&mut self) -> &mut ::protobuf::RepeatedField<Validator> {
        &mut self.validator_updates
    }

    // Take field
    pub fn take_validator_updates(&mut self) -> ::protobuf::RepeatedField<Validator> {
        ::std::mem::replace(&mut self.validator_updates, ::protobuf::RepeatedField::new())
    }

    pub fn get_validator_updates(&self) -> &[Validator] {
        &self.validator_updates
    }

    fn get_validator_updates_for_reflect(&self) -> &::protobuf::RepeatedField<Validator> {
        &self.validator_updates
    }

    fn mut_validator_updates_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Validator> {
        &mut self.validator_updates
    }

    // .types.ConsensusParams consensus_param_updates = 2;

    pub fn clear_consensus_param_updates(&mut self) {
        self.consensus_param_updates.clear();
    }

    pub fn has_consensus_param_updates(&self) -> bool {
        self.consensus_param_updates.is_some()
    }

    // Param is passed by value, moved
    pub fn set_consensus_param_updates(&mut self, v: ConsensusParams) {
        self.consensus_param_updates = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_consensus_param_updates(&mut self) -> &mut ConsensusParams {
        if self.consensus_param_updates.is_none() {
            self.consensus_param_updates.set_default();
        }
        self.consensus_param_updates.as_mut().unwrap()
    }

    // Take field
    pub fn take_consensus_param_updates(&mut self) -> ConsensusParams {
        self.consensus_param_updates.take().unwrap_or_else(|| ConsensusParams::new())
    }

    pub fn get_consensus_param_updates(&self) -> &ConsensusParams {
        self.consensus_param_updates.as_ref().unwrap_or_else(|| ConsensusParams::default_instance())
    }

    fn get_consensus_param_updates_for_reflect(&self) -> &::protobuf::SingularPtrField<ConsensusParams> {
        &self.consensus_param_updates
    }

    fn mut_consensus_param_updates_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ConsensusParams> {
        &mut self.consensus_param_updates
    }
}

impl ::protobuf::Message for ResponseEndBlock {
    fn is_initialized(&self) -> bool {
        for v in &self.validator_updates {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.consensus_param_updates {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.validator_updates)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.consensus_param_updates)?;
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
        for value in &self.validator_updates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.consensus_param_updates.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.validator_updates {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.consensus_param_updates.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseEndBlock {
    fn new() -> ResponseEndBlock {
        ResponseEndBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseEndBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Validator>>(
                    "validator_updates",
                    ResponseEndBlock::get_validator_updates_for_reflect,
                    ResponseEndBlock::mut_validator_updates_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ConsensusParams>>(
                    "consensus_param_updates",
                    ResponseEndBlock::get_consensus_param_updates_for_reflect,
                    ResponseEndBlock::mut_consensus_param_updates_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseEndBlock>(
                    "ResponseEndBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseEndBlock {
    fn clear(&mut self) {
        self.clear_validator_updates();
        self.clear_consensus_param_updates();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseEndBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseEndBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseCommit {
    // message fields
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseCommit {}

impl ResponseCommit {
    pub fn new() -> ResponseCommit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseCommit {
        static mut instance: ::protobuf::lazy::Lazy<ResponseCommit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseCommit,
        };
        unsafe {
            instance.get(ResponseCommit::new)
        }
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for ResponseCommit {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseCommit {
    fn new() -> ResponseCommit {
        ResponseCommit::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseCommit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ResponseCommit::get_data_for_reflect,
                    ResponseCommit::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseCommit>(
                    "ResponseCommit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseCommit {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseCommit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseCommit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConsensusParams {
    // message fields
    pub block_size: ::protobuf::SingularPtrField<BlockSize>,
    pub tx_size: ::protobuf::SingularPtrField<TxSize>,
    pub block_gossip: ::protobuf::SingularPtrField<BlockGossip>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConsensusParams {}

impl ConsensusParams {
    pub fn new() -> ConsensusParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConsensusParams {
        static mut instance: ::protobuf::lazy::Lazy<ConsensusParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConsensusParams,
        };
        unsafe {
            instance.get(ConsensusParams::new)
        }
    }

    // .types.BlockSize block_size = 1;

    pub fn clear_block_size(&mut self) {
        self.block_size.clear();
    }

    pub fn has_block_size(&self) -> bool {
        self.block_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block_size(&mut self, v: BlockSize) {
        self.block_size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_size(&mut self) -> &mut BlockSize {
        if self.block_size.is_none() {
            self.block_size.set_default();
        }
        self.block_size.as_mut().unwrap()
    }

    // Take field
    pub fn take_block_size(&mut self) -> BlockSize {
        self.block_size.take().unwrap_or_else(|| BlockSize::new())
    }

    pub fn get_block_size(&self) -> &BlockSize {
        self.block_size.as_ref().unwrap_or_else(|| BlockSize::default_instance())
    }

    fn get_block_size_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockSize> {
        &self.block_size
    }

    fn mut_block_size_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockSize> {
        &mut self.block_size
    }

    // .types.TxSize tx_size = 2;

    pub fn clear_tx_size(&mut self) {
        self.tx_size.clear();
    }

    pub fn has_tx_size(&self) -> bool {
        self.tx_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tx_size(&mut self, v: TxSize) {
        self.tx_size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx_size(&mut self) -> &mut TxSize {
        if self.tx_size.is_none() {
            self.tx_size.set_default();
        }
        self.tx_size.as_mut().unwrap()
    }

    // Take field
    pub fn take_tx_size(&mut self) -> TxSize {
        self.tx_size.take().unwrap_or_else(|| TxSize::new())
    }

    pub fn get_tx_size(&self) -> &TxSize {
        self.tx_size.as_ref().unwrap_or_else(|| TxSize::default_instance())
    }

    fn get_tx_size_for_reflect(&self) -> &::protobuf::SingularPtrField<TxSize> {
        &self.tx_size
    }

    fn mut_tx_size_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TxSize> {
        &mut self.tx_size
    }

    // .types.BlockGossip block_gossip = 3;

    pub fn clear_block_gossip(&mut self) {
        self.block_gossip.clear();
    }

    pub fn has_block_gossip(&self) -> bool {
        self.block_gossip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block_gossip(&mut self, v: BlockGossip) {
        self.block_gossip = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_gossip(&mut self) -> &mut BlockGossip {
        if self.block_gossip.is_none() {
            self.block_gossip.set_default();
        }
        self.block_gossip.as_mut().unwrap()
    }

    // Take field
    pub fn take_block_gossip(&mut self) -> BlockGossip {
        self.block_gossip.take().unwrap_or_else(|| BlockGossip::new())
    }

    pub fn get_block_gossip(&self) -> &BlockGossip {
        self.block_gossip.as_ref().unwrap_or_else(|| BlockGossip::default_instance())
    }

    fn get_block_gossip_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockGossip> {
        &self.block_gossip
    }

    fn mut_block_gossip_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockGossip> {
        &mut self.block_gossip
    }
}

impl ::protobuf::Message for ConsensusParams {
    fn is_initialized(&self) -> bool {
        for v in &self.block_size {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tx_size {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.block_gossip {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.block_size)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tx_size)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.block_gossip)?;
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
        if let Some(ref v) = self.block_size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.tx_size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.block_gossip.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.block_size.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.tx_size.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.block_gossip.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConsensusParams {
    fn new() -> ConsensusParams {
        ConsensusParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConsensusParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockSize>>(
                    "block_size",
                    ConsensusParams::get_block_size_for_reflect,
                    ConsensusParams::mut_block_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TxSize>>(
                    "tx_size",
                    ConsensusParams::get_tx_size_for_reflect,
                    ConsensusParams::mut_tx_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockGossip>>(
                    "block_gossip",
                    ConsensusParams::get_block_gossip_for_reflect,
                    ConsensusParams::mut_block_gossip_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConsensusParams>(
                    "ConsensusParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConsensusParams {
    fn clear(&mut self) {
        self.clear_block_size();
        self.clear_tx_size();
        self.clear_block_gossip();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConsensusParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConsensusParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockSize {
    // message fields
    pub max_bytes: i32,
    pub max_txs: i32,
    pub max_gas: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockSize {}

impl BlockSize {
    pub fn new() -> BlockSize {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockSize {
        static mut instance: ::protobuf::lazy::Lazy<BlockSize> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockSize,
        };
        unsafe {
            instance.get(BlockSize::new)
        }
    }

    // int32 max_bytes = 1;

    pub fn clear_max_bytes(&mut self) {
        self.max_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_bytes(&mut self, v: i32) {
        self.max_bytes = v;
    }

    pub fn get_max_bytes(&self) -> i32 {
        self.max_bytes
    }

    fn get_max_bytes_for_reflect(&self) -> &i32 {
        &self.max_bytes
    }

    fn mut_max_bytes_for_reflect(&mut self) -> &mut i32 {
        &mut self.max_bytes
    }

    // int32 max_txs = 2;

    pub fn clear_max_txs(&mut self) {
        self.max_txs = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_txs(&mut self, v: i32) {
        self.max_txs = v;
    }

    pub fn get_max_txs(&self) -> i32 {
        self.max_txs
    }

    fn get_max_txs_for_reflect(&self) -> &i32 {
        &self.max_txs
    }

    fn mut_max_txs_for_reflect(&mut self) -> &mut i32 {
        &mut self.max_txs
    }

    // int64 max_gas = 3;

    pub fn clear_max_gas(&mut self) {
        self.max_gas = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_gas(&mut self, v: i64) {
        self.max_gas = v;
    }

    pub fn get_max_gas(&self) -> i64 {
        self.max_gas
    }

    fn get_max_gas_for_reflect(&self) -> &i64 {
        &self.max_gas
    }

    fn mut_max_gas_for_reflect(&mut self) -> &mut i64 {
        &mut self.max_gas
    }
}

impl ::protobuf::Message for BlockSize {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max_bytes = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max_txs = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.max_gas = tmp;
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
        if self.max_bytes != 0 {
            my_size += ::protobuf::rt::value_size(1, self.max_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.max_txs != 0 {
            my_size += ::protobuf::rt::value_size(2, self.max_txs, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.max_gas != 0 {
            my_size += ::protobuf::rt::value_size(3, self.max_gas, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.max_bytes != 0 {
            os.write_int32(1, self.max_bytes)?;
        }
        if self.max_txs != 0 {
            os.write_int32(2, self.max_txs)?;
        }
        if self.max_gas != 0 {
            os.write_int64(3, self.max_gas)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlockSize {
    fn new() -> BlockSize {
        BlockSize::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockSize>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_bytes",
                    BlockSize::get_max_bytes_for_reflect,
                    BlockSize::mut_max_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_txs",
                    BlockSize::get_max_txs_for_reflect,
                    BlockSize::mut_max_txs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "max_gas",
                    BlockSize::get_max_gas_for_reflect,
                    BlockSize::mut_max_gas_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockSize>(
                    "BlockSize",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockSize {
    fn clear(&mut self) {
        self.clear_max_bytes();
        self.clear_max_txs();
        self.clear_max_gas();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockSize {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TxSize {
    // message fields
    pub max_bytes: i32,
    pub max_gas: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TxSize {}

impl TxSize {
    pub fn new() -> TxSize {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TxSize {
        static mut instance: ::protobuf::lazy::Lazy<TxSize> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TxSize,
        };
        unsafe {
            instance.get(TxSize::new)
        }
    }

    // int32 max_bytes = 1;

    pub fn clear_max_bytes(&mut self) {
        self.max_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_bytes(&mut self, v: i32) {
        self.max_bytes = v;
    }

    pub fn get_max_bytes(&self) -> i32 {
        self.max_bytes
    }

    fn get_max_bytes_for_reflect(&self) -> &i32 {
        &self.max_bytes
    }

    fn mut_max_bytes_for_reflect(&mut self) -> &mut i32 {
        &mut self.max_bytes
    }

    // int64 max_gas = 2;

    pub fn clear_max_gas(&mut self) {
        self.max_gas = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_gas(&mut self, v: i64) {
        self.max_gas = v;
    }

    pub fn get_max_gas(&self) -> i64 {
        self.max_gas
    }

    fn get_max_gas_for_reflect(&self) -> &i64 {
        &self.max_gas
    }

    fn mut_max_gas_for_reflect(&mut self) -> &mut i64 {
        &mut self.max_gas
    }
}

impl ::protobuf::Message for TxSize {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max_bytes = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.max_gas = tmp;
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
        if self.max_bytes != 0 {
            my_size += ::protobuf::rt::value_size(1, self.max_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.max_gas != 0 {
            my_size += ::protobuf::rt::value_size(2, self.max_gas, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.max_bytes != 0 {
            os.write_int32(1, self.max_bytes)?;
        }
        if self.max_gas != 0 {
            os.write_int64(2, self.max_gas)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TxSize {
    fn new() -> TxSize {
        TxSize::new()
    }

    fn descriptor_static(_: ::std::option::Option<TxSize>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_bytes",
                    TxSize::get_max_bytes_for_reflect,
                    TxSize::mut_max_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "max_gas",
                    TxSize::get_max_gas_for_reflect,
                    TxSize::mut_max_gas_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TxSize>(
                    "TxSize",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TxSize {
    fn clear(&mut self) {
        self.clear_max_bytes();
        self.clear_max_gas();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TxSize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TxSize {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockGossip {
    // message fields
    pub block_part_size_bytes: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockGossip {}

impl BlockGossip {
    pub fn new() -> BlockGossip {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockGossip {
        static mut instance: ::protobuf::lazy::Lazy<BlockGossip> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockGossip,
        };
        unsafe {
            instance.get(BlockGossip::new)
        }
    }

    // int32 block_part_size_bytes = 1;

    pub fn clear_block_part_size_bytes(&mut self) {
        self.block_part_size_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_block_part_size_bytes(&mut self, v: i32) {
        self.block_part_size_bytes = v;
    }

    pub fn get_block_part_size_bytes(&self) -> i32 {
        self.block_part_size_bytes
    }

    fn get_block_part_size_bytes_for_reflect(&self) -> &i32 {
        &self.block_part_size_bytes
    }

    fn mut_block_part_size_bytes_for_reflect(&mut self) -> &mut i32 {
        &mut self.block_part_size_bytes
    }
}

impl ::protobuf::Message for BlockGossip {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.block_part_size_bytes = tmp;
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
        if self.block_part_size_bytes != 0 {
            my_size += ::protobuf::rt::value_size(1, self.block_part_size_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.block_part_size_bytes != 0 {
            os.write_int32(1, self.block_part_size_bytes)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlockGossip {
    fn new() -> BlockGossip {
        BlockGossip::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockGossip>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "block_part_size_bytes",
                    BlockGossip::get_block_part_size_bytes_for_reflect,
                    BlockGossip::mut_block_part_size_bytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockGossip>(
                    "BlockGossip",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockGossip {
    fn clear(&mut self) {
        self.clear_block_part_size_bytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockGossip {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockGossip {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Header {
    // message fields
    pub chain_id: ::std::string::String,
    pub height: i64,
    pub time: i64,
    pub num_txs: i32,
    pub last_block_id: ::protobuf::SingularPtrField<BlockID>,
    pub last_commit_hash: ::std::vec::Vec<u8>,
    pub data_hash: ::std::vec::Vec<u8>,
    pub validators_hash: ::std::vec::Vec<u8>,
    pub app_hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Header {}

impl Header {
    pub fn new() -> Header {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Header {
        static mut instance: ::protobuf::lazy::Lazy<Header> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Header,
        };
        unsafe {
            instance.get(Header::new)
        }
    }

    // string chain_id = 1;

    pub fn clear_chain_id(&mut self) {
        self.chain_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_chain_id(&mut self, v: ::std::string::String) {
        self.chain_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chain_id(&mut self) -> &mut ::std::string::String {
        &mut self.chain_id
    }

    // Take field
    pub fn take_chain_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.chain_id, ::std::string::String::new())
    }

    pub fn get_chain_id(&self) -> &str {
        &self.chain_id
    }

    fn get_chain_id_for_reflect(&self) -> &::std::string::String {
        &self.chain_id
    }

    fn mut_chain_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.chain_id
    }

    // int64 height = 2;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i64) {
        self.height = v;
    }

    pub fn get_height(&self) -> i64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &i64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut i64 {
        &mut self.height
    }

    // int64 time = 3;

    pub fn clear_time(&mut self) {
        self.time = 0;
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: i64) {
        self.time = v;
    }

    pub fn get_time(&self) -> i64 {
        self.time
    }

    fn get_time_for_reflect(&self) -> &i64 {
        &self.time
    }

    fn mut_time_for_reflect(&mut self) -> &mut i64 {
        &mut self.time
    }

    // int32 num_txs = 4;

    pub fn clear_num_txs(&mut self) {
        self.num_txs = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_txs(&mut self, v: i32) {
        self.num_txs = v;
    }

    pub fn get_num_txs(&self) -> i32 {
        self.num_txs
    }

    fn get_num_txs_for_reflect(&self) -> &i32 {
        &self.num_txs
    }

    fn mut_num_txs_for_reflect(&mut self) -> &mut i32 {
        &mut self.num_txs
    }

    // .types.BlockID last_block_id = 5;

    pub fn clear_last_block_id(&mut self) {
        self.last_block_id.clear();
    }

    pub fn has_last_block_id(&self) -> bool {
        self.last_block_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_block_id(&mut self, v: BlockID) {
        self.last_block_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_block_id(&mut self) -> &mut BlockID {
        if self.last_block_id.is_none() {
            self.last_block_id.set_default();
        }
        self.last_block_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_last_block_id(&mut self) -> BlockID {
        self.last_block_id.take().unwrap_or_else(|| BlockID::new())
    }

    pub fn get_last_block_id(&self) -> &BlockID {
        self.last_block_id.as_ref().unwrap_or_else(|| BlockID::default_instance())
    }

    fn get_last_block_id_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockID> {
        &self.last_block_id
    }

    fn mut_last_block_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockID> {
        &mut self.last_block_id
    }

    // bytes last_commit_hash = 6;

    pub fn clear_last_commit_hash(&mut self) {
        self.last_commit_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_last_commit_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.last_commit_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_commit_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.last_commit_hash
    }

    // Take field
    pub fn take_last_commit_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.last_commit_hash, ::std::vec::Vec::new())
    }

    pub fn get_last_commit_hash(&self) -> &[u8] {
        &self.last_commit_hash
    }

    fn get_last_commit_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.last_commit_hash
    }

    fn mut_last_commit_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.last_commit_hash
    }

    // bytes data_hash = 7;

    pub fn clear_data_hash(&mut self) {
        self.data_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_data_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.data_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data_hash
    }

    // Take field
    pub fn take_data_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data_hash, ::std::vec::Vec::new())
    }

    pub fn get_data_hash(&self) -> &[u8] {
        &self.data_hash
    }

    fn get_data_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data_hash
    }

    fn mut_data_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data_hash
    }

    // bytes validators_hash = 8;

    pub fn clear_validators_hash(&mut self) {
        self.validators_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_validators_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.validators_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_validators_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.validators_hash
    }

    // Take field
    pub fn take_validators_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.validators_hash, ::std::vec::Vec::new())
    }

    pub fn get_validators_hash(&self) -> &[u8] {
        &self.validators_hash
    }

    fn get_validators_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.validators_hash
    }

    fn mut_validators_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.validators_hash
    }

    // bytes app_hash = 9;

    pub fn clear_app_hash(&mut self) {
        self.app_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_app_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.app_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_app_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.app_hash
    }

    // Take field
    pub fn take_app_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.app_hash, ::std::vec::Vec::new())
    }

    pub fn get_app_hash(&self) -> &[u8] {
        &self.app_hash
    }

    fn get_app_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.app_hash
    }

    fn mut_app_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.app_hash
    }
}

impl ::protobuf::Message for Header {
    fn is_initialized(&self) -> bool {
        for v in &self.last_block_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.chain_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.height = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.time = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_txs = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.last_block_id)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.last_commit_hash)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data_hash)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.validators_hash)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.app_hash)?;
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
        if !self.chain_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.chain_id);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.time != 0 {
            my_size += ::protobuf::rt::value_size(3, self.time, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.num_txs != 0 {
            my_size += ::protobuf::rt::value_size(4, self.num_txs, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.last_block_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.last_commit_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.last_commit_hash);
        }
        if !self.data_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.data_hash);
        }
        if !self.validators_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(8, &self.validators_hash);
        }
        if !self.app_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(9, &self.app_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.chain_id.is_empty() {
            os.write_string(1, &self.chain_id)?;
        }
        if self.height != 0 {
            os.write_int64(2, self.height)?;
        }
        if self.time != 0 {
            os.write_int64(3, self.time)?;
        }
        if self.num_txs != 0 {
            os.write_int32(4, self.num_txs)?;
        }
        if let Some(ref v) = self.last_block_id.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.last_commit_hash.is_empty() {
            os.write_bytes(6, &self.last_commit_hash)?;
        }
        if !self.data_hash.is_empty() {
            os.write_bytes(7, &self.data_hash)?;
        }
        if !self.validators_hash.is_empty() {
            os.write_bytes(8, &self.validators_hash)?;
        }
        if !self.app_hash.is_empty() {
            os.write_bytes(9, &self.app_hash)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Header {
    fn new() -> Header {
        Header::new()
    }

    fn descriptor_static(_: ::std::option::Option<Header>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "chain_id",
                    Header::get_chain_id_for_reflect,
                    Header::mut_chain_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "height",
                    Header::get_height_for_reflect,
                    Header::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "time",
                    Header::get_time_for_reflect,
                    Header::mut_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_txs",
                    Header::get_num_txs_for_reflect,
                    Header::mut_num_txs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockID>>(
                    "last_block_id",
                    Header::get_last_block_id_for_reflect,
                    Header::mut_last_block_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "last_commit_hash",
                    Header::get_last_commit_hash_for_reflect,
                    Header::mut_last_commit_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data_hash",
                    Header::get_data_hash_for_reflect,
                    Header::mut_data_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "validators_hash",
                    Header::get_validators_hash_for_reflect,
                    Header::mut_validators_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "app_hash",
                    Header::get_app_hash_for_reflect,
                    Header::mut_app_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Header>(
                    "Header",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Header {
    fn clear(&mut self) {
        self.clear_chain_id();
        self.clear_height();
        self.clear_time();
        self.clear_num_txs();
        self.clear_last_block_id();
        self.clear_last_commit_hash();
        self.clear_data_hash();
        self.clear_validators_hash();
        self.clear_app_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Header {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockID {
    // message fields
    pub hash: ::std::vec::Vec<u8>,
    pub parts: ::protobuf::SingularPtrField<PartSetHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockID {}

impl BlockID {
    pub fn new() -> BlockID {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockID {
        static mut instance: ::protobuf::lazy::Lazy<BlockID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockID,
        };
        unsafe {
            instance.get(BlockID::new)
        }
    }

    // bytes hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // .types.PartSetHeader parts = 2;

    pub fn clear_parts(&mut self) {
        self.parts.clear();
    }

    pub fn has_parts(&self) -> bool {
        self.parts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parts(&mut self, v: PartSetHeader) {
        self.parts = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parts(&mut self) -> &mut PartSetHeader {
        if self.parts.is_none() {
            self.parts.set_default();
        }
        self.parts.as_mut().unwrap()
    }

    // Take field
    pub fn take_parts(&mut self) -> PartSetHeader {
        self.parts.take().unwrap_or_else(|| PartSetHeader::new())
    }

    pub fn get_parts(&self) -> &PartSetHeader {
        self.parts.as_ref().unwrap_or_else(|| PartSetHeader::default_instance())
    }

    fn get_parts_for_reflect(&self) -> &::protobuf::SingularPtrField<PartSetHeader> {
        &self.parts
    }

    fn mut_parts_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PartSetHeader> {
        &mut self.parts
    }
}

impl ::protobuf::Message for BlockID {
    fn is_initialized(&self) -> bool {
        for v in &self.parts {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.parts)?;
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
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.hash);
        }
        if let Some(ref v) = self.parts.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.hash.is_empty() {
            os.write_bytes(1, &self.hash)?;
        }
        if let Some(ref v) = self.parts.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlockID {
    fn new() -> BlockID {
        BlockID::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockID>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    BlockID::get_hash_for_reflect,
                    BlockID::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PartSetHeader>>(
                    "parts",
                    BlockID::get_parts_for_reflect,
                    BlockID::mut_parts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockID>(
                    "BlockID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockID {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_parts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockID {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PartSetHeader {
    // message fields
    pub total: i32,
    pub hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PartSetHeader {}

impl PartSetHeader {
    pub fn new() -> PartSetHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PartSetHeader {
        static mut instance: ::protobuf::lazy::Lazy<PartSetHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PartSetHeader,
        };
        unsafe {
            instance.get(PartSetHeader::new)
        }
    }

    // int32 total = 1;

    pub fn clear_total(&mut self) {
        self.total = 0;
    }

    // Param is passed by value, moved
    pub fn set_total(&mut self, v: i32) {
        self.total = v;
    }

    pub fn get_total(&self) -> i32 {
        self.total
    }

    fn get_total_for_reflect(&self) -> &i32 {
        &self.total
    }

    fn mut_total_for_reflect(&mut self) -> &mut i32 {
        &mut self.total
    }

    // bytes hash = 2;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }
}

impl ::protobuf::Message for PartSetHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.total = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
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
        if self.total != 0 {
            my_size += ::protobuf::rt::value_size(1, self.total, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.total != 0 {
            os.write_int32(1, self.total)?;
        }
        if !self.hash.is_empty() {
            os.write_bytes(2, &self.hash)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PartSetHeader {
    fn new() -> PartSetHeader {
        PartSetHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<PartSetHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "total",
                    PartSetHeader::get_total_for_reflect,
                    PartSetHeader::mut_total_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    PartSetHeader::get_hash_for_reflect,
                    PartSetHeader::mut_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PartSetHeader>(
                    "PartSetHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PartSetHeader {
    fn clear(&mut self) {
        self.clear_total();
        self.clear_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PartSetHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PartSetHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Validator {
    // message fields
    pub pub_key: ::std::vec::Vec<u8>,
    pub power: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Validator {}

impl Validator {
    pub fn new() -> Validator {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Validator {
        static mut instance: ::protobuf::lazy::Lazy<Validator> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Validator,
        };
        unsafe {
            instance.get(Validator::new)
        }
    }

    // bytes pub_key = 1;

    pub fn clear_pub_key(&mut self) {
        self.pub_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_pub_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.pub_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.pub_key
    }

    // Take field
    pub fn take_pub_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.pub_key, ::std::vec::Vec::new())
    }

    pub fn get_pub_key(&self) -> &[u8] {
        &self.pub_key
    }

    fn get_pub_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.pub_key
    }

    fn mut_pub_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.pub_key
    }

    // int64 power = 2;

    pub fn clear_power(&mut self) {
        self.power = 0;
    }

    // Param is passed by value, moved
    pub fn set_power(&mut self, v: i64) {
        self.power = v;
    }

    pub fn get_power(&self) -> i64 {
        self.power
    }

    fn get_power_for_reflect(&self) -> &i64 {
        &self.power
    }

    fn mut_power_for_reflect(&mut self) -> &mut i64 {
        &mut self.power
    }
}

impl ::protobuf::Message for Validator {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.pub_key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.power = tmp;
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
        if !self.pub_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.pub_key);
        }
        if self.power != 0 {
            my_size += ::protobuf::rt::value_size(2, self.power, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.pub_key.is_empty() {
            os.write_bytes(1, &self.pub_key)?;
        }
        if self.power != 0 {
            os.write_int64(2, self.power)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Validator {
    fn new() -> Validator {
        Validator::new()
    }

    fn descriptor_static(_: ::std::option::Option<Validator>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "pub_key",
                    Validator::get_pub_key_for_reflect,
                    Validator::mut_pub_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "power",
                    Validator::get_power_for_reflect,
                    Validator::mut_power_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Validator>(
                    "Validator",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Validator {
    fn clear(&mut self) {
        self.clear_pub_key();
        self.clear_power();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Validator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Validator {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Evidence {
    // message fields
    pub pub_key: ::std::vec::Vec<u8>,
    pub height: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Evidence {}

impl Evidence {
    pub fn new() -> Evidence {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Evidence {
        static mut instance: ::protobuf::lazy::Lazy<Evidence> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Evidence,
        };
        unsafe {
            instance.get(Evidence::new)
        }
    }

    // bytes pub_key = 1;

    pub fn clear_pub_key(&mut self) {
        self.pub_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_pub_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.pub_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.pub_key
    }

    // Take field
    pub fn take_pub_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.pub_key, ::std::vec::Vec::new())
    }

    pub fn get_pub_key(&self) -> &[u8] {
        &self.pub_key
    }

    fn get_pub_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.pub_key
    }

    fn mut_pub_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.pub_key
    }

    // int64 height = 2;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i64) {
        self.height = v;
    }

    pub fn get_height(&self) -> i64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &i64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut i64 {
        &mut self.height
    }
}

impl ::protobuf::Message for Evidence {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.pub_key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.height = tmp;
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
        if !self.pub_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.pub_key);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.pub_key.is_empty() {
            os.write_bytes(1, &self.pub_key)?;
        }
        if self.height != 0 {
            os.write_int64(2, self.height)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Evidence {
    fn new() -> Evidence {
        Evidence::new()
    }

    fn descriptor_static(_: ::std::option::Option<Evidence>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "pub_key",
                    Evidence::get_pub_key_for_reflect,
                    Evidence::mut_pub_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "height",
                    Evidence::get_height_for_reflect,
                    Evidence::mut_height_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Evidence>(
                    "Evidence",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Evidence {
    fn clear(&mut self) {
        self.clear_pub_key();
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Evidence {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Evidence {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11proto/types.proto\x12\x05types\"0\n\x06KVPair\x12\x10\n\x03key\x18\
    \x01\x20\x01(\x0cR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05va\
    lue\"2\n\x08KI64Pair\x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\x03key\x12\
    \x14\n\x05value\x18\x02\x20\x01(\x03R\x05value\"\xc6\x04\n\x07Request\
    \x12(\n\x04echo\x18\x02\x20\x01(\x0b2\x12.types.RequestEchoH\0R\x04echo\
    \x12+\n\x05flush\x18\x03\x20\x01(\x0b2\x13.types.RequestFlushH\0R\x05flu\
    sh\x12(\n\x04info\x18\x04\x20\x01(\x0b2\x12.types.RequestInfoH\0R\x04inf\
    o\x128\n\nset_option\x18\x05\x20\x01(\x0b2\x17.types.RequestSetOptionH\0\
    R\tsetOption\x128\n\ninit_chain\x18\x06\x20\x01(\x0b2\x17.types.RequestI\
    nitChainH\0R\tinitChain\x12+\n\x05query\x18\x07\x20\x01(\x0b2\x13.types.\
    RequestQueryH\0R\x05query\x12;\n\x0bbegin_block\x18\x08\x20\x01(\x0b2\
    \x18.types.RequestBeginBlockH\0R\nbeginBlock\x122\n\x08check_tx\x18\t\
    \x20\x01(\x0b2\x15.types.RequestCheckTxH\0R\x07checkTx\x128\n\ndeliver_t\
    x\x18\x13\x20\x01(\x0b2\x17.types.RequestDeliverTxH\0R\tdeliverTx\x125\n\
    \tend_block\x18\x0b\x20\x01(\x0b2\x16.types.RequestEndBlockH\0R\x08endBl\
    ock\x12.\n\x06commit\x18\x0c\x20\x01(\x0b2\x14.types.RequestCommitH\0R\
    \x06commitB\x07\n\x05value\"'\n\x0bRequestEcho\x12\x18\n\x07message\x18\
    \x01\x20\x01(\tR\x07message\"\x0e\n\x0cRequestFlush\"'\n\x0bRequestInfo\
    \x12\x18\n\x07version\x18\x01\x20\x01(\tR\x07version\":\n\x10RequestSetO\
    ption\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x01(\tR\x05value\"l\n\x10RequestInitChain\x120\n\nvalidators\
    \x18\x01\x20\x03(\x0b2\x10.types.ValidatorR\nvalidators\x12&\n\x0fapp_st\
    ate_bytes\x18\x02\x20\x01(\x0cR\rappStateBytes\"d\n\x0cRequestQuery\x12\
    \x12\n\x04data\x18\x01\x20\x01(\x0cR\x04data\x12\x12\n\x04path\x18\x02\
    \x20\x01(\tR\x04path\x12\x16\n\x06height\x18\x03\x20\x01(\x03R\x06height\
    \x12\x14\n\x05prove\x18\x04\x20\x01(\x08R\x05prove\"\xbf\x01\n\x11Reques\
    tBeginBlock\x12\x12\n\x04hash\x18\x01\x20\x01(\x0cR\x04hash\x12%\n\x06he\
    ader\x18\x02\x20\x01(\x0b2\r.types.HeaderR\x06header\x12+\n\x11absent_va\
    lidators\x18\x03\x20\x03(\x05R\x10absentValidators\x12B\n\x14byzantine_v\
    alidators\x18\x04\x20\x03(\x0b2\x0f.types.EvidenceR\x13byzantineValidato\
    rs\"\x20\n\x0eRequestCheckTx\x12\x0e\n\x02tx\x18\x01\x20\x01(\x0cR\x02tx\
    \"\"\n\x10RequestDeliverTx\x12\x0e\n\x02tx\x18\x01\x20\x01(\x0cR\x02tx\"\
    )\n\x0fRequestEndBlock\x12\x16\n\x06height\x18\x01\x20\x01(\x03R\x06heig\
    ht\"\x0f\n\rRequestCommit\"\x8c\x05\n\x08Response\x128\n\texception\x18\
    \x01\x20\x01(\x0b2\x18.types.ResponseExceptionH\0R\texception\x12)\n\x04\
    echo\x18\x02\x20\x01(\x0b2\x13.types.ResponseEchoH\0R\x04echo\x12,\n\x05\
    flush\x18\x03\x20\x01(\x0b2\x14.types.ResponseFlushH\0R\x05flush\x12)\n\
    \x04info\x18\x04\x20\x01(\x0b2\x13.types.ResponseInfoH\0R\x04info\x129\n\
    \nset_option\x18\x05\x20\x01(\x0b2\x18.types.ResponseSetOptionH\0R\tsetO\
    ption\x129\n\ninit_chain\x18\x06\x20\x01(\x0b2\x18.types.ResponseInitCha\
    inH\0R\tinitChain\x12,\n\x05query\x18\x07\x20\x01(\x0b2\x14.types.Respon\
    seQueryH\0R\x05query\x12<\n\x0bbegin_block\x18\x08\x20\x01(\x0b2\x19.typ\
    es.ResponseBeginBlockH\0R\nbeginBlock\x123\n\x08check_tx\x18\t\x20\x01(\
    \x0b2\x16.types.ResponseCheckTxH\0R\x07checkTx\x129\n\ndeliver_tx\x18\n\
    \x20\x01(\x0b2\x18.types.ResponseDeliverTxH\0R\tdeliverTx\x126\n\tend_bl\
    ock\x18\x0b\x20\x01(\x0b2\x17.types.ResponseEndBlockH\0R\x08endBlock\x12\
    /\n\x06commit\x18\x0c\x20\x01(\x0b2\x15.types.ResponseCommitH\0R\x06comm\
    itB\x07\n\x05value\")\n\x11ResponseException\x12\x14\n\x05error\x18\x01\
    \x20\x01(\tR\x05error\"(\n\x0cResponseEcho\x12\x18\n\x07message\x18\x01\
    \x20\x01(\tR\x07message\"\x0f\n\rResponseFlush\"\x97\x01\n\x0cResponseIn\
    fo\x12\x12\n\x04data\x18\x01\x20\x01(\tR\x04data\x12\x18\n\x07version\
    \x18\x02\x20\x01(\tR\x07version\x12*\n\x11last_block_height\x18\x03\x20\
    \x01(\x03R\x0flastBlockHeight\x12-\n\x13last_block_app_hash\x18\x04\x20\
    \x01(\x0cR\x10lastBlockAppHash\"M\n\x11ResponseSetOption\x12\x12\n\x04co\
    de\x18\x01\x20\x01(\rR\x04code\x12\x10\n\x03log\x18\x03\x20\x01(\tR\x03l\
    og\x12\x12\n\x04info\x18\x04\x20\x01(\tR\x04info\"\x13\n\x11ResponseInit\
    Chain\"\xb5\x01\n\rResponseQuery\x12\x12\n\x04code\x18\x01\x20\x01(\rR\
    \x04code\x12\x10\n\x03log\x18\x03\x20\x01(\tR\x03log\x12\x12\n\x04info\
    \x18\x04\x20\x01(\tR\x04info\x12\x14\n\x05index\x18\x05\x20\x01(\x03R\
    \x05index\x12\x10\n\x03key\x18\x06\x20\x01(\x0cR\x03key\x12\x14\n\x05val\
    ue\x18\x07\x20\x01(\x0cR\x05value\x12\x14\n\x05proof\x18\x08\x20\x01(\
    \x0cR\x05proof\x12\x16\n\x06height\x18\t\x20\x01(\x03R\x06height\"\x14\n\
    \x12ResponseBeginBlock\"\xdf\x01\n\x0fResponseCheckTx\x12\x12\n\x04code\
    \x18\x01\x20\x01(\rR\x04code\x12\x12\n\x04data\x18\x02\x20\x01(\x0cR\x04\
    data\x12\x10\n\x03log\x18\x03\x20\x01(\tR\x03log\x12\x12\n\x04info\x18\
    \x04\x20\x01(\tR\x04info\x12\x1d\n\ngas_wanted\x18\x05\x20\x01(\x03R\tga\
    sWanted\x12\x19\n\x08gas_used\x18\x06\x20\x01(\x03R\x07gasUsed\x12!\n\
    \x04tags\x18\x07\x20\x03(\x0b2\r.types.KVPairR\x04tags\x12!\n\x03fee\x18\
    \x08\x20\x01(\x0b2\x0f.types.KI64PairR\x03fee\"\xe1\x01\n\x11ResponseDel\
    iverTx\x12\x12\n\x04code\x18\x01\x20\x01(\rR\x04code\x12\x12\n\x04data\
    \x18\x02\x20\x01(\x0cR\x04data\x12\x10\n\x03log\x18\x03\x20\x01(\tR\x03l\
    og\x12\x12\n\x04info\x18\x04\x20\x01(\tR\x04info\x12\x1d\n\ngas_wanted\
    \x18\x05\x20\x01(\x03R\tgasWanted\x12\x19\n\x08gas_used\x18\x06\x20\x01(\
    \x03R\x07gasUsed\x12!\n\x04tags\x18\x07\x20\x03(\x0b2\r.types.KVPairR\
    \x04tags\x12!\n\x03fee\x18\x08\x20\x01(\x0b2\x0f.types.KI64PairR\x03fee\
    \"\xa1\x01\n\x10ResponseEndBlock\x12=\n\x11validator_updates\x18\x01\x20\
    \x03(\x0b2\x10.types.ValidatorR\x10validatorUpdates\x12N\n\x17consensus_\
    param_updates\x18\x02\x20\x01(\x0b2\x16.types.ConsensusParamsR\x15consen\
    susParamUpdates\"$\n\x0eResponseCommit\x12\x12\n\x04data\x18\x02\x20\x01\
    (\x0cR\x04data\"\xa1\x01\n\x0fConsensusParams\x12/\n\nblock_size\x18\x01\
    \x20\x01(\x0b2\x10.types.BlockSizeR\tblockSize\x12&\n\x07tx_size\x18\x02\
    \x20\x01(\x0b2\r.types.TxSizeR\x06txSize\x125\n\x0cblock_gossip\x18\x03\
    \x20\x01(\x0b2\x12.types.BlockGossipR\x0bblockGossip\"Z\n\tBlockSize\x12\
    \x1b\n\tmax_bytes\x18\x01\x20\x01(\x05R\x08maxBytes\x12\x17\n\x07max_txs\
    \x18\x02\x20\x01(\x05R\x06maxTxs\x12\x17\n\x07max_gas\x18\x03\x20\x01(\
    \x03R\x06maxGas\">\n\x06TxSize\x12\x1b\n\tmax_bytes\x18\x01\x20\x01(\x05\
    R\x08maxBytes\x12\x17\n\x07max_gas\x18\x02\x20\x01(\x03R\x06maxGas\"@\n\
    \x0bBlockGossip\x121\n\x15block_part_size_bytes\x18\x01\x20\x01(\x05R\
    \x12blockPartSizeBytes\"\xa7\x02\n\x06Header\x12\x19\n\x08chain_id\x18\
    \x01\x20\x01(\tR\x07chainId\x12\x16\n\x06height\x18\x02\x20\x01(\x03R\
    \x06height\x12\x12\n\x04time\x18\x03\x20\x01(\x03R\x04time\x12\x17\n\x07\
    num_txs\x18\x04\x20\x01(\x05R\x06numTxs\x122\n\rlast_block_id\x18\x05\
    \x20\x01(\x0b2\x0e.types.BlockIDR\x0blastBlockId\x12(\n\x10last_commit_h\
    ash\x18\x06\x20\x01(\x0cR\x0elastCommitHash\x12\x1b\n\tdata_hash\x18\x07\
    \x20\x01(\x0cR\x08dataHash\x12'\n\x0fvalidators_hash\x18\x08\x20\x01(\
    \x0cR\x0evalidatorsHash\x12\x19\n\x08app_hash\x18\t\x20\x01(\x0cR\x07app\
    Hash\"I\n\x07BlockID\x12\x12\n\x04hash\x18\x01\x20\x01(\x0cR\x04hash\x12\
    *\n\x05parts\x18\x02\x20\x01(\x0b2\x14.types.PartSetHeaderR\x05parts\"9\
    \n\rPartSetHeader\x12\x14\n\x05total\x18\x01\x20\x01(\x05R\x05total\x12\
    \x12\n\x04hash\x18\x02\x20\x01(\x0cR\x04hash\":\n\tValidator\x12\x17\n\
    \x07pub_key\x18\x01\x20\x01(\x0cR\x06pubKey\x12\x14\n\x05power\x18\x02\
    \x20\x01(\x03R\x05power\";\n\x08Evidence\x12\x17\n\x07pub_key\x18\x01\
    \x20\x01(\x0cR\x06pubKey\x12\x16\n\x06height\x18\x02\x20\x01(\x03R\x06he\
    ight2\x8c\x05\n\x0fABCIApplication\x12/\n\x04Echo\x12\x12.types.RequestE\
    cho\x1a\x13.types.ResponseEcho\x122\n\x05Flush\x12\x13.types.RequestFlus\
    h\x1a\x14.types.ResponseFlush\x12/\n\x04Info\x12\x12.types.RequestInfo\
    \x1a\x13.types.ResponseInfo\x12>\n\tSetOption\x12\x17.types.RequestSetOp\
    tion\x1a\x18.types.ResponseSetOption\x12>\n\tDeliverTx\x12\x17.types.Req\
    uestDeliverTx\x1a\x18.types.ResponseDeliverTx\x128\n\x07CheckTx\x12\x15.\
    types.RequestCheckTx\x1a\x16.types.ResponseCheckTx\x122\n\x05Query\x12\
    \x13.types.RequestQuery\x1a\x14.types.ResponseQuery\x125\n\x06Commit\x12\
    \x14.types.RequestCommit\x1a\x15.types.ResponseCommit\x12>\n\tInitChain\
    \x12\x17.types.RequestInitChain\x1a\x18.types.ResponseInitChain\x12A\n\n\
    BeginBlock\x12\x18.types.RequestBeginBlock\x1a\x19.types.ResponseBeginBl\
    ock\x12;\n\x08EndBlock\x12\x16.types.RequestEndBlock\x1a\x17.types.Respo\
    nseEndBlockb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
