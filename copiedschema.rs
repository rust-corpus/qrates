pub mod types {
    use serde_derive::{Deserialize, Serialize};
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct CrateHash(pub(super) u128);
    impl From<u128> for CrateHash {
        fn from(value: u128) -> Self {
            Self(value)
        }
    }
    impl From<usize> for CrateHash {
        fn from(value: usize) -> Self {
            Self(value as u128)
        }
    }
    impl Into<usize> for CrateHash {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for CrateHash {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u128>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u128>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(CrateHash))
        }
    }
    impl CrateHash {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u128) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    impl From<u128> for CrateHash {
        fn from(value: u128) -> Self {
            Self(value)
        }
    }
    impl std::fmt::LowerHex for CrateHash {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:x}", self.0)
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct DefPathHash(pub(super) (u64, u64));
    impl From<(u64, u64)> for DefPathHash {
        fn from(value: (u64, u64)) -> Self {
            Self(value)
        }
    }
    impl std::fmt::LowerHex for DefPathHash {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:x}{:x}", (self.0).0, (self.0).1)
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct AdtVariantIndex(pub(super) u16);
    impl From<u16> for AdtVariantIndex {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    impl From<usize> for AdtVariantIndex {
        fn from(value: usize) -> Self {
            Self(value as u16)
        }
    }
    impl Into<usize> for AdtVariantIndex {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for AdtVariantIndex {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u16>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u16>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(AdtVariantIndex))
        }
    }
    impl AdtVariantIndex {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u16) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct TupleFieldIndex(pub(super) u16);
    impl From<u16> for TupleFieldIndex {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    impl From<usize> for TupleFieldIndex {
        fn from(value: usize) -> Self {
            Self(value as u16)
        }
    }
    impl Into<usize> for TupleFieldIndex {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for TupleFieldIndex {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u16>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u16>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(TupleFieldIndex))
        }
    }
    impl TupleFieldIndex {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u16) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct FnParamIndex(pub(super) u16);
    impl From<u16> for FnParamIndex {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    impl From<usize> for FnParamIndex {
        fn from(value: usize) -> Self {
            Self(value as u16)
        }
    }
    impl Into<usize> for FnParamIndex {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for FnParamIndex {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u16>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u16>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(FnParamIndex))
        }
    }
    impl FnParamIndex {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u16) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct OperandIndex(pub(super) u16);
    impl From<u16> for OperandIndex {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    impl From<usize> for OperandIndex {
        fn from(value: usize) -> Self {
            Self(value as u16)
        }
    }
    impl Into<usize> for OperandIndex {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for OperandIndex {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u16>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u16>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(OperandIndex))
        }
    }
    impl OperandIndex {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u16) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct StatementIndex(pub(super) u16);
    impl From<u16> for StatementIndex {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    impl From<usize> for StatementIndex {
        fn from(value: usize) -> Self {
            Self(value as u16)
        }
    }
    impl Into<usize> for StatementIndex {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for StatementIndex {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u16>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u16>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(StatementIndex))
        }
    }
    impl StatementIndex {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u16) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct CallArgIndex(pub(super) u16);
    impl From<u16> for CallArgIndex {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    impl From<usize> for CallArgIndex {
        fn from(value: usize) -> Self {
            Self(value as u16)
        }
    }
    impl Into<usize> for CallArgIndex {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for CallArgIndex {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u16>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u16>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(CallArgIndex))
        }
    }
    impl CallArgIndex {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u16) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct ThirCallArgIndex(pub(super) u16);
    impl From<u16> for ThirCallArgIndex {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    impl From<usize> for ThirCallArgIndex {
        fn from(value: usize) -> Self {
            Self(value as u16)
        }
    }
    impl Into<usize> for ThirCallArgIndex {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for ThirCallArgIndex {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u16>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u16>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(ThirCallArgIndex))
        }
    }
    impl ThirCallArgIndex {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u16) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct MatchArmIdx(pub(super) u16);
    impl From<u16> for MatchArmIdx {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    impl From<usize> for MatchArmIdx {
        fn from(value: usize) -> Self {
            Self(value as u16)
        }
    }
    impl Into<usize> for MatchArmIdx {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for MatchArmIdx {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u16>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u16>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(MatchArmIdx))
        }
    }
    impl MatchArmIdx {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u16) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct FieldIndex(pub(super) u16);
    impl From<u16> for FieldIndex {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    impl From<usize> for FieldIndex {
        fn from(value: usize) -> Self {
            Self(value as u16)
        }
    }
    impl Into<usize> for FieldIndex {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for FieldIndex {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u16>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u16>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(FieldIndex))
        }
    }
    impl FieldIndex {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u16) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Module(pub(super) u32);
    impl From<u32> for Module {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Module {
        fn from(value: usize) -> Self {
            Self(value as u32)
        }
    }
    impl Into<usize> for Module {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Module {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u32>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u32>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Module))
        }
    }
    impl Module {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u32) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Item(pub(super) u32);
    impl From<u32> for Item {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Item {
        fn from(value: usize) -> Self {
            Self(value as u32)
        }
    }
    impl Into<usize> for Item {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Item {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u32>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u32>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Item))
        }
    }
    impl Item {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u32) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Scope(pub(super) u32);
    impl From<u32> for Scope {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Scope {
        fn from(value: usize) -> Self {
            Self(value as u32)
        }
    }
    impl Into<usize> for Scope {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Scope {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u32>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u32>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Scope))
        }
    }
    impl Scope {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u32) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct FunctionCall(pub(super) u32);
    impl From<u32> for FunctionCall {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }
    impl From<usize> for FunctionCall {
        fn from(value: usize) -> Self {
            Self(value as u32)
        }
    }
    impl Into<usize> for FunctionCall {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for FunctionCall {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u32>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u32>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(FunctionCall))
        }
    }
    impl FunctionCall {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u32) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Span(pub(super) u64);
    impl From<u64> for Span {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Span {
        fn from(value: usize) -> Self {
            Self(value as u64)
        }
    }
    impl Into<usize> for Span {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Span {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u64>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u64>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Span))
        }
    }
    impl Span {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u64) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Type(pub(super) u64);
    impl From<u64> for Type {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Type {
        fn from(value: usize) -> Self {
            Self(value as u64)
        }
    }
    impl Into<usize> for Type {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Type {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u64>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u64>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Type))
        }
    }
    impl Type {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u64) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Field(pub(super) u64);
    impl From<u64> for Field {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Field {
        fn from(value: usize) -> Self {
            Self(value as u64)
        }
    }
    impl Into<usize> for Field {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Field {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u64>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u64>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Field))
        }
    }
    impl Field {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u64) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Operand(pub(super) u64);
    impl From<u64> for Operand {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Operand {
        fn from(value: usize) -> Self {
            Self(value as u64)
        }
    }
    impl Into<usize> for Operand {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Operand {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u64>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u64>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Operand))
        }
    }
    impl Operand {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u64) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct BasicBlock(pub(super) u64);
    impl From<u64> for BasicBlock {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl From<usize> for BasicBlock {
        fn from(value: usize) -> Self {
            Self(value as u64)
        }
    }
    impl Into<usize> for BasicBlock {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for BasicBlock {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u64>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u64>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(BasicBlock))
        }
    }
    impl BasicBlock {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u64) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Statement(pub(super) u64);
    impl From<u64> for Statement {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Statement {
        fn from(value: usize) -> Self {
            Self(value as u64)
        }
    }
    impl Into<usize> for Statement {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Statement {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u64>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u64>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Statement))
        }
    }
    impl Statement {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u64) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct ThirBlock(pub(super) u64);
    impl From<u64> for ThirBlock {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl From<usize> for ThirBlock {
        fn from(value: usize) -> Self {
            Self(value as u64)
        }
    }
    impl Into<usize> for ThirBlock {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for ThirBlock {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u64>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u64>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(ThirBlock))
        }
    }
    impl ThirBlock {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u64) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct ThirExpr(pub(super) u64);
    impl From<u64> for ThirExpr {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl From<usize> for ThirExpr {
        fn from(value: usize) -> Self {
            Self(value as u64)
        }
    }
    impl Into<usize> for ThirExpr {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for ThirExpr {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u64>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u64>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(ThirExpr))
        }
    }
    impl ThirExpr {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u64) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct ThirPat(pub(super) u64);
    impl From<u64> for ThirPat {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl From<usize> for ThirPat {
        fn from(value: usize) -> Self {
            Self(value as u64)
        }
    }
    impl Into<usize> for ThirPat {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for ThirPat {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u64>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u64>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(ThirPat))
        }
    }
    impl ThirPat {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u64) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct ThirStmt(pub(super) u64);
    impl From<u64> for ThirStmt {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl From<usize> for ThirStmt {
        fn from(value: usize) -> Self {
            Self(value as u64)
        }
    }
    impl Into<usize> for ThirStmt {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for ThirStmt {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u64>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u64>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(ThirStmt))
        }
    }
    impl ThirStmt {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u64) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct InternedString(pub(super) u64);
    impl From<u64> for InternedString {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl From<usize> for InternedString {
        fn from(value: usize) -> Self {
            Self(value as u64)
        }
    }
    impl Into<usize> for InternedString {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for InternedString {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u64>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u64>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(InternedString))
        }
    }
    impl InternedString {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u64) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Package(pub(super) u32);
    impl From<u32> for Package {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Package {
        fn from(value: usize) -> Self {
            Self(value as u32)
        }
    }
    impl Into<usize> for Package {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Package {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u32>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u32>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Package))
        }
    }
    impl Package {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u32) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct PackageVersion(pub(super) u32);
    impl From<u32> for PackageVersion {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }
    impl From<usize> for PackageVersion {
        fn from(value: usize) -> Self {
            Self(value as u32)
        }
    }
    impl Into<usize> for PackageVersion {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for PackageVersion {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u32>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u32>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(PackageVersion))
        }
    }
    impl PackageVersion {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u32) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Krate(pub(super) u32);
    impl From<u32> for Krate {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Krate {
        fn from(value: usize) -> Self {
            Self(value as u32)
        }
    }
    impl Into<usize> for Krate {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Krate {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u32>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u32>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Krate))
        }
    }
    impl Krate {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u32) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Edition(pub(super) u8);
    impl From<u8> for Edition {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Edition {
        fn from(value: usize) -> Self {
            Self(value as u8)
        }
    }
    impl Into<usize> for Edition {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Edition {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u8>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u8>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Edition))
        }
    }
    impl Edition {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u8) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Name(pub(super) u32);
    impl From<u32> for Name {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Name {
        fn from(value: usize) -> Self {
            Self(value as u32)
        }
    }
    impl Into<usize> for Name {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Name {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u32>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u32>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Name))
        }
    }
    impl Name {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u32) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct RelativeDefId(pub(super) u32);
    impl From<u32> for RelativeDefId {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }
    impl From<usize> for RelativeDefId {
        fn from(value: usize) -> Self {
            Self(value as u32)
        }
    }
    impl Into<usize> for RelativeDefId {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for RelativeDefId {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u32>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u32>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(RelativeDefId))
        }
    }
    impl RelativeDefId {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u32) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct SummaryId(pub(super) u32);
    impl From<u32> for SummaryId {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }
    impl From<usize> for SummaryId {
        fn from(value: usize) -> Self {
            Self(value as u32)
        }
    }
    impl Into<usize> for SummaryId {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for SummaryId {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u32>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u32>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(SummaryId))
        }
    }
    impl SummaryId {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u32) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Abi(pub(super) u8);
    impl From<u8> for Abi {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Abi {
        fn from(value: usize) -> Self {
            Self(value as u8)
        }
    }
    impl Into<usize> for Abi {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Abi {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u8>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u8>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Abi))
        }
    }
    impl Abi {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u8) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct DefPath(pub(super) u64);
    impl From<u64> for DefPath {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    impl From<usize> for DefPath {
        fn from(value: usize) -> Self {
            Self(value as u64)
        }
    }
    impl Into<usize> for DefPath {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for DefPath {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u64>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u64>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(DefPath))
        }
    }
    impl DefPath {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u64) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct Build(pub(super) u32);
    impl From<u32> for Build {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }
    impl From<usize> for Build {
        fn from(value: usize) -> Self {
            Self(value as u32)
        }
    }
    impl Into<usize> for Build {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for Build {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u32>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u32>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(Build))
        }
    }
    impl Build {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u32) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct SpanFileName(pub(super) u32);
    impl From<u32> for SpanFileName {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }
    impl From<usize> for SpanFileName {
        fn from(value: usize) -> Self {
            Self(value as u32)
        }
    }
    impl Into<usize> for SpanFileName {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for SpanFileName {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u32>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u32>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(SpanFileName))
        }
    }
    impl SpanFileName {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u32) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct CrateCfgKey(pub(super) u16);
    impl From<u16> for CrateCfgKey {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    impl From<usize> for CrateCfgKey {
        fn from(value: usize) -> Self {
            Self(value as u16)
        }
    }
    impl Into<usize> for CrateCfgKey {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for CrateCfgKey {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u16>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u16>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(CrateCfgKey))
        }
    }
    impl CrateCfgKey {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u16) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct CrateCfgValue(pub(super) u16);
    impl From<u16> for CrateCfgValue {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }
    impl From<usize> for CrateCfgValue {
        fn from(value: usize) -> Self {
            Self(value as u16)
        }
    }
    impl Into<usize> for CrateCfgValue {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for CrateCfgValue {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u16>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u16>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(CrateCfgValue))
        }
    }
    impl CrateCfgValue {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u16) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct TyKind(pub(super) u8);
    impl From<u8> for TyKind {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }
    impl From<usize> for TyKind {
        fn from(value: usize) -> Self {
            Self(value as u8)
        }
    }
    impl Into<usize> for TyKind {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for TyKind {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u8>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u8>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(TyKind))
        }
    }
    impl TyKind {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u8) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct StatementKind(pub(super) u8);
    impl From<u8> for StatementKind {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }
    impl From<usize> for StatementKind {
        fn from(value: usize) -> Self {
            Self(value as u8)
        }
    }
    impl Into<usize> for StatementKind {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for StatementKind {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u8>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u8>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(StatementKind))
        }
    }
    impl StatementKind {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u8) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct BinOp(pub(super) u8);
    impl From<u8> for BinOp {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }
    impl From<usize> for BinOp {
        fn from(value: usize) -> Self {
            Self(value as u8)
        }
    }
    impl Into<usize> for BinOp {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for BinOp {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u8>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u8>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(BinOp))
        }
    }
    impl BinOp {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u8) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct NullOp(pub(super) u8);
    impl From<u8> for NullOp {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }
    impl From<usize> for NullOp {
        fn from(value: usize) -> Self {
            Self(value as u8)
        }
    }
    impl Into<usize> for NullOp {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for NullOp {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u8>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u8>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(NullOp))
        }
    }
    impl NullOp {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u8) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct UnOp(pub(super) u8);
    impl From<u8> for UnOp {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }
    impl From<usize> for UnOp {
        fn from(value: usize) -> Self {
            Self(value as u8)
        }
    }
    impl Into<usize> for UnOp {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for UnOp {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u8>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u8>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(UnOp))
        }
    }
    impl UnOp {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u8) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct TerminatorKind(pub(super) u8);
    impl From<u8> for TerminatorKind {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }
    impl From<usize> for TerminatorKind {
        fn from(value: usize) -> Self {
            Self(value as u8)
        }
    }
    impl Into<usize> for TerminatorKind {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for TerminatorKind {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u8>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u8>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(TerminatorKind))
        }
    }
    impl TerminatorKind {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u8) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct ThirBinOp(pub(super) u8);
    impl From<u8> for ThirBinOp {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }
    impl From<usize> for ThirBinOp {
        fn from(value: usize) -> Self {
            Self(value as u8)
        }
    }
    impl Into<usize> for ThirBinOp {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for ThirBinOp {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u8>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u8>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(ThirBinOp))
        }
    }
    impl ThirBinOp {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u8) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct ThirLogicalOp(pub(super) u8);
    impl From<u8> for ThirLogicalOp {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }
    impl From<usize> for ThirLogicalOp {
        fn from(value: usize) -> Self {
            Self(value as u8)
        }
    }
    impl Into<usize> for ThirLogicalOp {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for ThirLogicalOp {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u8>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u8>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(ThirLogicalOp))
        }
    }
    impl ThirLogicalOp {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u8) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[derive(
        Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord, Default,
    )]
    pub struct ThirUnOp(pub(super) u8);
    impl From<u8> for ThirUnOp {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }
    impl From<usize> for ThirUnOp {
        fn from(value: usize) -> Self {
            Self(value as u8)
        }
    }
    impl Into<usize> for ThirUnOp {
        fn into(self) -> usize {
            self.0 as usize
        }
    }
    impl redb::Value for ThirUnOp {
        type SelfType<'a> = Self;
        type AsBytes<'a> = &'a [u8];
        fn fixed_width() -> Option<usize> {
            Some(std::mem::size_of::<u8>())
        }
        fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a> {
            let value = <u8>::from_bytes(data);
            Self(value)
        }
        fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
            self.0.as_bytes()
        }
        fn type_name() -> redb::TypeName {
            redb::TypeName::new(stringify!(ThirUnOp))
        }
    }
    impl ThirUnOp {
        #[doc = r" Shift the id by given `offset`."]
        pub fn shift(&self, offset: u8) -> Self {
            Self(self.0.checked_add(offset).expect("Overflow!"))
        }
        #[doc = r" Get the underlying index."]
        pub fn index(&self) -> usize {
            self.0 as usize
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum Mutability {
        Mutable = 0,
        Immutable = 1,
        Const = 2,
        Unknown = 3,
    }
    impl Default for Mutability {
        fn default() -> Self {
            Mutability::Unknown
        }
    }
    impl std::fmt::Display for Mutability {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum Constness {
        Const = 0,
        NotConst = 1,
        Unknown = 2,
    }
    impl Default for Constness {
        fn default() -> Self {
            Constness::Unknown
        }
    }
    impl std::fmt::Display for Constness {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum TyVisibility {
        Public = 0,
        Restricted = 1,
        Invisible = 2,
        Unknown = 3,
    }
    impl Default for TyVisibility {
        fn default() -> Self {
            TyVisibility::Unknown
        }
    }
    impl std::fmt::Display for TyVisibility {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum Safety {
        Unsafe = 0,
        Safe = 1,
        Unknown = 2,
    }
    impl Default for Safety {
        fn default() -> Self {
            Safety::Unknown
        }
    }
    impl std::fmt::Display for Safety {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum ImplPolarity {
        Positive = 0,
        Negative = 1,
        Unknown = 2,
    }
    impl Default for ImplPolarity {
        fn default() -> Self {
            ImplPolarity::Unknown
        }
    }
    impl std::fmt::Display for ImplPolarity {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum ScopeSafety {
        Safe = 0,
        BuiltinUnsafe = 1,
        FnUnsafe = 2,
        ExplicitUnsafe = 3,
        Unknown = 4,
    }
    impl Default for ScopeSafety {
        fn default() -> Self {
            ScopeSafety::Unknown
        }
    }
    impl std::fmt::Display for ScopeSafety {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum AdtKind {
        Struct = 0,
        Union = 1,
        Enum = 2,
        Unknown = 3,
    }
    impl Default for AdtKind {
        fn default() -> Self {
            AdtKind::Unknown
        }
    }
    impl std::fmt::Display for AdtKind {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum TyPrimitive {
        Unknown = 0,
        Bool = 1,
        Char = 2,
        Isize = 3,
        I8 = 4,
        I16 = 5,
        I32 = 6,
        I64 = 7,
        I128 = 8,
        Usize = 9,
        U8 = 10,
        U16 = 11,
        U32 = 12,
        U64 = 13,
        U128 = 14,
        F16 = 15,
        F32 = 16,
        F64 = 17,
        F128 = 18,
        Str = 19,
        Never = 20,
    }
    impl Default for TyPrimitive {
        fn default() -> Self {
            TyPrimitive::Unknown
        }
    }
    impl std::fmt::Display for TyPrimitive {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum TyDefKind {
        Unknown = 0,
        TyAlias = 1,
        Enum = 2,
        Struct = 3,
        Union = 4,
    }
    impl Default for TyDefKind {
        fn default() -> Self {
            TyDefKind::Unknown
        }
    }
    impl std::fmt::Display for TyDefKind {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum Defaultness {
        Unknown = 0,
        DefaultWithValue = 1,
        DefaultNoValue = 2,
        Final = 3,
    }
    impl Default for Defaultness {
        fn default() -> Self {
            Defaultness::Unknown
        }
    }
    impl std::fmt::Display for Defaultness {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum OperandKind {
        Unknown = 0,
        Copy = 1,
        Move = 2,
        Constant = 3,
    }
    impl Default for OperandKind {
        fn default() -> Self {
            OperandKind::Unknown
        }
    }
    impl std::fmt::Display for OperandKind {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum AggregateKind {
        Unknown = 0,
        Array = 1,
        Tuple = 2,
        Adt = 3,
        Closure = 4,
        Coroutine = 5,
        CoroutineClosure = 6,
        RawPtr = 7,
    }
    impl Default for AggregateKind {
        fn default() -> Self {
            AggregateKind::Unknown
        }
    }
    impl std::fmt::Display for AggregateKind {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum CastKind {
        Unknown = 0,
        Misc = 1,
        ReifyFnPointer = 2,
        UnsafeFnPointer = 3,
        UnsafeClosureFnPointer = 4,
        SafeClosureFnPointer = 5,
        MutToConstPointer = 6,
        ArrayToPointer = 7,
        UnsizePointer = 8,
        PointerExposeProvenance = 9,
        PointerWithExposedProvenance = 10,
        DynStar = 11,
        IntToInt = 12,
        FloatToInt = 13,
        IntToFloat = 14,
        FloatToFloat = 15,
        PtrToPtr = 16,
        FnPtrToPtr = 17,
        Transmute = 18,
    }
    impl Default for CastKind {
        fn default() -> Self {
            CastKind::Unknown
        }
    }
    impl std::fmt::Display for CastKind {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum BorrowKind {
        Unknown = 0,
        Shared = 1,
        Shallow = 2,
        Deep = 3,
        ClosureCapture = 4,
        Mut = 5,
        MutTwoPhase = 6,
    }
    impl Default for BorrowKind {
        fn default() -> Self {
            BorrowKind::Unknown
        }
    }
    impl std::fmt::Display for BorrowKind {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum BasicBlockKind {
        Regular = 0,
        Entry = 1,
        CleanUp = 2,
    }
    impl Default for BasicBlockKind {
        fn default() -> Self {
            BasicBlockKind::Regular
        }
    }
    impl std::fmt::Display for BasicBlockKind {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum SpanExpansionKind {
        Unknown = 0,
        Root = 1,
        MacroBang = 2,
        MacroAttr = 3,
        MacroDerive = 4,
        AstPassStdImports = 5,
        AstPassTestHarness = 6,
        AstPassProcMacroHarness = 7,
        DesugaringCondTemporary = 8,
        DesugaringQuestionMark = 9,
        DesugaringTryBlock = 10,
        DesugaringOpaqueTy = 11,
        DesugaringAsync = 12,
        DesugaringAwait = 13,
        DesugaringForLoop = 14,
        DesugaringWhileLoop = 15,
        DesugaringLetElse = 16,
        DesugaringYeetExpr = 17,
        DesugaringBoundModifier = 18,
        Inlined = 19,
    }
    impl Default for SpanExpansionKind {
        fn default() -> Self {
            SpanExpansionKind::Unknown
        }
    }
    impl std::fmt::Display for SpanExpansionKind {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum BlockCheckMode {
        DefaultBlock = 0,
        UnsafeBlockCompilerGenerated = 1,
        UnsafeBlockUserProvided = 2,
        PushUnsafeBlockCompilerGenerated = 3,
        PushUnsafeBlockUserProvided = 4,
        PopUnsafeBlockCompilerGenerated = 5,
        PopUnsafeBlockUserProvided = 6,
    }
    impl Default for BlockCheckMode {
        fn default() -> Self {
            BlockCheckMode::DefaultBlock
        }
    }
    impl std::fmt::Display for BlockCheckMode {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum UnwindAction {
        Unknown = 0,
        Continue = 1,
        Unreachable = 2,
        Terminate = 3,
        Cleanup = 4,
    }
    impl Default for UnwindAction {
        fn default() -> Self {
            UnwindAction::Unknown
        }
    }
    impl std::fmt::Display for UnwindAction {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum PointerCoercion {
        Unknown = 0,
        ReifyFnPointer = 1,
        UnsafeFnPointer = 2,
        ClosreFnPointer = 3,
        MutToConstPointer = 4,
        ArrayToPointer = 5,
        Unsize = 6,
        DynStar = 7,
    }
    impl Default for PointerCoercion {
        fn default() -> Self {
            PointerCoercion::Unknown
        }
    }
    impl std::fmt::Display for PointerCoercion {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum MatchSource {
        Unknown = 0,
        Normal = 1,
        Postfix = 2,
        ForLoopDesugar = 3,
        TryDesugar = 4,
        AwaitDesugar = 5,
        FormatArgs = 6,
    }
    impl Default for MatchSource {
        fn default() -> Self {
            MatchSource::Unknown
        }
    }
    impl std::fmt::Display for MatchSource {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum Movability {
        Unknown = 0,
        None = 1,
        Movable = 2,
        Static = 3,
    }
    impl Default for Movability {
        fn default() -> Self {
            Movability::Unknown
        }
    }
    impl std::fmt::Display for Movability {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    #[repr(u8)]
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
    pub enum LitKind {
        Unknown = 0,
        Str = 1,
        ByteStr = 2,
        CStr = 3,
        Byte = 4,
        Char = 5,
        Int = 6,
        Float = 7,
        Bool = 8,
        Err = 9,
    }
    impl Default for LitKind {
        fn default() -> Self {
            LitKind::Unknown
        }
    }
    impl std::fmt::Display for LitKind {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}
pub mod tables {
    use super::types::*;
    use crate::data_structures::InterningTable;
    use crate::storage::load_elts_relation;
    use crate::storage::load_elts_relation_into_relation;
    use crate::storage::save_elts_relation;
    use anyhow::Result;
    use serde_derive::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    #[derive(Default, Deserialize, Serialize)]
    #[doc = r" Interning tables."]
    pub struct InterningTables {
        pub strings: InterningTable<InternedString, String>,
        pub package_names: InterningTable<Package, InternedString>,
        pub package_versions: InterningTable<PackageVersion, InternedString>,
        pub crate_names: InterningTable<Krate, InternedString>,
        pub editions: InterningTable<Edition, InternedString>,
        pub names: InterningTable<Name, InternedString>,
        pub relative_def_paths: InterningTable<RelativeDefId, InternedString>,
        pub summary_keys: InterningTable<SummaryId, InternedString>,
        pub abis: InterningTable<Abi, InternedString>,
        pub def_paths:
            InterningTable<DefPath, (Krate, CrateHash, RelativeDefId, DefPathHash, SummaryId)>,
        pub builds: InterningTable<Build, (Package, PackageVersion, Krate, CrateHash, Edition)>,
        pub span_file_names: InterningTable<SpanFileName, InternedString>,
        pub crate_cfg_keys: InterningTable<CrateCfgKey, InternedString>,
        pub crate_cfg_values: InterningTable<CrateCfgValue, InternedString>,
        pub type_kinds: InterningTable<TyKind, InternedString>,
        pub statement_kinds: InterningTable<StatementKind, InternedString>,
        pub binary_op_kind: InterningTable<BinOp, InternedString>,
        pub nullary_op_kind: InterningTable<NullOp, InternedString>,
        pub unary_op_kind: InterningTable<UnOp, InternedString>,
        pub terminator_kinds: InterningTable<TerminatorKind, InternedString>,
        pub thir_binary_op_kind: InterningTable<ThirBinOp, InternedString>,
        pub thir_logical_op_kind: InterningTable<ThirLogicalOp, InternedString>,
        pub thir_unary_op_kind: InterningTable<ThirUnOp, InternedString>,
    }
    impl<K, V0> Into<Vec<(K, V0)>> for InterningTable<K, (V0,)>
    where
        K: crate::data_structures::InterningTableKey,
        V0: crate::data_structures::InterningTableValue,
    {
        fn into(self) -> Vec<(K, V0)> {
            self.contents
                .into_iter()
                .enumerate()
                .map(|(i, (v0,))| (i.into(), v0))
                .collect()
        }
    }
    impl<K, V0, V1> Into<Vec<(K, V0, V1)>> for InterningTable<K, (V0, V1)>
    where
        K: crate::data_structures::InterningTableKey,
        V0: crate::data_structures::InterningTableValue,
        V1: crate::data_structures::InterningTableValue,
    {
        fn into(self) -> Vec<(K, V0, V1)> {
            self.contents
                .into_iter()
                .enumerate()
                .map(|(i, (v0, v1))| (i.into(), v0, v1))
                .collect()
        }
    }
    impl<K, V0, V1, V2> Into<Vec<(K, V0, V1, V2)>> for InterningTable<K, (V0, V1, V2)>
    where
        K: crate::data_structures::InterningTableKey,
        V0: crate::data_structures::InterningTableValue,
        V1: crate::data_structures::InterningTableValue,
        V2: crate::data_structures::InterningTableValue,
    {
        fn into(self) -> Vec<(K, V0, V1, V2)> {
            self.contents
                .into_iter()
                .enumerate()
                .map(|(i, (v0, v1, v2))| (i.into(), v0, v1, v2))
                .collect()
        }
    }
    impl<K, V0, V1, V2, V3> Into<Vec<(K, V0, V1, V2, V3)>> for InterningTable<K, (V0, V1, V2, V3)>
    where
        K: crate::data_structures::InterningTableKey,
        V0: crate::data_structures::InterningTableValue,
        V1: crate::data_structures::InterningTableValue,
        V2: crate::data_structures::InterningTableValue,
        V3: crate::data_structures::InterningTableValue,
    {
        fn into(self) -> Vec<(K, V0, V1, V2, V3)> {
            self.contents
                .into_iter()
                .enumerate()
                .map(|(i, (v0, v1, v2, v3))| (i.into(), v0, v1, v2, v3))
                .collect()
        }
    }
    impl<K, V0, V1, V2, V3, V4> Into<Vec<(K, V0, V1, V2, V3, V4)>>
        for InterningTable<K, (V0, V1, V2, V3, V4)>
    where
        K: crate::data_structures::InterningTableKey,
        V0: crate::data_structures::InterningTableValue,
        V1: crate::data_structures::InterningTableValue,
        V2: crate::data_structures::InterningTableValue,
        V3: crate::data_structures::InterningTableValue,
        V4: crate::data_structures::InterningTableValue,
    {
        fn into(self) -> Vec<(K, V0, V1, V2, V3, V4)> {
            self.contents
                .into_iter()
                .enumerate()
                .map(|(i, (v0, v1, v2, v3, v4))| (i.into(), v0, v1, v2, v3, v4))
                .collect()
        }
    }
    use crate::data_structures::Relation;
    #[derive(Default, Deserialize, Serialize)]
    #[doc = r" Relations between various entities of the Rust program."]
    pub struct Relations {
        pub def_path_span: Relation<(DefPath, Span)>,
        pub type_description: Relation<(Type, InternedString, InternedString)>,
        pub build_crate_types: Relation<(Build, InternedString)>,
        pub root_modules: Relation<(Build, Module)>,
        pub submodules: Relation<(DefPath, Module, Module, Name, TyVisibility, Abi)>,
        pub function_definitions:
            Relation<(Item, DefPath, Module, TyVisibility, Safety, Abi, Type)>,
        pub function_parameter_types: Relation<(Item, FnParamIndex, Type)>,
        pub function_unsafe_use: Relation<(DefPath, bool)>,
        pub function_unsafe_reasons: Relation<(DefPath, u32, InternedString)>,
        pub thir_bodies: Relation<(Item, DefPath, ThirBlock)>,
        pub thir_blocks: Relation<(ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span)>,
        pub thir_stmts: Relation<(ThirStmt, ThirBlock, ThirBlock, StatementIndex)>,
        pub thir_stmts_expr: Relation<(ThirStmt, ThirExpr)>,
        pub thir_stmts_let: Relation<(ThirStmt, ThirExpr, ThirBlock, Span)>,
        pub thir_block_expr: Relation<(ThirBlock, ThirExpr)>,
        pub thir_exprs: Relation<(ThirExpr, ThirBlock, ThirBlock, Type, Span)>,
        pub thir_exprs_scope: Relation<(ThirExpr, ThirExpr)>,
        pub thir_exprs_box: Relation<(ThirExpr, ThirExpr)>,
        pub thir_exprs_if: Relation<(ThirExpr, ThirExpr, ThirExpr, ThirExpr)>,
        pub thir_exprs_call: Relation<(ThirExpr, Type, ThirExpr, Safety, Abi, Type)>,
        pub thir_exprs_call_arg: Relation<(ThirExpr, ThirCallArgIndex, ThirExpr)>,
        pub thir_exprs_call_const_target: Relation<(ThirExpr, DefPath)>,
        pub thir_exprs_call_const_target_desc:
            Relation<(ThirExpr, InternedString, InternedString, InternedString)>,
        pub thir_exprs_call_const_target_self: Relation<(ThirExpr, Type)>,
        pub thir_exprs_deref: Relation<(ThirExpr, ThirExpr)>,
        pub thir_exprs_binary: Relation<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>,
        pub thir_exprs_logical_op: Relation<(ThirExpr, ThirLogicalOp, ThirExpr, ThirExpr)>,
        pub thir_exprs_unary: Relation<(ThirExpr, ThirUnOp, ThirExpr)>,
        pub thir_exprs_cast: Relation<(ThirExpr, ThirExpr)>,
        pub thir_exprs_use: Relation<(ThirExpr, ThirExpr)>,
        pub thir_exprs_never_to_any: Relation<(ThirExpr, ThirExpr)>,
        pub thir_exprs_pointer_coercion: Relation<(ThirExpr, PointerCoercion, ThirExpr, bool)>,
        pub thir_exprs_loop: Relation<(ThirExpr, ThirExpr)>,
        pub thir_exprs_let: Relation<(ThirExpr, ThirExpr, ThirPat)>,
        pub thir_pats: Relation<(ThirPat, Type, Span)>,
        pub thir_exprs_match: Relation<(ThirExpr, ThirExpr, MatchSource)>,
        pub thir_match_arms: Relation<(ThirExpr, MatchArmIdx, ThirExpr, ThirExpr)>,
        pub thir_exprs_block: Relation<(ThirExpr, ThirBlock)>,
        pub thir_exprs_assign: Relation<(ThirExpr, ThirExpr, ThirExpr)>,
        pub thir_exprs_assign_op: Relation<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>,
        pub thir_exprs_field: Relation<(ThirExpr, ThirExpr, AdtVariantIndex)>,
        pub thir_exprs_index: Relation<(ThirExpr, ThirExpr, ThirExpr)>,
        pub thir_exprs_var_ref: Relation<(ThirExpr,)>,
        pub thir_exprs_upvar_ref: Relation<(ThirExpr, DefPath)>,
        pub thir_exprs_borrow: Relation<(ThirExpr, BorrowKind, ThirExpr)>,
        pub thir_exprs_raw_borrow: Relation<(ThirExpr, Mutability, ThirExpr)>,
        pub thir_exprs_break: Relation<(ThirExpr, ThirExpr)>,
        pub thir_exprs_continue: Relation<(ThirExpr,)>,
        pub thir_exprs_return: Relation<(ThirExpr, ThirExpr)>,
        pub thir_exprs_become: Relation<(ThirExpr, ThirExpr)>,
        pub thir_exprs_const_block: Relation<(ThirExpr, DefPath)>,
        pub thir_exprs_repeat: Relation<(ThirExpr, ThirExpr)>,
        pub thir_exprs_array: Relation<(ThirExpr,)>,
        pub thir_array_elements: Relation<(ThirExpr, u64, ThirExpr)>,
        pub thir_exprs_tuple: Relation<(ThirExpr,)>,
        pub thir_tuple_elements: Relation<(ThirExpr, TupleFieldIndex, ThirExpr)>,
        pub thir_exprs_adt: Relation<(ThirExpr, ThirExpr, AdtVariantIndex)>,
        pub thir_adt_field_expr: Relation<(ThirExpr, FieldIndex, ThirExpr)>,
        pub thir_exprs_place_type_ascription: Relation<(ThirExpr, ThirExpr, Span)>,
        pub thir_exprs_value_type_ascription: Relation<(ThirExpr, ThirExpr, Span)>,
        pub thir_exprs_closure: Relation<(ThirExpr, DefPath, Movability)>,
        pub thir_closure_upvars: Relation<(ThirExpr, u32, ThirExpr)>,
        pub thir_exprs_literal: Relation<(ThirExpr, LitKind, bool)>,
        pub thir_exprs_non_hir_literal: Relation<(ThirExpr, u128)>,
        pub thir_exprs_zst_literal: Relation<(ThirExpr,)>,
        pub thir_exprs_named_const: Relation<(ThirExpr, DefPath)>,
        pub thir_exprs_const_param: Relation<(ThirExpr, DefPath)>,
        pub thir_exprs_static_ref: Relation<(ThirExpr, Type, DefPath)>,
        pub thir_exprs_inline_asm: Relation<(ThirExpr,)>,
        pub thir_exprs_offset_of: Relation<(ThirExpr, Type)>,
        pub thir_exprs_thread_local_ref: Relation<(ThirExpr, DefPath)>,
        pub thir_exprs_yield: Relation<(ThirExpr, ThirExpr)>,
        pub static_definitions: Relation<(DefPath, Item, Module, Name, TyVisibility, Mutability)>,
        pub impl_definitions: Relation<(
            DefPath,
            Item,
            Module,
            Name,
            TyVisibility,
            Safety,
            ImplPolarity,
            Defaultness,
            Constness,
            Type,
        )>,
        pub trait_impls: Relation<(Item, Type, DefPath)>,
        pub global_asm_blocks: Relation<(DefPath, Item, Module, Name, TyVisibility)>,
        pub items: Relation<(DefPath, Item, Module, Name, TyVisibility)>,
        pub mir_cfgs: Relation<(Item, DefPath, Scope)>,
        pub subscopes: Relation<(Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span)>,
        pub spans: Relation<(
            Span,
            Span,
            SpanExpansionKind,
            InternedString,
            SpanFileName,
            u16,
            u16,
        )>,
        pub macro_expansions: Relation<(Span, InternedString, SpanFileName, u16, u16)>,
        pub crate_cfgs: Relation<(Build, CrateCfgKey, CrateCfgValue)>,
        pub crate_authors: Relation<(Build, InternedString)>,
        pub crate_keywords: Relation<(Build, InternedString)>,
        pub crate_categories: Relation<(Build, InternedString)>,
        pub type_defs: Relation<(Item, Type, DefPath, InternedString, TyVisibility, TyDefKind)>,
        pub types: Relation<(Type, TyKind)>,
        pub types_primitive: Relation<(Type, TyPrimitive)>,
        pub types_adt_def: Relation<(Type, DefPath, AdtKind, bool, bool)>,
        pub types_adt_variant: Relation<(Type, AdtVariantIndex, DefPath, InternedString)>,
        pub types_adt_field: Relation<(
            Field,
            Type,
            AdtVariantIndex,
            DefPath,
            InternedString,
            TyVisibility,
            Type,
        )>,
        pub types_adt_field_visible_in: Relation<(Field, DefPath)>,
        pub types_foreign: Relation<(Type, DefPath)>,
        pub types_array: Relation<(Type, Type)>,
        pub types_slice: Relation<(Type, Type)>,
        pub types_raw_ptr: Relation<(Type, Type, Mutability)>,
        pub types_ref: Relation<(Type, Type, Mutability)>,
        pub types_fn_def: Relation<(Type, DefPath)>,
        pub types_fn_ptr: Relation<(Type,)>,
        pub types_dynamic: Relation<(Type,)>,
        pub types_dynamic_trait: Relation<(Type, DefPath, bool)>,
        pub types_closure: Relation<(Type, DefPath)>,
        pub types_coroutine: Relation<(Type, DefPath)>,
        pub types_coroutine_witness: Relation<(Type,)>,
        pub types_coroutine_closure: Relation<(Type, DefPath)>,
        pub types_pat: Relation<(Type,)>,
        pub types_tuple: Relation<(Type,)>,
        pub types_tuple_element: Relation<(Type, TupleFieldIndex, Type)>,
        pub types_projection: Relation<(Type, DefPath, DefPath)>,
        pub types_opaque: Relation<(Type, DefPath)>,
        pub types_inherent: Relation<(Type, DefPath)>,
        pub types_weak: Relation<(Type, DefPath)>,
        pub types_param: Relation<(Type, u32, InternedString)>,
        pub traits: Relation<(
            Item,
            DefPath,
            InternedString,
            TyVisibility,
            bool,
            bool,
            Safety,
        )>,
        pub trait_items: Relation<(Item, DefPath, Defaultness)>,
        pub basic_blocks: Relation<(BasicBlock, DefPath, BasicBlockKind)>,
        pub statements: Relation<(Statement, BasicBlock, StatementIndex, StatementKind, Scope)>,
        pub statements_assign_use: Relation<(Statement, Type, Operand)>,
        pub statements_assign_thead_local_ref: Relation<(Statement, Type, DefPath)>,
        pub statements_assign_repeat: Relation<(Statement, Type, Operand, u64)>,
        pub statements_assign_ref: Relation<(Statement, Type, Type, BorrowKind)>,
        pub statements_assign_address: Relation<(Statement, Type, Type, Mutability)>,
        pub statements_assign_len: Relation<(Statement, Type, Type)>,
        pub statements_assign_cast: Relation<(Statement, Type, CastKind, Operand, Type)>,
        pub statements_assign_binary_op: Relation<(Statement, Type, BinOp, Operand, Operand)>,
        pub statements_assign_checked_binary_op:
            Relation<(Statement, Type, BinOp, Operand, Operand)>,
        pub statements_assign_nullary_op: Relation<(Statement, Type, NullOp, Type)>,
        pub statements_assign_unary_op: Relation<(Statement, Type, UnOp, Operand)>,
        pub statements_assign_discriminant: Relation<(Statement, Type, Type)>,
        pub statements_assign_aggregate: Relation<(Statement, Type, AggregateKind)>,
        pub statements_assign_aggregate_operands: Relation<(Statement, OperandIndex, Operand)>,
        pub statements_assign_shallow_init_box: Relation<(Statement, Operand, Type)>,
        pub statements_assign_copy_for_deref: Relation<(Statement, Type)>,
        pub statements_inline_asm_inputs: Relation<(Statement, Operand)>,
        pub statements_inline_asm_outputs: Relation<(Statement, Type)>,
        pub operands: Relation<(Operand, OperandKind, Type)>,
        pub terminators: Relation<(BasicBlock, TerminatorKind, Scope)>,
        pub terminators_goto: Relation<(BasicBlock, BasicBlock)>,
        pub terminators_switch_int: Relation<(BasicBlock, Operand)>,
        pub terminators_switch_int_targets: Relation<(BasicBlock, u128, BasicBlock)>,
        pub terminators_drop: Relation<(BasicBlock, Type, BasicBlock)>,
        pub terminators_drop_and_replace:
            Relation<(BasicBlock, Type, Operand, BasicBlock, BasicBlock)>,
        pub terminators_call: Relation<(
            BasicBlock,
            FunctionCall,
            Operand,
            Safety,
            Abi,
            Type,
            BasicBlock,
            Span,
        )>,
        pub terminators_call_arg: Relation<(FunctionCall, CallArgIndex, Operand)>,
        pub terminators_call_const_target: Relation<(FunctionCall, DefPath)>,
        pub terminators_call_const_target_desc:
            Relation<(FunctionCall, InternedString, InternedString, InternedString)>,
        pub terminators_call_const_target_self: Relation<(FunctionCall, Type)>,
        pub terminators_call_macro_backtrace: Relation<(FunctionCall, InternedString)>,
        pub terminators_assert: Relation<(BasicBlock, Operand, bool, BasicBlock)>,
        pub terminators_yield: Relation<(BasicBlock, Operand, BasicBlock, BasicBlock)>,
        pub terminators_false_edges: Relation<(BasicBlock, BasicBlock, BasicBlock)>,
        pub terminators_false_unwind: Relation<(BasicBlock, BasicBlock)>,
        pub terminators_inline_asm: Relation<(BasicBlock,)>,
        pub terminators_unwind_action: Relation<(BasicBlock, UnwindAction, BasicBlock)>,
    }
    #[derive(Deserialize, Serialize)]
    #[doc = r" Counters for generating unique identifiers."]
    pub struct Counters {
        pub(crate) modules: u32,
        pub(crate) items: u32,
        pub(crate) scopes: u32,
        pub(crate) functioncalls: u32,
        pub(crate) spans: u64,
        pub(crate) types: u64,
        pub(crate) fields: u64,
        pub(crate) operands: u64,
        pub(crate) basicblocks: u64,
        pub(crate) statements: u64,
        pub(crate) thirblocks: u64,
        pub(crate) thirexprs: u64,
        pub(crate) thirpats: u64,
        pub(crate) thirstmts: u64,
    }
    impl Counters {
        fn get_fresh_module(&mut self) -> Module {
            let value = self.modules.into();
            self.modules += 1;
            value
        }
        fn get_fresh_item(&mut self) -> Item {
            let value = self.items.into();
            self.items += 1;
            value
        }
        fn get_fresh_scope(&mut self) -> Scope {
            let value = self.scopes.into();
            self.scopes += 1;
            value
        }
        fn get_fresh_functioncall(&mut self) -> FunctionCall {
            let value = self.functioncalls.into();
            self.functioncalls += 1;
            value
        }
        fn get_fresh_span(&mut self) -> Span {
            let value = self.spans.into();
            self.spans += 1;
            value
        }
        fn get_root_parent_span(&mut self) -> Span {
            0u64.into()
        }
        fn get_fresh_type(&mut self) -> Type {
            let value = self.types.into();
            self.types += 1;
            value
        }
        fn get_fresh_field(&mut self) -> Field {
            let value = self.fields.into();
            self.fields += 1;
            value
        }
        fn get_fresh_operand(&mut self) -> Operand {
            let value = self.operands.into();
            self.operands += 1;
            value
        }
        fn get_fresh_basicblock(&mut self) -> BasicBlock {
            let value = self.basicblocks.into();
            self.basicblocks += 1;
            value
        }
        fn get_no_block(&mut self) -> BasicBlock {
            0u64.into()
        }
        fn get_fresh_statement(&mut self) -> Statement {
            let value = self.statements.into();
            self.statements += 1;
            value
        }
        fn get_fresh_thirblock(&mut self) -> ThirBlock {
            let value = self.thirblocks.into();
            self.thirblocks += 1;
            value
        }
        fn get_no_thir_block(&mut self) -> ThirBlock {
            0u64.into()
        }
        fn get_fresh_thirexpr(&mut self) -> ThirExpr {
            let value = self.thirexprs.into();
            self.thirexprs += 1;
            value
        }
        fn get_no_thir_expr(&mut self) -> ThirExpr {
            0u64.into()
        }
        fn get_fresh_thirpat(&mut self) -> ThirPat {
            let value = self.thirpats.into();
            self.thirpats += 1;
            value
        }
        fn get_no_thir_pat(&mut self) -> ThirPat {
            0u64.into()
        }
        fn get_fresh_thirstmt(&mut self) -> ThirStmt {
            let value = self.thirstmts.into();
            self.thirstmts += 1;
            value
        }
        fn get_no_thir_stmt(&mut self) -> ThirStmt {
            0u64.into()
        }
    }
    impl Default for Counters {
        fn default() -> Self {
            Self {
                modules: 0,
                items: 0,
                scopes: 0,
                functioncalls: 0,
                spans: 1,
                types: 0,
                fields: 0,
                operands: 0,
                basicblocks: 1,
                statements: 0,
                thirblocks: 1,
                thirexprs: 1,
                thirpats: 1,
                thirstmts: 1,
            }
        }
    }
    #[derive(Default, Deserialize, Serialize)]
    pub struct Tables {
        #[doc = r" Relations between Rust program elements."]
        pub(crate) relations: Relations,
        #[doc = r" Counters used for generating ids."]
        pub(crate) counters: Counters,
        #[doc = r" Interning tables that link typed ids to untyped interning ids."]
        pub(crate) interning_tables: InterningTables,
    }
    impl Tables {
        pub fn register_strings(&mut self, value_1: String) -> InternedString {
            let value_0 = self.interning_tables.strings.intern(value_1);
            value_0
        }
        pub fn register_package_names(&mut self, value_2: String) -> Package {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.package_names.intern(value_1);
            value_0
        }
        pub fn register_package_versions(&mut self, value_2: String) -> PackageVersion {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.package_versions.intern(value_1);
            value_0
        }
        pub fn register_crate_names(&mut self, value_2: String) -> Krate {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.crate_names.intern(value_1);
            value_0
        }
        pub fn register_editions(&mut self, value_2: String) -> Edition {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.editions.intern(value_1);
            value_0
        }
        pub fn register_names(&mut self, value_2: String) -> Name {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.names.intern(value_1);
            value_0
        }
        pub fn register_relative_def_paths(&mut self, value_2: String) -> RelativeDefId {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.relative_def_paths.intern(value_1);
            value_0
        }
        pub fn register_summary_keys(&mut self, value_2: String) -> SummaryId {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.summary_keys.intern(value_1);
            value_0
        }
        pub fn register_abis(&mut self, value_2: String) -> Abi {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.abis.intern(value_1);
            value_0
        }
        pub fn register_def_paths(
            &mut self,
            value_krate_2: String,
            value_cratehash_0: CrateHash,
            value_relativedefid_2: String,
            value_defpathhash_0: DefPathHash,
            value_summaryid_2: String,
        ) -> DefPath {
            let value_krate_1 = self.interning_tables.strings.intern(value_krate_2);
            let value_krate_0 = self.interning_tables.crate_names.intern(value_krate_1);
            let value_relativedefid_1 = self.interning_tables.strings.intern(value_relativedefid_2);
            let value_relativedefid_0 = self
                .interning_tables
                .relative_def_paths
                .intern(value_relativedefid_1);
            let value_summaryid_1 = self.interning_tables.strings.intern(value_summaryid_2);
            let value_summaryid_0 = self.interning_tables.summary_keys.intern(value_summaryid_1);
            self.interning_tables.def_paths.intern((
                value_krate_0,
                value_cratehash_0,
                value_relativedefid_0,
                value_defpathhash_0,
                value_summaryid_0,
            ))
        }
        pub fn register_builds(
            &mut self,
            value_package_2: String,
            value_packageversion_2: String,
            value_krate_2: String,
            value_cratehash_0: CrateHash,
            value_edition_2: String,
        ) -> Build {
            let value_package_1 = self.interning_tables.strings.intern(value_package_2);
            let value_package_0 = self.interning_tables.package_names.intern(value_package_1);
            let value_packageversion_1 =
                self.interning_tables.strings.intern(value_packageversion_2);
            let value_packageversion_0 = self
                .interning_tables
                .package_versions
                .intern(value_packageversion_1);
            let value_krate_1 = self.interning_tables.strings.intern(value_krate_2);
            let value_krate_0 = self.interning_tables.crate_names.intern(value_krate_1);
            let value_edition_1 = self.interning_tables.strings.intern(value_edition_2);
            let value_edition_0 = self.interning_tables.editions.intern(value_edition_1);
            self.interning_tables.builds.intern((
                value_package_0,
                value_packageversion_0,
                value_krate_0,
                value_cratehash_0,
                value_edition_0,
            ))
        }
        pub fn register_span_file_names(&mut self, value_2: String) -> SpanFileName {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.span_file_names.intern(value_1);
            value_0
        }
        pub fn register_crate_cfg_keys(&mut self, value_2: String) -> CrateCfgKey {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.crate_cfg_keys.intern(value_1);
            value_0
        }
        pub fn register_crate_cfg_values(&mut self, value_2: String) -> CrateCfgValue {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.crate_cfg_values.intern(value_1);
            value_0
        }
        pub fn register_type_kinds(&mut self, value_2: String) -> TyKind {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.type_kinds.intern(value_1);
            value_0
        }
        pub fn register_statement_kinds(&mut self, value_2: String) -> StatementKind {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.statement_kinds.intern(value_1);
            value_0
        }
        pub fn register_binary_op_kind(&mut self, value_2: String) -> BinOp {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.binary_op_kind.intern(value_1);
            value_0
        }
        pub fn register_nullary_op_kind(&mut self, value_2: String) -> NullOp {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.nullary_op_kind.intern(value_1);
            value_0
        }
        pub fn register_unary_op_kind(&mut self, value_2: String) -> UnOp {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.unary_op_kind.intern(value_1);
            value_0
        }
        pub fn register_terminator_kinds(&mut self, value_2: String) -> TerminatorKind {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.terminator_kinds.intern(value_1);
            value_0
        }
        pub fn register_thir_binary_op_kind(&mut self, value_2: String) -> ThirBinOp {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.thir_binary_op_kind.intern(value_1);
            value_0
        }
        pub fn register_thir_logical_op_kind(&mut self, value_2: String) -> ThirLogicalOp {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.thir_logical_op_kind.intern(value_1);
            value_0
        }
        pub fn register_thir_unary_op_kind(&mut self, value_2: String) -> ThirUnOp {
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.thir_unary_op_kind.intern(value_1);
            value_0
        }
        pub fn register_def_path_span(&mut self, def_path_0: DefPath, span_0: Span) -> () {
            self.relations.def_path_span.insert((def_path_0, span_0));
            ()
        }
        pub fn register_type_description(
            &mut self,
            ty_0: Type,
            description_1: String,
            generics_1: String,
        ) -> () {
            let description_0 = self.interning_tables.strings.intern(description_1);
            let generics_0 = self.interning_tables.strings.intern(generics_1);
            self.relations
                .type_description
                .insert((ty_0, description_0, generics_0));
            ()
        }
        pub fn register_build_crate_types(&mut self, build_0: Build, crate_type_1: String) -> () {
            let crate_type_0 = self.interning_tables.strings.intern(crate_type_1);
            self.relations
                .build_crate_types
                .insert((build_0, crate_type_0));
            ()
        }
        pub fn register_root_modules(&mut self, build_0: Build) -> (Module,) {
            let root_module = self.counters.get_fresh_module();
            self.relations.root_modules.insert((build_0, root_module));
            (root_module,)
        }
        pub fn register_submodules(
            &mut self,
            def_path_0: DefPath,
            parent_0: Module,
            name_2: String,
            visibility_0: TyVisibility,
            abi_2: String,
        ) -> (Module,) {
            let child = self.counters.get_fresh_module();
            let name_1 = self.interning_tables.strings.intern(name_2);
            let name_0 = self.interning_tables.names.intern(name_1);
            let abi_1 = self.interning_tables.strings.intern(abi_2);
            let abi_0 = self.interning_tables.abis.intern(abi_1);
            self.relations.submodules.insert((
                def_path_0,
                parent_0,
                child,
                name_0,
                visibility_0,
                abi_0,
            ));
            (child,)
        }
        pub fn register_function_definitions(
            &mut self,
            def_path_0: DefPath,
            module_0: Module,
            visibility_0: TyVisibility,
            unsafety_0: Safety,
            abi_2: String,
            return_ty_0: Type,
        ) -> (Item,) {
            let item = self.counters.get_fresh_item();
            let abi_1 = self.interning_tables.strings.intern(abi_2);
            let abi_0 = self.interning_tables.abis.intern(abi_1);
            self.relations.function_definitions.insert((
                item,
                def_path_0,
                module_0,
                visibility_0,
                unsafety_0,
                abi_0,
                return_ty_0,
            ));
            (item,)
        }
        pub fn register_function_parameter_types(
            &mut self,
            function_0: Item,
            index_0: FnParamIndex,
            typ_0: Type,
        ) -> () {
            self.relations
                .function_parameter_types
                .insert((function_0, index_0, typ_0));
            ()
        }
        pub fn register_function_unsafe_use(
            &mut self,
            def_path_0: DefPath,
            uses_unsafe_0: bool,
        ) -> () {
            self.relations
                .function_unsafe_use
                .insert((def_path_0, uses_unsafe_0));
            ()
        }
        pub fn register_function_unsafe_reasons(
            &mut self,
            def_path_0: DefPath,
            index_0: u32,
            reason_1: String,
        ) -> () {
            let reason_0 = self.interning_tables.strings.intern(reason_1);
            self.relations
                .function_unsafe_reasons
                .insert((def_path_0, index_0, reason_0));
            ()
        }
        pub fn register_thir_bodies(&mut self, item_0: Item, def_path_0: DefPath) -> (ThirBlock,) {
            let body = self.counters.get_fresh_thirblock();
            self.relations
                .thir_bodies
                .insert((item_0, def_path_0, body));
            (body,)
        }
        pub fn register_thir_blocks(
            &mut self,
            parent_0: ThirBlock,
            safety_0: ScopeSafety,
            check_mode_0: BlockCheckMode,
            span_0: Span,
        ) -> (ThirBlock,) {
            let block = self.counters.get_fresh_thirblock();
            self.relations
                .thir_blocks
                .insert((parent_0, block, safety_0, check_mode_0, span_0));
            (block,)
        }
        pub fn register_thir_stmts(
            &mut self,
            stmt_0: ThirStmt,
            block_0: ThirBlock,
            closest_unsafe_block_0: ThirBlock,
            index_0: StatementIndex,
        ) -> () {
            self.relations
                .thir_stmts
                .insert((stmt_0, block_0, closest_unsafe_block_0, index_0));
            ()
        }
        pub fn register_thir_stmts_expr(&mut self, expr_0: ThirExpr) -> (ThirStmt,) {
            let stmt = self.counters.get_fresh_thirstmt();
            self.relations.thir_stmts_expr.insert((stmt, expr_0));
            (stmt,)
        }
        pub fn register_thir_stmts_let(
            &mut self,
            initializer_0: ThirExpr,
            else_block_0: ThirBlock,
            span_0: Span,
        ) -> (ThirStmt,) {
            let stmt = self.counters.get_fresh_thirstmt();
            self.relations
                .thir_stmts_let
                .insert((stmt, initializer_0, else_block_0, span_0));
            (stmt,)
        }
        pub fn register_thir_block_expr(&mut self, block_0: ThirBlock, expr_0: ThirExpr) -> () {
            self.relations.thir_block_expr.insert((block_0, expr_0));
            ()
        }
        pub fn register_thir_exprs(
            &mut self,
            expr_0: ThirExpr,
            block_0: ThirBlock,
            closest_unsafe_block_0: ThirBlock,
            ty_0: Type,
            span_0: Span,
        ) -> () {
            self.relations.thir_exprs.insert((
                expr_0,
                block_0,
                closest_unsafe_block_0,
                ty_0,
                span_0,
            ));
            ()
        }
        pub fn register_thir_exprs_scope(&mut self, inner_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_scope.insert((expr, inner_0));
            (expr,)
        }
        pub fn register_thir_exprs_box(&mut self, inner_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_box.insert((expr, inner_0));
            (expr,)
        }
        pub fn register_thir_exprs_if(
            &mut self,
            cond_0: ThirExpr,
            then_expr_0: ThirExpr,
            else_expr_0: ThirExpr,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_if
                .insert((expr, cond_0, then_expr_0, else_expr_0));
            (expr,)
        }
        pub fn register_thir_exprs_call(
            &mut self,
            ty_0: Type,
            fun_0: ThirExpr,
            unsafety_0: Safety,
            abi_2: String,
            return_ty_0: Type,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            let abi_1 = self.interning_tables.strings.intern(abi_2);
            let abi_0 = self.interning_tables.abis.intern(abi_1);
            self.relations.thir_exprs_call.insert((
                expr,
                ty_0,
                fun_0,
                unsafety_0,
                abi_0,
                return_ty_0,
            ));
            (expr,)
        }
        pub fn register_thir_exprs_call_arg(
            &mut self,
            call_0: ThirExpr,
            index_0: ThirCallArgIndex,
            arg_0: ThirExpr,
        ) -> () {
            self.relations
                .thir_exprs_call_arg
                .insert((call_0, index_0, arg_0));
            ()
        }
        pub fn register_thir_exprs_call_const_target(
            &mut self,
            fun_0: ThirExpr,
            def_id_0: DefPath,
        ) -> () {
            self.relations
                .thir_exprs_call_const_target
                .insert((fun_0, def_id_0));
            ()
        }
        pub fn register_thir_exprs_call_const_target_desc(
            &mut self,
            fun_0: ThirExpr,
            target_1: String,
            function_generics_1: String,
            type_generics_1: String,
        ) -> () {
            let target_0 = self.interning_tables.strings.intern(target_1);
            let function_generics_0 = self.interning_tables.strings.intern(function_generics_1);
            let type_generics_0 = self.interning_tables.strings.intern(type_generics_1);
            self.relations.thir_exprs_call_const_target_desc.insert((
                fun_0,
                target_0,
                function_generics_0,
                type_generics_0,
            ));
            ()
        }
        pub fn register_thir_exprs_call_const_target_self(
            &mut self,
            fun_0: ThirExpr,
            typ_0: Type,
        ) -> () {
            self.relations
                .thir_exprs_call_const_target_self
                .insert((fun_0, typ_0));
            ()
        }
        pub fn register_thir_exprs_deref(&mut self, inner_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_deref.insert((expr, inner_0));
            (expr,)
        }
        pub fn register_thir_exprs_binary(
            &mut self,
            op_2: String,
            lhs_0: ThirExpr,
            rhs_0: ThirExpr,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            let op_1 = self.interning_tables.strings.intern(op_2);
            let op_0 = self.interning_tables.thir_binary_op_kind.intern(op_1);
            self.relations
                .thir_exprs_binary
                .insert((expr, op_0, lhs_0, rhs_0));
            (expr,)
        }
        pub fn register_thir_exprs_logical_op(
            &mut self,
            op_2: String,
            lhs_0: ThirExpr,
            rhs_0: ThirExpr,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            let op_1 = self.interning_tables.strings.intern(op_2);
            let op_0 = self.interning_tables.thir_logical_op_kind.intern(op_1);
            self.relations
                .thir_exprs_logical_op
                .insert((expr, op_0, lhs_0, rhs_0));
            (expr,)
        }
        pub fn register_thir_exprs_unary(&mut self, op_2: String, arg_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            let op_1 = self.interning_tables.strings.intern(op_2);
            let op_0 = self.interning_tables.thir_unary_op_kind.intern(op_1);
            self.relations.thir_exprs_unary.insert((expr, op_0, arg_0));
            (expr,)
        }
        pub fn register_thir_exprs_cast(&mut self, source_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_cast.insert((expr, source_0));
            (expr,)
        }
        pub fn register_thir_exprs_use(&mut self, source_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_use.insert((expr, source_0));
            (expr,)
        }
        pub fn register_thir_exprs_never_to_any(&mut self, source_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_never_to_any
                .insert((expr, source_0));
            (expr,)
        }
        pub fn register_thir_exprs_pointer_coercion(
            &mut self,
            cast_0: PointerCoercion,
            source_0: ThirExpr,
            is_from_as_cast_0: bool,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_pointer_coercion.insert((
                expr,
                cast_0,
                source_0,
                is_from_as_cast_0,
            ));
            (expr,)
        }
        pub fn register_thir_exprs_loop(&mut self, body_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_loop.insert((expr, body_0));
            (expr,)
        }
        pub fn register_thir_exprs_let(
            &mut self,
            inner_0: ThirExpr,
            pat_0: ThirPat,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_let.insert((expr, inner_0, pat_0));
            (expr,)
        }
        pub fn register_thir_pats(&mut self, ty_0: Type, span_0: Span) -> (ThirPat,) {
            let pat = self.counters.get_fresh_thirpat();
            self.relations.thir_pats.insert((pat, ty_0, span_0));
            (pat,)
        }
        pub fn register_thir_exprs_match(
            &mut self,
            scrutinee_0: ThirExpr,
            match_source_0: MatchSource,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_match
                .insert((expr, scrutinee_0, match_source_0));
            (expr,)
        }
        pub fn register_thir_match_arms(
            &mut self,
            match_expr_0: ThirExpr,
            arm_idx_0: MatchArmIdx,
            guard_0: ThirExpr,
            body_0: ThirExpr,
        ) -> () {
            self.relations
                .thir_match_arms
                .insert((match_expr_0, arm_idx_0, guard_0, body_0));
            ()
        }
        pub fn register_thir_exprs_block(&mut self, block_0: ThirBlock) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_block.insert((expr, block_0));
            (expr,)
        }
        pub fn register_thir_exprs_assign(
            &mut self,
            lhs_0: ThirExpr,
            rhs_0: ThirExpr,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_assign
                .insert((expr, lhs_0, rhs_0));
            (expr,)
        }
        pub fn register_thir_exprs_assign_op(
            &mut self,
            op_2: String,
            lhs_0: ThirExpr,
            rhs_0: ThirExpr,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            let op_1 = self.interning_tables.strings.intern(op_2);
            let op_0 = self.interning_tables.thir_binary_op_kind.intern(op_1);
            self.relations
                .thir_exprs_assign_op
                .insert((expr, op_0, lhs_0, rhs_0));
            (expr,)
        }
        pub fn register_thir_exprs_field(
            &mut self,
            lhs_0: ThirExpr,
            variant_idx_0: AdtVariantIndex,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_field
                .insert((expr, lhs_0, variant_idx_0));
            (expr,)
        }
        pub fn register_thir_exprs_index(
            &mut self,
            lhs_0: ThirExpr,
            index_0: ThirExpr,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_index
                .insert((expr, lhs_0, index_0));
            (expr,)
        }
        pub fn register_thir_exprs_var_ref(&mut self) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_var_ref.insert((expr,));
            (expr,)
        }
        pub fn register_thir_exprs_upvar_ref(&mut self, closure_def_id_0: DefPath) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_upvar_ref
                .insert((expr, closure_def_id_0));
            (expr,)
        }
        pub fn register_thir_exprs_borrow(
            &mut self,
            borrow_kind_0: BorrowKind,
            arg_0: ThirExpr,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_borrow
                .insert((expr, borrow_kind_0, arg_0));
            (expr,)
        }
        pub fn register_thir_exprs_raw_borrow(
            &mut self,
            mutability_0: Mutability,
            arg_0: ThirExpr,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_raw_borrow
                .insert((expr, mutability_0, arg_0));
            (expr,)
        }
        pub fn register_thir_exprs_break(&mut self, value_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_break.insert((expr, value_0));
            (expr,)
        }
        pub fn register_thir_exprs_continue(&mut self) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_continue.insert((expr,));
            (expr,)
        }
        pub fn register_thir_exprs_return(&mut self, value_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_return.insert((expr, value_0));
            (expr,)
        }
        pub fn register_thir_exprs_become(&mut self, value_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_become.insert((expr, value_0));
            (expr,)
        }
        pub fn register_thir_exprs_const_block(&mut self, did_0: DefPath) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_const_block.insert((expr, did_0));
            (expr,)
        }
        pub fn register_thir_exprs_repeat(&mut self, value_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_repeat.insert((expr, value_0));
            (expr,)
        }
        pub fn register_thir_exprs_array(&mut self) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_array.insert((expr,));
            (expr,)
        }
        pub fn register_thir_array_elements(
            &mut self,
            array_expr_0: ThirExpr,
            index_0: u64,
            element_0: ThirExpr,
        ) -> () {
            self.relations
                .thir_array_elements
                .insert((array_expr_0, index_0, element_0));
            ()
        }
        pub fn register_thir_exprs_tuple(&mut self) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_tuple.insert((expr,));
            (expr,)
        }
        pub fn register_thir_tuple_elements(
            &mut self,
            tuple_expr_0: ThirExpr,
            index_0: TupleFieldIndex,
            element_0: ThirExpr,
        ) -> () {
            self.relations
                .thir_tuple_elements
                .insert((tuple_expr_0, index_0, element_0));
            ()
        }
        pub fn register_thir_exprs_adt(
            &mut self,
            base_0: ThirExpr,
            variant_idx_0: AdtVariantIndex,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_adt
                .insert((expr, base_0, variant_idx_0));
            (expr,)
        }
        pub fn register_thir_adt_field_expr(
            &mut self,
            adt_0: ThirExpr,
            field_idx_0: FieldIndex,
            expr_0: ThirExpr,
        ) -> () {
            self.relations
                .thir_adt_field_expr
                .insert((adt_0, field_idx_0, expr_0));
            ()
        }
        pub fn register_thir_exprs_place_type_ascription(
            &mut self,
            source_0: ThirExpr,
            user_ty_span_0: Span,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_place_type_ascription.insert((
                expr,
                source_0,
                user_ty_span_0,
            ));
            (expr,)
        }
        pub fn register_thir_exprs_value_type_ascription(
            &mut self,
            source_0: ThirExpr,
            user_ty_span_0: Span,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_value_type_ascription.insert((
                expr,
                source_0,
                user_ty_span_0,
            ));
            (expr,)
        }
        pub fn register_thir_exprs_closure(
            &mut self,
            closure_id_0: DefPath,
            movability_0: Movability,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_closure
                .insert((expr, closure_id_0, movability_0));
            (expr,)
        }
        pub fn register_thir_closure_upvars(
            &mut self,
            closure_expr_0: ThirExpr,
            index_0: u32,
            upvar_0: ThirExpr,
        ) -> () {
            self.relations
                .thir_closure_upvars
                .insert((closure_expr_0, index_0, upvar_0));
            ()
        }
        pub fn register_thir_exprs_literal(&mut self, lit_0: LitKind, neg_0: bool) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_literal
                .insert((expr, lit_0, neg_0));
            (expr,)
        }
        pub fn register_thir_exprs_non_hir_literal(&mut self, scalar_0: u128) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_non_hir_literal
                .insert((expr, scalar_0));
            (expr,)
        }
        pub fn register_thir_exprs_zst_literal(&mut self) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_zst_literal.insert((expr,));
            (expr,)
        }
        pub fn register_thir_exprs_named_const(&mut self, def_id_0: DefPath) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_named_const
                .insert((expr, def_id_0));
            (expr,)
        }
        pub fn register_thir_exprs_const_param(&mut self, def_id_0: DefPath) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_const_param
                .insert((expr, def_id_0));
            (expr,)
        }
        pub fn register_thir_exprs_static_ref(
            &mut self,
            ty_0: Type,
            def_id_0: DefPath,
        ) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_static_ref
                .insert((expr, ty_0, def_id_0));
            (expr,)
        }
        pub fn register_thir_exprs_inline_asm(&mut self) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_inline_asm.insert((expr,));
            (expr,)
        }
        pub fn register_thir_exprs_offset_of(&mut self, container_0: Type) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_offset_of
                .insert((expr, container_0));
            (expr,)
        }
        pub fn register_thir_exprs_thread_local_ref(&mut self, def_id_0: DefPath) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations
                .thir_exprs_thread_local_ref
                .insert((expr, def_id_0));
            (expr,)
        }
        pub fn register_thir_exprs_yield(&mut self, value_0: ThirExpr) -> (ThirExpr,) {
            let expr = self.counters.get_fresh_thirexpr();
            self.relations.thir_exprs_yield.insert((expr, value_0));
            (expr,)
        }
        pub fn register_static_definitions(
            &mut self,
            def_path_0: DefPath,
            module_0: Module,
            name_2: String,
            visibility_0: TyVisibility,
            mutability_0: Mutability,
        ) -> (Item,) {
            let item = self.counters.get_fresh_item();
            let name_1 = self.interning_tables.strings.intern(name_2);
            let name_0 = self.interning_tables.names.intern(name_1);
            self.relations.static_definitions.insert((
                def_path_0,
                item,
                module_0,
                name_0,
                visibility_0,
                mutability_0,
            ));
            (item,)
        }
        pub fn register_impl_definitions(
            &mut self,
            def_path_0: DefPath,
            module_0: Module,
            name_2: String,
            visibility_0: TyVisibility,
            unsafety_0: Safety,
            polarity_0: ImplPolarity,
            defaultness_0: Defaultness,
            constness_0: Constness,
            typ_0: Type,
        ) -> (Item,) {
            let item = self.counters.get_fresh_item();
            let name_1 = self.interning_tables.strings.intern(name_2);
            let name_0 = self.interning_tables.names.intern(name_1);
            self.relations.impl_definitions.insert((
                def_path_0,
                item,
                module_0,
                name_0,
                visibility_0,
                unsafety_0,
                polarity_0,
                defaultness_0,
                constness_0,
                typ_0,
            ));
            (item,)
        }
        pub fn register_trait_impls(
            &mut self,
            item_0: Item,
            typ_0: Type,
            trait_def_path_0: DefPath,
        ) -> () {
            self.relations
                .trait_impls
                .insert((item_0, typ_0, trait_def_path_0));
            ()
        }
        pub fn register_global_asm_blocks(
            &mut self,
            def_path_0: DefPath,
            module_0: Module,
            name_2: String,
            visibility_0: TyVisibility,
        ) -> (Item,) {
            let item = self.counters.get_fresh_item();
            let name_1 = self.interning_tables.strings.intern(name_2);
            let name_0 = self.interning_tables.names.intern(name_1);
            self.relations.global_asm_blocks.insert((
                def_path_0,
                item,
                module_0,
                name_0,
                visibility_0,
            ));
            (item,)
        }
        pub fn register_items(
            &mut self,
            def_path_0: DefPath,
            module_0: Module,
            name_2: String,
            visibility_0: TyVisibility,
        ) -> (Item,) {
            let item = self.counters.get_fresh_item();
            let name_1 = self.interning_tables.strings.intern(name_2);
            let name_0 = self.interning_tables.names.intern(name_1);
            self.relations
                .items
                .insert((def_path_0, item, module_0, name_0, visibility_0));
            (item,)
        }
        pub fn register_mir_cfgs(&mut self, item_0: Item, body_def_path_0: DefPath) -> (Scope,) {
            let root_scope = self.counters.get_fresh_scope();
            self.relations
                .mir_cfgs
                .insert((item_0, body_def_path_0, root_scope));
            (root_scope,)
        }
        pub fn register_subscopes(
            &mut self,
            parent_0: Scope,
            safety_0: ScopeSafety,
            check_mode_0: BlockCheckMode,
            explicit_unsafe_group_0: u32,
            span_0: Span,
        ) -> (Scope,) {
            let child = self.counters.get_fresh_scope();
            self.relations.subscopes.insert((
                parent_0,
                child,
                safety_0,
                check_mode_0,
                explicit_unsafe_group_0,
                span_0,
            ));
            (child,)
        }
        pub fn register_spans(
            &mut self,
            call_site_span_0: Span,
            expansion_kind_0: SpanExpansionKind,
            expansion_kind_descr_1: String,
            file_name_2: String,
            line_0: u16,
            col_0: u16,
        ) -> (Span,) {
            let span = self.counters.get_fresh_span();
            let expansion_kind_descr_0 =
                self.interning_tables.strings.intern(expansion_kind_descr_1);
            let file_name_1 = self.interning_tables.strings.intern(file_name_2);
            let file_name_0 = self.interning_tables.span_file_names.intern(file_name_1);
            self.relations.spans.insert((
                span,
                call_site_span_0,
                expansion_kind_0,
                expansion_kind_descr_0,
                file_name_0,
                line_0,
                col_0,
            ));
            (span,)
        }
        pub fn register_macro_expansions(
            &mut self,
            span_0: Span,
            macro_symbol_1: String,
            macro_definition_file_name_2: String,
            line_0: u16,
            col_0: u16,
        ) -> () {
            let macro_symbol_0 = self.interning_tables.strings.intern(macro_symbol_1);
            let macro_definition_file_name_1 = self
                .interning_tables
                .strings
                .intern(macro_definition_file_name_2);
            let macro_definition_file_name_0 = self
                .interning_tables
                .span_file_names
                .intern(macro_definition_file_name_1);
            self.relations.macro_expansions.insert((
                span_0,
                macro_symbol_0,
                macro_definition_file_name_0,
                line_0,
                col_0,
            ));
            ()
        }
        pub fn register_crate_cfgs(
            &mut self,
            build_0: Build,
            key_2: String,
            value_2: String,
        ) -> () {
            let key_1 = self.interning_tables.strings.intern(key_2);
            let key_0 = self.interning_tables.crate_cfg_keys.intern(key_1);
            let value_1 = self.interning_tables.strings.intern(value_2);
            let value_0 = self.interning_tables.crate_cfg_values.intern(value_1);
            self.relations.crate_cfgs.insert((build_0, key_0, value_0));
            ()
        }
        pub fn register_crate_authors(&mut self, build_0: Build, author_1: String) -> () {
            let author_0 = self.interning_tables.strings.intern(author_1);
            self.relations.crate_authors.insert((build_0, author_0));
            ()
        }
        pub fn register_crate_keywords(&mut self, build_0: Build, keyword_1: String) -> () {
            let keyword_0 = self.interning_tables.strings.intern(keyword_1);
            self.relations.crate_keywords.insert((build_0, keyword_0));
            ()
        }
        pub fn register_crate_categories(&mut self, build_0: Build, category_1: String) -> () {
            let category_0 = self.interning_tables.strings.intern(category_1);
            self.relations
                .crate_categories
                .insert((build_0, category_0));
            ()
        }
        pub fn register_type_defs(
            &mut self,
            typ_0: Type,
            def_path_0: DefPath,
            name_1: String,
            visibility_0: TyVisibility,
            kind_0: TyDefKind,
        ) -> (Item,) {
            let item = self.counters.get_fresh_item();
            let name_0 = self.interning_tables.strings.intern(name_1);
            self.relations.type_defs.insert((
                item,
                typ_0,
                def_path_0,
                name_0,
                visibility_0,
                kind_0,
            ));
            (item,)
        }
        pub fn register_types(&mut self, kind_2: String) -> (Type,) {
            let typ = self.counters.get_fresh_type();
            let kind_1 = self.interning_tables.strings.intern(kind_2);
            let kind_0 = self.interning_tables.type_kinds.intern(kind_1);
            self.relations.types.insert((typ, kind_0));
            (typ,)
        }
        pub fn register_types_primitive(
            &mut self,
            typ_0: Type,
            primitive_kind_0: TyPrimitive,
        ) -> () {
            self.relations
                .types_primitive
                .insert((typ_0, primitive_kind_0));
            ()
        }
        pub fn register_types_adt_def(
            &mut self,
            typ_0: Type,
            def_path_0: DefPath,
            kind_0: AdtKind,
            c_repr_0: bool,
            is_phantom_0: bool,
        ) -> () {
            self.relations.types_adt_def.insert((
                typ_0,
                def_path_0,
                kind_0,
                c_repr_0,
                is_phantom_0,
            ));
            ()
        }
        pub fn register_types_adt_variant(
            &mut self,
            adt_0: Type,
            index_0: AdtVariantIndex,
            def_path_0: DefPath,
            ident_1: String,
        ) -> () {
            let ident_0 = self.interning_tables.strings.intern(ident_1);
            self.relations
                .types_adt_variant
                .insert((adt_0, index_0, def_path_0, ident_0));
            ()
        }
        pub fn register_types_adt_field(
            &mut self,
            adt_0: Type,
            index_0: AdtVariantIndex,
            def_path_0: DefPath,
            ident_1: String,
            visibility_0: TyVisibility,
            typ_0: Type,
        ) -> (Field,) {
            let field = self.counters.get_fresh_field();
            let ident_0 = self.interning_tables.strings.intern(ident_1);
            self.relations.types_adt_field.insert((
                field,
                adt_0,
                index_0,
                def_path_0,
                ident_0,
                visibility_0,
                typ_0,
            ));
            (field,)
        }
        pub fn register_types_adt_field_visible_in(
            &mut self,
            field_0: Field,
            module_0: DefPath,
        ) -> () {
            self.relations
                .types_adt_field_visible_in
                .insert((field_0, module_0));
            ()
        }
        pub fn register_types_foreign(&mut self, typ_0: Type, foreign_def_path_0: DefPath) -> () {
            self.relations
                .types_foreign
                .insert((typ_0, foreign_def_path_0));
            ()
        }
        pub fn register_types_array(&mut self, typ_0: Type, element_type_0: Type) -> () {
            self.relations.types_array.insert((typ_0, element_type_0));
            ()
        }
        pub fn register_types_slice(&mut self, typ_0: Type, element_type_0: Type) -> () {
            self.relations.types_slice.insert((typ_0, element_type_0));
            ()
        }
        pub fn register_types_raw_ptr(
            &mut self,
            typ_0: Type,
            target_type_0: Type,
            mutability_0: Mutability,
        ) -> () {
            self.relations
                .types_raw_ptr
                .insert((typ_0, target_type_0, mutability_0));
            ()
        }
        pub fn register_types_ref(
            &mut self,
            typ_0: Type,
            target_type_0: Type,
            mutability_0: Mutability,
        ) -> () {
            self.relations
                .types_ref
                .insert((typ_0, target_type_0, mutability_0));
            ()
        }
        pub fn register_types_fn_def(&mut self, typ_0: Type, def_path_0: DefPath) -> () {
            self.relations.types_fn_def.insert((typ_0, def_path_0));
            ()
        }
        pub fn register_types_fn_ptr(&mut self, typ_0: Type) -> () {
            self.relations.types_fn_ptr.insert((typ_0,));
            ()
        }
        pub fn register_types_dynamic(&mut self, typ_0: Type) -> () {
            self.relations.types_dynamic.insert((typ_0,));
            ()
        }
        pub fn register_types_dynamic_trait(
            &mut self,
            typ_0: Type,
            def_path_0: DefPath,
            is_auto_0: bool,
        ) -> () {
            self.relations
                .types_dynamic_trait
                .insert((typ_0, def_path_0, is_auto_0));
            ()
        }
        pub fn register_types_closure(&mut self, typ_0: Type, def_path_0: DefPath) -> () {
            self.relations.types_closure.insert((typ_0, def_path_0));
            ()
        }
        pub fn register_types_coroutine(&mut self, typ_0: Type, def_path_0: DefPath) -> () {
            self.relations.types_coroutine.insert((typ_0, def_path_0));
            ()
        }
        pub fn register_types_coroutine_witness(&mut self, typ_0: Type) -> () {
            self.relations.types_coroutine_witness.insert((typ_0,));
            ()
        }
        pub fn register_types_coroutine_closure(&mut self, typ_0: Type, def_path_0: DefPath) -> () {
            self.relations
                .types_coroutine_closure
                .insert((typ_0, def_path_0));
            ()
        }
        pub fn register_types_pat(&mut self, typ_0: Type) -> () {
            self.relations.types_pat.insert((typ_0,));
            ()
        }
        pub fn register_types_tuple(&mut self, typ_0: Type) -> () {
            self.relations.types_tuple.insert((typ_0,));
            ()
        }
        pub fn register_types_tuple_element(
            &mut self,
            tuple_type_0: Type,
            index_0: TupleFieldIndex,
            typ_0: Type,
        ) -> () {
            self.relations
                .types_tuple_element
                .insert((tuple_type_0, index_0, typ_0));
            ()
        }
        pub fn register_types_projection(
            &mut self,
            typ_0: Type,
            trait_def_path_0: DefPath,
            trait_item_0: DefPath,
        ) -> () {
            self.relations
                .types_projection
                .insert((typ_0, trait_def_path_0, trait_item_0));
            ()
        }
        pub fn register_types_opaque(&mut self, typ_0: Type, def_path_0: DefPath) -> () {
            self.relations.types_opaque.insert((typ_0, def_path_0));
            ()
        }
        pub fn register_types_inherent(&mut self, typ_0: Type, def_path_0: DefPath) -> () {
            self.relations.types_inherent.insert((typ_0, def_path_0));
            ()
        }
        pub fn register_types_weak(&mut self, typ_0: Type, def_path_0: DefPath) -> () {
            self.relations.types_weak.insert((typ_0, def_path_0));
            ()
        }
        pub fn register_types_param(&mut self, typ_0: Type, index_0: u32, name_1: String) -> () {
            let name_0 = self.interning_tables.strings.intern(name_1);
            self.relations.types_param.insert((typ_0, index_0, name_0));
            ()
        }
        pub fn register_traits(
            &mut self,
            def_path_0: DefPath,
            name_1: String,
            visibility_0: TyVisibility,
            is_auto_0: bool,
            is_marker_0: bool,
            unsafety_0: Safety,
        ) -> (Item,) {
            let item = self.counters.get_fresh_item();
            let name_0 = self.interning_tables.strings.intern(name_1);
            self.relations.traits.insert((
                item,
                def_path_0,
                name_0,
                visibility_0,
                is_auto_0,
                is_marker_0,
                unsafety_0,
            ));
            (item,)
        }
        pub fn register_trait_items(
            &mut self,
            trait_id_0: Item,
            def_path_0: DefPath,
            defaultness_0: Defaultness,
        ) -> () {
            self.relations
                .trait_items
                .insert((trait_id_0, def_path_0, defaultness_0));
            ()
        }
        pub fn register_basic_blocks(
            &mut self,
            mir_0: DefPath,
            kind_0: BasicBlockKind,
        ) -> (BasicBlock,) {
            let block = self.counters.get_fresh_basicblock();
            self.relations.basic_blocks.insert((block, mir_0, kind_0));
            (block,)
        }
        pub fn register_statements(
            &mut self,
            stmt_0: Statement,
            block_0: BasicBlock,
            index_0: StatementIndex,
            kind_2: String,
            scope_0: Scope,
        ) -> () {
            let kind_1 = self.interning_tables.strings.intern(kind_2);
            let kind_0 = self.interning_tables.statement_kinds.intern(kind_1);
            self.relations
                .statements
                .insert((stmt_0, block_0, index_0, kind_0, scope_0));
            ()
        }
        pub fn register_statements_assign_use(
            &mut self,
            target_type_0: Type,
            operand_0: Operand,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            self.relations
                .statements_assign_use
                .insert((stmt, target_type_0, operand_0));
            (stmt,)
        }
        pub fn register_statements_assign_thead_local_ref(
            &mut self,
            target_type_0: Type,
            def_path_0: DefPath,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            self.relations.statements_assign_thead_local_ref.insert((
                stmt,
                target_type_0,
                def_path_0,
            ));
            (stmt,)
        }
        pub fn register_statements_assign_repeat(
            &mut self,
            target_type_0: Type,
            operand_0: Operand,
            count_0: u64,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            self.relations.statements_assign_repeat.insert((
                stmt,
                target_type_0,
                operand_0,
                count_0,
            ));
            (stmt,)
        }
        pub fn register_statements_assign_ref(
            &mut self,
            target_type_0: Type,
            source_type_0: Type,
            kind_0: BorrowKind,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            self.relations.statements_assign_ref.insert((
                stmt,
                target_type_0,
                source_type_0,
                kind_0,
            ));
            (stmt,)
        }
        pub fn register_statements_assign_address(
            &mut self,
            target_type_0: Type,
            source_type_0: Type,
            mutability_0: Mutability,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            self.relations.statements_assign_address.insert((
                stmt,
                target_type_0,
                source_type_0,
                mutability_0,
            ));
            (stmt,)
        }
        pub fn register_statements_assign_len(
            &mut self,
            target_type_0: Type,
            source_type_0: Type,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            self.relations
                .statements_assign_len
                .insert((stmt, target_type_0, source_type_0));
            (stmt,)
        }
        pub fn register_statements_assign_cast(
            &mut self,
            target_type_0: Type,
            kind_0: CastKind,
            operand_0: Operand,
            typ_0: Type,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            self.relations.statements_assign_cast.insert((
                stmt,
                target_type_0,
                kind_0,
                operand_0,
                typ_0,
            ));
            (stmt,)
        }
        pub fn register_statements_assign_binary_op(
            &mut self,
            target_type_0: Type,
            kind_2: String,
            first_0: Operand,
            second_0: Operand,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            let kind_1 = self.interning_tables.strings.intern(kind_2);
            let kind_0 = self.interning_tables.binary_op_kind.intern(kind_1);
            self.relations.statements_assign_binary_op.insert((
                stmt,
                target_type_0,
                kind_0,
                first_0,
                second_0,
            ));
            (stmt,)
        }
        pub fn register_statements_assign_checked_binary_op(
            &mut self,
            target_type_0: Type,
            kind_2: String,
            first_0: Operand,
            second_0: Operand,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            let kind_1 = self.interning_tables.strings.intern(kind_2);
            let kind_0 = self.interning_tables.binary_op_kind.intern(kind_1);
            self.relations.statements_assign_checked_binary_op.insert((
                stmt,
                target_type_0,
                kind_0,
                first_0,
                second_0,
            ));
            (stmt,)
        }
        pub fn register_statements_assign_nullary_op(
            &mut self,
            target_type_0: Type,
            kind_2: String,
            source_type_0: Type,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            let kind_1 = self.interning_tables.strings.intern(kind_2);
            let kind_0 = self.interning_tables.nullary_op_kind.intern(kind_1);
            self.relations.statements_assign_nullary_op.insert((
                stmt,
                target_type_0,
                kind_0,
                source_type_0,
            ));
            (stmt,)
        }
        pub fn register_statements_assign_unary_op(
            &mut self,
            target_type_0: Type,
            kind_2: String,
            operand_0: Operand,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            let kind_1 = self.interning_tables.strings.intern(kind_2);
            let kind_0 = self.interning_tables.unary_op_kind.intern(kind_1);
            self.relations.statements_assign_unary_op.insert((
                stmt,
                target_type_0,
                kind_0,
                operand_0,
            ));
            (stmt,)
        }
        pub fn register_statements_assign_discriminant(
            &mut self,
            target_type_0: Type,
            source_type_0: Type,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            self.relations.statements_assign_discriminant.insert((
                stmt,
                target_type_0,
                source_type_0,
            ));
            (stmt,)
        }
        pub fn register_statements_assign_aggregate(
            &mut self,
            target_type_0: Type,
            kind_0: AggregateKind,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            self.relations
                .statements_assign_aggregate
                .insert((stmt, target_type_0, kind_0));
            (stmt,)
        }
        pub fn register_statements_assign_aggregate_operands(
            &mut self,
            stmt_0: Statement,
            index_0: OperandIndex,
            operand_0: Operand,
        ) -> () {
            self.relations
                .statements_assign_aggregate_operands
                .insert((stmt_0, index_0, operand_0));
            ()
        }
        pub fn register_statements_assign_shallow_init_box(
            &mut self,
            operand_0: Operand,
            typ_0: Type,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            self.relations
                .statements_assign_shallow_init_box
                .insert((stmt, operand_0, typ_0));
            (stmt,)
        }
        pub fn register_statements_assign_copy_for_deref(
            &mut self,
            place_type_0: Type,
        ) -> (Statement,) {
            let stmt = self.counters.get_fresh_statement();
            self.relations
                .statements_assign_copy_for_deref
                .insert((stmt, place_type_0));
            (stmt,)
        }
        pub fn register_statements_inline_asm_inputs(
            &mut self,
            stmt_0: Statement,
            operand_0: Operand,
        ) -> () {
            self.relations
                .statements_inline_asm_inputs
                .insert((stmt_0, operand_0));
            ()
        }
        pub fn register_statements_inline_asm_outputs(
            &mut self,
            stmt_0: Statement,
            typ_0: Type,
        ) -> () {
            self.relations
                .statements_inline_asm_outputs
                .insert((stmt_0, typ_0));
            ()
        }
        pub fn register_operands(&mut self, kind_0: OperandKind, typ_0: Type) -> (Operand,) {
            let operand = self.counters.get_fresh_operand();
            self.relations.operands.insert((operand, kind_0, typ_0));
            (operand,)
        }
        pub fn register_terminators(
            &mut self,
            block_0: BasicBlock,
            kind_2: String,
            scope_0: Scope,
        ) -> () {
            let kind_1 = self.interning_tables.strings.intern(kind_2);
            let kind_0 = self.interning_tables.terminator_kinds.intern(kind_1);
            self.relations
                .terminators
                .insert((block_0, kind_0, scope_0));
            ()
        }
        pub fn register_terminators_goto(
            &mut self,
            block_0: BasicBlock,
            target_0: BasicBlock,
        ) -> () {
            self.relations.terminators_goto.insert((block_0, target_0));
            ()
        }
        pub fn register_terminators_switch_int(
            &mut self,
            block_0: BasicBlock,
            discriminant_0: Operand,
        ) -> () {
            self.relations
                .terminators_switch_int
                .insert((block_0, discriminant_0));
            ()
        }
        pub fn register_terminators_switch_int_targets(
            &mut self,
            block_0: BasicBlock,
            condition_value_0: u128,
            target_0: BasicBlock,
        ) -> () {
            self.relations.terminators_switch_int_targets.insert((
                block_0,
                condition_value_0,
                target_0,
            ));
            ()
        }
        pub fn register_terminators_drop(
            &mut self,
            block_0: BasicBlock,
            location_0: Type,
            target_0: BasicBlock,
        ) -> () {
            self.relations
                .terminators_drop
                .insert((block_0, location_0, target_0));
            ()
        }
        pub fn register_terminators_drop_and_replace(
            &mut self,
            block_0: BasicBlock,
            location_0: Type,
            value_0: Operand,
            target_0: BasicBlock,
            unwind_0: BasicBlock,
        ) -> () {
            self.relations
                .terminators_drop_and_replace
                .insert((block_0, location_0, value_0, target_0, unwind_0));
            ()
        }
        pub fn register_terminators_call(
            &mut self,
            block_0: BasicBlock,
            func_0: Operand,
            unsafety_0: Safety,
            abi_2: String,
            return_ty_0: Type,
            destination_0: BasicBlock,
            span_0: Span,
        ) -> (FunctionCall,) {
            let call = self.counters.get_fresh_functioncall();
            let abi_1 = self.interning_tables.strings.intern(abi_2);
            let abi_0 = self.interning_tables.abis.intern(abi_1);
            self.relations.terminators_call.insert((
                block_0,
                call,
                func_0,
                unsafety_0,
                abi_0,
                return_ty_0,
                destination_0,
                span_0,
            ));
            (call,)
        }
        pub fn register_terminators_call_arg(
            &mut self,
            call_0: FunctionCall,
            index_0: CallArgIndex,
            arg_0: Operand,
        ) -> () {
            self.relations
                .terminators_call_arg
                .insert((call_0, index_0, arg_0));
            ()
        }
        pub fn register_terminators_call_const_target(
            &mut self,
            call_0: FunctionCall,
            def_path_0: DefPath,
        ) -> () {
            self.relations
                .terminators_call_const_target
                .insert((call_0, def_path_0));
            ()
        }
        pub fn register_terminators_call_const_target_desc(
            &mut self,
            call_0: FunctionCall,
            target_1: String,
            function_generics_1: String,
            type_generics_1: String,
        ) -> () {
            let target_0 = self.interning_tables.strings.intern(target_1);
            let function_generics_0 = self.interning_tables.strings.intern(function_generics_1);
            let type_generics_0 = self.interning_tables.strings.intern(type_generics_1);
            self.relations.terminators_call_const_target_desc.insert((
                call_0,
                target_0,
                function_generics_0,
                type_generics_0,
            ));
            ()
        }
        pub fn register_terminators_call_const_target_self(
            &mut self,
            call_0: FunctionCall,
            typ_0: Type,
        ) -> () {
            self.relations
                .terminators_call_const_target_self
                .insert((call_0, typ_0));
            ()
        }
        pub fn register_terminators_call_macro_backtrace(
            &mut self,
            call_0: FunctionCall,
            macro_path_1: String,
        ) -> () {
            let macro_path_0 = self.interning_tables.strings.intern(macro_path_1);
            self.relations
                .terminators_call_macro_backtrace
                .insert((call_0, macro_path_0));
            ()
        }
        pub fn register_terminators_assert(
            &mut self,
            block_0: BasicBlock,
            cond_0: Operand,
            expected_0: bool,
            target_0: BasicBlock,
        ) -> () {
            self.relations
                .terminators_assert
                .insert((block_0, cond_0, expected_0, target_0));
            ()
        }
        pub fn register_terminators_yield(
            &mut self,
            block_0: BasicBlock,
            value_0: Operand,
            resume_0: BasicBlock,
            drop_0: BasicBlock,
        ) -> () {
            self.relations
                .terminators_yield
                .insert((block_0, value_0, resume_0, drop_0));
            ()
        }
        pub fn register_terminators_false_edges(
            &mut self,
            block_0: BasicBlock,
            real_target_0: BasicBlock,
            imaginary_target_0: BasicBlock,
        ) -> () {
            self.relations.terminators_false_edges.insert((
                block_0,
                real_target_0,
                imaginary_target_0,
            ));
            ()
        }
        pub fn register_terminators_false_unwind(
            &mut self,
            block_0: BasicBlock,
            real_target_0: BasicBlock,
        ) -> () {
            self.relations
                .terminators_false_unwind
                .insert((block_0, real_target_0));
            ()
        }
        pub fn register_terminators_inline_asm(&mut self, block_0: BasicBlock) -> () {
            self.relations.terminators_inline_asm.insert((block_0,));
            ()
        }
        pub fn register_terminators_unwind_action(
            &mut self,
            block_0: BasicBlock,
            action_0: UnwindAction,
            cleanup_0: BasicBlock,
        ) -> () {
            self.relations
                .terminators_unwind_action
                .insert((block_0, action_0, cleanup_0));
            ()
        }
    }
    impl Tables {
        pub fn get_fresh_module(&mut self) -> Module {
            self.counters.get_fresh_module()
        }
        pub fn get_fresh_item(&mut self) -> Item {
            self.counters.get_fresh_item()
        }
        pub fn get_fresh_scope(&mut self) -> Scope {
            self.counters.get_fresh_scope()
        }
        pub fn get_fresh_functioncall(&mut self) -> FunctionCall {
            self.counters.get_fresh_functioncall()
        }
        pub fn get_fresh_span(&mut self) -> Span {
            self.counters.get_fresh_span()
        }
        pub fn get_root_parent_span(&mut self) -> Span {
            self.counters.get_root_parent_span()
        }
        pub fn get_fresh_type(&mut self) -> Type {
            self.counters.get_fresh_type()
        }
        pub fn get_fresh_field(&mut self) -> Field {
            self.counters.get_fresh_field()
        }
        pub fn get_fresh_operand(&mut self) -> Operand {
            self.counters.get_fresh_operand()
        }
        pub fn get_fresh_basicblock(&mut self) -> BasicBlock {
            self.counters.get_fresh_basicblock()
        }
        pub fn get_no_block(&mut self) -> BasicBlock {
            self.counters.get_no_block()
        }
        pub fn get_fresh_statement(&mut self) -> Statement {
            self.counters.get_fresh_statement()
        }
        pub fn get_fresh_thirblock(&mut self) -> ThirBlock {
            self.counters.get_fresh_thirblock()
        }
        pub fn get_no_thir_block(&mut self) -> ThirBlock {
            self.counters.get_no_thir_block()
        }
        pub fn get_fresh_thirexpr(&mut self) -> ThirExpr {
            self.counters.get_fresh_thirexpr()
        }
        pub fn get_no_thir_expr(&mut self) -> ThirExpr {
            self.counters.get_no_thir_expr()
        }
        pub fn get_fresh_thirpat(&mut self) -> ThirPat {
            self.counters.get_fresh_thirpat()
        }
        pub fn get_no_thir_pat(&mut self) -> ThirPat {
            self.counters.get_no_thir_pat()
        }
        pub fn get_fresh_thirstmt(&mut self) -> ThirStmt {
            self.counters.get_fresh_thirstmt()
        }
        pub fn get_no_thir_stmt(&mut self) -> ThirStmt {
            self.counters.get_no_thir_stmt()
        }
    }
    impl Tables {
        pub fn print_statistics(&self) {
            println!("counter {} value: {}", "modules", self.counters.modules);
            println!("counter {} value: {}", "items", self.counters.items);
            println!("counter {} value: {}", "scopes", self.counters.scopes);
            println!(
                "counter {} value: {}",
                "functioncalls", self.counters.functioncalls
            );
            println!("counter {} value: {}", "spans", self.counters.spans);
            println!("counter {} value: {}", "types", self.counters.types);
            println!("counter {} value: {}", "fields", self.counters.fields);
            println!("counter {} value: {}", "operands", self.counters.operands);
            println!(
                "counter {} value: {}",
                "basicblocks", self.counters.basicblocks
            );
            println!(
                "counter {} value: {}",
                "statements", self.counters.statements
            );
            println!(
                "counter {} value: {}",
                "thirblocks", self.counters.thirblocks
            );
            println!("counter {} value: {}", "thirexprs", self.counters.thirexprs);
            println!("counter {} value: {}", "thirpats", self.counters.thirpats);
            println!("counter {} value: {}", "thirstmts", self.counters.thirstmts);
            println!(
                "interning table {} count: {}",
                "strings",
                self.interning_tables.strings.len()
            );
            println!(
                "interning table {} count: {}",
                "package_names",
                self.interning_tables.package_names.len()
            );
            println!(
                "interning table {} count: {}",
                "package_versions",
                self.interning_tables.package_versions.len()
            );
            println!(
                "interning table {} count: {}",
                "crate_names",
                self.interning_tables.crate_names.len()
            );
            println!(
                "interning table {} count: {}",
                "editions",
                self.interning_tables.editions.len()
            );
            println!(
                "interning table {} count: {}",
                "names",
                self.interning_tables.names.len()
            );
            println!(
                "interning table {} count: {}",
                "relative_def_paths",
                self.interning_tables.relative_def_paths.len()
            );
            println!(
                "interning table {} count: {}",
                "summary_keys",
                self.interning_tables.summary_keys.len()
            );
            println!(
                "interning table {} count: {}",
                "abis",
                self.interning_tables.abis.len()
            );
            println!(
                "interning table {} count: {}",
                "def_paths",
                self.interning_tables.def_paths.len()
            );
            println!(
                "interning table {} count: {}",
                "builds",
                self.interning_tables.builds.len()
            );
            println!(
                "interning table {} count: {}",
                "span_file_names",
                self.interning_tables.span_file_names.len()
            );
            println!(
                "interning table {} count: {}",
                "crate_cfg_keys",
                self.interning_tables.crate_cfg_keys.len()
            );
            println!(
                "interning table {} count: {}",
                "crate_cfg_values",
                self.interning_tables.crate_cfg_values.len()
            );
            println!(
                "interning table {} count: {}",
                "type_kinds",
                self.interning_tables.type_kinds.len()
            );
            println!(
                "interning table {} count: {}",
                "statement_kinds",
                self.interning_tables.statement_kinds.len()
            );
            println!(
                "interning table {} count: {}",
                "binary_op_kind",
                self.interning_tables.binary_op_kind.len()
            );
            println!(
                "interning table {} count: {}",
                "nullary_op_kind",
                self.interning_tables.nullary_op_kind.len()
            );
            println!(
                "interning table {} count: {}",
                "unary_op_kind",
                self.interning_tables.unary_op_kind.len()
            );
            println!(
                "interning table {} count: {}",
                "terminator_kinds",
                self.interning_tables.terminator_kinds.len()
            );
            println!(
                "interning table {} count: {}",
                "thir_binary_op_kind",
                self.interning_tables.thir_binary_op_kind.len()
            );
            println!(
                "interning table {} count: {}",
                "thir_logical_op_kind",
                self.interning_tables.thir_logical_op_kind.len()
            );
            println!(
                "interning table {} count: {}",
                "thir_unary_op_kind",
                self.interning_tables.thir_unary_op_kind.len()
            );
            println!(
                "relation {} count: {}",
                "def_path_span",
                self.relations.def_path_span.len()
            );
            println!(
                "relation {} count: {}",
                "type_description",
                self.relations.type_description.len()
            );
            println!(
                "relation {} count: {}",
                "build_crate_types",
                self.relations.build_crate_types.len()
            );
            println!(
                "relation {} count: {}",
                "root_modules",
                self.relations.root_modules.len()
            );
            println!(
                "relation {} count: {}",
                "submodules",
                self.relations.submodules.len()
            );
            println!(
                "relation {} count: {}",
                "function_definitions",
                self.relations.function_definitions.len()
            );
            println!(
                "relation {} count: {}",
                "function_parameter_types",
                self.relations.function_parameter_types.len()
            );
            println!(
                "relation {} count: {}",
                "function_unsafe_use",
                self.relations.function_unsafe_use.len()
            );
            println!(
                "relation {} count: {}",
                "function_unsafe_reasons",
                self.relations.function_unsafe_reasons.len()
            );
            println!(
                "relation {} count: {}",
                "thir_bodies",
                self.relations.thir_bodies.len()
            );
            println!(
                "relation {} count: {}",
                "thir_blocks",
                self.relations.thir_blocks.len()
            );
            println!(
                "relation {} count: {}",
                "thir_stmts",
                self.relations.thir_stmts.len()
            );
            println!(
                "relation {} count: {}",
                "thir_stmts_expr",
                self.relations.thir_stmts_expr.len()
            );
            println!(
                "relation {} count: {}",
                "thir_stmts_let",
                self.relations.thir_stmts_let.len()
            );
            println!(
                "relation {} count: {}",
                "thir_block_expr",
                self.relations.thir_block_expr.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs",
                self.relations.thir_exprs.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_scope",
                self.relations.thir_exprs_scope.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_box",
                self.relations.thir_exprs_box.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_if",
                self.relations.thir_exprs_if.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_call",
                self.relations.thir_exprs_call.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_call_arg",
                self.relations.thir_exprs_call_arg.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_call_const_target",
                self.relations.thir_exprs_call_const_target.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_call_const_target_desc",
                self.relations.thir_exprs_call_const_target_desc.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_call_const_target_self",
                self.relations.thir_exprs_call_const_target_self.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_deref",
                self.relations.thir_exprs_deref.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_binary",
                self.relations.thir_exprs_binary.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_logical_op",
                self.relations.thir_exprs_logical_op.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_unary",
                self.relations.thir_exprs_unary.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_cast",
                self.relations.thir_exprs_cast.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_use",
                self.relations.thir_exprs_use.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_never_to_any",
                self.relations.thir_exprs_never_to_any.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_pointer_coercion",
                self.relations.thir_exprs_pointer_coercion.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_loop",
                self.relations.thir_exprs_loop.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_let",
                self.relations.thir_exprs_let.len()
            );
            println!(
                "relation {} count: {}",
                "thir_pats",
                self.relations.thir_pats.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_match",
                self.relations.thir_exprs_match.len()
            );
            println!(
                "relation {} count: {}",
                "thir_match_arms",
                self.relations.thir_match_arms.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_block",
                self.relations.thir_exprs_block.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_assign",
                self.relations.thir_exprs_assign.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_assign_op",
                self.relations.thir_exprs_assign_op.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_field",
                self.relations.thir_exprs_field.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_index",
                self.relations.thir_exprs_index.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_var_ref",
                self.relations.thir_exprs_var_ref.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_upvar_ref",
                self.relations.thir_exprs_upvar_ref.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_borrow",
                self.relations.thir_exprs_borrow.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_raw_borrow",
                self.relations.thir_exprs_raw_borrow.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_break",
                self.relations.thir_exprs_break.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_continue",
                self.relations.thir_exprs_continue.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_return",
                self.relations.thir_exprs_return.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_become",
                self.relations.thir_exprs_become.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_const_block",
                self.relations.thir_exprs_const_block.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_repeat",
                self.relations.thir_exprs_repeat.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_array",
                self.relations.thir_exprs_array.len()
            );
            println!(
                "relation {} count: {}",
                "thir_array_elements",
                self.relations.thir_array_elements.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_tuple",
                self.relations.thir_exprs_tuple.len()
            );
            println!(
                "relation {} count: {}",
                "thir_tuple_elements",
                self.relations.thir_tuple_elements.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_adt",
                self.relations.thir_exprs_adt.len()
            );
            println!(
                "relation {} count: {}",
                "thir_adt_field_expr",
                self.relations.thir_adt_field_expr.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_place_type_ascription",
                self.relations.thir_exprs_place_type_ascription.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_value_type_ascription",
                self.relations.thir_exprs_value_type_ascription.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_closure",
                self.relations.thir_exprs_closure.len()
            );
            println!(
                "relation {} count: {}",
                "thir_closure_upvars",
                self.relations.thir_closure_upvars.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_literal",
                self.relations.thir_exprs_literal.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_non_hir_literal",
                self.relations.thir_exprs_non_hir_literal.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_zst_literal",
                self.relations.thir_exprs_zst_literal.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_named_const",
                self.relations.thir_exprs_named_const.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_const_param",
                self.relations.thir_exprs_const_param.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_static_ref",
                self.relations.thir_exprs_static_ref.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_inline_asm",
                self.relations.thir_exprs_inline_asm.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_offset_of",
                self.relations.thir_exprs_offset_of.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_thread_local_ref",
                self.relations.thir_exprs_thread_local_ref.len()
            );
            println!(
                "relation {} count: {}",
                "thir_exprs_yield",
                self.relations.thir_exprs_yield.len()
            );
            println!(
                "relation {} count: {}",
                "static_definitions",
                self.relations.static_definitions.len()
            );
            println!(
                "relation {} count: {}",
                "impl_definitions",
                self.relations.impl_definitions.len()
            );
            println!(
                "relation {} count: {}",
                "trait_impls",
                self.relations.trait_impls.len()
            );
            println!(
                "relation {} count: {}",
                "global_asm_blocks",
                self.relations.global_asm_blocks.len()
            );
            println!("relation {} count: {}", "items", self.relations.items.len());
            println!(
                "relation {} count: {}",
                "mir_cfgs",
                self.relations.mir_cfgs.len()
            );
            println!(
                "relation {} count: {}",
                "subscopes",
                self.relations.subscopes.len()
            );
            println!("relation {} count: {}", "spans", self.relations.spans.len());
            println!(
                "relation {} count: {}",
                "macro_expansions",
                self.relations.macro_expansions.len()
            );
            println!(
                "relation {} count: {}",
                "crate_cfgs",
                self.relations.crate_cfgs.len()
            );
            println!(
                "relation {} count: {}",
                "crate_authors",
                self.relations.crate_authors.len()
            );
            println!(
                "relation {} count: {}",
                "crate_keywords",
                self.relations.crate_keywords.len()
            );
            println!(
                "relation {} count: {}",
                "crate_categories",
                self.relations.crate_categories.len()
            );
            println!(
                "relation {} count: {}",
                "type_defs",
                self.relations.type_defs.len()
            );
            println!("relation {} count: {}", "types", self.relations.types.len());
            println!(
                "relation {} count: {}",
                "types_primitive",
                self.relations.types_primitive.len()
            );
            println!(
                "relation {} count: {}",
                "types_adt_def",
                self.relations.types_adt_def.len()
            );
            println!(
                "relation {} count: {}",
                "types_adt_variant",
                self.relations.types_adt_variant.len()
            );
            println!(
                "relation {} count: {}",
                "types_adt_field",
                self.relations.types_adt_field.len()
            );
            println!(
                "relation {} count: {}",
                "types_adt_field_visible_in",
                self.relations.types_adt_field_visible_in.len()
            );
            println!(
                "relation {} count: {}",
                "types_foreign",
                self.relations.types_foreign.len()
            );
            println!(
                "relation {} count: {}",
                "types_array",
                self.relations.types_array.len()
            );
            println!(
                "relation {} count: {}",
                "types_slice",
                self.relations.types_slice.len()
            );
            println!(
                "relation {} count: {}",
                "types_raw_ptr",
                self.relations.types_raw_ptr.len()
            );
            println!(
                "relation {} count: {}",
                "types_ref",
                self.relations.types_ref.len()
            );
            println!(
                "relation {} count: {}",
                "types_fn_def",
                self.relations.types_fn_def.len()
            );
            println!(
                "relation {} count: {}",
                "types_fn_ptr",
                self.relations.types_fn_ptr.len()
            );
            println!(
                "relation {} count: {}",
                "types_dynamic",
                self.relations.types_dynamic.len()
            );
            println!(
                "relation {} count: {}",
                "types_dynamic_trait",
                self.relations.types_dynamic_trait.len()
            );
            println!(
                "relation {} count: {}",
                "types_closure",
                self.relations.types_closure.len()
            );
            println!(
                "relation {} count: {}",
                "types_coroutine",
                self.relations.types_coroutine.len()
            );
            println!(
                "relation {} count: {}",
                "types_coroutine_witness",
                self.relations.types_coroutine_witness.len()
            );
            println!(
                "relation {} count: {}",
                "types_coroutine_closure",
                self.relations.types_coroutine_closure.len()
            );
            println!(
                "relation {} count: {}",
                "types_pat",
                self.relations.types_pat.len()
            );
            println!(
                "relation {} count: {}",
                "types_tuple",
                self.relations.types_tuple.len()
            );
            println!(
                "relation {} count: {}",
                "types_tuple_element",
                self.relations.types_tuple_element.len()
            );
            println!(
                "relation {} count: {}",
                "types_projection",
                self.relations.types_projection.len()
            );
            println!(
                "relation {} count: {}",
                "types_opaque",
                self.relations.types_opaque.len()
            );
            println!(
                "relation {} count: {}",
                "types_inherent",
                self.relations.types_inherent.len()
            );
            println!(
                "relation {} count: {}",
                "types_weak",
                self.relations.types_weak.len()
            );
            println!(
                "relation {} count: {}",
                "types_param",
                self.relations.types_param.len()
            );
            println!(
                "relation {} count: {}",
                "traits",
                self.relations.traits.len()
            );
            println!(
                "relation {} count: {}",
                "trait_items",
                self.relations.trait_items.len()
            );
            println!(
                "relation {} count: {}",
                "basic_blocks",
                self.relations.basic_blocks.len()
            );
            println!(
                "relation {} count: {}",
                "statements",
                self.relations.statements.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_use",
                self.relations.statements_assign_use.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_thead_local_ref",
                self.relations.statements_assign_thead_local_ref.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_repeat",
                self.relations.statements_assign_repeat.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_ref",
                self.relations.statements_assign_ref.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_address",
                self.relations.statements_assign_address.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_len",
                self.relations.statements_assign_len.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_cast",
                self.relations.statements_assign_cast.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_binary_op",
                self.relations.statements_assign_binary_op.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_checked_binary_op",
                self.relations.statements_assign_checked_binary_op.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_nullary_op",
                self.relations.statements_assign_nullary_op.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_unary_op",
                self.relations.statements_assign_unary_op.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_discriminant",
                self.relations.statements_assign_discriminant.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_aggregate",
                self.relations.statements_assign_aggregate.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_aggregate_operands",
                self.relations.statements_assign_aggregate_operands.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_shallow_init_box",
                self.relations.statements_assign_shallow_init_box.len()
            );
            println!(
                "relation {} count: {}",
                "statements_assign_copy_for_deref",
                self.relations.statements_assign_copy_for_deref.len()
            );
            println!(
                "relation {} count: {}",
                "statements_inline_asm_inputs",
                self.relations.statements_inline_asm_inputs.len()
            );
            println!(
                "relation {} count: {}",
                "statements_inline_asm_outputs",
                self.relations.statements_inline_asm_outputs.len()
            );
            println!(
                "relation {} count: {}",
                "operands",
                self.relations.operands.len()
            );
            println!(
                "relation {} count: {}",
                "terminators",
                self.relations.terminators.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_goto",
                self.relations.terminators_goto.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_switch_int",
                self.relations.terminators_switch_int.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_switch_int_targets",
                self.relations.terminators_switch_int_targets.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_drop",
                self.relations.terminators_drop.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_drop_and_replace",
                self.relations.terminators_drop_and_replace.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_call",
                self.relations.terminators_call.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_call_arg",
                self.relations.terminators_call_arg.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_call_const_target",
                self.relations.terminators_call_const_target.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_call_const_target_desc",
                self.relations.terminators_call_const_target_desc.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_call_const_target_self",
                self.relations.terminators_call_const_target_self.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_call_macro_backtrace",
                self.relations.terminators_call_macro_backtrace.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_assert",
                self.relations.terminators_assert.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_yield",
                self.relations.terminators_yield.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_false_edges",
                self.relations.terminators_false_edges.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_false_unwind",
                self.relations.terminators_false_unwind.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_inline_asm",
                self.relations.terminators_inline_asm.len()
            );
            println!(
                "relation {} count: {}",
                "terminators_unwind_action",
                self.relations.terminators_unwind_action.len()
            );
        }
    }
    pub struct TableMerger {
        pub(crate) tables: Tables,
    }
    impl TableMerger {
        pub fn new(tables: super::tables::Tables) -> Self {
            Self { tables }
        }
        pub fn merge(&mut self, other: super::tables::Tables) {
            let strings: HashMap<_, _> = other
                .interning_tables
                .strings
                .into_iter()
                .map(|(key, value)| {
                    let new_value = value;
                    let new_key = self.tables.interning_tables.strings.intern(new_value);
                    (key, new_key)
                })
                .collect();
            let package_names: HashMap<_, _> = other
                .interning_tables
                .package_names
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self.tables.interning_tables.package_names.intern(new_value);
                    (key, new_key)
                })
                .collect();
            let package_versions: HashMap<_, _> = other
                .interning_tables
                .package_versions
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self
                        .tables
                        .interning_tables
                        .package_versions
                        .intern(new_value);
                    (key, new_key)
                })
                .collect();
            let crate_names: HashMap<_, _> = other
                .interning_tables
                .crate_names
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self.tables.interning_tables.crate_names.intern(new_value);
                    (key, new_key)
                })
                .collect();
            let editions: HashMap<_, _> = other
                .interning_tables
                .editions
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self.tables.interning_tables.editions.intern(new_value);
                    (key, new_key)
                })
                .collect();
            let names: HashMap<_, _> = other
                .interning_tables
                .names
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self.tables.interning_tables.names.intern(new_value);
                    (key, new_key)
                })
                .collect();
            let relative_def_paths: HashMap<_, _> = other
                .interning_tables
                .relative_def_paths
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self
                        .tables
                        .interning_tables
                        .relative_def_paths
                        .intern(new_value);
                    (key, new_key)
                })
                .collect();
            let summary_keys: HashMap<_, _> = other
                .interning_tables
                .summary_keys
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self.tables.interning_tables.summary_keys.intern(new_value);
                    (key, new_key)
                })
                .collect();
            let abis: HashMap<_, _> = other
                .interning_tables
                .abis
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self.tables.interning_tables.abis.intern(new_value);
                    (key, new_key)
                })
                .collect();
            let span_file_names: HashMap<_, _> = other
                .interning_tables
                .span_file_names
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self
                        .tables
                        .interning_tables
                        .span_file_names
                        .intern(new_value);
                    (key, new_key)
                })
                .collect();
            let crate_cfg_keys: HashMap<_, _> = other
                .interning_tables
                .crate_cfg_keys
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self
                        .tables
                        .interning_tables
                        .crate_cfg_keys
                        .intern(new_value);
                    (key, new_key)
                })
                .collect();
            let crate_cfg_values: HashMap<_, _> = other
                .interning_tables
                .crate_cfg_values
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self
                        .tables
                        .interning_tables
                        .crate_cfg_values
                        .intern(new_value);
                    (key, new_key)
                })
                .collect();
            let type_kinds: HashMap<_, _> = other
                .interning_tables
                .type_kinds
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self.tables.interning_tables.type_kinds.intern(new_value);
                    (key, new_key)
                })
                .collect();
            let statement_kinds: HashMap<_, _> = other
                .interning_tables
                .statement_kinds
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self
                        .tables
                        .interning_tables
                        .statement_kinds
                        .intern(new_value);
                    (key, new_key)
                })
                .collect();
            let binary_op_kind: HashMap<_, _> = other
                .interning_tables
                .binary_op_kind
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self
                        .tables
                        .interning_tables
                        .binary_op_kind
                        .intern(new_value);
                    (key, new_key)
                })
                .collect();
            let nullary_op_kind: HashMap<_, _> = other
                .interning_tables
                .nullary_op_kind
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self
                        .tables
                        .interning_tables
                        .nullary_op_kind
                        .intern(new_value);
                    (key, new_key)
                })
                .collect();
            let unary_op_kind: HashMap<_, _> = other
                .interning_tables
                .unary_op_kind
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self.tables.interning_tables.unary_op_kind.intern(new_value);
                    (key, new_key)
                })
                .collect();
            let terminator_kinds: HashMap<_, _> = other
                .interning_tables
                .terminator_kinds
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self
                        .tables
                        .interning_tables
                        .terminator_kinds
                        .intern(new_value);
                    (key, new_key)
                })
                .collect();
            let thir_binary_op_kind: HashMap<_, _> = other
                .interning_tables
                .thir_binary_op_kind
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self
                        .tables
                        .interning_tables
                        .thir_binary_op_kind
                        .intern(new_value);
                    (key, new_key)
                })
                .collect();
            let thir_logical_op_kind: HashMap<_, _> = other
                .interning_tables
                .thir_logical_op_kind
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self
                        .tables
                        .interning_tables
                        .thir_logical_op_kind
                        .intern(new_value);
                    (key, new_key)
                })
                .collect();
            let thir_unary_op_kind: HashMap<_, _> = other
                .interning_tables
                .thir_unary_op_kind
                .into_iter()
                .map(|(key, value)| {
                    let new_value = strings[&value];
                    let new_key = self
                        .tables
                        .interning_tables
                        .thir_unary_op_kind
                        .intern(new_value);
                    (key, new_key)
                })
                .collect();
            let def_paths: HashMap<_, _> = other
                .interning_tables
                .def_paths
                .into_iter()
                .map(|(key, (tmp_1, tmp_3, tmp_5, tmp_7, tmp_9))| {
                    let tmp_2 = crate_names[&tmp_1];
                    let tmp_4 = tmp_3;
                    let tmp_6 = relative_def_paths[&tmp_5];
                    let tmp_8 = tmp_7;
                    let tmp_10 = summary_keys[&tmp_9];
                    let new_key = self
                        .tables
                        .interning_tables
                        .def_paths
                        .intern((tmp_2, tmp_4, tmp_6, tmp_8, tmp_10));
                    (key, new_key)
                })
                .collect();
            let builds: HashMap<_, _> = other
                .interning_tables
                .builds
                .into_iter()
                .map(|(key, (tmp_11, tmp_13, tmp_15, tmp_17, tmp_19))| {
                    let tmp_12 = package_names[&tmp_11];
                    let tmp_14 = package_versions[&tmp_13];
                    let tmp_16 = crate_names[&tmp_15];
                    let tmp_18 = tmp_17;
                    let tmp_20 = editions[&tmp_19];
                    let new_key = self
                        .tables
                        .interning_tables
                        .builds
                        .intern((tmp_12, tmp_14, tmp_16, tmp_18, tmp_20));
                    (key, new_key)
                })
                .collect();
            for (def_path, span) in other.relations.def_path_span.iter() {
                let tmp_21 = def_paths[def_path];
                let tmp_22 = if span.index() >= 1usize {
                    span.shift(self.tables.counters.spans - (1usize as u64))
                } else {
                    *span
                };
                self.tables.relations.def_path_span.insert((tmp_21, tmp_22));
            }
            for (ty, description, generics) in other.relations.type_description.iter() {
                let tmp_23 = ty.shift(self.tables.counters.types - (0usize as u64));
                let tmp_24 = strings[description];
                let tmp_25 = strings[generics];
                self.tables
                    .relations
                    .type_description
                    .insert((tmp_23, tmp_24, tmp_25));
            }
            for (build, crate_type) in other.relations.build_crate_types.iter() {
                let tmp_26 = builds[build];
                let tmp_27 = strings[crate_type];
                self.tables
                    .relations
                    .build_crate_types
                    .insert((tmp_26, tmp_27));
            }
            for (build, root_module) in other.relations.root_modules.iter() {
                let tmp_28 = builds[build];
                let tmp_29 = root_module.shift(self.tables.counters.modules - (0usize as u32));
                self.tables.relations.root_modules.insert((tmp_28, tmp_29));
            }
            for (def_path, parent, child, name, visibility, abi) in
                other.relations.submodules.iter()
            {
                let tmp_30 = def_paths[def_path];
                let tmp_31 = parent.shift(self.tables.counters.modules - (0usize as u32));
                let tmp_32 = child.shift(self.tables.counters.modules - (0usize as u32));
                let tmp_33 = names[name];
                let tmp_34 = *visibility;
                let tmp_35 = abis[abi];
                self.tables
                    .relations
                    .submodules
                    .insert((tmp_30, tmp_31, tmp_32, tmp_33, tmp_34, tmp_35));
            }
            for (item, def_path, module, visibility, unsafety, abi, return_ty) in
                other.relations.function_definitions.iter()
            {
                let tmp_36 = item.shift(self.tables.counters.items - (0usize as u32));
                let tmp_37 = def_paths[def_path];
                let tmp_38 = module.shift(self.tables.counters.modules - (0usize as u32));
                let tmp_39 = *visibility;
                let tmp_40 = *unsafety;
                let tmp_41 = abis[abi];
                let tmp_42 = return_ty.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .function_definitions
                    .insert((tmp_36, tmp_37, tmp_38, tmp_39, tmp_40, tmp_41, tmp_42));
            }
            for (function, index, typ) in other.relations.function_parameter_types.iter() {
                let tmp_43 = function.shift(self.tables.counters.items - (0usize as u32));
                let tmp_44 = *index;
                let tmp_45 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .function_parameter_types
                    .insert((tmp_43, tmp_44, tmp_45));
            }
            for (def_path, uses_unsafe) in other.relations.function_unsafe_use.iter() {
                let tmp_46 = def_paths[def_path];
                let tmp_47 = *uses_unsafe;
                self.tables
                    .relations
                    .function_unsafe_use
                    .insert((tmp_46, tmp_47));
            }
            for (def_path, index, reason) in other.relations.function_unsafe_reasons.iter() {
                let tmp_48 = def_paths[def_path];
                let tmp_49 = *index;
                let tmp_50 = strings[reason];
                self.tables
                    .relations
                    .function_unsafe_reasons
                    .insert((tmp_48, tmp_49, tmp_50));
            }
            for (item, def_path, body) in other.relations.thir_bodies.iter() {
                let tmp_51 = item.shift(self.tables.counters.items - (0usize as u32));
                let tmp_52 = def_paths[def_path];
                let tmp_53 = if body.index() >= 1usize {
                    body.shift(self.tables.counters.thirblocks - (1usize as u64))
                } else {
                    *body
                };
                self.tables
                    .relations
                    .thir_bodies
                    .insert((tmp_51, tmp_52, tmp_53));
            }
            for (parent, block, safety, check_mode, span) in other.relations.thir_blocks.iter() {
                let tmp_54 = if parent.index() >= 1usize {
                    parent.shift(self.tables.counters.thirblocks - (1usize as u64))
                } else {
                    *parent
                };
                let tmp_55 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.thirblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_56 = *safety;
                let tmp_57 = *check_mode;
                let tmp_58 = if span.index() >= 1usize {
                    span.shift(self.tables.counters.spans - (1usize as u64))
                } else {
                    *span
                };
                self.tables
                    .relations
                    .thir_blocks
                    .insert((tmp_54, tmp_55, tmp_56, tmp_57, tmp_58));
            }
            for (stmt, block, closest_unsafe_block, index) in other.relations.thir_stmts.iter() {
                let tmp_59 = if stmt.index() >= 1usize {
                    stmt.shift(self.tables.counters.thirstmts - (1usize as u64))
                } else {
                    *stmt
                };
                let tmp_60 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.thirblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_61 = if closest_unsafe_block.index() >= 1usize {
                    closest_unsafe_block.shift(self.tables.counters.thirblocks - (1usize as u64))
                } else {
                    *closest_unsafe_block
                };
                let tmp_62 = *index;
                self.tables
                    .relations
                    .thir_stmts
                    .insert((tmp_59, tmp_60, tmp_61, tmp_62));
            }
            for (stmt, expr) in other.relations.thir_stmts_expr.iter() {
                let tmp_63 = if stmt.index() >= 1usize {
                    stmt.shift(self.tables.counters.thirstmts - (1usize as u64))
                } else {
                    *stmt
                };
                let tmp_64 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                self.tables
                    .relations
                    .thir_stmts_expr
                    .insert((tmp_63, tmp_64));
            }
            for (stmt, initializer, else_block, span) in other.relations.thir_stmts_let.iter() {
                let tmp_65 = if stmt.index() >= 1usize {
                    stmt.shift(self.tables.counters.thirstmts - (1usize as u64))
                } else {
                    *stmt
                };
                let tmp_66 = if initializer.index() >= 1usize {
                    initializer.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *initializer
                };
                let tmp_67 = if else_block.index() >= 1usize {
                    else_block.shift(self.tables.counters.thirblocks - (1usize as u64))
                } else {
                    *else_block
                };
                let tmp_68 = if span.index() >= 1usize {
                    span.shift(self.tables.counters.spans - (1usize as u64))
                } else {
                    *span
                };
                self.tables
                    .relations
                    .thir_stmts_let
                    .insert((tmp_65, tmp_66, tmp_67, tmp_68));
            }
            for (block, expr) in other.relations.thir_block_expr.iter() {
                let tmp_69 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.thirblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_70 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                self.tables
                    .relations
                    .thir_block_expr
                    .insert((tmp_69, tmp_70));
            }
            for (expr, block, closest_unsafe_block, ty, span) in other.relations.thir_exprs.iter() {
                let tmp_71 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_72 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.thirblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_73 = if closest_unsafe_block.index() >= 1usize {
                    closest_unsafe_block.shift(self.tables.counters.thirblocks - (1usize as u64))
                } else {
                    *closest_unsafe_block
                };
                let tmp_74 = ty.shift(self.tables.counters.types - (0usize as u64));
                let tmp_75 = if span.index() >= 1usize {
                    span.shift(self.tables.counters.spans - (1usize as u64))
                } else {
                    *span
                };
                self.tables
                    .relations
                    .thir_exprs
                    .insert((tmp_71, tmp_72, tmp_73, tmp_74, tmp_75));
            }
            for (expr, inner) in other.relations.thir_exprs_scope.iter() {
                let tmp_76 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_77 = if inner.index() >= 1usize {
                    inner.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *inner
                };
                self.tables
                    .relations
                    .thir_exprs_scope
                    .insert((tmp_76, tmp_77));
            }
            for (expr, inner) in other.relations.thir_exprs_box.iter() {
                let tmp_78 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_79 = if inner.index() >= 1usize {
                    inner.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *inner
                };
                self.tables
                    .relations
                    .thir_exprs_box
                    .insert((tmp_78, tmp_79));
            }
            for (expr, cond, then_expr, else_expr) in other.relations.thir_exprs_if.iter() {
                let tmp_80 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_81 = if cond.index() >= 1usize {
                    cond.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *cond
                };
                let tmp_82 = if then_expr.index() >= 1usize {
                    then_expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *then_expr
                };
                let tmp_83 = if else_expr.index() >= 1usize {
                    else_expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *else_expr
                };
                self.tables
                    .relations
                    .thir_exprs_if
                    .insert((tmp_80, tmp_81, tmp_82, tmp_83));
            }
            for (expr, ty, fun, unsafety, abi, return_ty) in other.relations.thir_exprs_call.iter()
            {
                let tmp_84 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_85 = ty.shift(self.tables.counters.types - (0usize as u64));
                let tmp_86 = if fun.index() >= 1usize {
                    fun.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *fun
                };
                let tmp_87 = *unsafety;
                let tmp_88 = abis[abi];
                let tmp_89 = return_ty.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .thir_exprs_call
                    .insert((tmp_84, tmp_85, tmp_86, tmp_87, tmp_88, tmp_89));
            }
            for (call, index, arg) in other.relations.thir_exprs_call_arg.iter() {
                let tmp_90 = if call.index() >= 1usize {
                    call.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *call
                };
                let tmp_91 = *index;
                let tmp_92 = if arg.index() >= 1usize {
                    arg.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *arg
                };
                self.tables
                    .relations
                    .thir_exprs_call_arg
                    .insert((tmp_90, tmp_91, tmp_92));
            }
            for (fun, def_id) in other.relations.thir_exprs_call_const_target.iter() {
                let tmp_93 = if fun.index() >= 1usize {
                    fun.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *fun
                };
                let tmp_94 = def_paths[def_id];
                self.tables
                    .relations
                    .thir_exprs_call_const_target
                    .insert((tmp_93, tmp_94));
            }
            for (fun, target, function_generics, type_generics) in
                other.relations.thir_exprs_call_const_target_desc.iter()
            {
                let tmp_95 = if fun.index() >= 1usize {
                    fun.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *fun
                };
                let tmp_96 = strings[target];
                let tmp_97 = strings[function_generics];
                let tmp_98 = strings[type_generics];
                self.tables
                    .relations
                    .thir_exprs_call_const_target_desc
                    .insert((tmp_95, tmp_96, tmp_97, tmp_98));
            }
            for (fun, typ) in other.relations.thir_exprs_call_const_target_self.iter() {
                let tmp_99 = if fun.index() >= 1usize {
                    fun.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *fun
                };
                let tmp_100 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .thir_exprs_call_const_target_self
                    .insert((tmp_99, tmp_100));
            }
            for (expr, inner) in other.relations.thir_exprs_deref.iter() {
                let tmp_101 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_102 = if inner.index() >= 1usize {
                    inner.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *inner
                };
                self.tables
                    .relations
                    .thir_exprs_deref
                    .insert((tmp_101, tmp_102));
            }
            for (expr, op, lhs, rhs) in other.relations.thir_exprs_binary.iter() {
                let tmp_103 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_104 = thir_binary_op_kind[op];
                let tmp_105 = if lhs.index() >= 1usize {
                    lhs.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *lhs
                };
                let tmp_106 = if rhs.index() >= 1usize {
                    rhs.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *rhs
                };
                self.tables
                    .relations
                    .thir_exprs_binary
                    .insert((tmp_103, tmp_104, tmp_105, tmp_106));
            }
            for (expr, op, lhs, rhs) in other.relations.thir_exprs_logical_op.iter() {
                let tmp_107 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_108 = thir_logical_op_kind[op];
                let tmp_109 = if lhs.index() >= 1usize {
                    lhs.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *lhs
                };
                let tmp_110 = if rhs.index() >= 1usize {
                    rhs.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *rhs
                };
                self.tables
                    .relations
                    .thir_exprs_logical_op
                    .insert((tmp_107, tmp_108, tmp_109, tmp_110));
            }
            for (expr, op, arg) in other.relations.thir_exprs_unary.iter() {
                let tmp_111 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_112 = thir_unary_op_kind[op];
                let tmp_113 = if arg.index() >= 1usize {
                    arg.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *arg
                };
                self.tables
                    .relations
                    .thir_exprs_unary
                    .insert((tmp_111, tmp_112, tmp_113));
            }
            for (expr, source) in other.relations.thir_exprs_cast.iter() {
                let tmp_114 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_115 = if source.index() >= 1usize {
                    source.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *source
                };
                self.tables
                    .relations
                    .thir_exprs_cast
                    .insert((tmp_114, tmp_115));
            }
            for (expr, source) in other.relations.thir_exprs_use.iter() {
                let tmp_116 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_117 = if source.index() >= 1usize {
                    source.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *source
                };
                self.tables
                    .relations
                    .thir_exprs_use
                    .insert((tmp_116, tmp_117));
            }
            for (expr, source) in other.relations.thir_exprs_never_to_any.iter() {
                let tmp_118 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_119 = if source.index() >= 1usize {
                    source.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *source
                };
                self.tables
                    .relations
                    .thir_exprs_never_to_any
                    .insert((tmp_118, tmp_119));
            }
            for (expr, cast, source, is_from_as_cast) in
                other.relations.thir_exprs_pointer_coercion.iter()
            {
                let tmp_120 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_121 = *cast;
                let tmp_122 = if source.index() >= 1usize {
                    source.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *source
                };
                let tmp_123 = *is_from_as_cast;
                self.tables
                    .relations
                    .thir_exprs_pointer_coercion
                    .insert((tmp_120, tmp_121, tmp_122, tmp_123));
            }
            for (expr, body) in other.relations.thir_exprs_loop.iter() {
                let tmp_124 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_125 = if body.index() >= 1usize {
                    body.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *body
                };
                self.tables
                    .relations
                    .thir_exprs_loop
                    .insert((tmp_124, tmp_125));
            }
            for (expr, inner, pat) in other.relations.thir_exprs_let.iter() {
                let tmp_126 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_127 = if inner.index() >= 1usize {
                    inner.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *inner
                };
                let tmp_128 = if pat.index() >= 1usize {
                    pat.shift(self.tables.counters.thirpats - (1usize as u64))
                } else {
                    *pat
                };
                self.tables
                    .relations
                    .thir_exprs_let
                    .insert((tmp_126, tmp_127, tmp_128));
            }
            for (pat, ty, span) in other.relations.thir_pats.iter() {
                let tmp_129 = if pat.index() >= 1usize {
                    pat.shift(self.tables.counters.thirpats - (1usize as u64))
                } else {
                    *pat
                };
                let tmp_130 = ty.shift(self.tables.counters.types - (0usize as u64));
                let tmp_131 = if span.index() >= 1usize {
                    span.shift(self.tables.counters.spans - (1usize as u64))
                } else {
                    *span
                };
                self.tables
                    .relations
                    .thir_pats
                    .insert((tmp_129, tmp_130, tmp_131));
            }
            for (expr, scrutinee, match_source) in other.relations.thir_exprs_match.iter() {
                let tmp_132 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_133 = if scrutinee.index() >= 1usize {
                    scrutinee.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *scrutinee
                };
                let tmp_134 = *match_source;
                self.tables
                    .relations
                    .thir_exprs_match
                    .insert((tmp_132, tmp_133, tmp_134));
            }
            for (match_expr, arm_idx, guard, body) in other.relations.thir_match_arms.iter() {
                let tmp_135 = if match_expr.index() >= 1usize {
                    match_expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *match_expr
                };
                let tmp_136 = *arm_idx;
                let tmp_137 = if guard.index() >= 1usize {
                    guard.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *guard
                };
                let tmp_138 = if body.index() >= 1usize {
                    body.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *body
                };
                self.tables
                    .relations
                    .thir_match_arms
                    .insert((tmp_135, tmp_136, tmp_137, tmp_138));
            }
            for (expr, block) in other.relations.thir_exprs_block.iter() {
                let tmp_139 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_140 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.thirblocks - (1usize as u64))
                } else {
                    *block
                };
                self.tables
                    .relations
                    .thir_exprs_block
                    .insert((tmp_139, tmp_140));
            }
            for (expr, lhs, rhs) in other.relations.thir_exprs_assign.iter() {
                let tmp_141 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_142 = if lhs.index() >= 1usize {
                    lhs.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *lhs
                };
                let tmp_143 = if rhs.index() >= 1usize {
                    rhs.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *rhs
                };
                self.tables
                    .relations
                    .thir_exprs_assign
                    .insert((tmp_141, tmp_142, tmp_143));
            }
            for (expr, op, lhs, rhs) in other.relations.thir_exprs_assign_op.iter() {
                let tmp_144 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_145 = thir_binary_op_kind[op];
                let tmp_146 = if lhs.index() >= 1usize {
                    lhs.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *lhs
                };
                let tmp_147 = if rhs.index() >= 1usize {
                    rhs.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *rhs
                };
                self.tables
                    .relations
                    .thir_exprs_assign_op
                    .insert((tmp_144, tmp_145, tmp_146, tmp_147));
            }
            for (expr, lhs, variant_idx) in other.relations.thir_exprs_field.iter() {
                let tmp_148 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_149 = if lhs.index() >= 1usize {
                    lhs.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *lhs
                };
                let tmp_150 = *variant_idx;
                self.tables
                    .relations
                    .thir_exprs_field
                    .insert((tmp_148, tmp_149, tmp_150));
            }
            for (expr, lhs, index) in other.relations.thir_exprs_index.iter() {
                let tmp_151 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_152 = if lhs.index() >= 1usize {
                    lhs.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *lhs
                };
                let tmp_153 = if index.index() >= 1usize {
                    index.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *index
                };
                self.tables
                    .relations
                    .thir_exprs_index
                    .insert((tmp_151, tmp_152, tmp_153));
            }
            for (expr,) in other.relations.thir_exprs_var_ref.iter() {
                let tmp_154 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                self.tables.relations.thir_exprs_var_ref.insert((tmp_154,));
            }
            for (expr, closure_def_id) in other.relations.thir_exprs_upvar_ref.iter() {
                let tmp_155 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_156 = def_paths[closure_def_id];
                self.tables
                    .relations
                    .thir_exprs_upvar_ref
                    .insert((tmp_155, tmp_156));
            }
            for (expr, borrow_kind, arg) in other.relations.thir_exprs_borrow.iter() {
                let tmp_157 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_158 = *borrow_kind;
                let tmp_159 = if arg.index() >= 1usize {
                    arg.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *arg
                };
                self.tables
                    .relations
                    .thir_exprs_borrow
                    .insert((tmp_157, tmp_158, tmp_159));
            }
            for (expr, mutability, arg) in other.relations.thir_exprs_raw_borrow.iter() {
                let tmp_160 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_161 = *mutability;
                let tmp_162 = if arg.index() >= 1usize {
                    arg.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *arg
                };
                self.tables
                    .relations
                    .thir_exprs_raw_borrow
                    .insert((tmp_160, tmp_161, tmp_162));
            }
            for (expr, value) in other.relations.thir_exprs_break.iter() {
                let tmp_163 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_164 = if value.index() >= 1usize {
                    value.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *value
                };
                self.tables
                    .relations
                    .thir_exprs_break
                    .insert((tmp_163, tmp_164));
            }
            for (expr,) in other.relations.thir_exprs_continue.iter() {
                let tmp_165 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                self.tables.relations.thir_exprs_continue.insert((tmp_165,));
            }
            for (expr, value) in other.relations.thir_exprs_return.iter() {
                let tmp_166 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_167 = if value.index() >= 1usize {
                    value.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *value
                };
                self.tables
                    .relations
                    .thir_exprs_return
                    .insert((tmp_166, tmp_167));
            }
            for (expr, value) in other.relations.thir_exprs_become.iter() {
                let tmp_168 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_169 = if value.index() >= 1usize {
                    value.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *value
                };
                self.tables
                    .relations
                    .thir_exprs_become
                    .insert((tmp_168, tmp_169));
            }
            for (expr, did) in other.relations.thir_exprs_const_block.iter() {
                let tmp_170 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_171 = def_paths[did];
                self.tables
                    .relations
                    .thir_exprs_const_block
                    .insert((tmp_170, tmp_171));
            }
            for (expr, value) in other.relations.thir_exprs_repeat.iter() {
                let tmp_172 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_173 = if value.index() >= 1usize {
                    value.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *value
                };
                self.tables
                    .relations
                    .thir_exprs_repeat
                    .insert((tmp_172, tmp_173));
            }
            for (expr,) in other.relations.thir_exprs_array.iter() {
                let tmp_174 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                self.tables.relations.thir_exprs_array.insert((tmp_174,));
            }
            for (array_expr, index, element) in other.relations.thir_array_elements.iter() {
                let tmp_175 = if array_expr.index() >= 1usize {
                    array_expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *array_expr
                };
                let tmp_176 = *index;
                let tmp_177 = if element.index() >= 1usize {
                    element.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *element
                };
                self.tables
                    .relations
                    .thir_array_elements
                    .insert((tmp_175, tmp_176, tmp_177));
            }
            for (expr,) in other.relations.thir_exprs_tuple.iter() {
                let tmp_178 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                self.tables.relations.thir_exprs_tuple.insert((tmp_178,));
            }
            for (tuple_expr, index, element) in other.relations.thir_tuple_elements.iter() {
                let tmp_179 = if tuple_expr.index() >= 1usize {
                    tuple_expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *tuple_expr
                };
                let tmp_180 = *index;
                let tmp_181 = if element.index() >= 1usize {
                    element.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *element
                };
                self.tables
                    .relations
                    .thir_tuple_elements
                    .insert((tmp_179, tmp_180, tmp_181));
            }
            for (expr, base, variant_idx) in other.relations.thir_exprs_adt.iter() {
                let tmp_182 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_183 = if base.index() >= 1usize {
                    base.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *base
                };
                let tmp_184 = *variant_idx;
                self.tables
                    .relations
                    .thir_exprs_adt
                    .insert((tmp_182, tmp_183, tmp_184));
            }
            for (adt, field_idx, expr) in other.relations.thir_adt_field_expr.iter() {
                let tmp_185 = if adt.index() >= 1usize {
                    adt.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *adt
                };
                let tmp_186 = *field_idx;
                let tmp_187 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                self.tables
                    .relations
                    .thir_adt_field_expr
                    .insert((tmp_185, tmp_186, tmp_187));
            }
            for (expr, source, user_ty_span) in
                other.relations.thir_exprs_place_type_ascription.iter()
            {
                let tmp_188 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_189 = if source.index() >= 1usize {
                    source.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *source
                };
                let tmp_190 = if user_ty_span.index() >= 1usize {
                    user_ty_span.shift(self.tables.counters.spans - (1usize as u64))
                } else {
                    *user_ty_span
                };
                self.tables
                    .relations
                    .thir_exprs_place_type_ascription
                    .insert((tmp_188, tmp_189, tmp_190));
            }
            for (expr, source, user_ty_span) in
                other.relations.thir_exprs_value_type_ascription.iter()
            {
                let tmp_191 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_192 = if source.index() >= 1usize {
                    source.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *source
                };
                let tmp_193 = if user_ty_span.index() >= 1usize {
                    user_ty_span.shift(self.tables.counters.spans - (1usize as u64))
                } else {
                    *user_ty_span
                };
                self.tables
                    .relations
                    .thir_exprs_value_type_ascription
                    .insert((tmp_191, tmp_192, tmp_193));
            }
            for (expr, closure_id, movability) in other.relations.thir_exprs_closure.iter() {
                let tmp_194 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_195 = def_paths[closure_id];
                let tmp_196 = *movability;
                self.tables
                    .relations
                    .thir_exprs_closure
                    .insert((tmp_194, tmp_195, tmp_196));
            }
            for (closure_expr, index, upvar) in other.relations.thir_closure_upvars.iter() {
                let tmp_197 = if closure_expr.index() >= 1usize {
                    closure_expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *closure_expr
                };
                let tmp_198 = *index;
                let tmp_199 = if upvar.index() >= 1usize {
                    upvar.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *upvar
                };
                self.tables
                    .relations
                    .thir_closure_upvars
                    .insert((tmp_197, tmp_198, tmp_199));
            }
            for (expr, lit, neg) in other.relations.thir_exprs_literal.iter() {
                let tmp_200 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_201 = *lit;
                let tmp_202 = *neg;
                self.tables
                    .relations
                    .thir_exprs_literal
                    .insert((tmp_200, tmp_201, tmp_202));
            }
            for (expr, scalar) in other.relations.thir_exprs_non_hir_literal.iter() {
                let tmp_203 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_204 = *scalar;
                self.tables
                    .relations
                    .thir_exprs_non_hir_literal
                    .insert((tmp_203, tmp_204));
            }
            for (expr,) in other.relations.thir_exprs_zst_literal.iter() {
                let tmp_205 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                self.tables
                    .relations
                    .thir_exprs_zst_literal
                    .insert((tmp_205,));
            }
            for (expr, def_id) in other.relations.thir_exprs_named_const.iter() {
                let tmp_206 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_207 = def_paths[def_id];
                self.tables
                    .relations
                    .thir_exprs_named_const
                    .insert((tmp_206, tmp_207));
            }
            for (expr, def_id) in other.relations.thir_exprs_const_param.iter() {
                let tmp_208 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_209 = def_paths[def_id];
                self.tables
                    .relations
                    .thir_exprs_const_param
                    .insert((tmp_208, tmp_209));
            }
            for (expr, ty, def_id) in other.relations.thir_exprs_static_ref.iter() {
                let tmp_210 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_211 = ty.shift(self.tables.counters.types - (0usize as u64));
                let tmp_212 = def_paths[def_id];
                self.tables
                    .relations
                    .thir_exprs_static_ref
                    .insert((tmp_210, tmp_211, tmp_212));
            }
            for (expr,) in other.relations.thir_exprs_inline_asm.iter() {
                let tmp_213 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                self.tables
                    .relations
                    .thir_exprs_inline_asm
                    .insert((tmp_213,));
            }
            for (expr, container) in other.relations.thir_exprs_offset_of.iter() {
                let tmp_214 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_215 = container.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .thir_exprs_offset_of
                    .insert((tmp_214, tmp_215));
            }
            for (expr, def_id) in other.relations.thir_exprs_thread_local_ref.iter() {
                let tmp_216 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_217 = def_paths[def_id];
                self.tables
                    .relations
                    .thir_exprs_thread_local_ref
                    .insert((tmp_216, tmp_217));
            }
            for (expr, value) in other.relations.thir_exprs_yield.iter() {
                let tmp_218 = if expr.index() >= 1usize {
                    expr.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *expr
                };
                let tmp_219 = if value.index() >= 1usize {
                    value.shift(self.tables.counters.thirexprs - (1usize as u64))
                } else {
                    *value
                };
                self.tables
                    .relations
                    .thir_exprs_yield
                    .insert((tmp_218, tmp_219));
            }
            for (def_path, item, module, name, visibility, mutability) in
                other.relations.static_definitions.iter()
            {
                let tmp_220 = def_paths[def_path];
                let tmp_221 = item.shift(self.tables.counters.items - (0usize as u32));
                let tmp_222 = module.shift(self.tables.counters.modules - (0usize as u32));
                let tmp_223 = names[name];
                let tmp_224 = *visibility;
                let tmp_225 = *mutability;
                self.tables
                    .relations
                    .static_definitions
                    .insert((tmp_220, tmp_221, tmp_222, tmp_223, tmp_224, tmp_225));
            }
            for (
                def_path,
                item,
                module,
                name,
                visibility,
                unsafety,
                polarity,
                defaultness,
                constness,
                typ,
            ) in other.relations.impl_definitions.iter()
            {
                let tmp_226 = def_paths[def_path];
                let tmp_227 = item.shift(self.tables.counters.items - (0usize as u32));
                let tmp_228 = module.shift(self.tables.counters.modules - (0usize as u32));
                let tmp_229 = names[name];
                let tmp_230 = *visibility;
                let tmp_231 = *unsafety;
                let tmp_232 = *polarity;
                let tmp_233 = *defaultness;
                let tmp_234 = *constness;
                let tmp_235 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables.relations.impl_definitions.insert((
                    tmp_226, tmp_227, tmp_228, tmp_229, tmp_230, tmp_231, tmp_232, tmp_233,
                    tmp_234, tmp_235,
                ));
            }
            for (item, typ, trait_def_path) in other.relations.trait_impls.iter() {
                let tmp_236 = item.shift(self.tables.counters.items - (0usize as u32));
                let tmp_237 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_238 = def_paths[trait_def_path];
                self.tables
                    .relations
                    .trait_impls
                    .insert((tmp_236, tmp_237, tmp_238));
            }
            for (def_path, item, module, name, visibility) in
                other.relations.global_asm_blocks.iter()
            {
                let tmp_239 = def_paths[def_path];
                let tmp_240 = item.shift(self.tables.counters.items - (0usize as u32));
                let tmp_241 = module.shift(self.tables.counters.modules - (0usize as u32));
                let tmp_242 = names[name];
                let tmp_243 = *visibility;
                self.tables
                    .relations
                    .global_asm_blocks
                    .insert((tmp_239, tmp_240, tmp_241, tmp_242, tmp_243));
            }
            for (def_path, item, module, name, visibility) in other.relations.items.iter() {
                let tmp_244 = def_paths[def_path];
                let tmp_245 = item.shift(self.tables.counters.items - (0usize as u32));
                let tmp_246 = module.shift(self.tables.counters.modules - (0usize as u32));
                let tmp_247 = names[name];
                let tmp_248 = *visibility;
                self.tables
                    .relations
                    .items
                    .insert((tmp_244, tmp_245, tmp_246, tmp_247, tmp_248));
            }
            for (item, body_def_path, root_scope) in other.relations.mir_cfgs.iter() {
                let tmp_249 = item.shift(self.tables.counters.items - (0usize as u32));
                let tmp_250 = def_paths[body_def_path];
                let tmp_251 = root_scope.shift(self.tables.counters.scopes - (0usize as u32));
                self.tables
                    .relations
                    .mir_cfgs
                    .insert((tmp_249, tmp_250, tmp_251));
            }
            for (parent, child, safety, check_mode, explicit_unsafe_group, span) in
                other.relations.subscopes.iter()
            {
                let tmp_252 = parent.shift(self.tables.counters.scopes - (0usize as u32));
                let tmp_253 = child.shift(self.tables.counters.scopes - (0usize as u32));
                let tmp_254 = *safety;
                let tmp_255 = *check_mode;
                let tmp_256 = *explicit_unsafe_group;
                let tmp_257 = if span.index() >= 1usize {
                    span.shift(self.tables.counters.spans - (1usize as u64))
                } else {
                    *span
                };
                self.tables
                    .relations
                    .subscopes
                    .insert((tmp_252, tmp_253, tmp_254, tmp_255, tmp_256, tmp_257));
            }
            for (
                span,
                call_site_span,
                expansion_kind,
                expansion_kind_descr,
                file_name,
                line,
                col,
            ) in other.relations.spans.iter()
            {
                let tmp_258 = if span.index() >= 1usize {
                    span.shift(self.tables.counters.spans - (1usize as u64))
                } else {
                    *span
                };
                let tmp_259 = if call_site_span.index() >= 1usize {
                    call_site_span.shift(self.tables.counters.spans - (1usize as u64))
                } else {
                    *call_site_span
                };
                let tmp_260 = *expansion_kind;
                let tmp_261 = strings[expansion_kind_descr];
                let tmp_262 = span_file_names[file_name];
                let tmp_263 = *line;
                let tmp_264 = *col;
                self.tables.relations.spans.insert((
                    tmp_258, tmp_259, tmp_260, tmp_261, tmp_262, tmp_263, tmp_264,
                ));
            }
            for (span, macro_symbol, macro_definition_file_name, line, col) in
                other.relations.macro_expansions.iter()
            {
                let tmp_265 = if span.index() >= 1usize {
                    span.shift(self.tables.counters.spans - (1usize as u64))
                } else {
                    *span
                };
                let tmp_266 = strings[macro_symbol];
                let tmp_267 = span_file_names[macro_definition_file_name];
                let tmp_268 = *line;
                let tmp_269 = *col;
                self.tables
                    .relations
                    .macro_expansions
                    .insert((tmp_265, tmp_266, tmp_267, tmp_268, tmp_269));
            }
            for (build, key, value) in other.relations.crate_cfgs.iter() {
                let tmp_270 = builds[build];
                let tmp_271 = crate_cfg_keys[key];
                let tmp_272 = crate_cfg_values[value];
                self.tables
                    .relations
                    .crate_cfgs
                    .insert((tmp_270, tmp_271, tmp_272));
            }
            for (build, author) in other.relations.crate_authors.iter() {
                let tmp_273 = builds[build];
                let tmp_274 = strings[author];
                self.tables
                    .relations
                    .crate_authors
                    .insert((tmp_273, tmp_274));
            }
            for (build, keyword) in other.relations.crate_keywords.iter() {
                let tmp_275 = builds[build];
                let tmp_276 = strings[keyword];
                self.tables
                    .relations
                    .crate_keywords
                    .insert((tmp_275, tmp_276));
            }
            for (build, category) in other.relations.crate_categories.iter() {
                let tmp_277 = builds[build];
                let tmp_278 = strings[category];
                self.tables
                    .relations
                    .crate_categories
                    .insert((tmp_277, tmp_278));
            }
            for (item, typ, def_path, name, visibility, kind) in other.relations.type_defs.iter() {
                let tmp_279 = item.shift(self.tables.counters.items - (0usize as u32));
                let tmp_280 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_281 = def_paths[def_path];
                let tmp_282 = strings[name];
                let tmp_283 = *visibility;
                let tmp_284 = *kind;
                self.tables
                    .relations
                    .type_defs
                    .insert((tmp_279, tmp_280, tmp_281, tmp_282, tmp_283, tmp_284));
            }
            for (typ, kind) in other.relations.types.iter() {
                let tmp_285 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_286 = type_kinds[kind];
                self.tables.relations.types.insert((tmp_285, tmp_286));
            }
            for (typ, primitive_kind) in other.relations.types_primitive.iter() {
                let tmp_287 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_288 = *primitive_kind;
                self.tables
                    .relations
                    .types_primitive
                    .insert((tmp_287, tmp_288));
            }
            for (typ, def_path, kind, c_repr, is_phantom) in other.relations.types_adt_def.iter() {
                let tmp_289 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_290 = def_paths[def_path];
                let tmp_291 = *kind;
                let tmp_292 = *c_repr;
                let tmp_293 = *is_phantom;
                self.tables
                    .relations
                    .types_adt_def
                    .insert((tmp_289, tmp_290, tmp_291, tmp_292, tmp_293));
            }
            for (adt, index, def_path, ident) in other.relations.types_adt_variant.iter() {
                let tmp_294 = adt.shift(self.tables.counters.types - (0usize as u64));
                let tmp_295 = *index;
                let tmp_296 = def_paths[def_path];
                let tmp_297 = strings[ident];
                self.tables
                    .relations
                    .types_adt_variant
                    .insert((tmp_294, tmp_295, tmp_296, tmp_297));
            }
            for (field, adt, index, def_path, ident, visibility, typ) in
                other.relations.types_adt_field.iter()
            {
                let tmp_298 = field.shift(self.tables.counters.fields - (0usize as u64));
                let tmp_299 = adt.shift(self.tables.counters.types - (0usize as u64));
                let tmp_300 = *index;
                let tmp_301 = def_paths[def_path];
                let tmp_302 = strings[ident];
                let tmp_303 = *visibility;
                let tmp_304 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables.relations.types_adt_field.insert((
                    tmp_298, tmp_299, tmp_300, tmp_301, tmp_302, tmp_303, tmp_304,
                ));
            }
            for (field, module) in other.relations.types_adt_field_visible_in.iter() {
                let tmp_305 = field.shift(self.tables.counters.fields - (0usize as u64));
                let tmp_306 = def_paths[module];
                self.tables
                    .relations
                    .types_adt_field_visible_in
                    .insert((tmp_305, tmp_306));
            }
            for (typ, foreign_def_path) in other.relations.types_foreign.iter() {
                let tmp_307 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_308 = def_paths[foreign_def_path];
                self.tables
                    .relations
                    .types_foreign
                    .insert((tmp_307, tmp_308));
            }
            for (typ, element_type) in other.relations.types_array.iter() {
                let tmp_309 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_310 = element_type.shift(self.tables.counters.types - (0usize as u64));
                self.tables.relations.types_array.insert((tmp_309, tmp_310));
            }
            for (typ, element_type) in other.relations.types_slice.iter() {
                let tmp_311 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_312 = element_type.shift(self.tables.counters.types - (0usize as u64));
                self.tables.relations.types_slice.insert((tmp_311, tmp_312));
            }
            for (typ, target_type, mutability) in other.relations.types_raw_ptr.iter() {
                let tmp_313 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_314 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_315 = *mutability;
                self.tables
                    .relations
                    .types_raw_ptr
                    .insert((tmp_313, tmp_314, tmp_315));
            }
            for (typ, target_type, mutability) in other.relations.types_ref.iter() {
                let tmp_316 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_317 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_318 = *mutability;
                self.tables
                    .relations
                    .types_ref
                    .insert((tmp_316, tmp_317, tmp_318));
            }
            for (typ, def_path) in other.relations.types_fn_def.iter() {
                let tmp_319 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_320 = def_paths[def_path];
                self.tables
                    .relations
                    .types_fn_def
                    .insert((tmp_319, tmp_320));
            }
            for (typ,) in other.relations.types_fn_ptr.iter() {
                let tmp_321 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables.relations.types_fn_ptr.insert((tmp_321,));
            }
            for (typ,) in other.relations.types_dynamic.iter() {
                let tmp_322 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables.relations.types_dynamic.insert((tmp_322,));
            }
            for (typ, def_path, is_auto) in other.relations.types_dynamic_trait.iter() {
                let tmp_323 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_324 = def_paths[def_path];
                let tmp_325 = *is_auto;
                self.tables
                    .relations
                    .types_dynamic_trait
                    .insert((tmp_323, tmp_324, tmp_325));
            }
            for (typ, def_path) in other.relations.types_closure.iter() {
                let tmp_326 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_327 = def_paths[def_path];
                self.tables
                    .relations
                    .types_closure
                    .insert((tmp_326, tmp_327));
            }
            for (typ, def_path) in other.relations.types_coroutine.iter() {
                let tmp_328 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_329 = def_paths[def_path];
                self.tables
                    .relations
                    .types_coroutine
                    .insert((tmp_328, tmp_329));
            }
            for (typ,) in other.relations.types_coroutine_witness.iter() {
                let tmp_330 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .types_coroutine_witness
                    .insert((tmp_330,));
            }
            for (typ, def_path) in other.relations.types_coroutine_closure.iter() {
                let tmp_331 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_332 = def_paths[def_path];
                self.tables
                    .relations
                    .types_coroutine_closure
                    .insert((tmp_331, tmp_332));
            }
            for (typ,) in other.relations.types_pat.iter() {
                let tmp_333 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables.relations.types_pat.insert((tmp_333,));
            }
            for (typ,) in other.relations.types_tuple.iter() {
                let tmp_334 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables.relations.types_tuple.insert((tmp_334,));
            }
            for (tuple_type, index, typ) in other.relations.types_tuple_element.iter() {
                let tmp_335 = tuple_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_336 = *index;
                let tmp_337 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .types_tuple_element
                    .insert((tmp_335, tmp_336, tmp_337));
            }
            for (typ, trait_def_path, trait_item) in other.relations.types_projection.iter() {
                let tmp_338 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_339 = def_paths[trait_def_path];
                let tmp_340 = def_paths[trait_item];
                self.tables
                    .relations
                    .types_projection
                    .insert((tmp_338, tmp_339, tmp_340));
            }
            for (typ, def_path) in other.relations.types_opaque.iter() {
                let tmp_341 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_342 = def_paths[def_path];
                self.tables
                    .relations
                    .types_opaque
                    .insert((tmp_341, tmp_342));
            }
            for (typ, def_path) in other.relations.types_inherent.iter() {
                let tmp_343 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_344 = def_paths[def_path];
                self.tables
                    .relations
                    .types_inherent
                    .insert((tmp_343, tmp_344));
            }
            for (typ, def_path) in other.relations.types_weak.iter() {
                let tmp_345 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_346 = def_paths[def_path];
                self.tables.relations.types_weak.insert((tmp_345, tmp_346));
            }
            for (typ, index, name) in other.relations.types_param.iter() {
                let tmp_347 = typ.shift(self.tables.counters.types - (0usize as u64));
                let tmp_348 = *index;
                let tmp_349 = strings[name];
                self.tables
                    .relations
                    .types_param
                    .insert((tmp_347, tmp_348, tmp_349));
            }
            for (item, def_path, name, visibility, is_auto, is_marker, unsafety) in
                other.relations.traits.iter()
            {
                let tmp_350 = item.shift(self.tables.counters.items - (0usize as u32));
                let tmp_351 = def_paths[def_path];
                let tmp_352 = strings[name];
                let tmp_353 = *visibility;
                let tmp_354 = *is_auto;
                let tmp_355 = *is_marker;
                let tmp_356 = *unsafety;
                self.tables.relations.traits.insert((
                    tmp_350, tmp_351, tmp_352, tmp_353, tmp_354, tmp_355, tmp_356,
                ));
            }
            for (trait_id, def_path, defaultness) in other.relations.trait_items.iter() {
                let tmp_357 = trait_id.shift(self.tables.counters.items - (0usize as u32));
                let tmp_358 = def_paths[def_path];
                let tmp_359 = *defaultness;
                self.tables
                    .relations
                    .trait_items
                    .insert((tmp_357, tmp_358, tmp_359));
            }
            for (block, mir, kind) in other.relations.basic_blocks.iter() {
                let tmp_360 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_361 = def_paths[mir];
                let tmp_362 = *kind;
                self.tables
                    .relations
                    .basic_blocks
                    .insert((tmp_360, tmp_361, tmp_362));
            }
            for (stmt, block, index, kind, scope) in other.relations.statements.iter() {
                let tmp_363 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_364 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_365 = *index;
                let tmp_366 = statement_kinds[kind];
                let tmp_367 = scope.shift(self.tables.counters.scopes - (0usize as u32));
                self.tables
                    .relations
                    .statements
                    .insert((tmp_363, tmp_364, tmp_365, tmp_366, tmp_367));
            }
            for (stmt, target_type, operand) in other.relations.statements_assign_use.iter() {
                let tmp_368 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_369 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_370 = operand.shift(self.tables.counters.operands - (0usize as u64));
                self.tables
                    .relations
                    .statements_assign_use
                    .insert((tmp_368, tmp_369, tmp_370));
            }
            for (stmt, target_type, def_path) in
                other.relations.statements_assign_thead_local_ref.iter()
            {
                let tmp_371 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_372 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_373 = def_paths[def_path];
                self.tables
                    .relations
                    .statements_assign_thead_local_ref
                    .insert((tmp_371, tmp_372, tmp_373));
            }
            for (stmt, target_type, operand, count) in
                other.relations.statements_assign_repeat.iter()
            {
                let tmp_374 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_375 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_376 = operand.shift(self.tables.counters.operands - (0usize as u64));
                let tmp_377 = *count;
                self.tables
                    .relations
                    .statements_assign_repeat
                    .insert((tmp_374, tmp_375, tmp_376, tmp_377));
            }
            for (stmt, target_type, source_type, kind) in
                other.relations.statements_assign_ref.iter()
            {
                let tmp_378 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_379 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_380 = source_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_381 = *kind;
                self.tables
                    .relations
                    .statements_assign_ref
                    .insert((tmp_378, tmp_379, tmp_380, tmp_381));
            }
            for (stmt, target_type, source_type, mutability) in
                other.relations.statements_assign_address.iter()
            {
                let tmp_382 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_383 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_384 = source_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_385 = *mutability;
                self.tables
                    .relations
                    .statements_assign_address
                    .insert((tmp_382, tmp_383, tmp_384, tmp_385));
            }
            for (stmt, target_type, source_type) in other.relations.statements_assign_len.iter() {
                let tmp_386 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_387 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_388 = source_type.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .statements_assign_len
                    .insert((tmp_386, tmp_387, tmp_388));
            }
            for (stmt, target_type, kind, operand, typ) in
                other.relations.statements_assign_cast.iter()
            {
                let tmp_389 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_390 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_391 = *kind;
                let tmp_392 = operand.shift(self.tables.counters.operands - (0usize as u64));
                let tmp_393 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .statements_assign_cast
                    .insert((tmp_389, tmp_390, tmp_391, tmp_392, tmp_393));
            }
            for (stmt, target_type, kind, first, second) in
                other.relations.statements_assign_binary_op.iter()
            {
                let tmp_394 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_395 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_396 = binary_op_kind[kind];
                let tmp_397 = first.shift(self.tables.counters.operands - (0usize as u64));
                let tmp_398 = second.shift(self.tables.counters.operands - (0usize as u64));
                self.tables
                    .relations
                    .statements_assign_binary_op
                    .insert((tmp_394, tmp_395, tmp_396, tmp_397, tmp_398));
            }
            for (stmt, target_type, kind, first, second) in
                other.relations.statements_assign_checked_binary_op.iter()
            {
                let tmp_399 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_400 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_401 = binary_op_kind[kind];
                let tmp_402 = first.shift(self.tables.counters.operands - (0usize as u64));
                let tmp_403 = second.shift(self.tables.counters.operands - (0usize as u64));
                self.tables
                    .relations
                    .statements_assign_checked_binary_op
                    .insert((tmp_399, tmp_400, tmp_401, tmp_402, tmp_403));
            }
            for (stmt, target_type, kind, source_type) in
                other.relations.statements_assign_nullary_op.iter()
            {
                let tmp_404 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_405 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_406 = nullary_op_kind[kind];
                let tmp_407 = source_type.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .statements_assign_nullary_op
                    .insert((tmp_404, tmp_405, tmp_406, tmp_407));
            }
            for (stmt, target_type, kind, operand) in
                other.relations.statements_assign_unary_op.iter()
            {
                let tmp_408 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_409 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_410 = unary_op_kind[kind];
                let tmp_411 = operand.shift(self.tables.counters.operands - (0usize as u64));
                self.tables
                    .relations
                    .statements_assign_unary_op
                    .insert((tmp_408, tmp_409, tmp_410, tmp_411));
            }
            for (stmt, target_type, source_type) in
                other.relations.statements_assign_discriminant.iter()
            {
                let tmp_412 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_413 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_414 = source_type.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .statements_assign_discriminant
                    .insert((tmp_412, tmp_413, tmp_414));
            }
            for (stmt, target_type, kind) in other.relations.statements_assign_aggregate.iter() {
                let tmp_415 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_416 = target_type.shift(self.tables.counters.types - (0usize as u64));
                let tmp_417 = *kind;
                self.tables
                    .relations
                    .statements_assign_aggregate
                    .insert((tmp_415, tmp_416, tmp_417));
            }
            for (stmt, index, operand) in
                other.relations.statements_assign_aggregate_operands.iter()
            {
                let tmp_418 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_419 = *index;
                let tmp_420 = operand.shift(self.tables.counters.operands - (0usize as u64));
                self.tables
                    .relations
                    .statements_assign_aggregate_operands
                    .insert((tmp_418, tmp_419, tmp_420));
            }
            for (stmt, operand, typ) in other.relations.statements_assign_shallow_init_box.iter() {
                let tmp_421 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_422 = operand.shift(self.tables.counters.operands - (0usize as u64));
                let tmp_423 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .statements_assign_shallow_init_box
                    .insert((tmp_421, tmp_422, tmp_423));
            }
            for (stmt, place_type) in other.relations.statements_assign_copy_for_deref.iter() {
                let tmp_424 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_425 = place_type.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .statements_assign_copy_for_deref
                    .insert((tmp_424, tmp_425));
            }
            for (stmt, operand) in other.relations.statements_inline_asm_inputs.iter() {
                let tmp_426 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_427 = operand.shift(self.tables.counters.operands - (0usize as u64));
                self.tables
                    .relations
                    .statements_inline_asm_inputs
                    .insert((tmp_426, tmp_427));
            }
            for (stmt, typ) in other.relations.statements_inline_asm_outputs.iter() {
                let tmp_428 = stmt.shift(self.tables.counters.statements - (0usize as u64));
                let tmp_429 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .statements_inline_asm_outputs
                    .insert((tmp_428, tmp_429));
            }
            for (operand, kind, typ) in other.relations.operands.iter() {
                let tmp_430 = operand.shift(self.tables.counters.operands - (0usize as u64));
                let tmp_431 = *kind;
                let tmp_432 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .operands
                    .insert((tmp_430, tmp_431, tmp_432));
            }
            for (block, kind, scope) in other.relations.terminators.iter() {
                let tmp_433 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_434 = terminator_kinds[kind];
                let tmp_435 = scope.shift(self.tables.counters.scopes - (0usize as u32));
                self.tables
                    .relations
                    .terminators
                    .insert((tmp_433, tmp_434, tmp_435));
            }
            for (block, target) in other.relations.terminators_goto.iter() {
                let tmp_436 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_437 = if target.index() >= 1usize {
                    target.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *target
                };
                self.tables
                    .relations
                    .terminators_goto
                    .insert((tmp_436, tmp_437));
            }
            for (block, discriminant) in other.relations.terminators_switch_int.iter() {
                let tmp_438 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_439 = discriminant.shift(self.tables.counters.operands - (0usize as u64));
                self.tables
                    .relations
                    .terminators_switch_int
                    .insert((tmp_438, tmp_439));
            }
            for (block, condition_value, target) in
                other.relations.terminators_switch_int_targets.iter()
            {
                let tmp_440 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_441 = *condition_value;
                let tmp_442 = if target.index() >= 1usize {
                    target.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *target
                };
                self.tables
                    .relations
                    .terminators_switch_int_targets
                    .insert((tmp_440, tmp_441, tmp_442));
            }
            for (block, location, target) in other.relations.terminators_drop.iter() {
                let tmp_443 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_444 = location.shift(self.tables.counters.types - (0usize as u64));
                let tmp_445 = if target.index() >= 1usize {
                    target.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *target
                };
                self.tables
                    .relations
                    .terminators_drop
                    .insert((tmp_443, tmp_444, tmp_445));
            }
            for (block, location, value, target, unwind) in
                other.relations.terminators_drop_and_replace.iter()
            {
                let tmp_446 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_447 = location.shift(self.tables.counters.types - (0usize as u64));
                let tmp_448 = value.shift(self.tables.counters.operands - (0usize as u64));
                let tmp_449 = if target.index() >= 1usize {
                    target.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *target
                };
                let tmp_450 = if unwind.index() >= 1usize {
                    unwind.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *unwind
                };
                self.tables
                    .relations
                    .terminators_drop_and_replace
                    .insert((tmp_446, tmp_447, tmp_448, tmp_449, tmp_450));
            }
            for (block, call, func, unsafety, abi, return_ty, destination, span) in
                other.relations.terminators_call.iter()
            {
                let tmp_451 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_452 = call.shift(self.tables.counters.functioncalls - (0usize as u32));
                let tmp_453 = func.shift(self.tables.counters.operands - (0usize as u64));
                let tmp_454 = *unsafety;
                let tmp_455 = abis[abi];
                let tmp_456 = return_ty.shift(self.tables.counters.types - (0usize as u64));
                let tmp_457 = if destination.index() >= 1usize {
                    destination.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *destination
                };
                let tmp_458 = if span.index() >= 1usize {
                    span.shift(self.tables.counters.spans - (1usize as u64))
                } else {
                    *span
                };
                self.tables.relations.terminators_call.insert((
                    tmp_451, tmp_452, tmp_453, tmp_454, tmp_455, tmp_456, tmp_457, tmp_458,
                ));
            }
            for (call, index, arg) in other.relations.terminators_call_arg.iter() {
                let tmp_459 = call.shift(self.tables.counters.functioncalls - (0usize as u32));
                let tmp_460 = *index;
                let tmp_461 = arg.shift(self.tables.counters.operands - (0usize as u64));
                self.tables
                    .relations
                    .terminators_call_arg
                    .insert((tmp_459, tmp_460, tmp_461));
            }
            for (call, def_path) in other.relations.terminators_call_const_target.iter() {
                let tmp_462 = call.shift(self.tables.counters.functioncalls - (0usize as u32));
                let tmp_463 = def_paths[def_path];
                self.tables
                    .relations
                    .terminators_call_const_target
                    .insert((tmp_462, tmp_463));
            }
            for (call, target, function_generics, type_generics) in
                other.relations.terminators_call_const_target_desc.iter()
            {
                let tmp_464 = call.shift(self.tables.counters.functioncalls - (0usize as u32));
                let tmp_465 = strings[target];
                let tmp_466 = strings[function_generics];
                let tmp_467 = strings[type_generics];
                self.tables
                    .relations
                    .terminators_call_const_target_desc
                    .insert((tmp_464, tmp_465, tmp_466, tmp_467));
            }
            for (call, typ) in other.relations.terminators_call_const_target_self.iter() {
                let tmp_468 = call.shift(self.tables.counters.functioncalls - (0usize as u32));
                let tmp_469 = typ.shift(self.tables.counters.types - (0usize as u64));
                self.tables
                    .relations
                    .terminators_call_const_target_self
                    .insert((tmp_468, tmp_469));
            }
            for (call, macro_path) in other.relations.terminators_call_macro_backtrace.iter() {
                let tmp_470 = call.shift(self.tables.counters.functioncalls - (0usize as u32));
                let tmp_471 = strings[macro_path];
                self.tables
                    .relations
                    .terminators_call_macro_backtrace
                    .insert((tmp_470, tmp_471));
            }
            for (block, cond, expected, target) in other.relations.terminators_assert.iter() {
                let tmp_472 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_473 = cond.shift(self.tables.counters.operands - (0usize as u64));
                let tmp_474 = *expected;
                let tmp_475 = if target.index() >= 1usize {
                    target.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *target
                };
                self.tables
                    .relations
                    .terminators_assert
                    .insert((tmp_472, tmp_473, tmp_474, tmp_475));
            }
            for (block, value, resume, drop) in other.relations.terminators_yield.iter() {
                let tmp_476 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_477 = value.shift(self.tables.counters.operands - (0usize as u64));
                let tmp_478 = if resume.index() >= 1usize {
                    resume.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *resume
                };
                let tmp_479 = if drop.index() >= 1usize {
                    drop.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *drop
                };
                self.tables
                    .relations
                    .terminators_yield
                    .insert((tmp_476, tmp_477, tmp_478, tmp_479));
            }
            for (block, real_target, imaginary_target) in
                other.relations.terminators_false_edges.iter()
            {
                let tmp_480 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_481 = if real_target.index() >= 1usize {
                    real_target.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *real_target
                };
                let tmp_482 = if imaginary_target.index() >= 1usize {
                    imaginary_target.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *imaginary_target
                };
                self.tables
                    .relations
                    .terminators_false_edges
                    .insert((tmp_480, tmp_481, tmp_482));
            }
            for (block, real_target) in other.relations.terminators_false_unwind.iter() {
                let tmp_483 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_484 = if real_target.index() >= 1usize {
                    real_target.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *real_target
                };
                self.tables
                    .relations
                    .terminators_false_unwind
                    .insert((tmp_483, tmp_484));
            }
            for (block,) in other.relations.terminators_inline_asm.iter() {
                let tmp_485 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                self.tables
                    .relations
                    .terminators_inline_asm
                    .insert((tmp_485,));
            }
            for (block, action, cleanup) in other.relations.terminators_unwind_action.iter() {
                let tmp_486 = if block.index() >= 1usize {
                    block.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *block
                };
                let tmp_487 = *action;
                let tmp_488 = if cleanup.index() >= 1usize {
                    cleanup.shift(self.tables.counters.basicblocks - (1usize as u64))
                } else {
                    *cleanup
                };
                self.tables
                    .relations
                    .terminators_unwind_action
                    .insert((tmp_486, tmp_487, tmp_488));
            }
            self.tables.counters.modules += other.counters.modules - (0usize as u32);
            self.tables.counters.items += other.counters.items - (0usize as u32);
            self.tables.counters.scopes += other.counters.scopes - (0usize as u32);
            self.tables.counters.functioncalls += other.counters.functioncalls - (0usize as u32);
            self.tables.counters.spans += other.counters.spans - (1usize as u64);
            self.tables.counters.types += other.counters.types - (0usize as u64);
            self.tables.counters.fields += other.counters.fields - (0usize as u64);
            self.tables.counters.operands += other.counters.operands - (0usize as u64);
            self.tables.counters.basicblocks += other.counters.basicblocks - (1usize as u64);
            self.tables.counters.statements += other.counters.statements - (0usize as u64);
            self.tables.counters.thirblocks += other.counters.thirblocks - (1usize as u64);
            self.tables.counters.thirexprs += other.counters.thirexprs - (1usize as u64);
            self.tables.counters.thirpats += other.counters.thirpats - (1usize as u64);
            self.tables.counters.thirstmts += other.counters.thirstmts - (1usize as u64);
        }
        pub fn tables(&mut self) -> &mut Tables {
            &mut self.tables
        }
    }
    impl Tables {
        pub fn load_multifile(database_root: &Path) -> Result<Tables> {
            let relations = load_multifile_relations(&database_root.join("relations"))?;
            let counters = load_counters(&database_root.join("counters.bincode"))?;
            let interning_tables = load_interning_tables(&database_root.join("interning"))?;
            Ok(Tables {
                relations,
                counters,
                interning_tables,
            })
        }
        pub fn load_single_file(tables_file: &Path) -> Result<Tables> {
            crate::storage::load(tables_file)
        }
        pub fn store_multifile(&self, database_root: &Path) -> Result<()> {
            let relations_path = database_root.join("relations");
            std::fs::create_dir_all(&relations_path)?;
            store_multifile_relations(&self.relations, &relations_path);
            let counters_path = database_root.join("counters.bincode");
            store_counters(&self.counters, &counters_path);
            let interning_tables_path = &database_root.join("interning");
            std::fs::create_dir_all(&interning_tables_path)?;
            store_multifile_interning_tables(&self.interning_tables, &interning_tables_path);
            Ok(())
        }
    }
    fn load_multifile_relations(path: &Path) -> Result<Relations> {
        Ok(Relations {
            def_path_span: unsafe {
                Relation::load(8883063296299969981u64, path.join("def_path_span"))
            }?,
            type_description: unsafe {
                Relation::load(14247184057110084413u64, path.join("type_description"))
            }?,
            build_crate_types: unsafe {
                Relation::load(11694151696289681161u64, path.join("build_crate_types"))
            }?,
            root_modules: unsafe {
                Relation::load(3608617112668148983u64, path.join("root_modules"))
            }?,
            submodules: unsafe {
                Relation::load(14223402788489432104u64, path.join("submodules"))
            }?,
            function_definitions: unsafe {
                Relation::load(6078223197665779672u64, path.join("function_definitions"))
            }?,
            function_parameter_types: unsafe {
                Relation::load(
                    17967330428057203654u64,
                    path.join("function_parameter_types"),
                )
            }?,
            function_unsafe_use: unsafe {
                Relation::load(3679592891414654765u64, path.join("function_unsafe_use"))
            }?,
            function_unsafe_reasons: unsafe {
                Relation::load(7306117882207793123u64, path.join("function_unsafe_reasons"))
            }?,
            thir_bodies: unsafe {
                Relation::load(9078912835344629248u64, path.join("thir_bodies"))
            }?,
            thir_blocks: unsafe {
                Relation::load(5712874088580563410u64, path.join("thir_blocks"))
            }?,
            thir_stmts: unsafe {
                Relation::load(11662528851258065727u64, path.join("thir_stmts"))
            }?,
            thir_stmts_expr: unsafe {
                Relation::load(6974731257762681957u64, path.join("thir_stmts_expr"))
            }?,
            thir_stmts_let: unsafe {
                Relation::load(3948572987018662193u64, path.join("thir_stmts_let"))
            }?,
            thir_block_expr: unsafe {
                Relation::load(13932710207231639296u64, path.join("thir_block_expr"))
            }?,
            thir_exprs: unsafe {
                Relation::load(14472862124235708861u64, path.join("thir_exprs"))
            }?,
            thir_exprs_scope: unsafe {
                Relation::load(15126226146931209448u64, path.join("thir_exprs_scope"))
            }?,
            thir_exprs_box: unsafe {
                Relation::load(17250276924894967218u64, path.join("thir_exprs_box"))
            }?,
            thir_exprs_if: unsafe {
                Relation::load(1136822200270957559u64, path.join("thir_exprs_if"))
            }?,
            thir_exprs_call: unsafe {
                Relation::load(10675202487661065417u64, path.join("thir_exprs_call"))
            }?,
            thir_exprs_call_arg: unsafe {
                Relation::load(12688704234864686271u64, path.join("thir_exprs_call_arg"))
            }?,
            thir_exprs_call_const_target: unsafe {
                Relation::load(
                    282454153203586278u64,
                    path.join("thir_exprs_call_const_target"),
                )
            }?,
            thir_exprs_call_const_target_desc: unsafe {
                Relation::load(
                    14557156226083258014u64,
                    path.join("thir_exprs_call_const_target_desc"),
                )
            }?,
            thir_exprs_call_const_target_self: unsafe {
                Relation::load(
                    11355230934198225002u64,
                    path.join("thir_exprs_call_const_target_self"),
                )
            }?,
            thir_exprs_deref: unsafe {
                Relation::load(13435985341992119844u64, path.join("thir_exprs_deref"))
            }?,
            thir_exprs_binary: unsafe {
                Relation::load(14413232755879446303u64, path.join("thir_exprs_binary"))
            }?,
            thir_exprs_logical_op: unsafe {
                Relation::load(18234645225945847180u64, path.join("thir_exprs_logical_op"))
            }?,
            thir_exprs_unary: unsafe {
                Relation::load(18219444126659999359u64, path.join("thir_exprs_unary"))
            }?,
            thir_exprs_cast: unsafe {
                Relation::load(15582714740127479454u64, path.join("thir_exprs_cast"))
            }?,
            thir_exprs_use: unsafe {
                Relation::load(18202594178015551134u64, path.join("thir_exprs_use"))
            }?,
            thir_exprs_never_to_any: unsafe {
                Relation::load(
                    17470269400781477793u64,
                    path.join("thir_exprs_never_to_any"),
                )
            }?,
            thir_exprs_pointer_coercion: unsafe {
                Relation::load(
                    6101599652373463408u64,
                    path.join("thir_exprs_pointer_coercion"),
                )
            }?,
            thir_exprs_loop: unsafe {
                Relation::load(3098191845330208770u64, path.join("thir_exprs_loop"))
            }?,
            thir_exprs_let: unsafe {
                Relation::load(9632315457169436561u64, path.join("thir_exprs_let"))
            }?,
            thir_pats: unsafe { Relation::load(10536163954959029661u64, path.join("thir_pats")) }?,
            thir_exprs_match: unsafe {
                Relation::load(5226457329152460944u64, path.join("thir_exprs_match"))
            }?,
            thir_match_arms: unsafe {
                Relation::load(9822157818036781677u64, path.join("thir_match_arms"))
            }?,
            thir_exprs_block: unsafe {
                Relation::load(15876734076667292329u64, path.join("thir_exprs_block"))
            }?,
            thir_exprs_assign: unsafe {
                Relation::load(15488937819083875309u64, path.join("thir_exprs_assign"))
            }?,
            thir_exprs_assign_op: unsafe {
                Relation::load(4838645763299739861u64, path.join("thir_exprs_assign_op"))
            }?,
            thir_exprs_field: unsafe {
                Relation::load(23925721899557405u64, path.join("thir_exprs_field"))
            }?,
            thir_exprs_index: unsafe {
                Relation::load(5832191315312159083u64, path.join("thir_exprs_index"))
            }?,
            thir_exprs_var_ref: unsafe {
                Relation::load(296084846650957428u64, path.join("thir_exprs_var_ref"))
            }?,
            thir_exprs_upvar_ref: unsafe {
                Relation::load(2787850572062464717u64, path.join("thir_exprs_upvar_ref"))
            }?,
            thir_exprs_borrow: unsafe {
                Relation::load(6349143721208772268u64, path.join("thir_exprs_borrow"))
            }?,
            thir_exprs_raw_borrow: unsafe {
                Relation::load(13059666409505041503u64, path.join("thir_exprs_raw_borrow"))
            }?,
            thir_exprs_break: unsafe {
                Relation::load(6740441886473887693u64, path.join("thir_exprs_break"))
            }?,
            thir_exprs_continue: unsafe {
                Relation::load(12830730509208436793u64, path.join("thir_exprs_continue"))
            }?,
            thir_exprs_return: unsafe {
                Relation::load(17654622868380585299u64, path.join("thir_exprs_return"))
            }?,
            thir_exprs_become: unsafe {
                Relation::load(16769260330021479451u64, path.join("thir_exprs_become"))
            }?,
            thir_exprs_const_block: unsafe {
                Relation::load(12642227791060480364u64, path.join("thir_exprs_const_block"))
            }?,
            thir_exprs_repeat: unsafe {
                Relation::load(2338369464665128106u64, path.join("thir_exprs_repeat"))
            }?,
            thir_exprs_array: unsafe {
                Relation::load(1124144228664747403u64, path.join("thir_exprs_array"))
            }?,
            thir_array_elements: unsafe {
                Relation::load(10250490187218554703u64, path.join("thir_array_elements"))
            }?,
            thir_exprs_tuple: unsafe {
                Relation::load(15653329952619259353u64, path.join("thir_exprs_tuple"))
            }?,
            thir_tuple_elements: unsafe {
                Relation::load(7005504983456194338u64, path.join("thir_tuple_elements"))
            }?,
            thir_exprs_adt: unsafe {
                Relation::load(13642234560780341906u64, path.join("thir_exprs_adt"))
            }?,
            thir_adt_field_expr: unsafe {
                Relation::load(14683105706445521333u64, path.join("thir_adt_field_expr"))
            }?,
            thir_exprs_place_type_ascription: unsafe {
                Relation::load(
                    10691137482463784180u64,
                    path.join("thir_exprs_place_type_ascription"),
                )
            }?,
            thir_exprs_value_type_ascription: unsafe {
                Relation::load(
                    5893459425189182978u64,
                    path.join("thir_exprs_value_type_ascription"),
                )
            }?,
            thir_exprs_closure: unsafe {
                Relation::load(11883660099806632522u64, path.join("thir_exprs_closure"))
            }?,
            thir_closure_upvars: unsafe {
                Relation::load(15546498482309699540u64, path.join("thir_closure_upvars"))
            }?,
            thir_exprs_literal: unsafe {
                Relation::load(17207401787417204123u64, path.join("thir_exprs_literal"))
            }?,
            thir_exprs_non_hir_literal: unsafe {
                Relation::load(
                    4124325780433447890u64,
                    path.join("thir_exprs_non_hir_literal"),
                )
            }?,
            thir_exprs_zst_literal: unsafe {
                Relation::load(11698888599584136937u64, path.join("thir_exprs_zst_literal"))
            }?,
            thir_exprs_named_const: unsafe {
                Relation::load(12988359671579403367u64, path.join("thir_exprs_named_const"))
            }?,
            thir_exprs_const_param: unsafe {
                Relation::load(4296964196940540570u64, path.join("thir_exprs_const_param"))
            }?,
            thir_exprs_static_ref: unsafe {
                Relation::load(11186498711369745685u64, path.join("thir_exprs_static_ref"))
            }?,
            thir_exprs_inline_asm: unsafe {
                Relation::load(17045657056275096603u64, path.join("thir_exprs_inline_asm"))
            }?,
            thir_exprs_offset_of: unsafe {
                Relation::load(13407437027651654372u64, path.join("thir_exprs_offset_of"))
            }?,
            thir_exprs_thread_local_ref: unsafe {
                Relation::load(
                    10937537484208518487u64,
                    path.join("thir_exprs_thread_local_ref"),
                )
            }?,
            thir_exprs_yield: unsafe {
                Relation::load(11224422256975424760u64, path.join("thir_exprs_yield"))
            }?,
            static_definitions: unsafe {
                Relation::load(15615015511242754149u64, path.join("static_definitions"))
            }?,
            impl_definitions: unsafe {
                Relation::load(7690441149453729173u64, path.join("impl_definitions"))
            }?,
            trait_impls: unsafe {
                Relation::load(14528000953543905781u64, path.join("trait_impls"))
            }?,
            global_asm_blocks: unsafe {
                Relation::load(10192848968817160673u64, path.join("global_asm_blocks"))
            }?,
            items: unsafe { Relation::load(1289592195786933916u64, path.join("items")) }?,
            mir_cfgs: unsafe { Relation::load(14299758314825397119u64, path.join("mir_cfgs")) }?,
            subscopes: unsafe { Relation::load(13606432809601680932u64, path.join("subscopes")) }?,
            spans: unsafe { Relation::load(14731319860987095787u64, path.join("spans")) }?,
            macro_expansions: unsafe {
                Relation::load(16522324850221623531u64, path.join("macro_expansions"))
            }?,
            crate_cfgs: unsafe {
                Relation::load(15473555631052060246u64, path.join("crate_cfgs"))
            }?,
            crate_authors: unsafe {
                Relation::load(10791542414620170368u64, path.join("crate_authors"))
            }?,
            crate_keywords: unsafe {
                Relation::load(5250907228800762424u64, path.join("crate_keywords"))
            }?,
            crate_categories: unsafe {
                Relation::load(95240473592181479u64, path.join("crate_categories"))
            }?,
            type_defs: unsafe { Relation::load(3193569606496579004u64, path.join("type_defs")) }?,
            types: unsafe { Relation::load(3975376537915609471u64, path.join("types")) }?,
            types_primitive: unsafe {
                Relation::load(2226877674460436540u64, path.join("types_primitive"))
            }?,
            types_adt_def: unsafe {
                Relation::load(8183742418156298214u64, path.join("types_adt_def"))
            }?,
            types_adt_variant: unsafe {
                Relation::load(2197893904118803340u64, path.join("types_adt_variant"))
            }?,
            types_adt_field: unsafe {
                Relation::load(7037499858270167988u64, path.join("types_adt_field"))
            }?,
            types_adt_field_visible_in: unsafe {
                Relation::load(
                    18004517893509822049u64,
                    path.join("types_adt_field_visible_in"),
                )
            }?,
            types_foreign: unsafe {
                Relation::load(4142907501147052699u64, path.join("types_foreign"))
            }?,
            types_array: unsafe {
                Relation::load(1131626646816840762u64, path.join("types_array"))
            }?,
            types_slice: unsafe {
                Relation::load(6446650263542674023u64, path.join("types_slice"))
            }?,
            types_raw_ptr: unsafe {
                Relation::load(5411092752360582208u64, path.join("types_raw_ptr"))
            }?,
            types_ref: unsafe { Relation::load(4446069966514798266u64, path.join("types_ref")) }?,
            types_fn_def: unsafe {
                Relation::load(1425750154069839048u64, path.join("types_fn_def"))
            }?,
            types_fn_ptr: unsafe {
                Relation::load(9445280361843553498u64, path.join("types_fn_ptr"))
            }?,
            types_dynamic: unsafe {
                Relation::load(10493468012836106481u64, path.join("types_dynamic"))
            }?,
            types_dynamic_trait: unsafe {
                Relation::load(15245729660191565254u64, path.join("types_dynamic_trait"))
            }?,
            types_closure: unsafe {
                Relation::load(18347839133875243449u64, path.join("types_closure"))
            }?,
            types_coroutine: unsafe {
                Relation::load(11592863599376068273u64, path.join("types_coroutine"))
            }?,
            types_coroutine_witness: unsafe {
                Relation::load(
                    17595719270309805751u64,
                    path.join("types_coroutine_witness"),
                )
            }?,
            types_coroutine_closure: unsafe {
                Relation::load(
                    10429624441391305088u64,
                    path.join("types_coroutine_closure"),
                )
            }?,
            types_pat: unsafe { Relation::load(15952850863293531464u64, path.join("types_pat")) }?,
            types_tuple: unsafe {
                Relation::load(11514292614567617999u64, path.join("types_tuple"))
            }?,
            types_tuple_element: unsafe {
                Relation::load(5818754280475673020u64, path.join("types_tuple_element"))
            }?,
            types_projection: unsafe {
                Relation::load(6998246431921405765u64, path.join("types_projection"))
            }?,
            types_opaque: unsafe {
                Relation::load(5423165845542059810u64, path.join("types_opaque"))
            }?,
            types_inherent: unsafe {
                Relation::load(6917214768906050830u64, path.join("types_inherent"))
            }?,
            types_weak: unsafe { Relation::load(886877941837007763u64, path.join("types_weak")) }?,
            types_param: unsafe {
                Relation::load(14436349763986252312u64, path.join("types_param"))
            }?,
            traits: unsafe { Relation::load(10046940701614007011u64, path.join("traits")) }?,
            trait_items: unsafe {
                Relation::load(17980637816433357124u64, path.join("trait_items"))
            }?,
            basic_blocks: unsafe {
                Relation::load(7184699705572388449u64, path.join("basic_blocks"))
            }?,
            statements: unsafe {
                Relation::load(17415903984110492204u64, path.join("statements"))
            }?,
            statements_assign_use: unsafe {
                Relation::load(2143429789654940834u64, path.join("statements_assign_use"))
            }?,
            statements_assign_thead_local_ref: unsafe {
                Relation::load(
                    16347330529269961148u64,
                    path.join("statements_assign_thead_local_ref"),
                )
            }?,
            statements_assign_repeat: unsafe {
                Relation::load(
                    8337461391872706546u64,
                    path.join("statements_assign_repeat"),
                )
            }?,
            statements_assign_ref: unsafe {
                Relation::load(2996089832552150050u64, path.join("statements_assign_ref"))
            }?,
            statements_assign_address: unsafe {
                Relation::load(
                    17409204682030536088u64,
                    path.join("statements_assign_address"),
                )
            }?,
            statements_assign_len: unsafe {
                Relation::load(16360895989675975031u64, path.join("statements_assign_len"))
            }?,
            statements_assign_cast: unsafe {
                Relation::load(10219327660195364344u64, path.join("statements_assign_cast"))
            }?,
            statements_assign_binary_op: unsafe {
                Relation::load(
                    2738073266338813909u64,
                    path.join("statements_assign_binary_op"),
                )
            }?,
            statements_assign_checked_binary_op: unsafe {
                Relation::load(
                    8137280794281117000u64,
                    path.join("statements_assign_checked_binary_op"),
                )
            }?,
            statements_assign_nullary_op: unsafe {
                Relation::load(
                    4054310599579763631u64,
                    path.join("statements_assign_nullary_op"),
                )
            }?,
            statements_assign_unary_op: unsafe {
                Relation::load(
                    2957248607918599194u64,
                    path.join("statements_assign_unary_op"),
                )
            }?,
            statements_assign_discriminant: unsafe {
                Relation::load(
                    11565188595208472216u64,
                    path.join("statements_assign_discriminant"),
                )
            }?,
            statements_assign_aggregate: unsafe {
                Relation::load(
                    14946105627318220629u64,
                    path.join("statements_assign_aggregate"),
                )
            }?,
            statements_assign_aggregate_operands: unsafe {
                Relation::load(
                    13820671585791218371u64,
                    path.join("statements_assign_aggregate_operands"),
                )
            }?,
            statements_assign_shallow_init_box: unsafe {
                Relation::load(
                    12139292931626599282u64,
                    path.join("statements_assign_shallow_init_box"),
                )
            }?,
            statements_assign_copy_for_deref: unsafe {
                Relation::load(
                    10060289130472003458u64,
                    path.join("statements_assign_copy_for_deref"),
                )
            }?,
            statements_inline_asm_inputs: unsafe {
                Relation::load(
                    2888522879853589191u64,
                    path.join("statements_inline_asm_inputs"),
                )
            }?,
            statements_inline_asm_outputs: unsafe {
                Relation::load(
                    11349904904689664070u64,
                    path.join("statements_inline_asm_outputs"),
                )
            }?,
            operands: unsafe { Relation::load(5820950675276320800u64, path.join("operands")) }?,
            terminators: unsafe {
                Relation::load(6839515169358340218u64, path.join("terminators"))
            }?,
            terminators_goto: unsafe {
                Relation::load(1991393721502951889u64, path.join("terminators_goto"))
            }?,
            terminators_switch_int: unsafe {
                Relation::load(15785734198669646692u64, path.join("terminators_switch_int"))
            }?,
            terminators_switch_int_targets: unsafe {
                Relation::load(
                    13713850366921519370u64,
                    path.join("terminators_switch_int_targets"),
                )
            }?,
            terminators_drop: unsafe {
                Relation::load(5304576143060721589u64, path.join("terminators_drop"))
            }?,
            terminators_drop_and_replace: unsafe {
                Relation::load(
                    14218106354975354834u64,
                    path.join("terminators_drop_and_replace"),
                )
            }?,
            terminators_call: unsafe {
                Relation::load(1605498164985976836u64, path.join("terminators_call"))
            }?,
            terminators_call_arg: unsafe {
                Relation::load(6170710286487383155u64, path.join("terminators_call_arg"))
            }?,
            terminators_call_const_target: unsafe {
                Relation::load(
                    5563931411730582203u64,
                    path.join("terminators_call_const_target"),
                )
            }?,
            terminators_call_const_target_desc: unsafe {
                Relation::load(
                    382182623806615358u64,
                    path.join("terminators_call_const_target_desc"),
                )
            }?,
            terminators_call_const_target_self: unsafe {
                Relation::load(
                    14690599567700872521u64,
                    path.join("terminators_call_const_target_self"),
                )
            }?,
            terminators_call_macro_backtrace: unsafe {
                Relation::load(
                    12477181522691837357u64,
                    path.join("terminators_call_macro_backtrace"),
                )
            }?,
            terminators_assert: unsafe {
                Relation::load(16835969248810489678u64, path.join("terminators_assert"))
            }?,
            terminators_yield: unsafe {
                Relation::load(8047909410674798880u64, path.join("terminators_yield"))
            }?,
            terminators_false_edges: unsafe {
                Relation::load(
                    12070114738280431417u64,
                    path.join("terminators_false_edges"),
                )
            }?,
            terminators_false_unwind: unsafe {
                Relation::load(
                    12046788150434641704u64,
                    path.join("terminators_false_unwind"),
                )
            }?,
            terminators_inline_asm: unsafe {
                Relation::load(9543573422760034863u64, path.join("terminators_inline_asm"))
            }?,
            terminators_unwind_action: unsafe {
                Relation::load(
                    265964774939904594u64,
                    path.join("terminators_unwind_action"),
                )
            }?,
        })
    }
    fn load_counters(path: &Path) -> Result<Counters> {
        crate::storage::load(&path)
    }
    fn load_interning_tables(path: &Path) -> Result<InterningTables> {
        Ok(InterningTables {
            strings: crate::storage::load(&path.join("strings.bincode"))?,
            package_names: unsafe {
                InterningTable::load(5753046719037099568u64, path.join("package_names"))?
            },
            package_versions: unsafe {
                InterningTable::load(7616596403864686833u64, path.join("package_versions"))?
            },
            crate_names: unsafe {
                InterningTable::load(16204848724287879413u64, path.join("crate_names"))?
            },
            editions: unsafe {
                InterningTable::load(11029148200302886001u64, path.join("editions"))?
            },
            names: unsafe { InterningTable::load(10407673464407571910u64, path.join("names"))? },
            relative_def_paths: unsafe {
                InterningTable::load(15140222954922639867u64, path.join("relative_def_paths"))?
            },
            summary_keys: unsafe {
                InterningTable::load(18304431051085712562u64, path.join("summary_keys"))?
            },
            abis: unsafe { InterningTable::load(7090802155184187506u64, path.join("abis"))? },
            def_paths: unsafe {
                InterningTable::load(4088999352158381537u64, path.join("def_paths"))?
            },
            builds: unsafe { InterningTable::load(16225872080495055883u64, path.join("builds"))? },
            span_file_names: unsafe {
                InterningTable::load(12030213377196136044u64, path.join("span_file_names"))?
            },
            crate_cfg_keys: unsafe {
                InterningTable::load(8740096367364431282u64, path.join("crate_cfg_keys"))?
            },
            crate_cfg_values: unsafe {
                InterningTable::load(12166464883024393290u64, path.join("crate_cfg_values"))?
            },
            type_kinds: unsafe {
                InterningTable::load(10162902544379113809u64, path.join("type_kinds"))?
            },
            statement_kinds: unsafe {
                InterningTable::load(6273093956909890961u64, path.join("statement_kinds"))?
            },
            binary_op_kind: unsafe {
                InterningTable::load(16753478943827638787u64, path.join("binary_op_kind"))?
            },
            nullary_op_kind: unsafe {
                InterningTable::load(9389093448705492600u64, path.join("nullary_op_kind"))?
            },
            unary_op_kind: unsafe {
                InterningTable::load(14289673856492345124u64, path.join("unary_op_kind"))?
            },
            terminator_kinds: unsafe {
                InterningTable::load(13371625579468820097u64, path.join("terminator_kinds"))?
            },
            thir_binary_op_kind: unsafe {
                InterningTable::load(16069664154929368055u64, path.join("thir_binary_op_kind"))?
            },
            thir_logical_op_kind: unsafe {
                InterningTable::load(3532929431539356339u64, path.join("thir_logical_op_kind"))?
            },
            thir_unary_op_kind: unsafe {
                InterningTable::load(9863820750922661495u64, path.join("thir_unary_op_kind"))?
            },
        })
    }
    fn store_multifile_relations(relations: &Relations, path: &Path) {
        unsafe {
            relations
                .def_path_span
                .save(8883063296299969981u64, path.join("def_path_span"))
        }
        unsafe {
            relations
                .type_description
                .save(14247184057110084413u64, path.join("type_description"))
        }
        unsafe {
            relations
                .build_crate_types
                .save(11694151696289681161u64, path.join("build_crate_types"))
        }
        unsafe {
            relations
                .root_modules
                .save(3608617112668148983u64, path.join("root_modules"))
        }
        unsafe {
            relations
                .submodules
                .save(14223402788489432104u64, path.join("submodules"))
        }
        unsafe {
            relations
                .function_definitions
                .save(6078223197665779672u64, path.join("function_definitions"))
        }
        unsafe {
            relations.function_parameter_types.save(
                17967330428057203654u64,
                path.join("function_parameter_types"),
            )
        }
        unsafe {
            relations
                .function_unsafe_use
                .save(3679592891414654765u64, path.join("function_unsafe_use"))
        }
        unsafe {
            relations
                .function_unsafe_reasons
                .save(7306117882207793123u64, path.join("function_unsafe_reasons"))
        }
        unsafe {
            relations
                .thir_bodies
                .save(9078912835344629248u64, path.join("thir_bodies"))
        }
        unsafe {
            relations
                .thir_blocks
                .save(5712874088580563410u64, path.join("thir_blocks"))
        }
        unsafe {
            relations
                .thir_stmts
                .save(11662528851258065727u64, path.join("thir_stmts"))
        }
        unsafe {
            relations
                .thir_stmts_expr
                .save(6974731257762681957u64, path.join("thir_stmts_expr"))
        }
        unsafe {
            relations
                .thir_stmts_let
                .save(3948572987018662193u64, path.join("thir_stmts_let"))
        }
        unsafe {
            relations
                .thir_block_expr
                .save(13932710207231639296u64, path.join("thir_block_expr"))
        }
        unsafe {
            relations
                .thir_exprs
                .save(14472862124235708861u64, path.join("thir_exprs"))
        }
        unsafe {
            relations
                .thir_exprs_scope
                .save(15126226146931209448u64, path.join("thir_exprs_scope"))
        }
        unsafe {
            relations
                .thir_exprs_box
                .save(17250276924894967218u64, path.join("thir_exprs_box"))
        }
        unsafe {
            relations
                .thir_exprs_if
                .save(1136822200270957559u64, path.join("thir_exprs_if"))
        }
        unsafe {
            relations
                .thir_exprs_call
                .save(10675202487661065417u64, path.join("thir_exprs_call"))
        }
        unsafe {
            relations
                .thir_exprs_call_arg
                .save(12688704234864686271u64, path.join("thir_exprs_call_arg"))
        }
        unsafe {
            relations.thir_exprs_call_const_target.save(
                282454153203586278u64,
                path.join("thir_exprs_call_const_target"),
            )
        }
        unsafe {
            relations.thir_exprs_call_const_target_desc.save(
                14557156226083258014u64,
                path.join("thir_exprs_call_const_target_desc"),
            )
        }
        unsafe {
            relations.thir_exprs_call_const_target_self.save(
                11355230934198225002u64,
                path.join("thir_exprs_call_const_target_self"),
            )
        }
        unsafe {
            relations
                .thir_exprs_deref
                .save(13435985341992119844u64, path.join("thir_exprs_deref"))
        }
        unsafe {
            relations
                .thir_exprs_binary
                .save(14413232755879446303u64, path.join("thir_exprs_binary"))
        }
        unsafe {
            relations
                .thir_exprs_logical_op
                .save(18234645225945847180u64, path.join("thir_exprs_logical_op"))
        }
        unsafe {
            relations
                .thir_exprs_unary
                .save(18219444126659999359u64, path.join("thir_exprs_unary"))
        }
        unsafe {
            relations
                .thir_exprs_cast
                .save(15582714740127479454u64, path.join("thir_exprs_cast"))
        }
        unsafe {
            relations
                .thir_exprs_use
                .save(18202594178015551134u64, path.join("thir_exprs_use"))
        }
        unsafe {
            relations.thir_exprs_never_to_any.save(
                17470269400781477793u64,
                path.join("thir_exprs_never_to_any"),
            )
        }
        unsafe {
            relations.thir_exprs_pointer_coercion.save(
                6101599652373463408u64,
                path.join("thir_exprs_pointer_coercion"),
            )
        }
        unsafe {
            relations
                .thir_exprs_loop
                .save(3098191845330208770u64, path.join("thir_exprs_loop"))
        }
        unsafe {
            relations
                .thir_exprs_let
                .save(9632315457169436561u64, path.join("thir_exprs_let"))
        }
        unsafe {
            relations
                .thir_pats
                .save(10536163954959029661u64, path.join("thir_pats"))
        }
        unsafe {
            relations
                .thir_exprs_match
                .save(5226457329152460944u64, path.join("thir_exprs_match"))
        }
        unsafe {
            relations
                .thir_match_arms
                .save(9822157818036781677u64, path.join("thir_match_arms"))
        }
        unsafe {
            relations
                .thir_exprs_block
                .save(15876734076667292329u64, path.join("thir_exprs_block"))
        }
        unsafe {
            relations
                .thir_exprs_assign
                .save(15488937819083875309u64, path.join("thir_exprs_assign"))
        }
        unsafe {
            relations
                .thir_exprs_assign_op
                .save(4838645763299739861u64, path.join("thir_exprs_assign_op"))
        }
        unsafe {
            relations
                .thir_exprs_field
                .save(23925721899557405u64, path.join("thir_exprs_field"))
        }
        unsafe {
            relations
                .thir_exprs_index
                .save(5832191315312159083u64, path.join("thir_exprs_index"))
        }
        unsafe {
            relations
                .thir_exprs_var_ref
                .save(296084846650957428u64, path.join("thir_exprs_var_ref"))
        }
        unsafe {
            relations
                .thir_exprs_upvar_ref
                .save(2787850572062464717u64, path.join("thir_exprs_upvar_ref"))
        }
        unsafe {
            relations
                .thir_exprs_borrow
                .save(6349143721208772268u64, path.join("thir_exprs_borrow"))
        }
        unsafe {
            relations
                .thir_exprs_raw_borrow
                .save(13059666409505041503u64, path.join("thir_exprs_raw_borrow"))
        }
        unsafe {
            relations
                .thir_exprs_break
                .save(6740441886473887693u64, path.join("thir_exprs_break"))
        }
        unsafe {
            relations
                .thir_exprs_continue
                .save(12830730509208436793u64, path.join("thir_exprs_continue"))
        }
        unsafe {
            relations
                .thir_exprs_return
                .save(17654622868380585299u64, path.join("thir_exprs_return"))
        }
        unsafe {
            relations
                .thir_exprs_become
                .save(16769260330021479451u64, path.join("thir_exprs_become"))
        }
        unsafe {
            relations
                .thir_exprs_const_block
                .save(12642227791060480364u64, path.join("thir_exprs_const_block"))
        }
        unsafe {
            relations
                .thir_exprs_repeat
                .save(2338369464665128106u64, path.join("thir_exprs_repeat"))
        }
        unsafe {
            relations
                .thir_exprs_array
                .save(1124144228664747403u64, path.join("thir_exprs_array"))
        }
        unsafe {
            relations
                .thir_array_elements
                .save(10250490187218554703u64, path.join("thir_array_elements"))
        }
        unsafe {
            relations
                .thir_exprs_tuple
                .save(15653329952619259353u64, path.join("thir_exprs_tuple"))
        }
        unsafe {
            relations
                .thir_tuple_elements
                .save(7005504983456194338u64, path.join("thir_tuple_elements"))
        }
        unsafe {
            relations
                .thir_exprs_adt
                .save(13642234560780341906u64, path.join("thir_exprs_adt"))
        }
        unsafe {
            relations
                .thir_adt_field_expr
                .save(14683105706445521333u64, path.join("thir_adt_field_expr"))
        }
        unsafe {
            relations.thir_exprs_place_type_ascription.save(
                10691137482463784180u64,
                path.join("thir_exprs_place_type_ascription"),
            )
        }
        unsafe {
            relations.thir_exprs_value_type_ascription.save(
                5893459425189182978u64,
                path.join("thir_exprs_value_type_ascription"),
            )
        }
        unsafe {
            relations
                .thir_exprs_closure
                .save(11883660099806632522u64, path.join("thir_exprs_closure"))
        }
        unsafe {
            relations
                .thir_closure_upvars
                .save(15546498482309699540u64, path.join("thir_closure_upvars"))
        }
        unsafe {
            relations
                .thir_exprs_literal
                .save(17207401787417204123u64, path.join("thir_exprs_literal"))
        }
        unsafe {
            relations.thir_exprs_non_hir_literal.save(
                4124325780433447890u64,
                path.join("thir_exprs_non_hir_literal"),
            )
        }
        unsafe {
            relations
                .thir_exprs_zst_literal
                .save(11698888599584136937u64, path.join("thir_exprs_zst_literal"))
        }
        unsafe {
            relations
                .thir_exprs_named_const
                .save(12988359671579403367u64, path.join("thir_exprs_named_const"))
        }
        unsafe {
            relations
                .thir_exprs_const_param
                .save(4296964196940540570u64, path.join("thir_exprs_const_param"))
        }
        unsafe {
            relations
                .thir_exprs_static_ref
                .save(11186498711369745685u64, path.join("thir_exprs_static_ref"))
        }
        unsafe {
            relations
                .thir_exprs_inline_asm
                .save(17045657056275096603u64, path.join("thir_exprs_inline_asm"))
        }
        unsafe {
            relations
                .thir_exprs_offset_of
                .save(13407437027651654372u64, path.join("thir_exprs_offset_of"))
        }
        unsafe {
            relations.thir_exprs_thread_local_ref.save(
                10937537484208518487u64,
                path.join("thir_exprs_thread_local_ref"),
            )
        }
        unsafe {
            relations
                .thir_exprs_yield
                .save(11224422256975424760u64, path.join("thir_exprs_yield"))
        }
        unsafe {
            relations
                .static_definitions
                .save(15615015511242754149u64, path.join("static_definitions"))
        }
        unsafe {
            relations
                .impl_definitions
                .save(7690441149453729173u64, path.join("impl_definitions"))
        }
        unsafe {
            relations
                .trait_impls
                .save(14528000953543905781u64, path.join("trait_impls"))
        }
        unsafe {
            relations
                .global_asm_blocks
                .save(10192848968817160673u64, path.join("global_asm_blocks"))
        }
        unsafe {
            relations
                .items
                .save(1289592195786933916u64, path.join("items"))
        }
        unsafe {
            relations
                .mir_cfgs
                .save(14299758314825397119u64, path.join("mir_cfgs"))
        }
        unsafe {
            relations
                .subscopes
                .save(13606432809601680932u64, path.join("subscopes"))
        }
        unsafe {
            relations
                .spans
                .save(14731319860987095787u64, path.join("spans"))
        }
        unsafe {
            relations
                .macro_expansions
                .save(16522324850221623531u64, path.join("macro_expansions"))
        }
        unsafe {
            relations
                .crate_cfgs
                .save(15473555631052060246u64, path.join("crate_cfgs"))
        }
        unsafe {
            relations
                .crate_authors
                .save(10791542414620170368u64, path.join("crate_authors"))
        }
        unsafe {
            relations
                .crate_keywords
                .save(5250907228800762424u64, path.join("crate_keywords"))
        }
        unsafe {
            relations
                .crate_categories
                .save(95240473592181479u64, path.join("crate_categories"))
        }
        unsafe {
            relations
                .type_defs
                .save(3193569606496579004u64, path.join("type_defs"))
        }
        unsafe {
            relations
                .types
                .save(3975376537915609471u64, path.join("types"))
        }
        unsafe {
            relations
                .types_primitive
                .save(2226877674460436540u64, path.join("types_primitive"))
        }
        unsafe {
            relations
                .types_adt_def
                .save(8183742418156298214u64, path.join("types_adt_def"))
        }
        unsafe {
            relations
                .types_adt_variant
                .save(2197893904118803340u64, path.join("types_adt_variant"))
        }
        unsafe {
            relations
                .types_adt_field
                .save(7037499858270167988u64, path.join("types_adt_field"))
        }
        unsafe {
            relations.types_adt_field_visible_in.save(
                18004517893509822049u64,
                path.join("types_adt_field_visible_in"),
            )
        }
        unsafe {
            relations
                .types_foreign
                .save(4142907501147052699u64, path.join("types_foreign"))
        }
        unsafe {
            relations
                .types_array
                .save(1131626646816840762u64, path.join("types_array"))
        }
        unsafe {
            relations
                .types_slice
                .save(6446650263542674023u64, path.join("types_slice"))
        }
        unsafe {
            relations
                .types_raw_ptr
                .save(5411092752360582208u64, path.join("types_raw_ptr"))
        }
        unsafe {
            relations
                .types_ref
                .save(4446069966514798266u64, path.join("types_ref"))
        }
        unsafe {
            relations
                .types_fn_def
                .save(1425750154069839048u64, path.join("types_fn_def"))
        }
        unsafe {
            relations
                .types_fn_ptr
                .save(9445280361843553498u64, path.join("types_fn_ptr"))
        }
        unsafe {
            relations
                .types_dynamic
                .save(10493468012836106481u64, path.join("types_dynamic"))
        }
        unsafe {
            relations
                .types_dynamic_trait
                .save(15245729660191565254u64, path.join("types_dynamic_trait"))
        }
        unsafe {
            relations
                .types_closure
                .save(18347839133875243449u64, path.join("types_closure"))
        }
        unsafe {
            relations
                .types_coroutine
                .save(11592863599376068273u64, path.join("types_coroutine"))
        }
        unsafe {
            relations.types_coroutine_witness.save(
                17595719270309805751u64,
                path.join("types_coroutine_witness"),
            )
        }
        unsafe {
            relations.types_coroutine_closure.save(
                10429624441391305088u64,
                path.join("types_coroutine_closure"),
            )
        }
        unsafe {
            relations
                .types_pat
                .save(15952850863293531464u64, path.join("types_pat"))
        }
        unsafe {
            relations
                .types_tuple
                .save(11514292614567617999u64, path.join("types_tuple"))
        }
        unsafe {
            relations
                .types_tuple_element
                .save(5818754280475673020u64, path.join("types_tuple_element"))
        }
        unsafe {
            relations
                .types_projection
                .save(6998246431921405765u64, path.join("types_projection"))
        }
        unsafe {
            relations
                .types_opaque
                .save(5423165845542059810u64, path.join("types_opaque"))
        }
        unsafe {
            relations
                .types_inherent
                .save(6917214768906050830u64, path.join("types_inherent"))
        }
        unsafe {
            relations
                .types_weak
                .save(886877941837007763u64, path.join("types_weak"))
        }
        unsafe {
            relations
                .types_param
                .save(14436349763986252312u64, path.join("types_param"))
        }
        unsafe {
            relations
                .traits
                .save(10046940701614007011u64, path.join("traits"))
        }
        unsafe {
            relations
                .trait_items
                .save(17980637816433357124u64, path.join("trait_items"))
        }
        unsafe {
            relations
                .basic_blocks
                .save(7184699705572388449u64, path.join("basic_blocks"))
        }
        unsafe {
            relations
                .statements
                .save(17415903984110492204u64, path.join("statements"))
        }
        unsafe {
            relations
                .statements_assign_use
                .save(2143429789654940834u64, path.join("statements_assign_use"))
        }
        unsafe {
            relations.statements_assign_thead_local_ref.save(
                16347330529269961148u64,
                path.join("statements_assign_thead_local_ref"),
            )
        }
        unsafe {
            relations.statements_assign_repeat.save(
                8337461391872706546u64,
                path.join("statements_assign_repeat"),
            )
        }
        unsafe {
            relations
                .statements_assign_ref
                .save(2996089832552150050u64, path.join("statements_assign_ref"))
        }
        unsafe {
            relations.statements_assign_address.save(
                17409204682030536088u64,
                path.join("statements_assign_address"),
            )
        }
        unsafe {
            relations
                .statements_assign_len
                .save(16360895989675975031u64, path.join("statements_assign_len"))
        }
        unsafe {
            relations
                .statements_assign_cast
                .save(10219327660195364344u64, path.join("statements_assign_cast"))
        }
        unsafe {
            relations.statements_assign_binary_op.save(
                2738073266338813909u64,
                path.join("statements_assign_binary_op"),
            )
        }
        unsafe {
            relations.statements_assign_checked_binary_op.save(
                8137280794281117000u64,
                path.join("statements_assign_checked_binary_op"),
            )
        }
        unsafe {
            relations.statements_assign_nullary_op.save(
                4054310599579763631u64,
                path.join("statements_assign_nullary_op"),
            )
        }
        unsafe {
            relations.statements_assign_unary_op.save(
                2957248607918599194u64,
                path.join("statements_assign_unary_op"),
            )
        }
        unsafe {
            relations.statements_assign_discriminant.save(
                11565188595208472216u64,
                path.join("statements_assign_discriminant"),
            )
        }
        unsafe {
            relations.statements_assign_aggregate.save(
                14946105627318220629u64,
                path.join("statements_assign_aggregate"),
            )
        }
        unsafe {
            relations.statements_assign_aggregate_operands.save(
                13820671585791218371u64,
                path.join("statements_assign_aggregate_operands"),
            )
        }
        unsafe {
            relations.statements_assign_shallow_init_box.save(
                12139292931626599282u64,
                path.join("statements_assign_shallow_init_box"),
            )
        }
        unsafe {
            relations.statements_assign_copy_for_deref.save(
                10060289130472003458u64,
                path.join("statements_assign_copy_for_deref"),
            )
        }
        unsafe {
            relations.statements_inline_asm_inputs.save(
                2888522879853589191u64,
                path.join("statements_inline_asm_inputs"),
            )
        }
        unsafe {
            relations.statements_inline_asm_outputs.save(
                11349904904689664070u64,
                path.join("statements_inline_asm_outputs"),
            )
        }
        unsafe {
            relations
                .operands
                .save(5820950675276320800u64, path.join("operands"))
        }
        unsafe {
            relations
                .terminators
                .save(6839515169358340218u64, path.join("terminators"))
        }
        unsafe {
            relations
                .terminators_goto
                .save(1991393721502951889u64, path.join("terminators_goto"))
        }
        unsafe {
            relations
                .terminators_switch_int
                .save(15785734198669646692u64, path.join("terminators_switch_int"))
        }
        unsafe {
            relations.terminators_switch_int_targets.save(
                13713850366921519370u64,
                path.join("terminators_switch_int_targets"),
            )
        }
        unsafe {
            relations
                .terminators_drop
                .save(5304576143060721589u64, path.join("terminators_drop"))
        }
        unsafe {
            relations.terminators_drop_and_replace.save(
                14218106354975354834u64,
                path.join("terminators_drop_and_replace"),
            )
        }
        unsafe {
            relations
                .terminators_call
                .save(1605498164985976836u64, path.join("terminators_call"))
        }
        unsafe {
            relations
                .terminators_call_arg
                .save(6170710286487383155u64, path.join("terminators_call_arg"))
        }
        unsafe {
            relations.terminators_call_const_target.save(
                5563931411730582203u64,
                path.join("terminators_call_const_target"),
            )
        }
        unsafe {
            relations.terminators_call_const_target_desc.save(
                382182623806615358u64,
                path.join("terminators_call_const_target_desc"),
            )
        }
        unsafe {
            relations.terminators_call_const_target_self.save(
                14690599567700872521u64,
                path.join("terminators_call_const_target_self"),
            )
        }
        unsafe {
            relations.terminators_call_macro_backtrace.save(
                12477181522691837357u64,
                path.join("terminators_call_macro_backtrace"),
            )
        }
        unsafe {
            relations
                .terminators_assert
                .save(16835969248810489678u64, path.join("terminators_assert"))
        }
        unsafe {
            relations
                .terminators_yield
                .save(8047909410674798880u64, path.join("terminators_yield"))
        }
        unsafe {
            relations.terminators_false_edges.save(
                12070114738280431417u64,
                path.join("terminators_false_edges"),
            )
        }
        unsafe {
            relations.terminators_false_unwind.save(
                12046788150434641704u64,
                path.join("terminators_false_unwind"),
            )
        }
        unsafe {
            relations
                .terminators_inline_asm
                .save(9543573422760034863u64, path.join("terminators_inline_asm"))
        }
        unsafe {
            relations.terminators_unwind_action.save(
                265964774939904594u64,
                path.join("terminators_unwind_action"),
            )
        }
    }
    fn store_counters(counters: &Counters, path: &Path) {
        crate::storage::save(counters, &path);
    }
    fn store_multifile_interning_tables(interning_tables: &InterningTables, path: &Path) {
        crate::storage::save(&interning_tables.strings, &path.join("strings.bincode"));
        unsafe {
            interning_tables
                .package_names
                .save(5753046719037099568u64, path.join("package_names"));
        }
        unsafe {
            interning_tables
                .package_versions
                .save(7616596403864686833u64, path.join("package_versions"));
        }
        unsafe {
            interning_tables
                .crate_names
                .save(16204848724287879413u64, path.join("crate_names"));
        }
        unsafe {
            interning_tables
                .editions
                .save(11029148200302886001u64, path.join("editions"));
        }
        unsafe {
            interning_tables
                .names
                .save(10407673464407571910u64, path.join("names"));
        }
        unsafe {
            interning_tables
                .relative_def_paths
                .save(15140222954922639867u64, path.join("relative_def_paths"));
        }
        unsafe {
            interning_tables
                .summary_keys
                .save(18304431051085712562u64, path.join("summary_keys"));
        }
        unsafe {
            interning_tables
                .abis
                .save(7090802155184187506u64, path.join("abis"));
        }
        unsafe {
            interning_tables
                .def_paths
                .save(4088999352158381537u64, path.join("def_paths"));
        }
        unsafe {
            interning_tables
                .builds
                .save(16225872080495055883u64, path.join("builds"));
        }
        unsafe {
            interning_tables
                .span_file_names
                .save(12030213377196136044u64, path.join("span_file_names"));
        }
        unsafe {
            interning_tables
                .crate_cfg_keys
                .save(8740096367364431282u64, path.join("crate_cfg_keys"));
        }
        unsafe {
            interning_tables
                .crate_cfg_values
                .save(12166464883024393290u64, path.join("crate_cfg_values"));
        }
        unsafe {
            interning_tables
                .type_kinds
                .save(10162902544379113809u64, path.join("type_kinds"));
        }
        unsafe {
            interning_tables
                .statement_kinds
                .save(6273093956909890961u64, path.join("statement_kinds"));
        }
        unsafe {
            interning_tables
                .binary_op_kind
                .save(16753478943827638787u64, path.join("binary_op_kind"));
        }
        unsafe {
            interning_tables
                .nullary_op_kind
                .save(9389093448705492600u64, path.join("nullary_op_kind"));
        }
        unsafe {
            interning_tables
                .unary_op_kind
                .save(14289673856492345124u64, path.join("unary_op_kind"));
        }
        unsafe {
            interning_tables
                .terminator_kinds
                .save(13371625579468820097u64, path.join("terminator_kinds"));
        }
        unsafe {
            interning_tables
                .thir_binary_op_kind
                .save(16069664154929368055u64, path.join("thir_binary_op_kind"));
        }
        unsafe {
            interning_tables
                .thir_logical_op_kind
                .save(3532929431539356339u64, path.join("thir_logical_op_kind"));
        }
        unsafe {
            interning_tables
                .thir_unary_op_kind
                .save(9863820750922661495u64, path.join("thir_unary_op_kind"));
        }
    }
    #[derive(Default)]
    pub struct Loader {
        pub(crate) database_root: PathBuf,
        def_path_span: std::cell::RefCell<Option<Vec<(DefPath, Span)>>>,
        type_description: std::cell::RefCell<Option<Vec<(Type, InternedString, InternedString)>>>,
        build_crate_types: std::cell::RefCell<Option<Vec<(Build, InternedString)>>>,
        root_modules: std::cell::RefCell<Option<Vec<(Build, Module)>>>,
        submodules:
            std::cell::RefCell<Option<Vec<(DefPath, Module, Module, Name, TyVisibility, Abi)>>>,
        function_definitions: std::cell::RefCell<
            Option<Vec<(Item, DefPath, Module, TyVisibility, Safety, Abi, Type)>>,
        >,
        function_parameter_types: std::cell::RefCell<Option<Vec<(Item, FnParamIndex, Type)>>>,
        function_unsafe_use: std::cell::RefCell<Option<Vec<(DefPath, bool)>>>,
        function_unsafe_reasons: std::cell::RefCell<Option<Vec<(DefPath, u32, InternedString)>>>,
        thir_bodies: std::cell::RefCell<Option<Vec<(Item, DefPath, ThirBlock)>>>,
        thir_blocks: std::cell::RefCell<
            Option<Vec<(ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span)>>,
        >,
        thir_stmts:
            std::cell::RefCell<Option<Vec<(ThirStmt, ThirBlock, ThirBlock, StatementIndex)>>>,
        thir_stmts_expr: std::cell::RefCell<Option<Vec<(ThirStmt, ThirExpr)>>>,
        thir_stmts_let: std::cell::RefCell<Option<Vec<(ThirStmt, ThirExpr, ThirBlock, Span)>>>,
        thir_block_expr: std::cell::RefCell<Option<Vec<(ThirBlock, ThirExpr)>>>,
        thir_exprs: std::cell::RefCell<Option<Vec<(ThirExpr, ThirBlock, ThirBlock, Type, Span)>>>,
        thir_exprs_scope: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr)>>>,
        thir_exprs_box: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr)>>>,
        thir_exprs_if: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr, ThirExpr, ThirExpr)>>>,
        thir_exprs_call:
            std::cell::RefCell<Option<Vec<(ThirExpr, Type, ThirExpr, Safety, Abi, Type)>>>,
        thir_exprs_call_arg:
            std::cell::RefCell<Option<Vec<(ThirExpr, ThirCallArgIndex, ThirExpr)>>>,
        thir_exprs_call_const_target: std::cell::RefCell<Option<Vec<(ThirExpr, DefPath)>>>,
        thir_exprs_call_const_target_desc: std::cell::RefCell<
            Option<Vec<(ThirExpr, InternedString, InternedString, InternedString)>>,
        >,
        thir_exprs_call_const_target_self: std::cell::RefCell<Option<Vec<(ThirExpr, Type)>>>,
        thir_exprs_deref: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr)>>>,
        thir_exprs_binary:
            std::cell::RefCell<Option<Vec<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>>>,
        thir_exprs_logical_op:
            std::cell::RefCell<Option<Vec<(ThirExpr, ThirLogicalOp, ThirExpr, ThirExpr)>>>,
        thir_exprs_unary: std::cell::RefCell<Option<Vec<(ThirExpr, ThirUnOp, ThirExpr)>>>,
        thir_exprs_cast: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr)>>>,
        thir_exprs_use: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr)>>>,
        thir_exprs_never_to_any: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr)>>>,
        thir_exprs_pointer_coercion:
            std::cell::RefCell<Option<Vec<(ThirExpr, PointerCoercion, ThirExpr, bool)>>>,
        thir_exprs_loop: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr)>>>,
        thir_exprs_let: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr, ThirPat)>>>,
        thir_pats: std::cell::RefCell<Option<Vec<(ThirPat, Type, Span)>>>,
        thir_exprs_match: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr, MatchSource)>>>,
        thir_match_arms:
            std::cell::RefCell<Option<Vec<(ThirExpr, MatchArmIdx, ThirExpr, ThirExpr)>>>,
        thir_exprs_block: std::cell::RefCell<Option<Vec<(ThirExpr, ThirBlock)>>>,
        thir_exprs_assign: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr, ThirExpr)>>>,
        thir_exprs_assign_op:
            std::cell::RefCell<Option<Vec<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>>>,
        thir_exprs_field: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr, AdtVariantIndex)>>>,
        thir_exprs_index: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr, ThirExpr)>>>,
        thir_exprs_var_ref: std::cell::RefCell<Option<Vec<(ThirExpr,)>>>,
        thir_exprs_upvar_ref: std::cell::RefCell<Option<Vec<(ThirExpr, DefPath)>>>,
        thir_exprs_borrow: std::cell::RefCell<Option<Vec<(ThirExpr, BorrowKind, ThirExpr)>>>,
        thir_exprs_raw_borrow: std::cell::RefCell<Option<Vec<(ThirExpr, Mutability, ThirExpr)>>>,
        thir_exprs_break: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr)>>>,
        thir_exprs_continue: std::cell::RefCell<Option<Vec<(ThirExpr,)>>>,
        thir_exprs_return: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr)>>>,
        thir_exprs_become: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr)>>>,
        thir_exprs_const_block: std::cell::RefCell<Option<Vec<(ThirExpr, DefPath)>>>,
        thir_exprs_repeat: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr)>>>,
        thir_exprs_array: std::cell::RefCell<Option<Vec<(ThirExpr,)>>>,
        thir_array_elements: std::cell::RefCell<Option<Vec<(ThirExpr, u64, ThirExpr)>>>,
        thir_exprs_tuple: std::cell::RefCell<Option<Vec<(ThirExpr,)>>>,
        thir_tuple_elements: std::cell::RefCell<Option<Vec<(ThirExpr, TupleFieldIndex, ThirExpr)>>>,
        thir_exprs_adt: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr, AdtVariantIndex)>>>,
        thir_adt_field_expr: std::cell::RefCell<Option<Vec<(ThirExpr, FieldIndex, ThirExpr)>>>,
        thir_exprs_place_type_ascription:
            std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr, Span)>>>,
        thir_exprs_value_type_ascription:
            std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr, Span)>>>,
        thir_exprs_closure: std::cell::RefCell<Option<Vec<(ThirExpr, DefPath, Movability)>>>,
        thir_closure_upvars: std::cell::RefCell<Option<Vec<(ThirExpr, u32, ThirExpr)>>>,
        thir_exprs_literal: std::cell::RefCell<Option<Vec<(ThirExpr, LitKind, bool)>>>,
        thir_exprs_non_hir_literal: std::cell::RefCell<Option<Vec<(ThirExpr, u128)>>>,
        thir_exprs_zst_literal: std::cell::RefCell<Option<Vec<(ThirExpr,)>>>,
        thir_exprs_named_const: std::cell::RefCell<Option<Vec<(ThirExpr, DefPath)>>>,
        thir_exprs_const_param: std::cell::RefCell<Option<Vec<(ThirExpr, DefPath)>>>,
        thir_exprs_static_ref: std::cell::RefCell<Option<Vec<(ThirExpr, Type, DefPath)>>>,
        thir_exprs_inline_asm: std::cell::RefCell<Option<Vec<(ThirExpr,)>>>,
        thir_exprs_offset_of: std::cell::RefCell<Option<Vec<(ThirExpr, Type)>>>,
        thir_exprs_thread_local_ref: std::cell::RefCell<Option<Vec<(ThirExpr, DefPath)>>>,
        thir_exprs_yield: std::cell::RefCell<Option<Vec<(ThirExpr, ThirExpr)>>>,
        static_definitions: std::cell::RefCell<
            Option<Vec<(DefPath, Item, Module, Name, TyVisibility, Mutability)>>,
        >,
        impl_definitions: std::cell::RefCell<
            Option<
                Vec<(
                    DefPath,
                    Item,
                    Module,
                    Name,
                    TyVisibility,
                    Safety,
                    ImplPolarity,
                    Defaultness,
                    Constness,
                    Type,
                )>,
            >,
        >,
        trait_impls: std::cell::RefCell<Option<Vec<(Item, Type, DefPath)>>>,
        global_asm_blocks:
            std::cell::RefCell<Option<Vec<(DefPath, Item, Module, Name, TyVisibility)>>>,
        items: std::cell::RefCell<Option<Vec<(DefPath, Item, Module, Name, TyVisibility)>>>,
        mir_cfgs: std::cell::RefCell<Option<Vec<(Item, DefPath, Scope)>>>,
        subscopes:
            std::cell::RefCell<Option<Vec<(Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span)>>>,
        spans: std::cell::RefCell<
            Option<
                Vec<(
                    Span,
                    Span,
                    SpanExpansionKind,
                    InternedString,
                    SpanFileName,
                    u16,
                    u16,
                )>,
            >,
        >,
        macro_expansions:
            std::cell::RefCell<Option<Vec<(Span, InternedString, SpanFileName, u16, u16)>>>,
        crate_cfgs: std::cell::RefCell<Option<Vec<(Build, CrateCfgKey, CrateCfgValue)>>>,
        crate_authors: std::cell::RefCell<Option<Vec<(Build, InternedString)>>>,
        crate_keywords: std::cell::RefCell<Option<Vec<(Build, InternedString)>>>,
        crate_categories: std::cell::RefCell<Option<Vec<(Build, InternedString)>>>,
        type_defs: std::cell::RefCell<
            Option<Vec<(Item, Type, DefPath, InternedString, TyVisibility, TyDefKind)>>,
        >,
        types: std::cell::RefCell<Option<Vec<(Type, TyKind)>>>,
        types_primitive: std::cell::RefCell<Option<Vec<(Type, TyPrimitive)>>>,
        types_adt_def: std::cell::RefCell<Option<Vec<(Type, DefPath, AdtKind, bool, bool)>>>,
        types_adt_variant:
            std::cell::RefCell<Option<Vec<(Type, AdtVariantIndex, DefPath, InternedString)>>>,
        types_adt_field: std::cell::RefCell<
            Option<
                Vec<(
                    Field,
                    Type,
                    AdtVariantIndex,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    Type,
                )>,
            >,
        >,
        types_adt_field_visible_in: std::cell::RefCell<Option<Vec<(Field, DefPath)>>>,
        types_foreign: std::cell::RefCell<Option<Vec<(Type, DefPath)>>>,
        types_array: std::cell::RefCell<Option<Vec<(Type, Type)>>>,
        types_slice: std::cell::RefCell<Option<Vec<(Type, Type)>>>,
        types_raw_ptr: std::cell::RefCell<Option<Vec<(Type, Type, Mutability)>>>,
        types_ref: std::cell::RefCell<Option<Vec<(Type, Type, Mutability)>>>,
        types_fn_def: std::cell::RefCell<Option<Vec<(Type, DefPath)>>>,
        types_fn_ptr: std::cell::RefCell<Option<Vec<(Type,)>>>,
        types_dynamic: std::cell::RefCell<Option<Vec<(Type,)>>>,
        types_dynamic_trait: std::cell::RefCell<Option<Vec<(Type, DefPath, bool)>>>,
        types_closure: std::cell::RefCell<Option<Vec<(Type, DefPath)>>>,
        types_coroutine: std::cell::RefCell<Option<Vec<(Type, DefPath)>>>,
        types_coroutine_witness: std::cell::RefCell<Option<Vec<(Type,)>>>,
        types_coroutine_closure: std::cell::RefCell<Option<Vec<(Type, DefPath)>>>,
        types_pat: std::cell::RefCell<Option<Vec<(Type,)>>>,
        types_tuple: std::cell::RefCell<Option<Vec<(Type,)>>>,
        types_tuple_element: std::cell::RefCell<Option<Vec<(Type, TupleFieldIndex, Type)>>>,
        types_projection: std::cell::RefCell<Option<Vec<(Type, DefPath, DefPath)>>>,
        types_opaque: std::cell::RefCell<Option<Vec<(Type, DefPath)>>>,
        types_inherent: std::cell::RefCell<Option<Vec<(Type, DefPath)>>>,
        types_weak: std::cell::RefCell<Option<Vec<(Type, DefPath)>>>,
        types_param: std::cell::RefCell<Option<Vec<(Type, u32, InternedString)>>>,
        traits: std::cell::RefCell<
            Option<
                Vec<(
                    Item,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    bool,
                    bool,
                    Safety,
                )>,
            >,
        >,
        trait_items: std::cell::RefCell<Option<Vec<(Item, DefPath, Defaultness)>>>,
        basic_blocks: std::cell::RefCell<Option<Vec<(BasicBlock, DefPath, BasicBlockKind)>>>,
        statements: std::cell::RefCell<
            Option<Vec<(Statement, BasicBlock, StatementIndex, StatementKind, Scope)>>,
        >,
        statements_assign_use: std::cell::RefCell<Option<Vec<(Statement, Type, Operand)>>>,
        statements_assign_thead_local_ref:
            std::cell::RefCell<Option<Vec<(Statement, Type, DefPath)>>>,
        statements_assign_repeat: std::cell::RefCell<Option<Vec<(Statement, Type, Operand, u64)>>>,
        statements_assign_ref: std::cell::RefCell<Option<Vec<(Statement, Type, Type, BorrowKind)>>>,
        statements_assign_address:
            std::cell::RefCell<Option<Vec<(Statement, Type, Type, Mutability)>>>,
        statements_assign_len: std::cell::RefCell<Option<Vec<(Statement, Type, Type)>>>,
        statements_assign_cast:
            std::cell::RefCell<Option<Vec<(Statement, Type, CastKind, Operand, Type)>>>,
        statements_assign_binary_op:
            std::cell::RefCell<Option<Vec<(Statement, Type, BinOp, Operand, Operand)>>>,
        statements_assign_checked_binary_op:
            std::cell::RefCell<Option<Vec<(Statement, Type, BinOp, Operand, Operand)>>>,
        statements_assign_nullary_op:
            std::cell::RefCell<Option<Vec<(Statement, Type, NullOp, Type)>>>,
        statements_assign_unary_op:
            std::cell::RefCell<Option<Vec<(Statement, Type, UnOp, Operand)>>>,
        statements_assign_discriminant: std::cell::RefCell<Option<Vec<(Statement, Type, Type)>>>,
        statements_assign_aggregate:
            std::cell::RefCell<Option<Vec<(Statement, Type, AggregateKind)>>>,
        statements_assign_aggregate_operands:
            std::cell::RefCell<Option<Vec<(Statement, OperandIndex, Operand)>>>,
        statements_assign_shallow_init_box:
            std::cell::RefCell<Option<Vec<(Statement, Operand, Type)>>>,
        statements_assign_copy_for_deref: std::cell::RefCell<Option<Vec<(Statement, Type)>>>,
        statements_inline_asm_inputs: std::cell::RefCell<Option<Vec<(Statement, Operand)>>>,
        statements_inline_asm_outputs: std::cell::RefCell<Option<Vec<(Statement, Type)>>>,
        operands: std::cell::RefCell<Option<Vec<(Operand, OperandKind, Type)>>>,
        terminators: std::cell::RefCell<Option<Vec<(BasicBlock, TerminatorKind, Scope)>>>,
        terminators_goto: std::cell::RefCell<Option<Vec<(BasicBlock, BasicBlock)>>>,
        terminators_switch_int: std::cell::RefCell<Option<Vec<(BasicBlock, Operand)>>>,
        terminators_switch_int_targets:
            std::cell::RefCell<Option<Vec<(BasicBlock, u128, BasicBlock)>>>,
        terminators_drop: std::cell::RefCell<Option<Vec<(BasicBlock, Type, BasicBlock)>>>,
        terminators_drop_and_replace:
            std::cell::RefCell<Option<Vec<(BasicBlock, Type, Operand, BasicBlock, BasicBlock)>>>,
        terminators_call: std::cell::RefCell<
            Option<
                Vec<(
                    BasicBlock,
                    FunctionCall,
                    Operand,
                    Safety,
                    Abi,
                    Type,
                    BasicBlock,
                    Span,
                )>,
            >,
        >,
        terminators_call_arg:
            std::cell::RefCell<Option<Vec<(FunctionCall, CallArgIndex, Operand)>>>,
        terminators_call_const_target: std::cell::RefCell<Option<Vec<(FunctionCall, DefPath)>>>,
        terminators_call_const_target_desc: std::cell::RefCell<
            Option<Vec<(FunctionCall, InternedString, InternedString, InternedString)>>,
        >,
        terminators_call_const_target_self: std::cell::RefCell<Option<Vec<(FunctionCall, Type)>>>,
        terminators_call_macro_backtrace:
            std::cell::RefCell<Option<Vec<(FunctionCall, InternedString)>>>,
        terminators_assert:
            std::cell::RefCell<Option<Vec<(BasicBlock, Operand, bool, BasicBlock)>>>,
        terminators_yield:
            std::cell::RefCell<Option<Vec<(BasicBlock, Operand, BasicBlock, BasicBlock)>>>,
        terminators_false_edges:
            std::cell::RefCell<Option<Vec<(BasicBlock, BasicBlock, BasicBlock)>>>,
        terminators_false_unwind: std::cell::RefCell<Option<Vec<(BasicBlock, BasicBlock)>>>,
        terminators_inline_asm: std::cell::RefCell<Option<Vec<(BasicBlock,)>>>,
        terminators_unwind_action:
            std::cell::RefCell<Option<Vec<(BasicBlock, UnwindAction, BasicBlock)>>>,
        selected_builds: std::cell::RefCell<
            Option<Vec<(Build, Package, PackageVersion, Krate, CrateHash, Edition)>>,
        >,
        build_script_builds: std::cell::RefCell<
            Option<Vec<(Build, Package, PackageVersion, Krate, CrateHash, Edition)>>,
        >,
        selected_modules: std::cell::RefCell<Option<Vec<(Build, Module)>>>,
        selected_mir_cfgs: std::cell::RefCell<Option<Vec<(Build, Item, DefPath, Scope)>>>,
        selected_scopes: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    DefPath,
                    Scope,
                    Scope,
                    ScopeSafety,
                    u32,
                    BlockCheckMode,
                    Span,
                )>,
            >,
        >,
        selected_type_defs: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    Item,
                    Type,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                )>,
            >,
        >,
        selected_adts: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    Item,
                    Type,
                    DefPath,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                    AdtKind,
                    bool,
                    bool,
                )>,
            >,
        >,
        selected_adt_field_types: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    Item,
                    Type,
                    AdtVariantIndex,
                    DefPath,
                    DefPath,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                    AdtKind,
                    bool,
                    bool,
                    InternedString,
                    TyVisibility,
                    Type,
                    TyKind,
                )>,
            >,
        >,
        types_unsafe_cell: std::cell::RefCell<Option<Vec<(Type, DefPath)>>>,
        types_union: std::cell::RefCell<Option<Vec<(Type, DefPath)>>>,
        unsafe_types: std::cell::RefCell<Option<Vec<(Type,)>>>,
        safe_wrapper_types: std::cell::RefCell<Option<Vec<(Type,)>>>,
        unsafe_blocks: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    DefPath,
                    Scope,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>,
            >,
        >,
        functions_unsafe_blocks: std::cell::RefCell<
            Option<Vec<(Build, Item, Scope, SpanExpansionKind, BlockCheckMode)>>,
        >,
        selected_function_definitions: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    Item,
                    DefPath,
                    Module,
                    TyVisibility,
                    Safety,
                    Abi,
                    Type,
                    bool,
                )>,
            >,
        >,
        unsafe_statements: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    Statement,
                    BasicBlock,
                    StatementIndex,
                    StatementKind,
                    Scope,
                    BlockCheckMode,
                )>,
            >,
        >,
        unsafe_terminators: std::cell::RefCell<
            Option<Vec<(Build, BasicBlock, TerminatorKind, Scope, BlockCheckMode)>>,
        >,
        unsafe_block_calls: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    BasicBlock,
                    Scope,
                    BlockCheckMode,
                    FunctionCall,
                    Safety,
                    Abi,
                    Type,
                )>,
            >,
        >,
        unsafe_block_calls_known_target: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    BasicBlock,
                    Scope,
                    Span,
                    BlockCheckMode,
                    FunctionCall,
                    DefPath,
                    Safety,
                    Abi,
                    Type,
                )>,
            >,
        >,
        unsafe_block_call_counts:
            std::cell::RefCell<Option<Vec<(Build, Scope, BlockCheckMode, u16)>>>,
        unsafe_block_no_calls: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    DefPath,
                    Scope,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>,
            >,
        >,
        adts_with_unsafe_cell_fields: std::cell::RefCell<
            Option<
                Vec<(
                    Type,
                    DefPath,
                    AdtKind,
                    bool,
                    InternedString,
                    TyVisibility,
                    Type,
                    TyKind,
                    DefPath,
                )>,
            >,
        >,
        selected_function_sizes: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    Item,
                    DefPath,
                    TyVisibility,
                    Safety,
                    Abi,
                    bool,
                    u64,
                    u64,
                    u64,
                )>,
            >,
        >,
        selected_build_sizes: std::cell::RefCell<Option<Vec<(Build, u64, u64, u64)>>>,
        selected_thir_bodies: std::cell::RefCell<Option<Vec<(Build, Item, DefPath, ThirBlock)>>>,
        selected_thir_blocks: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    DefPath,
                    ThirBlock,
                    ThirBlock,
                    ScopeSafety,
                    BlockCheckMode,
                    Span,
                )>,
            >,
        >,
        unsafe_thir_blocks: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    DefPath,
                    ThirBlock,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>,
            >,
        >,
        unsafe_thir_stmts: std::cell::RefCell<
            Option<Vec<(Build, ThirStmt, ThirBlock, StatementIndex, BlockCheckMode)>>,
        >,
        functions_unsafe_thir_blocks: std::cell::RefCell<
            Option<Vec<(Build, Item, ThirBlock, SpanExpansionKind, BlockCheckMode)>>,
        >,
        selected_function_thir_sizes: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    Item,
                    DefPath,
                    TyVisibility,
                    Safety,
                    Abi,
                    bool,
                    u64,
                    u64,
                    u64,
                )>,
            >,
        >,
        selected_build_thir_sizes: std::cell::RefCell<Option<Vec<(Build, u64, u64, u64)>>>,
        unsafe_thir_block_calls: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    ThirBlock,
                    BlockCheckMode,
                    ThirExpr,
                    ThirExpr,
                    Safety,
                    Abi,
                    Type,
                )>,
            >,
        >,
        unsafe_thir_block_call_counts:
            std::cell::RefCell<Option<Vec<(Build, ThirBlock, BlockCheckMode, u16)>>>,
        unsafe_thir_block_no_calls: std::cell::RefCell<
            Option<
                Vec<(
                    Build,
                    DefPath,
                    ThirBlock,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>,
            >,
        >,
        strings: std::cell::RefCell<Option<InterningTable<InternedString, String>>>,
        package_names: std::cell::RefCell<Option<InterningTable<Package, InternedString>>>,
        package_versions:
            std::cell::RefCell<Option<InterningTable<PackageVersion, InternedString>>>,
        crate_names: std::cell::RefCell<Option<InterningTable<Krate, InternedString>>>,
        editions: std::cell::RefCell<Option<InterningTable<Edition, InternedString>>>,
        names: std::cell::RefCell<Option<InterningTable<Name, InternedString>>>,
        relative_def_paths:
            std::cell::RefCell<Option<InterningTable<RelativeDefId, InternedString>>>,
        summary_keys: std::cell::RefCell<Option<InterningTable<SummaryId, InternedString>>>,
        abis: std::cell::RefCell<Option<InterningTable<Abi, InternedString>>>,
        def_paths: std::cell::RefCell<
            Option<
                InterningTable<DefPath, (Krate, CrateHash, RelativeDefId, DefPathHash, SummaryId)>,
            >,
        >,
        builds: std::cell::RefCell<
            Option<InterningTable<Build, (Package, PackageVersion, Krate, CrateHash, Edition)>>,
        >,
        span_file_names: std::cell::RefCell<Option<InterningTable<SpanFileName, InternedString>>>,
        crate_cfg_keys: std::cell::RefCell<Option<InterningTable<CrateCfgKey, InternedString>>>,
        crate_cfg_values: std::cell::RefCell<Option<InterningTable<CrateCfgValue, InternedString>>>,
        type_kinds: std::cell::RefCell<Option<InterningTable<TyKind, InternedString>>>,
        statement_kinds: std::cell::RefCell<Option<InterningTable<StatementKind, InternedString>>>,
        binary_op_kind: std::cell::RefCell<Option<InterningTable<BinOp, InternedString>>>,
        nullary_op_kind: std::cell::RefCell<Option<InterningTable<NullOp, InternedString>>>,
        unary_op_kind: std::cell::RefCell<Option<InterningTable<UnOp, InternedString>>>,
        terminator_kinds:
            std::cell::RefCell<Option<InterningTable<TerminatorKind, InternedString>>>,
        thir_binary_op_kind: std::cell::RefCell<Option<InterningTable<ThirBinOp, InternedString>>>,
        thir_logical_op_kind:
            std::cell::RefCell<Option<InterningTable<ThirLogicalOp, InternedString>>>,
        thir_unary_op_kind: std::cell::RefCell<Option<InterningTable<ThirUnOp, InternedString>>>,
    }
    impl Loader {
        pub fn new(database_root: PathBuf) -> Self {
            Self {
                database_root,
                ..Loader::default()
            }
        }
        pub fn load_iter_def_path_span(&self) -> impl Iterator<Item = (DefPath, Span)> {
            unsafe {
                load_elts_relation::<(DefPath, Span)>(
                    8883063296299969981u64,
                    self.database_root.join("relations/def_path_span"),
                )
            }
            .unwrap()
        }
        pub fn load_def_path_span(&self) -> std::cell::Ref<Vec<(DefPath, Span)>> {
            if self.def_path_span.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(DefPath, Span)>(
                        8883063296299969981u64,
                        self.database_root.join("relations/def_path_span"),
                    )
                }
                .unwrap();
                *self.def_path_span.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.def_path_span.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_def_path_span(&self, facts: Vec<(DefPath, Span)>) {
            unsafe {
                save_elts_relation::<(DefPath, Span)>(
                    facts,
                    8883063296299969981u64,
                    self.database_root.join("relations/def_path_span"),
                );
            }
        }
        pub fn store_iter_def_path_span(&self, facts: impl IntoIterator<Item = (DefPath, Span)>) {
            unsafe {
                save_elts_relation::<(DefPath, Span)>(
                    facts,
                    8883063296299969981u64,
                    self.database_root.join("relations/def_path_span"),
                );
            }
        }
        pub fn load_def_path_span_as_map(&self) -> std::collections::HashMap<DefPath, Span> {
            self.load_def_path_span().iter().copied().collect()
        }
        pub fn load_iter_type_description(
            &self,
        ) -> impl Iterator<Item = (Type, InternedString, InternedString)> {
            unsafe {
                load_elts_relation::<(Type, InternedString, InternedString)>(
                    14247184057110084413u64,
                    self.database_root.join("relations/type_description"),
                )
            }
            .unwrap()
        }
        pub fn load_type_description(
            &self,
        ) -> std::cell::Ref<Vec<(Type, InternedString, InternedString)>> {
            if self.type_description.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, InternedString, InternedString)>(
                        14247184057110084413u64,
                        self.database_root.join("relations/type_description"),
                    )
                }
                .unwrap();
                *self.type_description.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.type_description.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_type_description(&self, facts: Vec<(Type, InternedString, InternedString)>) {
            unsafe {
                save_elts_relation::<(Type, InternedString, InternedString)>(
                    facts,
                    14247184057110084413u64,
                    self.database_root.join("relations/type_description"),
                );
            }
        }
        pub fn store_iter_type_description(
            &self,
            facts: impl IntoIterator<Item = (Type, InternedString, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, InternedString, InternedString)>(
                    facts,
                    14247184057110084413u64,
                    self.database_root.join("relations/type_description"),
                );
            }
        }
        pub fn load_iter_build_crate_types(&self) -> impl Iterator<Item = (Build, InternedString)> {
            unsafe {
                load_elts_relation::<(Build, InternedString)>(
                    11694151696289681161u64,
                    self.database_root.join("relations/build_crate_types"),
                )
            }
            .unwrap()
        }
        pub fn load_build_crate_types(&self) -> std::cell::Ref<Vec<(Build, InternedString)>> {
            if self.build_crate_types.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, InternedString)>(
                        11694151696289681161u64,
                        self.database_root.join("relations/build_crate_types"),
                    )
                }
                .unwrap();
                *self.build_crate_types.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.build_crate_types.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_build_crate_types(&self, facts: Vec<(Build, InternedString)>) {
            unsafe {
                save_elts_relation::<(Build, InternedString)>(
                    facts,
                    11694151696289681161u64,
                    self.database_root.join("relations/build_crate_types"),
                );
            }
        }
        pub fn store_iter_build_crate_types(
            &self,
            facts: impl IntoIterator<Item = (Build, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, InternedString)>(
                    facts,
                    11694151696289681161u64,
                    self.database_root.join("relations/build_crate_types"),
                );
            }
        }
        pub fn load_build_crate_types_as_map(
            &self,
        ) -> std::collections::HashMap<Build, InternedString> {
            self.load_build_crate_types().iter().copied().collect()
        }
        pub fn load_iter_root_modules(&self) -> impl Iterator<Item = (Build, Module)> {
            unsafe {
                load_elts_relation::<(Build, Module)>(
                    3608617112668148983u64,
                    self.database_root.join("relations/root_modules"),
                )
            }
            .unwrap()
        }
        pub fn load_root_modules(&self) -> std::cell::Ref<Vec<(Build, Module)>> {
            if self.root_modules.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, Module)>(
                        3608617112668148983u64,
                        self.database_root.join("relations/root_modules"),
                    )
                }
                .unwrap();
                *self.root_modules.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.root_modules.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_root_modules(&self, facts: Vec<(Build, Module)>) {
            unsafe {
                save_elts_relation::<(Build, Module)>(
                    facts,
                    3608617112668148983u64,
                    self.database_root.join("relations/root_modules"),
                );
            }
        }
        pub fn store_iter_root_modules(&self, facts: impl IntoIterator<Item = (Build, Module)>) {
            unsafe {
                save_elts_relation::<(Build, Module)>(
                    facts,
                    3608617112668148983u64,
                    self.database_root.join("relations/root_modules"),
                );
            }
        }
        pub fn load_root_modules_as_map(&self) -> std::collections::HashMap<Build, Module> {
            self.load_root_modules().iter().copied().collect()
        }
        pub fn load_iter_submodules(
            &self,
        ) -> impl Iterator<Item = (DefPath, Module, Module, Name, TyVisibility, Abi)> {
            unsafe {
                load_elts_relation::<(DefPath, Module, Module, Name, TyVisibility, Abi)>(
                    14223402788489432104u64,
                    self.database_root.join("relations/submodules"),
                )
            }
            .unwrap()
        }
        pub fn load_submodules(
            &self,
        ) -> std::cell::Ref<Vec<(DefPath, Module, Module, Name, TyVisibility, Abi)>> {
            if self.submodules.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        DefPath,
                        Module,
                        Module,
                        Name,
                        TyVisibility,
                        Abi,
                    )>(
                        14223402788489432104u64,
                        self.database_root.join("relations/submodules"),
                    )
                }
                .unwrap();
                *self.submodules.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.submodules.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_submodules(
            &self,
            facts: Vec<(DefPath, Module, Module, Name, TyVisibility, Abi)>,
        ) {
            unsafe {
                save_elts_relation::<(DefPath, Module, Module, Name, TyVisibility, Abi)>(
                    facts,
                    14223402788489432104u64,
                    self.database_root.join("relations/submodules"),
                );
            }
        }
        pub fn store_iter_submodules(
            &self,
            facts: impl IntoIterator<Item = (DefPath, Module, Module, Name, TyVisibility, Abi)>,
        ) {
            unsafe {
                save_elts_relation::<(DefPath, Module, Module, Name, TyVisibility, Abi)>(
                    facts,
                    14223402788489432104u64,
                    self.database_root.join("relations/submodules"),
                );
            }
        }
        pub fn load_iter_function_definitions(
            &self,
        ) -> impl Iterator<Item = (Item, DefPath, Module, TyVisibility, Safety, Abi, Type)>
        {
            unsafe {
                load_elts_relation::<(Item, DefPath, Module, TyVisibility, Safety, Abi, Type)>(
                    6078223197665779672u64,
                    self.database_root.join("relations/function_definitions"),
                )
            }
            .unwrap()
        }
        pub fn load_function_definitions(
            &self,
        ) -> std::cell::Ref<Vec<(Item, DefPath, Module, TyVisibility, Safety, Abi, Type)>> {
            if self.function_definitions.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Item,
                        DefPath,
                        Module,
                        TyVisibility,
                        Safety,
                        Abi,
                        Type,
                    )>(
                        6078223197665779672u64,
                        self.database_root.join("relations/function_definitions"),
                    )
                }
                .unwrap();
                *self.function_definitions.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.function_definitions.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_function_definitions(
            &self,
            facts: Vec<(Item, DefPath, Module, TyVisibility, Safety, Abi, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Item, DefPath, Module, TyVisibility, Safety, Abi, Type)>(
                    facts,
                    6078223197665779672u64,
                    self.database_root.join("relations/function_definitions"),
                );
            }
        }
        pub fn store_iter_function_definitions(
            &self,
            facts: impl IntoIterator<Item = (Item, DefPath, Module, TyVisibility, Safety, Abi, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Item, DefPath, Module, TyVisibility, Safety, Abi, Type)>(
                    facts,
                    6078223197665779672u64,
                    self.database_root.join("relations/function_definitions"),
                );
            }
        }
        pub fn load_iter_function_parameter_types(
            &self,
        ) -> impl Iterator<Item = (Item, FnParamIndex, Type)> {
            unsafe {
                load_elts_relation::<(Item, FnParamIndex, Type)>(
                    17967330428057203654u64,
                    self.database_root
                        .join("relations/function_parameter_types"),
                )
            }
            .unwrap()
        }
        pub fn load_function_parameter_types(
            &self,
        ) -> std::cell::Ref<Vec<(Item, FnParamIndex, Type)>> {
            if self.function_parameter_types.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Item, FnParamIndex, Type)>(
                        17967330428057203654u64,
                        self.database_root
                            .join("relations/function_parameter_types"),
                    )
                }
                .unwrap();
                *self.function_parameter_types.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.function_parameter_types.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_function_parameter_types(&self, facts: Vec<(Item, FnParamIndex, Type)>) {
            unsafe {
                save_elts_relation::<(Item, FnParamIndex, Type)>(
                    facts,
                    17967330428057203654u64,
                    self.database_root
                        .join("relations/function_parameter_types"),
                );
            }
        }
        pub fn store_iter_function_parameter_types(
            &self,
            facts: impl IntoIterator<Item = (Item, FnParamIndex, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Item, FnParamIndex, Type)>(
                    facts,
                    17967330428057203654u64,
                    self.database_root
                        .join("relations/function_parameter_types"),
                );
            }
        }
        pub fn load_iter_function_unsafe_use(&self) -> impl Iterator<Item = (DefPath, bool)> {
            unsafe {
                load_elts_relation::<(DefPath, bool)>(
                    3679592891414654765u64,
                    self.database_root.join("relations/function_unsafe_use"),
                )
            }
            .unwrap()
        }
        pub fn load_function_unsafe_use(&self) -> std::cell::Ref<Vec<(DefPath, bool)>> {
            if self.function_unsafe_use.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(DefPath, bool)>(
                        3679592891414654765u64,
                        self.database_root.join("relations/function_unsafe_use"),
                    )
                }
                .unwrap();
                *self.function_unsafe_use.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.function_unsafe_use.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_function_unsafe_use(&self, facts: Vec<(DefPath, bool)>) {
            unsafe {
                save_elts_relation::<(DefPath, bool)>(
                    facts,
                    3679592891414654765u64,
                    self.database_root.join("relations/function_unsafe_use"),
                );
            }
        }
        pub fn store_iter_function_unsafe_use(
            &self,
            facts: impl IntoIterator<Item = (DefPath, bool)>,
        ) {
            unsafe {
                save_elts_relation::<(DefPath, bool)>(
                    facts,
                    3679592891414654765u64,
                    self.database_root.join("relations/function_unsafe_use"),
                );
            }
        }
        pub fn load_function_unsafe_use_as_map(&self) -> std::collections::HashMap<DefPath, bool> {
            self.load_function_unsafe_use().iter().copied().collect()
        }
        pub fn load_iter_function_unsafe_reasons(
            &self,
        ) -> impl Iterator<Item = (DefPath, u32, InternedString)> {
            unsafe {
                load_elts_relation::<(DefPath, u32, InternedString)>(
                    7306117882207793123u64,
                    self.database_root.join("relations/function_unsafe_reasons"),
                )
            }
            .unwrap()
        }
        pub fn load_function_unsafe_reasons(
            &self,
        ) -> std::cell::Ref<Vec<(DefPath, u32, InternedString)>> {
            if self.function_unsafe_reasons.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(DefPath, u32, InternedString)>(
                        7306117882207793123u64,
                        self.database_root.join("relations/function_unsafe_reasons"),
                    )
                }
                .unwrap();
                *self.function_unsafe_reasons.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.function_unsafe_reasons.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_function_unsafe_reasons(&self, facts: Vec<(DefPath, u32, InternedString)>) {
            unsafe {
                save_elts_relation::<(DefPath, u32, InternedString)>(
                    facts,
                    7306117882207793123u64,
                    self.database_root.join("relations/function_unsafe_reasons"),
                );
            }
        }
        pub fn store_iter_function_unsafe_reasons(
            &self,
            facts: impl IntoIterator<Item = (DefPath, u32, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(DefPath, u32, InternedString)>(
                    facts,
                    7306117882207793123u64,
                    self.database_root.join("relations/function_unsafe_reasons"),
                );
            }
        }
        pub fn load_iter_thir_bodies(&self) -> impl Iterator<Item = (Item, DefPath, ThirBlock)> {
            unsafe {
                load_elts_relation::<(Item, DefPath, ThirBlock)>(
                    9078912835344629248u64,
                    self.database_root.join("relations/thir_bodies"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_bodies(&self) -> std::cell::Ref<Vec<(Item, DefPath, ThirBlock)>> {
            if self.thir_bodies.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Item, DefPath, ThirBlock)>(
                        9078912835344629248u64,
                        self.database_root.join("relations/thir_bodies"),
                    )
                }
                .unwrap();
                *self.thir_bodies.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_bodies.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_thir_bodies(&self, facts: Vec<(Item, DefPath, ThirBlock)>) {
            unsafe {
                save_elts_relation::<(Item, DefPath, ThirBlock)>(
                    facts,
                    9078912835344629248u64,
                    self.database_root.join("relations/thir_bodies"),
                );
            }
        }
        pub fn store_iter_thir_bodies(
            &self,
            facts: impl IntoIterator<Item = (Item, DefPath, ThirBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(Item, DefPath, ThirBlock)>(
                    facts,
                    9078912835344629248u64,
                    self.database_root.join("relations/thir_bodies"),
                );
            }
        }
        pub fn load_iter_thir_blocks(
            &self,
        ) -> impl Iterator<Item = (ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span)>
        {
            unsafe {
                load_elts_relation::<(ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span)>(
                    5712874088580563410u64,
                    self.database_root.join("relations/thir_blocks"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_blocks(
            &self,
        ) -> std::cell::Ref<Vec<(ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span)>>
        {
            if self.thir_blocks.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        ThirBlock,
                        ThirBlock,
                        ScopeSafety,
                        BlockCheckMode,
                        Span,
                    )>(
                        5712874088580563410u64,
                        self.database_root.join("relations/thir_blocks"),
                    )
                }
                .unwrap();
                *self.thir_blocks.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_blocks.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_thir_blocks(
            &self,
            facts: Vec<(ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span)>(
                    facts,
                    5712874088580563410u64,
                    self.database_root.join("relations/thir_blocks"),
                );
            }
        }
        pub fn store_iter_thir_blocks(
            &self,
            facts: impl IntoIterator<Item = (ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span)>(
                    facts,
                    5712874088580563410u64,
                    self.database_root.join("relations/thir_blocks"),
                );
            }
        }
        pub fn load_iter_thir_stmts(
            &self,
        ) -> impl Iterator<Item = (ThirStmt, ThirBlock, ThirBlock, StatementIndex)> {
            unsafe {
                load_elts_relation::<(ThirStmt, ThirBlock, ThirBlock, StatementIndex)>(
                    11662528851258065727u64,
                    self.database_root.join("relations/thir_stmts"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_stmts(
            &self,
        ) -> std::cell::Ref<Vec<(ThirStmt, ThirBlock, ThirBlock, StatementIndex)>> {
            if self.thir_stmts.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        ThirStmt,
                        ThirBlock,
                        ThirBlock,
                        StatementIndex,
                    )>(
                        11662528851258065727u64,
                        self.database_root.join("relations/thir_stmts"),
                    )
                }
                .unwrap();
                *self.thir_stmts.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_stmts.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_thir_stmts(
            &self,
            facts: Vec<(ThirStmt, ThirBlock, ThirBlock, StatementIndex)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirStmt, ThirBlock, ThirBlock, StatementIndex)>(
                    facts,
                    11662528851258065727u64,
                    self.database_root.join("relations/thir_stmts"),
                );
            }
        }
        pub fn store_iter_thir_stmts(
            &self,
            facts: impl IntoIterator<Item = (ThirStmt, ThirBlock, ThirBlock, StatementIndex)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirStmt, ThirBlock, ThirBlock, StatementIndex)>(
                    facts,
                    11662528851258065727u64,
                    self.database_root.join("relations/thir_stmts"),
                );
            }
        }
        pub fn load_iter_thir_stmts_expr(&self) -> impl Iterator<Item = (ThirStmt, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirStmt, ThirExpr)>(
                    6974731257762681957u64,
                    self.database_root.join("relations/thir_stmts_expr"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_stmts_expr(&self) -> std::cell::Ref<Vec<(ThirStmt, ThirExpr)>> {
            if self.thir_stmts_expr.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirStmt, ThirExpr)>(
                        6974731257762681957u64,
                        self.database_root.join("relations/thir_stmts_expr"),
                    )
                }
                .unwrap();
                *self.thir_stmts_expr.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_stmts_expr.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_stmts_expr(&self, facts: Vec<(ThirStmt, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirStmt, ThirExpr)>(
                    facts,
                    6974731257762681957u64,
                    self.database_root.join("relations/thir_stmts_expr"),
                );
            }
        }
        pub fn store_iter_thir_stmts_expr(
            &self,
            facts: impl IntoIterator<Item = (ThirStmt, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirStmt, ThirExpr)>(
                    facts,
                    6974731257762681957u64,
                    self.database_root.join("relations/thir_stmts_expr"),
                );
            }
        }
        pub fn load_thir_stmts_expr_as_map(&self) -> std::collections::HashMap<ThirStmt, ThirExpr> {
            self.load_thir_stmts_expr().iter().copied().collect()
        }
        pub fn load_iter_thir_stmts_let(
            &self,
        ) -> impl Iterator<Item = (ThirStmt, ThirExpr, ThirBlock, Span)> {
            unsafe {
                load_elts_relation::<(ThirStmt, ThirExpr, ThirBlock, Span)>(
                    3948572987018662193u64,
                    self.database_root.join("relations/thir_stmts_let"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_stmts_let(
            &self,
        ) -> std::cell::Ref<Vec<(ThirStmt, ThirExpr, ThirBlock, Span)>> {
            if self.thir_stmts_let.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirStmt, ThirExpr, ThirBlock, Span)>(
                        3948572987018662193u64,
                        self.database_root.join("relations/thir_stmts_let"),
                    )
                }
                .unwrap();
                *self.thir_stmts_let.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_stmts_let.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_stmts_let(&self, facts: Vec<(ThirStmt, ThirExpr, ThirBlock, Span)>) {
            unsafe {
                save_elts_relation::<(ThirStmt, ThirExpr, ThirBlock, Span)>(
                    facts,
                    3948572987018662193u64,
                    self.database_root.join("relations/thir_stmts_let"),
                );
            }
        }
        pub fn store_iter_thir_stmts_let(
            &self,
            facts: impl IntoIterator<Item = (ThirStmt, ThirExpr, ThirBlock, Span)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirStmt, ThirExpr, ThirBlock, Span)>(
                    facts,
                    3948572987018662193u64,
                    self.database_root.join("relations/thir_stmts_let"),
                );
            }
        }
        pub fn load_iter_thir_block_expr(&self) -> impl Iterator<Item = (ThirBlock, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirBlock, ThirExpr)>(
                    13932710207231639296u64,
                    self.database_root.join("relations/thir_block_expr"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_block_expr(&self) -> std::cell::Ref<Vec<(ThirBlock, ThirExpr)>> {
            if self.thir_block_expr.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirBlock, ThirExpr)>(
                        13932710207231639296u64,
                        self.database_root.join("relations/thir_block_expr"),
                    )
                }
                .unwrap();
                *self.thir_block_expr.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_block_expr.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_block_expr(&self, facts: Vec<(ThirBlock, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirBlock, ThirExpr)>(
                    facts,
                    13932710207231639296u64,
                    self.database_root.join("relations/thir_block_expr"),
                );
            }
        }
        pub fn store_iter_thir_block_expr(
            &self,
            facts: impl IntoIterator<Item = (ThirBlock, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirBlock, ThirExpr)>(
                    facts,
                    13932710207231639296u64,
                    self.database_root.join("relations/thir_block_expr"),
                );
            }
        }
        pub fn load_thir_block_expr_as_map(
            &self,
        ) -> std::collections::HashMap<ThirBlock, ThirExpr> {
            self.load_thir_block_expr().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirBlock, ThirBlock, Type, Span)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirBlock, ThirBlock, Type, Span)>(
                    14472862124235708861u64,
                    self.database_root.join("relations/thir_exprs"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, ThirBlock, ThirBlock, Type, Span)>> {
            if self.thir_exprs.borrow().is_none() {
                let relation =
                    unsafe {
                        load_elts_relation_into_relation::<(
                            ThirExpr,
                            ThirBlock,
                            ThirBlock,
                            Type,
                            Span,
                        )>(
                            14472862124235708861u64,
                            self.database_root.join("relations/thir_exprs"),
                        )
                    }
                    .unwrap();
                *self.thir_exprs.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_thir_exprs(&self, facts: Vec<(ThirExpr, ThirBlock, ThirBlock, Type, Span)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirBlock, ThirBlock, Type, Span)>(
                    facts,
                    14472862124235708861u64,
                    self.database_root.join("relations/thir_exprs"),
                );
            }
        }
        pub fn store_iter_thir_exprs(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirBlock, ThirBlock, Type, Span)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirBlock, ThirBlock, Type, Span)>(
                    facts,
                    14472862124235708861u64,
                    self.database_root.join("relations/thir_exprs"),
                );
            }
        }
        pub fn load_iter_thir_exprs_scope(&self) -> impl Iterator<Item = (ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr)>(
                    15126226146931209448u64,
                    self.database_root.join("relations/thir_exprs_scope"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_scope(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr)>> {
            if self.thir_exprs_scope.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr)>(
                        15126226146931209448u64,
                        self.database_root.join("relations/thir_exprs_scope"),
                    )
                }
                .unwrap();
                *self.thir_exprs_scope.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_scope.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_scope(&self, facts: Vec<(ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    15126226146931209448u64,
                    self.database_root.join("relations/thir_exprs_scope"),
                );
            }
        }
        pub fn store_iter_thir_exprs_scope(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    15126226146931209448u64,
                    self.database_root.join("relations/thir_exprs_scope"),
                );
            }
        }
        pub fn load_thir_exprs_scope_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, ThirExpr> {
            self.load_thir_exprs_scope().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_box(&self) -> impl Iterator<Item = (ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr)>(
                    17250276924894967218u64,
                    self.database_root.join("relations/thir_exprs_box"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_box(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr)>> {
            if self.thir_exprs_box.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr)>(
                        17250276924894967218u64,
                        self.database_root.join("relations/thir_exprs_box"),
                    )
                }
                .unwrap();
                *self.thir_exprs_box.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_box.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_box(&self, facts: Vec<(ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    17250276924894967218u64,
                    self.database_root.join("relations/thir_exprs_box"),
                );
            }
        }
        pub fn store_iter_thir_exprs_box(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    17250276924894967218u64,
                    self.database_root.join("relations/thir_exprs_box"),
                );
            }
        }
        pub fn load_thir_exprs_box_as_map(&self) -> std::collections::HashMap<ThirExpr, ThirExpr> {
            self.load_thir_exprs_box().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_if(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirExpr, ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr, ThirExpr, ThirExpr)>(
                    1136822200270957559u64,
                    self.database_root.join("relations/thir_exprs_if"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_if(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr, ThirExpr, ThirExpr)>> {
            if self.thir_exprs_if.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr, ThirExpr, ThirExpr)>(
                        1136822200270957559u64,
                        self.database_root.join("relations/thir_exprs_if"),
                    )
                }
                .unwrap();
                *self.thir_exprs_if.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_if.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_if(&self, facts: Vec<(ThirExpr, ThirExpr, ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, ThirExpr, ThirExpr)>(
                    facts,
                    1136822200270957559u64,
                    self.database_root.join("relations/thir_exprs_if"),
                );
            }
        }
        pub fn store_iter_thir_exprs_if(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr, ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, ThirExpr, ThirExpr)>(
                    facts,
                    1136822200270957559u64,
                    self.database_root.join("relations/thir_exprs_if"),
                );
            }
        }
        pub fn load_iter_thir_exprs_call(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, Type, ThirExpr, Safety, Abi, Type)> {
            unsafe {
                load_elts_relation::<(ThirExpr, Type, ThirExpr, Safety, Abi, Type)>(
                    10675202487661065417u64,
                    self.database_root.join("relations/thir_exprs_call"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_call(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, Type, ThirExpr, Safety, Abi, Type)>> {
            if self.thir_exprs_call.borrow().is_none() {
                let relation =
                    unsafe {
                        load_elts_relation_into_relation::<(
                            ThirExpr,
                            Type,
                            ThirExpr,
                            Safety,
                            Abi,
                            Type,
                        )>(
                            10675202487661065417u64,
                            self.database_root.join("relations/thir_exprs_call"),
                        )
                    }
                    .unwrap();
                *self.thir_exprs_call.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_call.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_call(
            &self,
            facts: Vec<(ThirExpr, Type, ThirExpr, Safety, Abi, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, Type, ThirExpr, Safety, Abi, Type)>(
                    facts,
                    10675202487661065417u64,
                    self.database_root.join("relations/thir_exprs_call"),
                );
            }
        }
        pub fn store_iter_thir_exprs_call(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, Type, ThirExpr, Safety, Abi, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, Type, ThirExpr, Safety, Abi, Type)>(
                    facts,
                    10675202487661065417u64,
                    self.database_root.join("relations/thir_exprs_call"),
                );
            }
        }
        pub fn load_iter_thir_exprs_call_arg(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirCallArgIndex, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirCallArgIndex, ThirExpr)>(
                    12688704234864686271u64,
                    self.database_root.join("relations/thir_exprs_call_arg"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_call_arg(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, ThirCallArgIndex, ThirExpr)>> {
            if self.thir_exprs_call_arg.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirCallArgIndex, ThirExpr)>(
                        12688704234864686271u64,
                        self.database_root.join("relations/thir_exprs_call_arg"),
                    )
                }
                .unwrap();
                *self.thir_exprs_call_arg.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_call_arg.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_call_arg(
            &self,
            facts: Vec<(ThirExpr, ThirCallArgIndex, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirCallArgIndex, ThirExpr)>(
                    facts,
                    12688704234864686271u64,
                    self.database_root.join("relations/thir_exprs_call_arg"),
                );
            }
        }
        pub fn store_iter_thir_exprs_call_arg(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirCallArgIndex, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirCallArgIndex, ThirExpr)>(
                    facts,
                    12688704234864686271u64,
                    self.database_root.join("relations/thir_exprs_call_arg"),
                );
            }
        }
        pub fn load_iter_thir_exprs_call_const_target(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, DefPath)> {
            unsafe {
                load_elts_relation::<(ThirExpr, DefPath)>(
                    282454153203586278u64,
                    self.database_root
                        .join("relations/thir_exprs_call_const_target"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_call_const_target(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, DefPath)>> {
            if self.thir_exprs_call_const_target.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, DefPath)>(
                        282454153203586278u64,
                        self.database_root
                            .join("relations/thir_exprs_call_const_target"),
                    )
                }
                .unwrap();
                *self.thir_exprs_call_const_target.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_call_const_target.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_call_const_target(&self, facts: Vec<(ThirExpr, DefPath)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath)>(
                    facts,
                    282454153203586278u64,
                    self.database_root
                        .join("relations/thir_exprs_call_const_target"),
                );
            }
        }
        pub fn store_iter_thir_exprs_call_const_target(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath)>(
                    facts,
                    282454153203586278u64,
                    self.database_root
                        .join("relations/thir_exprs_call_const_target"),
                );
            }
        }
        pub fn load_thir_exprs_call_const_target_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, DefPath> {
            self.load_thir_exprs_call_const_target()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_thir_exprs_call_const_target_desc(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, InternedString, InternedString, InternedString)>
        {
            unsafe {
                load_elts_relation::<(ThirExpr, InternedString, InternedString, InternedString)>(
                    14557156226083258014u64,
                    self.database_root
                        .join("relations/thir_exprs_call_const_target_desc"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_call_const_target_desc(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, InternedString, InternedString, InternedString)>>
        {
            if self.thir_exprs_call_const_target_desc.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        ThirExpr,
                        InternedString,
                        InternedString,
                        InternedString,
                    )>(
                        14557156226083258014u64,
                        self.database_root
                            .join("relations/thir_exprs_call_const_target_desc"),
                    )
                }
                .unwrap();
                *self.thir_exprs_call_const_target_desc.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_call_const_target_desc.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_call_const_target_desc(
            &self,
            facts: Vec<(ThirExpr, InternedString, InternedString, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, InternedString, InternedString, InternedString)>(
                    facts,
                    14557156226083258014u64,
                    self.database_root
                        .join("relations/thir_exprs_call_const_target_desc"),
                );
            }
        }
        pub fn store_iter_thir_exprs_call_const_target_desc(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, InternedString, InternedString, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, InternedString, InternedString, InternedString)>(
                    facts,
                    14557156226083258014u64,
                    self.database_root
                        .join("relations/thir_exprs_call_const_target_desc"),
                );
            }
        }
        pub fn load_iter_thir_exprs_call_const_target_self(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, Type)> {
            unsafe {
                load_elts_relation::<(ThirExpr, Type)>(
                    11355230934198225002u64,
                    self.database_root
                        .join("relations/thir_exprs_call_const_target_self"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_call_const_target_self(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, Type)>> {
            if self.thir_exprs_call_const_target_self.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, Type)>(
                        11355230934198225002u64,
                        self.database_root
                            .join("relations/thir_exprs_call_const_target_self"),
                    )
                }
                .unwrap();
                *self.thir_exprs_call_const_target_self.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_call_const_target_self.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_call_const_target_self(&self, facts: Vec<(ThirExpr, Type)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, Type)>(
                    facts,
                    11355230934198225002u64,
                    self.database_root
                        .join("relations/thir_exprs_call_const_target_self"),
                );
            }
        }
        pub fn store_iter_thir_exprs_call_const_target_self(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, Type)>(
                    facts,
                    11355230934198225002u64,
                    self.database_root
                        .join("relations/thir_exprs_call_const_target_self"),
                );
            }
        }
        pub fn load_thir_exprs_call_const_target_self_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, Type> {
            self.load_thir_exprs_call_const_target_self()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_thir_exprs_deref(&self) -> impl Iterator<Item = (ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr)>(
                    13435985341992119844u64,
                    self.database_root.join("relations/thir_exprs_deref"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_deref(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr)>> {
            if self.thir_exprs_deref.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr)>(
                        13435985341992119844u64,
                        self.database_root.join("relations/thir_exprs_deref"),
                    )
                }
                .unwrap();
                *self.thir_exprs_deref.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_deref.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_deref(&self, facts: Vec<(ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    13435985341992119844u64,
                    self.database_root.join("relations/thir_exprs_deref"),
                );
            }
        }
        pub fn store_iter_thir_exprs_deref(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    13435985341992119844u64,
                    self.database_root.join("relations/thir_exprs_deref"),
                );
            }
        }
        pub fn load_thir_exprs_deref_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, ThirExpr> {
            self.load_thir_exprs_deref().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_binary(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirBinOp, ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>(
                    14413232755879446303u64,
                    self.database_root.join("relations/thir_exprs_binary"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_binary(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>> {
            if self.thir_exprs_binary.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>(
                        14413232755879446303u64,
                        self.database_root.join("relations/thir_exprs_binary"),
                    )
                }
                .unwrap();
                *self.thir_exprs_binary.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_binary.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_binary(
            &self,
            facts: Vec<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>(
                    facts,
                    14413232755879446303u64,
                    self.database_root.join("relations/thir_exprs_binary"),
                );
            }
        }
        pub fn store_iter_thir_exprs_binary(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>(
                    facts,
                    14413232755879446303u64,
                    self.database_root.join("relations/thir_exprs_binary"),
                );
            }
        }
        pub fn load_iter_thir_exprs_logical_op(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirLogicalOp, ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirLogicalOp, ThirExpr, ThirExpr)>(
                    18234645225945847180u64,
                    self.database_root.join("relations/thir_exprs_logical_op"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_logical_op(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, ThirLogicalOp, ThirExpr, ThirExpr)>> {
            if self.thir_exprs_logical_op.borrow().is_none() {
                let relation =
                    unsafe {
                        load_elts_relation_into_relation::<(
                            ThirExpr,
                            ThirLogicalOp,
                            ThirExpr,
                            ThirExpr,
                        )>(
                            18234645225945847180u64,
                            self.database_root.join("relations/thir_exprs_logical_op"),
                        )
                    }
                    .unwrap();
                *self.thir_exprs_logical_op.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_logical_op.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_logical_op(
            &self,
            facts: Vec<(ThirExpr, ThirLogicalOp, ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirLogicalOp, ThirExpr, ThirExpr)>(
                    facts,
                    18234645225945847180u64,
                    self.database_root.join("relations/thir_exprs_logical_op"),
                );
            }
        }
        pub fn store_iter_thir_exprs_logical_op(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirLogicalOp, ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirLogicalOp, ThirExpr, ThirExpr)>(
                    facts,
                    18234645225945847180u64,
                    self.database_root.join("relations/thir_exprs_logical_op"),
                );
            }
        }
        pub fn load_iter_thir_exprs_unary(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirUnOp, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirUnOp, ThirExpr)>(
                    18219444126659999359u64,
                    self.database_root.join("relations/thir_exprs_unary"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_unary(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirUnOp, ThirExpr)>> {
            if self.thir_exprs_unary.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirUnOp, ThirExpr)>(
                        18219444126659999359u64,
                        self.database_root.join("relations/thir_exprs_unary"),
                    )
                }
                .unwrap();
                *self.thir_exprs_unary.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_unary.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_unary(&self, facts: Vec<(ThirExpr, ThirUnOp, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirUnOp, ThirExpr)>(
                    facts,
                    18219444126659999359u64,
                    self.database_root.join("relations/thir_exprs_unary"),
                );
            }
        }
        pub fn store_iter_thir_exprs_unary(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirUnOp, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirUnOp, ThirExpr)>(
                    facts,
                    18219444126659999359u64,
                    self.database_root.join("relations/thir_exprs_unary"),
                );
            }
        }
        pub fn load_iter_thir_exprs_cast(&self) -> impl Iterator<Item = (ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr)>(
                    15582714740127479454u64,
                    self.database_root.join("relations/thir_exprs_cast"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_cast(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr)>> {
            if self.thir_exprs_cast.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr)>(
                        15582714740127479454u64,
                        self.database_root.join("relations/thir_exprs_cast"),
                    )
                }
                .unwrap();
                *self.thir_exprs_cast.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_cast.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_cast(&self, facts: Vec<(ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    15582714740127479454u64,
                    self.database_root.join("relations/thir_exprs_cast"),
                );
            }
        }
        pub fn store_iter_thir_exprs_cast(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    15582714740127479454u64,
                    self.database_root.join("relations/thir_exprs_cast"),
                );
            }
        }
        pub fn load_thir_exprs_cast_as_map(&self) -> std::collections::HashMap<ThirExpr, ThirExpr> {
            self.load_thir_exprs_cast().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_use(&self) -> impl Iterator<Item = (ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr)>(
                    18202594178015551134u64,
                    self.database_root.join("relations/thir_exprs_use"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_use(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr)>> {
            if self.thir_exprs_use.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr)>(
                        18202594178015551134u64,
                        self.database_root.join("relations/thir_exprs_use"),
                    )
                }
                .unwrap();
                *self.thir_exprs_use.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_use.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_use(&self, facts: Vec<(ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    18202594178015551134u64,
                    self.database_root.join("relations/thir_exprs_use"),
                );
            }
        }
        pub fn store_iter_thir_exprs_use(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    18202594178015551134u64,
                    self.database_root.join("relations/thir_exprs_use"),
                );
            }
        }
        pub fn load_thir_exprs_use_as_map(&self) -> std::collections::HashMap<ThirExpr, ThirExpr> {
            self.load_thir_exprs_use().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_never_to_any(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr)>(
                    17470269400781477793u64,
                    self.database_root.join("relations/thir_exprs_never_to_any"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_never_to_any(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr)>> {
            if self.thir_exprs_never_to_any.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr)>(
                        17470269400781477793u64,
                        self.database_root.join("relations/thir_exprs_never_to_any"),
                    )
                }
                .unwrap();
                *self.thir_exprs_never_to_any.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_never_to_any.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_never_to_any(&self, facts: Vec<(ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    17470269400781477793u64,
                    self.database_root.join("relations/thir_exprs_never_to_any"),
                );
            }
        }
        pub fn store_iter_thir_exprs_never_to_any(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    17470269400781477793u64,
                    self.database_root.join("relations/thir_exprs_never_to_any"),
                );
            }
        }
        pub fn load_thir_exprs_never_to_any_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, ThirExpr> {
            self.load_thir_exprs_never_to_any()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_thir_exprs_pointer_coercion(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, PointerCoercion, ThirExpr, bool)> {
            unsafe {
                load_elts_relation::<(ThirExpr, PointerCoercion, ThirExpr, bool)>(
                    6101599652373463408u64,
                    self.database_root
                        .join("relations/thir_exprs_pointer_coercion"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_pointer_coercion(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, PointerCoercion, ThirExpr, bool)>> {
            if self.thir_exprs_pointer_coercion.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, PointerCoercion, ThirExpr, bool)>(
                        6101599652373463408u64,
                        self.database_root
                            .join("relations/thir_exprs_pointer_coercion"),
                    )
                }
                .unwrap();
                *self.thir_exprs_pointer_coercion.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_pointer_coercion.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_pointer_coercion(
            &self,
            facts: Vec<(ThirExpr, PointerCoercion, ThirExpr, bool)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, PointerCoercion, ThirExpr, bool)>(
                    facts,
                    6101599652373463408u64,
                    self.database_root
                        .join("relations/thir_exprs_pointer_coercion"),
                );
            }
        }
        pub fn store_iter_thir_exprs_pointer_coercion(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, PointerCoercion, ThirExpr, bool)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, PointerCoercion, ThirExpr, bool)>(
                    facts,
                    6101599652373463408u64,
                    self.database_root
                        .join("relations/thir_exprs_pointer_coercion"),
                );
            }
        }
        pub fn load_iter_thir_exprs_loop(&self) -> impl Iterator<Item = (ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr)>(
                    3098191845330208770u64,
                    self.database_root.join("relations/thir_exprs_loop"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_loop(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr)>> {
            if self.thir_exprs_loop.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr)>(
                        3098191845330208770u64,
                        self.database_root.join("relations/thir_exprs_loop"),
                    )
                }
                .unwrap();
                *self.thir_exprs_loop.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_loop.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_loop(&self, facts: Vec<(ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    3098191845330208770u64,
                    self.database_root.join("relations/thir_exprs_loop"),
                );
            }
        }
        pub fn store_iter_thir_exprs_loop(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    3098191845330208770u64,
                    self.database_root.join("relations/thir_exprs_loop"),
                );
            }
        }
        pub fn load_thir_exprs_loop_as_map(&self) -> std::collections::HashMap<ThirExpr, ThirExpr> {
            self.load_thir_exprs_loop().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_let(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirExpr, ThirPat)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr, ThirPat)>(
                    9632315457169436561u64,
                    self.database_root.join("relations/thir_exprs_let"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_let(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr, ThirPat)>> {
            if self.thir_exprs_let.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr, ThirPat)>(
                        9632315457169436561u64,
                        self.database_root.join("relations/thir_exprs_let"),
                    )
                }
                .unwrap();
                *self.thir_exprs_let.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_let.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_let(&self, facts: Vec<(ThirExpr, ThirExpr, ThirPat)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, ThirPat)>(
                    facts,
                    9632315457169436561u64,
                    self.database_root.join("relations/thir_exprs_let"),
                );
            }
        }
        pub fn store_iter_thir_exprs_let(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr, ThirPat)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, ThirPat)>(
                    facts,
                    9632315457169436561u64,
                    self.database_root.join("relations/thir_exprs_let"),
                );
            }
        }
        pub fn load_iter_thir_pats(&self) -> impl Iterator<Item = (ThirPat, Type, Span)> {
            unsafe {
                load_elts_relation::<(ThirPat, Type, Span)>(
                    10536163954959029661u64,
                    self.database_root.join("relations/thir_pats"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_pats(&self) -> std::cell::Ref<Vec<(ThirPat, Type, Span)>> {
            if self.thir_pats.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirPat, Type, Span)>(
                        10536163954959029661u64,
                        self.database_root.join("relations/thir_pats"),
                    )
                }
                .unwrap();
                *self.thir_pats.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_pats.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_thir_pats(&self, facts: Vec<(ThirPat, Type, Span)>) {
            unsafe {
                save_elts_relation::<(ThirPat, Type, Span)>(
                    facts,
                    10536163954959029661u64,
                    self.database_root.join("relations/thir_pats"),
                );
            }
        }
        pub fn store_iter_thir_pats(&self, facts: impl IntoIterator<Item = (ThirPat, Type, Span)>) {
            unsafe {
                save_elts_relation::<(ThirPat, Type, Span)>(
                    facts,
                    10536163954959029661u64,
                    self.database_root.join("relations/thir_pats"),
                );
            }
        }
        pub fn load_iter_thir_exprs_match(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirExpr, MatchSource)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr, MatchSource)>(
                    5226457329152460944u64,
                    self.database_root.join("relations/thir_exprs_match"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_match(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr, MatchSource)>> {
            if self.thir_exprs_match.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr, MatchSource)>(
                        5226457329152460944u64,
                        self.database_root.join("relations/thir_exprs_match"),
                    )
                }
                .unwrap();
                *self.thir_exprs_match.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_match.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_match(&self, facts: Vec<(ThirExpr, ThirExpr, MatchSource)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, MatchSource)>(
                    facts,
                    5226457329152460944u64,
                    self.database_root.join("relations/thir_exprs_match"),
                );
            }
        }
        pub fn store_iter_thir_exprs_match(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr, MatchSource)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, MatchSource)>(
                    facts,
                    5226457329152460944u64,
                    self.database_root.join("relations/thir_exprs_match"),
                );
            }
        }
        pub fn load_iter_thir_match_arms(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, MatchArmIdx, ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, MatchArmIdx, ThirExpr, ThirExpr)>(
                    9822157818036781677u64,
                    self.database_root.join("relations/thir_match_arms"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_match_arms(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, MatchArmIdx, ThirExpr, ThirExpr)>> {
            if self.thir_match_arms.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, MatchArmIdx, ThirExpr, ThirExpr)>(
                        9822157818036781677u64,
                        self.database_root.join("relations/thir_match_arms"),
                    )
                }
                .unwrap();
                *self.thir_match_arms.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_match_arms.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_match_arms(
            &self,
            facts: Vec<(ThirExpr, MatchArmIdx, ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, MatchArmIdx, ThirExpr, ThirExpr)>(
                    facts,
                    9822157818036781677u64,
                    self.database_root.join("relations/thir_match_arms"),
                );
            }
        }
        pub fn store_iter_thir_match_arms(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, MatchArmIdx, ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, MatchArmIdx, ThirExpr, ThirExpr)>(
                    facts,
                    9822157818036781677u64,
                    self.database_root.join("relations/thir_match_arms"),
                );
            }
        }
        pub fn load_iter_thir_exprs_block(&self) -> impl Iterator<Item = (ThirExpr, ThirBlock)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirBlock)>(
                    15876734076667292329u64,
                    self.database_root.join("relations/thir_exprs_block"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_block(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirBlock)>> {
            if self.thir_exprs_block.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirBlock)>(
                        15876734076667292329u64,
                        self.database_root.join("relations/thir_exprs_block"),
                    )
                }
                .unwrap();
                *self.thir_exprs_block.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_block.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_block(&self, facts: Vec<(ThirExpr, ThirBlock)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirBlock)>(
                    facts,
                    15876734076667292329u64,
                    self.database_root.join("relations/thir_exprs_block"),
                );
            }
        }
        pub fn store_iter_thir_exprs_block(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirBlock)>(
                    facts,
                    15876734076667292329u64,
                    self.database_root.join("relations/thir_exprs_block"),
                );
            }
        }
        pub fn load_thir_exprs_block_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, ThirBlock> {
            self.load_thir_exprs_block().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_assign(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr, ThirExpr)>(
                    15488937819083875309u64,
                    self.database_root.join("relations/thir_exprs_assign"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_assign(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr, ThirExpr)>> {
            if self.thir_exprs_assign.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr, ThirExpr)>(
                        15488937819083875309u64,
                        self.database_root.join("relations/thir_exprs_assign"),
                    )
                }
                .unwrap();
                *self.thir_exprs_assign.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_assign.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_assign(&self, facts: Vec<(ThirExpr, ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, ThirExpr)>(
                    facts,
                    15488937819083875309u64,
                    self.database_root.join("relations/thir_exprs_assign"),
                );
            }
        }
        pub fn store_iter_thir_exprs_assign(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, ThirExpr)>(
                    facts,
                    15488937819083875309u64,
                    self.database_root.join("relations/thir_exprs_assign"),
                );
            }
        }
        pub fn load_iter_thir_exprs_assign_op(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirBinOp, ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>(
                    4838645763299739861u64,
                    self.database_root.join("relations/thir_exprs_assign_op"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_assign_op(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>> {
            if self.thir_exprs_assign_op.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>(
                        4838645763299739861u64,
                        self.database_root.join("relations/thir_exprs_assign_op"),
                    )
                }
                .unwrap();
                *self.thir_exprs_assign_op.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_assign_op.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_assign_op(
            &self,
            facts: Vec<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>(
                    facts,
                    4838645763299739861u64,
                    self.database_root.join("relations/thir_exprs_assign_op"),
                );
            }
        }
        pub fn store_iter_thir_exprs_assign_op(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirBinOp, ThirExpr, ThirExpr)>(
                    facts,
                    4838645763299739861u64,
                    self.database_root.join("relations/thir_exprs_assign_op"),
                );
            }
        }
        pub fn load_iter_thir_exprs_field(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirExpr, AdtVariantIndex)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr, AdtVariantIndex)>(
                    23925721899557405u64,
                    self.database_root.join("relations/thir_exprs_field"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_field(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr, AdtVariantIndex)>> {
            if self.thir_exprs_field.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr, AdtVariantIndex)>(
                        23925721899557405u64,
                        self.database_root.join("relations/thir_exprs_field"),
                    )
                }
                .unwrap();
                *self.thir_exprs_field.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_field.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_field(&self, facts: Vec<(ThirExpr, ThirExpr, AdtVariantIndex)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, AdtVariantIndex)>(
                    facts,
                    23925721899557405u64,
                    self.database_root.join("relations/thir_exprs_field"),
                );
            }
        }
        pub fn store_iter_thir_exprs_field(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr, AdtVariantIndex)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, AdtVariantIndex)>(
                    facts,
                    23925721899557405u64,
                    self.database_root.join("relations/thir_exprs_field"),
                );
            }
        }
        pub fn load_iter_thir_exprs_index(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr, ThirExpr)>(
                    5832191315312159083u64,
                    self.database_root.join("relations/thir_exprs_index"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_index(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr, ThirExpr)>> {
            if self.thir_exprs_index.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr, ThirExpr)>(
                        5832191315312159083u64,
                        self.database_root.join("relations/thir_exprs_index"),
                    )
                }
                .unwrap();
                *self.thir_exprs_index.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_index.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_index(&self, facts: Vec<(ThirExpr, ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, ThirExpr)>(
                    facts,
                    5832191315312159083u64,
                    self.database_root.join("relations/thir_exprs_index"),
                );
            }
        }
        pub fn store_iter_thir_exprs_index(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, ThirExpr)>(
                    facts,
                    5832191315312159083u64,
                    self.database_root.join("relations/thir_exprs_index"),
                );
            }
        }
        pub fn load_iter_thir_exprs_var_ref(&self) -> impl Iterator<Item = (ThirExpr,)> {
            unsafe {
                load_elts_relation::<(ThirExpr,)>(
                    296084846650957428u64,
                    self.database_root.join("relations/thir_exprs_var_ref"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_var_ref(&self) -> std::cell::Ref<Vec<(ThirExpr,)>> {
            if self.thir_exprs_var_ref.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr,)>(
                        296084846650957428u64,
                        self.database_root.join("relations/thir_exprs_var_ref"),
                    )
                }
                .unwrap();
                *self.thir_exprs_var_ref.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_var_ref.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_var_ref(&self, facts: Vec<(ThirExpr,)>) {
            unsafe {
                save_elts_relation::<(ThirExpr,)>(
                    facts,
                    296084846650957428u64,
                    self.database_root.join("relations/thir_exprs_var_ref"),
                );
            }
        }
        pub fn store_iter_thir_exprs_var_ref(&self, facts: impl IntoIterator<Item = (ThirExpr,)>) {
            unsafe {
                save_elts_relation::<(ThirExpr,)>(
                    facts,
                    296084846650957428u64,
                    self.database_root.join("relations/thir_exprs_var_ref"),
                );
            }
        }
        pub fn load_iter_thir_exprs_upvar_ref(&self) -> impl Iterator<Item = (ThirExpr, DefPath)> {
            unsafe {
                load_elts_relation::<(ThirExpr, DefPath)>(
                    2787850572062464717u64,
                    self.database_root.join("relations/thir_exprs_upvar_ref"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_upvar_ref(&self) -> std::cell::Ref<Vec<(ThirExpr, DefPath)>> {
            if self.thir_exprs_upvar_ref.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, DefPath)>(
                        2787850572062464717u64,
                        self.database_root.join("relations/thir_exprs_upvar_ref"),
                    )
                }
                .unwrap();
                *self.thir_exprs_upvar_ref.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_upvar_ref.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_upvar_ref(&self, facts: Vec<(ThirExpr, DefPath)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath)>(
                    facts,
                    2787850572062464717u64,
                    self.database_root.join("relations/thir_exprs_upvar_ref"),
                );
            }
        }
        pub fn store_iter_thir_exprs_upvar_ref(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath)>(
                    facts,
                    2787850572062464717u64,
                    self.database_root.join("relations/thir_exprs_upvar_ref"),
                );
            }
        }
        pub fn load_thir_exprs_upvar_ref_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, DefPath> {
            self.load_thir_exprs_upvar_ref().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_borrow(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, BorrowKind, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, BorrowKind, ThirExpr)>(
                    6349143721208772268u64,
                    self.database_root.join("relations/thir_exprs_borrow"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_borrow(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, BorrowKind, ThirExpr)>> {
            if self.thir_exprs_borrow.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, BorrowKind, ThirExpr)>(
                        6349143721208772268u64,
                        self.database_root.join("relations/thir_exprs_borrow"),
                    )
                }
                .unwrap();
                *self.thir_exprs_borrow.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_borrow.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_borrow(&self, facts: Vec<(ThirExpr, BorrowKind, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, BorrowKind, ThirExpr)>(
                    facts,
                    6349143721208772268u64,
                    self.database_root.join("relations/thir_exprs_borrow"),
                );
            }
        }
        pub fn store_iter_thir_exprs_borrow(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, BorrowKind, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, BorrowKind, ThirExpr)>(
                    facts,
                    6349143721208772268u64,
                    self.database_root.join("relations/thir_exprs_borrow"),
                );
            }
        }
        pub fn load_iter_thir_exprs_raw_borrow(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, Mutability, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, Mutability, ThirExpr)>(
                    13059666409505041503u64,
                    self.database_root.join("relations/thir_exprs_raw_borrow"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_raw_borrow(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, Mutability, ThirExpr)>> {
            if self.thir_exprs_raw_borrow.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, Mutability, ThirExpr)>(
                        13059666409505041503u64,
                        self.database_root.join("relations/thir_exprs_raw_borrow"),
                    )
                }
                .unwrap();
                *self.thir_exprs_raw_borrow.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_raw_borrow.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_raw_borrow(&self, facts: Vec<(ThirExpr, Mutability, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, Mutability, ThirExpr)>(
                    facts,
                    13059666409505041503u64,
                    self.database_root.join("relations/thir_exprs_raw_borrow"),
                );
            }
        }
        pub fn store_iter_thir_exprs_raw_borrow(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, Mutability, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, Mutability, ThirExpr)>(
                    facts,
                    13059666409505041503u64,
                    self.database_root.join("relations/thir_exprs_raw_borrow"),
                );
            }
        }
        pub fn load_iter_thir_exprs_break(&self) -> impl Iterator<Item = (ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr)>(
                    6740441886473887693u64,
                    self.database_root.join("relations/thir_exprs_break"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_break(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr)>> {
            if self.thir_exprs_break.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr)>(
                        6740441886473887693u64,
                        self.database_root.join("relations/thir_exprs_break"),
                    )
                }
                .unwrap();
                *self.thir_exprs_break.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_break.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_break(&self, facts: Vec<(ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    6740441886473887693u64,
                    self.database_root.join("relations/thir_exprs_break"),
                );
            }
        }
        pub fn store_iter_thir_exprs_break(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    6740441886473887693u64,
                    self.database_root.join("relations/thir_exprs_break"),
                );
            }
        }
        pub fn load_thir_exprs_break_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, ThirExpr> {
            self.load_thir_exprs_break().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_continue(&self) -> impl Iterator<Item = (ThirExpr,)> {
            unsafe {
                load_elts_relation::<(ThirExpr,)>(
                    12830730509208436793u64,
                    self.database_root.join("relations/thir_exprs_continue"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_continue(&self) -> std::cell::Ref<Vec<(ThirExpr,)>> {
            if self.thir_exprs_continue.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr,)>(
                        12830730509208436793u64,
                        self.database_root.join("relations/thir_exprs_continue"),
                    )
                }
                .unwrap();
                *self.thir_exprs_continue.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_continue.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_continue(&self, facts: Vec<(ThirExpr,)>) {
            unsafe {
                save_elts_relation::<(ThirExpr,)>(
                    facts,
                    12830730509208436793u64,
                    self.database_root.join("relations/thir_exprs_continue"),
                );
            }
        }
        pub fn store_iter_thir_exprs_continue(&self, facts: impl IntoIterator<Item = (ThirExpr,)>) {
            unsafe {
                save_elts_relation::<(ThirExpr,)>(
                    facts,
                    12830730509208436793u64,
                    self.database_root.join("relations/thir_exprs_continue"),
                );
            }
        }
        pub fn load_iter_thir_exprs_return(&self) -> impl Iterator<Item = (ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr)>(
                    17654622868380585299u64,
                    self.database_root.join("relations/thir_exprs_return"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_return(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr)>> {
            if self.thir_exprs_return.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr)>(
                        17654622868380585299u64,
                        self.database_root.join("relations/thir_exprs_return"),
                    )
                }
                .unwrap();
                *self.thir_exprs_return.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_return.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_return(&self, facts: Vec<(ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    17654622868380585299u64,
                    self.database_root.join("relations/thir_exprs_return"),
                );
            }
        }
        pub fn store_iter_thir_exprs_return(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    17654622868380585299u64,
                    self.database_root.join("relations/thir_exprs_return"),
                );
            }
        }
        pub fn load_thir_exprs_return_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, ThirExpr> {
            self.load_thir_exprs_return().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_become(&self) -> impl Iterator<Item = (ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr)>(
                    16769260330021479451u64,
                    self.database_root.join("relations/thir_exprs_become"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_become(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr)>> {
            if self.thir_exprs_become.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr)>(
                        16769260330021479451u64,
                        self.database_root.join("relations/thir_exprs_become"),
                    )
                }
                .unwrap();
                *self.thir_exprs_become.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_become.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_become(&self, facts: Vec<(ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    16769260330021479451u64,
                    self.database_root.join("relations/thir_exprs_become"),
                );
            }
        }
        pub fn store_iter_thir_exprs_become(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    16769260330021479451u64,
                    self.database_root.join("relations/thir_exprs_become"),
                );
            }
        }
        pub fn load_thir_exprs_become_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, ThirExpr> {
            self.load_thir_exprs_become().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_const_block(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, DefPath)> {
            unsafe {
                load_elts_relation::<(ThirExpr, DefPath)>(
                    12642227791060480364u64,
                    self.database_root.join("relations/thir_exprs_const_block"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_const_block(&self) -> std::cell::Ref<Vec<(ThirExpr, DefPath)>> {
            if self.thir_exprs_const_block.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, DefPath)>(
                        12642227791060480364u64,
                        self.database_root.join("relations/thir_exprs_const_block"),
                    )
                }
                .unwrap();
                *self.thir_exprs_const_block.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_const_block.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_const_block(&self, facts: Vec<(ThirExpr, DefPath)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath)>(
                    facts,
                    12642227791060480364u64,
                    self.database_root.join("relations/thir_exprs_const_block"),
                );
            }
        }
        pub fn store_iter_thir_exprs_const_block(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath)>(
                    facts,
                    12642227791060480364u64,
                    self.database_root.join("relations/thir_exprs_const_block"),
                );
            }
        }
        pub fn load_thir_exprs_const_block_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, DefPath> {
            self.load_thir_exprs_const_block().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_repeat(&self) -> impl Iterator<Item = (ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr)>(
                    2338369464665128106u64,
                    self.database_root.join("relations/thir_exprs_repeat"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_repeat(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr)>> {
            if self.thir_exprs_repeat.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr)>(
                        2338369464665128106u64,
                        self.database_root.join("relations/thir_exprs_repeat"),
                    )
                }
                .unwrap();
                *self.thir_exprs_repeat.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_repeat.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_repeat(&self, facts: Vec<(ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    2338369464665128106u64,
                    self.database_root.join("relations/thir_exprs_repeat"),
                );
            }
        }
        pub fn store_iter_thir_exprs_repeat(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    2338369464665128106u64,
                    self.database_root.join("relations/thir_exprs_repeat"),
                );
            }
        }
        pub fn load_thir_exprs_repeat_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, ThirExpr> {
            self.load_thir_exprs_repeat().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_array(&self) -> impl Iterator<Item = (ThirExpr,)> {
            unsafe {
                load_elts_relation::<(ThirExpr,)>(
                    1124144228664747403u64,
                    self.database_root.join("relations/thir_exprs_array"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_array(&self) -> std::cell::Ref<Vec<(ThirExpr,)>> {
            if self.thir_exprs_array.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr,)>(
                        1124144228664747403u64,
                        self.database_root.join("relations/thir_exprs_array"),
                    )
                }
                .unwrap();
                *self.thir_exprs_array.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_array.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_array(&self, facts: Vec<(ThirExpr,)>) {
            unsafe {
                save_elts_relation::<(ThirExpr,)>(
                    facts,
                    1124144228664747403u64,
                    self.database_root.join("relations/thir_exprs_array"),
                );
            }
        }
        pub fn store_iter_thir_exprs_array(&self, facts: impl IntoIterator<Item = (ThirExpr,)>) {
            unsafe {
                save_elts_relation::<(ThirExpr,)>(
                    facts,
                    1124144228664747403u64,
                    self.database_root.join("relations/thir_exprs_array"),
                );
            }
        }
        pub fn load_iter_thir_array_elements(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, u64, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, u64, ThirExpr)>(
                    10250490187218554703u64,
                    self.database_root.join("relations/thir_array_elements"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_array_elements(&self) -> std::cell::Ref<Vec<(ThirExpr, u64, ThirExpr)>> {
            if self.thir_array_elements.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, u64, ThirExpr)>(
                        10250490187218554703u64,
                        self.database_root.join("relations/thir_array_elements"),
                    )
                }
                .unwrap();
                *self.thir_array_elements.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_array_elements.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_array_elements(&self, facts: Vec<(ThirExpr, u64, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, u64, ThirExpr)>(
                    facts,
                    10250490187218554703u64,
                    self.database_root.join("relations/thir_array_elements"),
                );
            }
        }
        pub fn store_iter_thir_array_elements(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, u64, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, u64, ThirExpr)>(
                    facts,
                    10250490187218554703u64,
                    self.database_root.join("relations/thir_array_elements"),
                );
            }
        }
        pub fn load_iter_thir_exprs_tuple(&self) -> impl Iterator<Item = (ThirExpr,)> {
            unsafe {
                load_elts_relation::<(ThirExpr,)>(
                    15653329952619259353u64,
                    self.database_root.join("relations/thir_exprs_tuple"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_tuple(&self) -> std::cell::Ref<Vec<(ThirExpr,)>> {
            if self.thir_exprs_tuple.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr,)>(
                        15653329952619259353u64,
                        self.database_root.join("relations/thir_exprs_tuple"),
                    )
                }
                .unwrap();
                *self.thir_exprs_tuple.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_tuple.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_tuple(&self, facts: Vec<(ThirExpr,)>) {
            unsafe {
                save_elts_relation::<(ThirExpr,)>(
                    facts,
                    15653329952619259353u64,
                    self.database_root.join("relations/thir_exprs_tuple"),
                );
            }
        }
        pub fn store_iter_thir_exprs_tuple(&self, facts: impl IntoIterator<Item = (ThirExpr,)>) {
            unsafe {
                save_elts_relation::<(ThirExpr,)>(
                    facts,
                    15653329952619259353u64,
                    self.database_root.join("relations/thir_exprs_tuple"),
                );
            }
        }
        pub fn load_iter_thir_tuple_elements(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, TupleFieldIndex, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, TupleFieldIndex, ThirExpr)>(
                    7005504983456194338u64,
                    self.database_root.join("relations/thir_tuple_elements"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_tuple_elements(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, TupleFieldIndex, ThirExpr)>> {
            if self.thir_tuple_elements.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, TupleFieldIndex, ThirExpr)>(
                        7005504983456194338u64,
                        self.database_root.join("relations/thir_tuple_elements"),
                    )
                }
                .unwrap();
                *self.thir_tuple_elements.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_tuple_elements.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_tuple_elements(&self, facts: Vec<(ThirExpr, TupleFieldIndex, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, TupleFieldIndex, ThirExpr)>(
                    facts,
                    7005504983456194338u64,
                    self.database_root.join("relations/thir_tuple_elements"),
                );
            }
        }
        pub fn store_iter_thir_tuple_elements(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, TupleFieldIndex, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, TupleFieldIndex, ThirExpr)>(
                    facts,
                    7005504983456194338u64,
                    self.database_root.join("relations/thir_tuple_elements"),
                );
            }
        }
        pub fn load_iter_thir_exprs_adt(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirExpr, AdtVariantIndex)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr, AdtVariantIndex)>(
                    13642234560780341906u64,
                    self.database_root.join("relations/thir_exprs_adt"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_adt(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr, AdtVariantIndex)>> {
            if self.thir_exprs_adt.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr, AdtVariantIndex)>(
                        13642234560780341906u64,
                        self.database_root.join("relations/thir_exprs_adt"),
                    )
                }
                .unwrap();
                *self.thir_exprs_adt.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_adt.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_adt(&self, facts: Vec<(ThirExpr, ThirExpr, AdtVariantIndex)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, AdtVariantIndex)>(
                    facts,
                    13642234560780341906u64,
                    self.database_root.join("relations/thir_exprs_adt"),
                );
            }
        }
        pub fn store_iter_thir_exprs_adt(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr, AdtVariantIndex)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, AdtVariantIndex)>(
                    facts,
                    13642234560780341906u64,
                    self.database_root.join("relations/thir_exprs_adt"),
                );
            }
        }
        pub fn load_iter_thir_adt_field_expr(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, FieldIndex, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, FieldIndex, ThirExpr)>(
                    14683105706445521333u64,
                    self.database_root.join("relations/thir_adt_field_expr"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_adt_field_expr(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, FieldIndex, ThirExpr)>> {
            if self.thir_adt_field_expr.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, FieldIndex, ThirExpr)>(
                        14683105706445521333u64,
                        self.database_root.join("relations/thir_adt_field_expr"),
                    )
                }
                .unwrap();
                *self.thir_adt_field_expr.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_adt_field_expr.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_adt_field_expr(&self, facts: Vec<(ThirExpr, FieldIndex, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, FieldIndex, ThirExpr)>(
                    facts,
                    14683105706445521333u64,
                    self.database_root.join("relations/thir_adt_field_expr"),
                );
            }
        }
        pub fn store_iter_thir_adt_field_expr(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, FieldIndex, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, FieldIndex, ThirExpr)>(
                    facts,
                    14683105706445521333u64,
                    self.database_root.join("relations/thir_adt_field_expr"),
                );
            }
        }
        pub fn load_iter_thir_exprs_place_type_ascription(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirExpr, Span)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr, Span)>(
                    10691137482463784180u64,
                    self.database_root
                        .join("relations/thir_exprs_place_type_ascription"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_place_type_ascription(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr, Span)>> {
            if self.thir_exprs_place_type_ascription.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr, Span)>(
                        10691137482463784180u64,
                        self.database_root
                            .join("relations/thir_exprs_place_type_ascription"),
                    )
                }
                .unwrap();
                *self.thir_exprs_place_type_ascription.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_place_type_ascription.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_place_type_ascription(
            &self,
            facts: Vec<(ThirExpr, ThirExpr, Span)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, Span)>(
                    facts,
                    10691137482463784180u64,
                    self.database_root
                        .join("relations/thir_exprs_place_type_ascription"),
                );
            }
        }
        pub fn store_iter_thir_exprs_place_type_ascription(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr, Span)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, Span)>(
                    facts,
                    10691137482463784180u64,
                    self.database_root
                        .join("relations/thir_exprs_place_type_ascription"),
                );
            }
        }
        pub fn load_iter_thir_exprs_value_type_ascription(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, ThirExpr, Span)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr, Span)>(
                    5893459425189182978u64,
                    self.database_root
                        .join("relations/thir_exprs_value_type_ascription"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_value_type_ascription(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr, Span)>> {
            if self.thir_exprs_value_type_ascription.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr, Span)>(
                        5893459425189182978u64,
                        self.database_root
                            .join("relations/thir_exprs_value_type_ascription"),
                    )
                }
                .unwrap();
                *self.thir_exprs_value_type_ascription.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_value_type_ascription.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_value_type_ascription(
            &self,
            facts: Vec<(ThirExpr, ThirExpr, Span)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, Span)>(
                    facts,
                    5893459425189182978u64,
                    self.database_root
                        .join("relations/thir_exprs_value_type_ascription"),
                );
            }
        }
        pub fn store_iter_thir_exprs_value_type_ascription(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr, Span)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr, Span)>(
                    facts,
                    5893459425189182978u64,
                    self.database_root
                        .join("relations/thir_exprs_value_type_ascription"),
                );
            }
        }
        pub fn load_iter_thir_exprs_closure(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, DefPath, Movability)> {
            unsafe {
                load_elts_relation::<(ThirExpr, DefPath, Movability)>(
                    11883660099806632522u64,
                    self.database_root.join("relations/thir_exprs_closure"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_closure(
            &self,
        ) -> std::cell::Ref<Vec<(ThirExpr, DefPath, Movability)>> {
            if self.thir_exprs_closure.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, DefPath, Movability)>(
                        11883660099806632522u64,
                        self.database_root.join("relations/thir_exprs_closure"),
                    )
                }
                .unwrap();
                *self.thir_exprs_closure.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_closure.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_closure(&self, facts: Vec<(ThirExpr, DefPath, Movability)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath, Movability)>(
                    facts,
                    11883660099806632522u64,
                    self.database_root.join("relations/thir_exprs_closure"),
                );
            }
        }
        pub fn store_iter_thir_exprs_closure(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, DefPath, Movability)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath, Movability)>(
                    facts,
                    11883660099806632522u64,
                    self.database_root.join("relations/thir_exprs_closure"),
                );
            }
        }
        pub fn load_iter_thir_closure_upvars(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, u32, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, u32, ThirExpr)>(
                    15546498482309699540u64,
                    self.database_root.join("relations/thir_closure_upvars"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_closure_upvars(&self) -> std::cell::Ref<Vec<(ThirExpr, u32, ThirExpr)>> {
            if self.thir_closure_upvars.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, u32, ThirExpr)>(
                        15546498482309699540u64,
                        self.database_root.join("relations/thir_closure_upvars"),
                    )
                }
                .unwrap();
                *self.thir_closure_upvars.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_closure_upvars.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_closure_upvars(&self, facts: Vec<(ThirExpr, u32, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, u32, ThirExpr)>(
                    facts,
                    15546498482309699540u64,
                    self.database_root.join("relations/thir_closure_upvars"),
                );
            }
        }
        pub fn store_iter_thir_closure_upvars(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, u32, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, u32, ThirExpr)>(
                    facts,
                    15546498482309699540u64,
                    self.database_root.join("relations/thir_closure_upvars"),
                );
            }
        }
        pub fn load_iter_thir_exprs_literal(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, LitKind, bool)> {
            unsafe {
                load_elts_relation::<(ThirExpr, LitKind, bool)>(
                    17207401787417204123u64,
                    self.database_root.join("relations/thir_exprs_literal"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_literal(&self) -> std::cell::Ref<Vec<(ThirExpr, LitKind, bool)>> {
            if self.thir_exprs_literal.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, LitKind, bool)>(
                        17207401787417204123u64,
                        self.database_root.join("relations/thir_exprs_literal"),
                    )
                }
                .unwrap();
                *self.thir_exprs_literal.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_literal.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_literal(&self, facts: Vec<(ThirExpr, LitKind, bool)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, LitKind, bool)>(
                    facts,
                    17207401787417204123u64,
                    self.database_root.join("relations/thir_exprs_literal"),
                );
            }
        }
        pub fn store_iter_thir_exprs_literal(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, LitKind, bool)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, LitKind, bool)>(
                    facts,
                    17207401787417204123u64,
                    self.database_root.join("relations/thir_exprs_literal"),
                );
            }
        }
        pub fn load_iter_thir_exprs_non_hir_literal(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, u128)> {
            unsafe {
                load_elts_relation::<(ThirExpr, u128)>(
                    4124325780433447890u64,
                    self.database_root
                        .join("relations/thir_exprs_non_hir_literal"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_non_hir_literal(&self) -> std::cell::Ref<Vec<(ThirExpr, u128)>> {
            if self.thir_exprs_non_hir_literal.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, u128)>(
                        4124325780433447890u64,
                        self.database_root
                            .join("relations/thir_exprs_non_hir_literal"),
                    )
                }
                .unwrap();
                *self.thir_exprs_non_hir_literal.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_non_hir_literal.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_non_hir_literal(&self, facts: Vec<(ThirExpr, u128)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, u128)>(
                    facts,
                    4124325780433447890u64,
                    self.database_root
                        .join("relations/thir_exprs_non_hir_literal"),
                );
            }
        }
        pub fn store_iter_thir_exprs_non_hir_literal(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, u128)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, u128)>(
                    facts,
                    4124325780433447890u64,
                    self.database_root
                        .join("relations/thir_exprs_non_hir_literal"),
                );
            }
        }
        pub fn load_thir_exprs_non_hir_literal_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, u128> {
            self.load_thir_exprs_non_hir_literal()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_thir_exprs_zst_literal(&self) -> impl Iterator<Item = (ThirExpr,)> {
            unsafe {
                load_elts_relation::<(ThirExpr,)>(
                    11698888599584136937u64,
                    self.database_root.join("relations/thir_exprs_zst_literal"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_zst_literal(&self) -> std::cell::Ref<Vec<(ThirExpr,)>> {
            if self.thir_exprs_zst_literal.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr,)>(
                        11698888599584136937u64,
                        self.database_root.join("relations/thir_exprs_zst_literal"),
                    )
                }
                .unwrap();
                *self.thir_exprs_zst_literal.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_zst_literal.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_zst_literal(&self, facts: Vec<(ThirExpr,)>) {
            unsafe {
                save_elts_relation::<(ThirExpr,)>(
                    facts,
                    11698888599584136937u64,
                    self.database_root.join("relations/thir_exprs_zst_literal"),
                );
            }
        }
        pub fn store_iter_thir_exprs_zst_literal(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr,)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr,)>(
                    facts,
                    11698888599584136937u64,
                    self.database_root.join("relations/thir_exprs_zst_literal"),
                );
            }
        }
        pub fn load_iter_thir_exprs_named_const(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, DefPath)> {
            unsafe {
                load_elts_relation::<(ThirExpr, DefPath)>(
                    12988359671579403367u64,
                    self.database_root.join("relations/thir_exprs_named_const"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_named_const(&self) -> std::cell::Ref<Vec<(ThirExpr, DefPath)>> {
            if self.thir_exprs_named_const.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, DefPath)>(
                        12988359671579403367u64,
                        self.database_root.join("relations/thir_exprs_named_const"),
                    )
                }
                .unwrap();
                *self.thir_exprs_named_const.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_named_const.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_named_const(&self, facts: Vec<(ThirExpr, DefPath)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath)>(
                    facts,
                    12988359671579403367u64,
                    self.database_root.join("relations/thir_exprs_named_const"),
                );
            }
        }
        pub fn store_iter_thir_exprs_named_const(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath)>(
                    facts,
                    12988359671579403367u64,
                    self.database_root.join("relations/thir_exprs_named_const"),
                );
            }
        }
        pub fn load_thir_exprs_named_const_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, DefPath> {
            self.load_thir_exprs_named_const().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_const_param(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, DefPath)> {
            unsafe {
                load_elts_relation::<(ThirExpr, DefPath)>(
                    4296964196940540570u64,
                    self.database_root.join("relations/thir_exprs_const_param"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_const_param(&self) -> std::cell::Ref<Vec<(ThirExpr, DefPath)>> {
            if self.thir_exprs_const_param.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, DefPath)>(
                        4296964196940540570u64,
                        self.database_root.join("relations/thir_exprs_const_param"),
                    )
                }
                .unwrap();
                *self.thir_exprs_const_param.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_const_param.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_const_param(&self, facts: Vec<(ThirExpr, DefPath)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath)>(
                    facts,
                    4296964196940540570u64,
                    self.database_root.join("relations/thir_exprs_const_param"),
                );
            }
        }
        pub fn store_iter_thir_exprs_const_param(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath)>(
                    facts,
                    4296964196940540570u64,
                    self.database_root.join("relations/thir_exprs_const_param"),
                );
            }
        }
        pub fn load_thir_exprs_const_param_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, DefPath> {
            self.load_thir_exprs_const_param().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_static_ref(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, Type, DefPath)> {
            unsafe {
                load_elts_relation::<(ThirExpr, Type, DefPath)>(
                    11186498711369745685u64,
                    self.database_root.join("relations/thir_exprs_static_ref"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_static_ref(&self) -> std::cell::Ref<Vec<(ThirExpr, Type, DefPath)>> {
            if self.thir_exprs_static_ref.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, Type, DefPath)>(
                        11186498711369745685u64,
                        self.database_root.join("relations/thir_exprs_static_ref"),
                    )
                }
                .unwrap();
                *self.thir_exprs_static_ref.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_static_ref.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_static_ref(&self, facts: Vec<(ThirExpr, Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, Type, DefPath)>(
                    facts,
                    11186498711369745685u64,
                    self.database_root.join("relations/thir_exprs_static_ref"),
                );
            }
        }
        pub fn store_iter_thir_exprs_static_ref(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, Type, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, Type, DefPath)>(
                    facts,
                    11186498711369745685u64,
                    self.database_root.join("relations/thir_exprs_static_ref"),
                );
            }
        }
        pub fn load_iter_thir_exprs_inline_asm(&self) -> impl Iterator<Item = (ThirExpr,)> {
            unsafe {
                load_elts_relation::<(ThirExpr,)>(
                    17045657056275096603u64,
                    self.database_root.join("relations/thir_exprs_inline_asm"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_inline_asm(&self) -> std::cell::Ref<Vec<(ThirExpr,)>> {
            if self.thir_exprs_inline_asm.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr,)>(
                        17045657056275096603u64,
                        self.database_root.join("relations/thir_exprs_inline_asm"),
                    )
                }
                .unwrap();
                *self.thir_exprs_inline_asm.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_inline_asm.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_inline_asm(&self, facts: Vec<(ThirExpr,)>) {
            unsafe {
                save_elts_relation::<(ThirExpr,)>(
                    facts,
                    17045657056275096603u64,
                    self.database_root.join("relations/thir_exprs_inline_asm"),
                );
            }
        }
        pub fn store_iter_thir_exprs_inline_asm(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr,)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr,)>(
                    facts,
                    17045657056275096603u64,
                    self.database_root.join("relations/thir_exprs_inline_asm"),
                );
            }
        }
        pub fn load_iter_thir_exprs_offset_of(&self) -> impl Iterator<Item = (ThirExpr, Type)> {
            unsafe {
                load_elts_relation::<(ThirExpr, Type)>(
                    13407437027651654372u64,
                    self.database_root.join("relations/thir_exprs_offset_of"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_offset_of(&self) -> std::cell::Ref<Vec<(ThirExpr, Type)>> {
            if self.thir_exprs_offset_of.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, Type)>(
                        13407437027651654372u64,
                        self.database_root.join("relations/thir_exprs_offset_of"),
                    )
                }
                .unwrap();
                *self.thir_exprs_offset_of.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_offset_of.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_offset_of(&self, facts: Vec<(ThirExpr, Type)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, Type)>(
                    facts,
                    13407437027651654372u64,
                    self.database_root.join("relations/thir_exprs_offset_of"),
                );
            }
        }
        pub fn store_iter_thir_exprs_offset_of(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, Type)>(
                    facts,
                    13407437027651654372u64,
                    self.database_root.join("relations/thir_exprs_offset_of"),
                );
            }
        }
        pub fn load_thir_exprs_offset_of_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, Type> {
            self.load_thir_exprs_offset_of().iter().copied().collect()
        }
        pub fn load_iter_thir_exprs_thread_local_ref(
            &self,
        ) -> impl Iterator<Item = (ThirExpr, DefPath)> {
            unsafe {
                load_elts_relation::<(ThirExpr, DefPath)>(
                    10937537484208518487u64,
                    self.database_root
                        .join("relations/thir_exprs_thread_local_ref"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_thread_local_ref(&self) -> std::cell::Ref<Vec<(ThirExpr, DefPath)>> {
            if self.thir_exprs_thread_local_ref.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, DefPath)>(
                        10937537484208518487u64,
                        self.database_root
                            .join("relations/thir_exprs_thread_local_ref"),
                    )
                }
                .unwrap();
                *self.thir_exprs_thread_local_ref.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_thread_local_ref.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_thread_local_ref(&self, facts: Vec<(ThirExpr, DefPath)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath)>(
                    facts,
                    10937537484208518487u64,
                    self.database_root
                        .join("relations/thir_exprs_thread_local_ref"),
                );
            }
        }
        pub fn store_iter_thir_exprs_thread_local_ref(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, DefPath)>(
                    facts,
                    10937537484208518487u64,
                    self.database_root
                        .join("relations/thir_exprs_thread_local_ref"),
                );
            }
        }
        pub fn load_thir_exprs_thread_local_ref_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, DefPath> {
            self.load_thir_exprs_thread_local_ref()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_thir_exprs_yield(&self) -> impl Iterator<Item = (ThirExpr, ThirExpr)> {
            unsafe {
                load_elts_relation::<(ThirExpr, ThirExpr)>(
                    11224422256975424760u64,
                    self.database_root.join("relations/thir_exprs_yield"),
                )
            }
            .unwrap()
        }
        pub fn load_thir_exprs_yield(&self) -> std::cell::Ref<Vec<(ThirExpr, ThirExpr)>> {
            if self.thir_exprs_yield.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(ThirExpr, ThirExpr)>(
                        11224422256975424760u64,
                        self.database_root.join("relations/thir_exprs_yield"),
                    )
                }
                .unwrap();
                *self.thir_exprs_yield.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.thir_exprs_yield.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_thir_exprs_yield(&self, facts: Vec<(ThirExpr, ThirExpr)>) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    11224422256975424760u64,
                    self.database_root.join("relations/thir_exprs_yield"),
                );
            }
        }
        pub fn store_iter_thir_exprs_yield(
            &self,
            facts: impl IntoIterator<Item = (ThirExpr, ThirExpr)>,
        ) {
            unsafe {
                save_elts_relation::<(ThirExpr, ThirExpr)>(
                    facts,
                    11224422256975424760u64,
                    self.database_root.join("relations/thir_exprs_yield"),
                );
            }
        }
        pub fn load_thir_exprs_yield_as_map(
            &self,
        ) -> std::collections::HashMap<ThirExpr, ThirExpr> {
            self.load_thir_exprs_yield().iter().copied().collect()
        }
        pub fn load_iter_static_definitions(
            &self,
        ) -> impl Iterator<Item = (DefPath, Item, Module, Name, TyVisibility, Mutability)> {
            unsafe {
                load_elts_relation::<(DefPath, Item, Module, Name, TyVisibility, Mutability)>(
                    15615015511242754149u64,
                    self.database_root.join("relations/static_definitions"),
                )
            }
            .unwrap()
        }
        pub fn load_static_definitions(
            &self,
        ) -> std::cell::Ref<Vec<(DefPath, Item, Module, Name, TyVisibility, Mutability)>> {
            if self.static_definitions.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        DefPath,
                        Item,
                        Module,
                        Name,
                        TyVisibility,
                        Mutability,
                    )>(
                        15615015511242754149u64,
                        self.database_root.join("relations/static_definitions"),
                    )
                }
                .unwrap();
                *self.static_definitions.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.static_definitions.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_static_definitions(
            &self,
            facts: Vec<(DefPath, Item, Module, Name, TyVisibility, Mutability)>,
        ) {
            unsafe {
                save_elts_relation::<(DefPath, Item, Module, Name, TyVisibility, Mutability)>(
                    facts,
                    15615015511242754149u64,
                    self.database_root.join("relations/static_definitions"),
                );
            }
        }
        pub fn store_iter_static_definitions(
            &self,
            facts: impl IntoIterator<Item = (DefPath, Item, Module, Name, TyVisibility, Mutability)>,
        ) {
            unsafe {
                save_elts_relation::<(DefPath, Item, Module, Name, TyVisibility, Mutability)>(
                    facts,
                    15615015511242754149u64,
                    self.database_root.join("relations/static_definitions"),
                );
            }
        }
        pub fn load_iter_impl_definitions(
            &self,
        ) -> impl Iterator<
            Item = (
                DefPath,
                Item,
                Module,
                Name,
                TyVisibility,
                Safety,
                ImplPolarity,
                Defaultness,
                Constness,
                Type,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    DefPath,
                    Item,
                    Module,
                    Name,
                    TyVisibility,
                    Safety,
                    ImplPolarity,
                    Defaultness,
                    Constness,
                    Type,
                )>(
                    7690441149453729173u64,
                    self.database_root.join("relations/impl_definitions"),
                )
            }
            .unwrap()
        }
        pub fn load_impl_definitions(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                DefPath,
                Item,
                Module,
                Name,
                TyVisibility,
                Safety,
                ImplPolarity,
                Defaultness,
                Constness,
                Type,
            )>,
        > {
            if self.impl_definitions.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        DefPath,
                        Item,
                        Module,
                        Name,
                        TyVisibility,
                        Safety,
                        ImplPolarity,
                        Defaultness,
                        Constness,
                        Type,
                    )>(
                        7690441149453729173u64,
                        self.database_root.join("relations/impl_definitions"),
                    )
                }
                .unwrap();
                *self.impl_definitions.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.impl_definitions.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_impl_definitions(
            &self,
            facts: Vec<(
                DefPath,
                Item,
                Module,
                Name,
                TyVisibility,
                Safety,
                ImplPolarity,
                Defaultness,
                Constness,
                Type,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    DefPath,
                    Item,
                    Module,
                    Name,
                    TyVisibility,
                    Safety,
                    ImplPolarity,
                    Defaultness,
                    Constness,
                    Type,
                )>(
                    facts,
                    7690441149453729173u64,
                    self.database_root.join("relations/impl_definitions"),
                );
            }
        }
        pub fn store_iter_impl_definitions(
            &self,
            facts: impl IntoIterator<
                Item = (
                    DefPath,
                    Item,
                    Module,
                    Name,
                    TyVisibility,
                    Safety,
                    ImplPolarity,
                    Defaultness,
                    Constness,
                    Type,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    DefPath,
                    Item,
                    Module,
                    Name,
                    TyVisibility,
                    Safety,
                    ImplPolarity,
                    Defaultness,
                    Constness,
                    Type,
                )>(
                    facts,
                    7690441149453729173u64,
                    self.database_root.join("relations/impl_definitions"),
                );
            }
        }
        pub fn load_iter_trait_impls(&self) -> impl Iterator<Item = (Item, Type, DefPath)> {
            unsafe {
                load_elts_relation::<(Item, Type, DefPath)>(
                    14528000953543905781u64,
                    self.database_root.join("relations/trait_impls"),
                )
            }
            .unwrap()
        }
        pub fn load_trait_impls(&self) -> std::cell::Ref<Vec<(Item, Type, DefPath)>> {
            if self.trait_impls.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Item, Type, DefPath)>(
                        14528000953543905781u64,
                        self.database_root.join("relations/trait_impls"),
                    )
                }
                .unwrap();
                *self.trait_impls.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.trait_impls.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_trait_impls(&self, facts: Vec<(Item, Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Item, Type, DefPath)>(
                    facts,
                    14528000953543905781u64,
                    self.database_root.join("relations/trait_impls"),
                );
            }
        }
        pub fn store_iter_trait_impls(
            &self,
            facts: impl IntoIterator<Item = (Item, Type, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(Item, Type, DefPath)>(
                    facts,
                    14528000953543905781u64,
                    self.database_root.join("relations/trait_impls"),
                );
            }
        }
        pub fn load_iter_global_asm_blocks(
            &self,
        ) -> impl Iterator<Item = (DefPath, Item, Module, Name, TyVisibility)> {
            unsafe {
                load_elts_relation::<(DefPath, Item, Module, Name, TyVisibility)>(
                    10192848968817160673u64,
                    self.database_root.join("relations/global_asm_blocks"),
                )
            }
            .unwrap()
        }
        pub fn load_global_asm_blocks(
            &self,
        ) -> std::cell::Ref<Vec<(DefPath, Item, Module, Name, TyVisibility)>> {
            if self.global_asm_blocks.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(DefPath, Item, Module, Name, TyVisibility)>(
                        10192848968817160673u64,
                        self.database_root.join("relations/global_asm_blocks"),
                    )
                }
                .unwrap();
                *self.global_asm_blocks.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.global_asm_blocks.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_global_asm_blocks(
            &self,
            facts: Vec<(DefPath, Item, Module, Name, TyVisibility)>,
        ) {
            unsafe {
                save_elts_relation::<(DefPath, Item, Module, Name, TyVisibility)>(
                    facts,
                    10192848968817160673u64,
                    self.database_root.join("relations/global_asm_blocks"),
                );
            }
        }
        pub fn store_iter_global_asm_blocks(
            &self,
            facts: impl IntoIterator<Item = (DefPath, Item, Module, Name, TyVisibility)>,
        ) {
            unsafe {
                save_elts_relation::<(DefPath, Item, Module, Name, TyVisibility)>(
                    facts,
                    10192848968817160673u64,
                    self.database_root.join("relations/global_asm_blocks"),
                );
            }
        }
        pub fn load_iter_items(
            &self,
        ) -> impl Iterator<Item = (DefPath, Item, Module, Name, TyVisibility)> {
            unsafe {
                load_elts_relation::<(DefPath, Item, Module, Name, TyVisibility)>(
                    1289592195786933916u64,
                    self.database_root.join("relations/items"),
                )
            }
            .unwrap()
        }
        pub fn load_items(
            &self,
        ) -> std::cell::Ref<Vec<(DefPath, Item, Module, Name, TyVisibility)>> {
            if self.items.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(DefPath, Item, Module, Name, TyVisibility)>(
                        1289592195786933916u64,
                        self.database_root.join("relations/items"),
                    )
                }
                .unwrap();
                *self.items.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.items.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_items(&self, facts: Vec<(DefPath, Item, Module, Name, TyVisibility)>) {
            unsafe {
                save_elts_relation::<(DefPath, Item, Module, Name, TyVisibility)>(
                    facts,
                    1289592195786933916u64,
                    self.database_root.join("relations/items"),
                );
            }
        }
        pub fn store_iter_items(
            &self,
            facts: impl IntoIterator<Item = (DefPath, Item, Module, Name, TyVisibility)>,
        ) {
            unsafe {
                save_elts_relation::<(DefPath, Item, Module, Name, TyVisibility)>(
                    facts,
                    1289592195786933916u64,
                    self.database_root.join("relations/items"),
                );
            }
        }
        pub fn load_iter_mir_cfgs(&self) -> impl Iterator<Item = (Item, DefPath, Scope)> {
            unsafe {
                load_elts_relation::<(Item, DefPath, Scope)>(
                    14299758314825397119u64,
                    self.database_root.join("relations/mir_cfgs"),
                )
            }
            .unwrap()
        }
        pub fn load_mir_cfgs(&self) -> std::cell::Ref<Vec<(Item, DefPath, Scope)>> {
            if self.mir_cfgs.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Item, DefPath, Scope)>(
                        14299758314825397119u64,
                        self.database_root.join("relations/mir_cfgs"),
                    )
                }
                .unwrap();
                *self.mir_cfgs.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.mir_cfgs.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_mir_cfgs(&self, facts: Vec<(Item, DefPath, Scope)>) {
            unsafe {
                save_elts_relation::<(Item, DefPath, Scope)>(
                    facts,
                    14299758314825397119u64,
                    self.database_root.join("relations/mir_cfgs"),
                );
            }
        }
        pub fn store_iter_mir_cfgs(&self, facts: impl IntoIterator<Item = (Item, DefPath, Scope)>) {
            unsafe {
                save_elts_relation::<(Item, DefPath, Scope)>(
                    facts,
                    14299758314825397119u64,
                    self.database_root.join("relations/mir_cfgs"),
                );
            }
        }
        pub fn load_iter_subscopes(
            &self,
        ) -> impl Iterator<Item = (Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span)> {
            unsafe {
                load_elts_relation::<(Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span)>(
                    13606432809601680932u64,
                    self.database_root.join("relations/subscopes"),
                )
            }
            .unwrap()
        }
        pub fn load_subscopes(
            &self,
        ) -> std::cell::Ref<Vec<(Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span)>> {
            if self.subscopes.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Scope,
                        Scope,
                        ScopeSafety,
                        BlockCheckMode,
                        u32,
                        Span,
                    )>(
                        13606432809601680932u64,
                        self.database_root.join("relations/subscopes"),
                    )
                }
                .unwrap();
                *self.subscopes.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.subscopes.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_subscopes(
            &self,
            facts: Vec<(Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span)>,
        ) {
            unsafe {
                save_elts_relation::<(Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span)>(
                    facts,
                    13606432809601680932u64,
                    self.database_root.join("relations/subscopes"),
                );
            }
        }
        pub fn store_iter_subscopes(
            &self,
            facts: impl IntoIterator<Item = (Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span)>,
        ) {
            unsafe {
                save_elts_relation::<(Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span)>(
                    facts,
                    13606432809601680932u64,
                    self.database_root.join("relations/subscopes"),
                );
            }
        }
        pub fn load_iter_spans(
            &self,
        ) -> impl Iterator<
            Item = (
                Span,
                Span,
                SpanExpansionKind,
                InternedString,
                SpanFileName,
                u16,
                u16,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Span,
                    Span,
                    SpanExpansionKind,
                    InternedString,
                    SpanFileName,
                    u16,
                    u16,
                )>(
                    14731319860987095787u64,
                    self.database_root.join("relations/spans"),
                )
            }
            .unwrap()
        }
        pub fn load_spans(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Span,
                Span,
                SpanExpansionKind,
                InternedString,
                SpanFileName,
                u16,
                u16,
            )>,
        > {
            if self.spans.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Span,
                        Span,
                        SpanExpansionKind,
                        InternedString,
                        SpanFileName,
                        u16,
                        u16,
                    )>(
                        14731319860987095787u64,
                        self.database_root.join("relations/spans"),
                    )
                }
                .unwrap();
                *self.spans.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.spans.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_spans(
            &self,
            facts: Vec<(
                Span,
                Span,
                SpanExpansionKind,
                InternedString,
                SpanFileName,
                u16,
                u16,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Span,
                    Span,
                    SpanExpansionKind,
                    InternedString,
                    SpanFileName,
                    u16,
                    u16,
                )>(
                    facts,
                    14731319860987095787u64,
                    self.database_root.join("relations/spans"),
                );
            }
        }
        pub fn store_iter_spans(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Span,
                    Span,
                    SpanExpansionKind,
                    InternedString,
                    SpanFileName,
                    u16,
                    u16,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Span,
                    Span,
                    SpanExpansionKind,
                    InternedString,
                    SpanFileName,
                    u16,
                    u16,
                )>(
                    facts,
                    14731319860987095787u64,
                    self.database_root.join("relations/spans"),
                );
            }
        }
        pub fn load_iter_macro_expansions(
            &self,
        ) -> impl Iterator<Item = (Span, InternedString, SpanFileName, u16, u16)> {
            unsafe {
                load_elts_relation::<(Span, InternedString, SpanFileName, u16, u16)>(
                    16522324850221623531u64,
                    self.database_root.join("relations/macro_expansions"),
                )
            }
            .unwrap()
        }
        pub fn load_macro_expansions(
            &self,
        ) -> std::cell::Ref<Vec<(Span, InternedString, SpanFileName, u16, u16)>> {
            if self.macro_expansions.borrow().is_none() {
                let relation =
                    unsafe {
                        load_elts_relation_into_relation::<(
                            Span,
                            InternedString,
                            SpanFileName,
                            u16,
                            u16,
                        )>(
                            16522324850221623531u64,
                            self.database_root.join("relations/macro_expansions"),
                        )
                    }
                    .unwrap();
                *self.macro_expansions.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.macro_expansions.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_macro_expansions(
            &self,
            facts: Vec<(Span, InternedString, SpanFileName, u16, u16)>,
        ) {
            unsafe {
                save_elts_relation::<(Span, InternedString, SpanFileName, u16, u16)>(
                    facts,
                    16522324850221623531u64,
                    self.database_root.join("relations/macro_expansions"),
                );
            }
        }
        pub fn store_iter_macro_expansions(
            &self,
            facts: impl IntoIterator<Item = (Span, InternedString, SpanFileName, u16, u16)>,
        ) {
            unsafe {
                save_elts_relation::<(Span, InternedString, SpanFileName, u16, u16)>(
                    facts,
                    16522324850221623531u64,
                    self.database_root.join("relations/macro_expansions"),
                );
            }
        }
        pub fn load_iter_crate_cfgs(
            &self,
        ) -> impl Iterator<Item = (Build, CrateCfgKey, CrateCfgValue)> {
            unsafe {
                load_elts_relation::<(Build, CrateCfgKey, CrateCfgValue)>(
                    15473555631052060246u64,
                    self.database_root.join("relations/crate_cfgs"),
                )
            }
            .unwrap()
        }
        pub fn load_crate_cfgs(&self) -> std::cell::Ref<Vec<(Build, CrateCfgKey, CrateCfgValue)>> {
            if self.crate_cfgs.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, CrateCfgKey, CrateCfgValue)>(
                        15473555631052060246u64,
                        self.database_root.join("relations/crate_cfgs"),
                    )
                }
                .unwrap();
                *self.crate_cfgs.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.crate_cfgs.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_crate_cfgs(&self, facts: Vec<(Build, CrateCfgKey, CrateCfgValue)>) {
            unsafe {
                save_elts_relation::<(Build, CrateCfgKey, CrateCfgValue)>(
                    facts,
                    15473555631052060246u64,
                    self.database_root.join("relations/crate_cfgs"),
                );
            }
        }
        pub fn store_iter_crate_cfgs(
            &self,
            facts: impl IntoIterator<Item = (Build, CrateCfgKey, CrateCfgValue)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, CrateCfgKey, CrateCfgValue)>(
                    facts,
                    15473555631052060246u64,
                    self.database_root.join("relations/crate_cfgs"),
                );
            }
        }
        pub fn load_iter_crate_authors(&self) -> impl Iterator<Item = (Build, InternedString)> {
            unsafe {
                load_elts_relation::<(Build, InternedString)>(
                    10791542414620170368u64,
                    self.database_root.join("relations/crate_authors"),
                )
            }
            .unwrap()
        }
        pub fn load_crate_authors(&self) -> std::cell::Ref<Vec<(Build, InternedString)>> {
            if self.crate_authors.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, InternedString)>(
                        10791542414620170368u64,
                        self.database_root.join("relations/crate_authors"),
                    )
                }
                .unwrap();
                *self.crate_authors.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.crate_authors.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_crate_authors(&self, facts: Vec<(Build, InternedString)>) {
            unsafe {
                save_elts_relation::<(Build, InternedString)>(
                    facts,
                    10791542414620170368u64,
                    self.database_root.join("relations/crate_authors"),
                );
            }
        }
        pub fn store_iter_crate_authors(
            &self,
            facts: impl IntoIterator<Item = (Build, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, InternedString)>(
                    facts,
                    10791542414620170368u64,
                    self.database_root.join("relations/crate_authors"),
                );
            }
        }
        pub fn load_crate_authors_as_map(
            &self,
        ) -> std::collections::HashMap<Build, InternedString> {
            self.load_crate_authors().iter().copied().collect()
        }
        pub fn load_iter_crate_keywords(&self) -> impl Iterator<Item = (Build, InternedString)> {
            unsafe {
                load_elts_relation::<(Build, InternedString)>(
                    5250907228800762424u64,
                    self.database_root.join("relations/crate_keywords"),
                )
            }
            .unwrap()
        }
        pub fn load_crate_keywords(&self) -> std::cell::Ref<Vec<(Build, InternedString)>> {
            if self.crate_keywords.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, InternedString)>(
                        5250907228800762424u64,
                        self.database_root.join("relations/crate_keywords"),
                    )
                }
                .unwrap();
                *self.crate_keywords.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.crate_keywords.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_crate_keywords(&self, facts: Vec<(Build, InternedString)>) {
            unsafe {
                save_elts_relation::<(Build, InternedString)>(
                    facts,
                    5250907228800762424u64,
                    self.database_root.join("relations/crate_keywords"),
                );
            }
        }
        pub fn store_iter_crate_keywords(
            &self,
            facts: impl IntoIterator<Item = (Build, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, InternedString)>(
                    facts,
                    5250907228800762424u64,
                    self.database_root.join("relations/crate_keywords"),
                );
            }
        }
        pub fn load_crate_keywords_as_map(
            &self,
        ) -> std::collections::HashMap<Build, InternedString> {
            self.load_crate_keywords().iter().copied().collect()
        }
        pub fn load_iter_crate_categories(&self) -> impl Iterator<Item = (Build, InternedString)> {
            unsafe {
                load_elts_relation::<(Build, InternedString)>(
                    95240473592181479u64,
                    self.database_root.join("relations/crate_categories"),
                )
            }
            .unwrap()
        }
        pub fn load_crate_categories(&self) -> std::cell::Ref<Vec<(Build, InternedString)>> {
            if self.crate_categories.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, InternedString)>(
                        95240473592181479u64,
                        self.database_root.join("relations/crate_categories"),
                    )
                }
                .unwrap();
                *self.crate_categories.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.crate_categories.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_crate_categories(&self, facts: Vec<(Build, InternedString)>) {
            unsafe {
                save_elts_relation::<(Build, InternedString)>(
                    facts,
                    95240473592181479u64,
                    self.database_root.join("relations/crate_categories"),
                );
            }
        }
        pub fn store_iter_crate_categories(
            &self,
            facts: impl IntoIterator<Item = (Build, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, InternedString)>(
                    facts,
                    95240473592181479u64,
                    self.database_root.join("relations/crate_categories"),
                );
            }
        }
        pub fn load_crate_categories_as_map(
            &self,
        ) -> std::collections::HashMap<Build, InternedString> {
            self.load_crate_categories().iter().copied().collect()
        }
        pub fn load_iter_type_defs(
            &self,
        ) -> impl Iterator<Item = (Item, Type, DefPath, InternedString, TyVisibility, TyDefKind)>
        {
            unsafe { load_elts_relation :: < (Item , Type , DefPath , InternedString , TyVisibility , TyDefKind ,) > (3193569606496579004u64 , self . database_root . join ("relations/type_defs")) } . unwrap ()
        }
        pub fn load_type_defs(
            &self,
        ) -> std::cell::Ref<Vec<(Item, Type, DefPath, InternedString, TyVisibility, TyDefKind)>>
        {
            if self.type_defs.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Item,
                        Type,
                        DefPath,
                        InternedString,
                        TyVisibility,
                        TyDefKind,
                    )>(
                        3193569606496579004u64,
                        self.database_root.join("relations/type_defs"),
                    )
                }
                .unwrap();
                *self.type_defs.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.type_defs.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_type_defs(
            &self,
            facts: Vec<(Item, Type, DefPath, InternedString, TyVisibility, TyDefKind)>,
        ) {
            unsafe {
                save_elts_relation::<(Item, Type, DefPath, InternedString, TyVisibility, TyDefKind)>(
                    facts,
                    3193569606496579004u64,
                    self.database_root.join("relations/type_defs"),
                );
            }
        }
        pub fn store_iter_type_defs(
            &self,
            facts: impl IntoIterator<
                Item = (Item, Type, DefPath, InternedString, TyVisibility, TyDefKind),
            >,
        ) {
            unsafe {
                save_elts_relation::<(Item, Type, DefPath, InternedString, TyVisibility, TyDefKind)>(
                    facts,
                    3193569606496579004u64,
                    self.database_root.join("relations/type_defs"),
                );
            }
        }
        pub fn load_iter_types(&self) -> impl Iterator<Item = (Type, TyKind)> {
            unsafe {
                load_elts_relation::<(Type, TyKind)>(
                    3975376537915609471u64,
                    self.database_root.join("relations/types"),
                )
            }
            .unwrap()
        }
        pub fn load_types(&self) -> std::cell::Ref<Vec<(Type, TyKind)>> {
            if self.types.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, TyKind)>(
                        3975376537915609471u64,
                        self.database_root.join("relations/types"),
                    )
                }
                .unwrap();
                *self.types.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_types(&self, facts: Vec<(Type, TyKind)>) {
            unsafe {
                save_elts_relation::<(Type, TyKind)>(
                    facts,
                    3975376537915609471u64,
                    self.database_root.join("relations/types"),
                );
            }
        }
        pub fn store_iter_types(&self, facts: impl IntoIterator<Item = (Type, TyKind)>) {
            unsafe {
                save_elts_relation::<(Type, TyKind)>(
                    facts,
                    3975376537915609471u64,
                    self.database_root.join("relations/types"),
                );
            }
        }
        pub fn load_types_as_map(&self) -> std::collections::HashMap<Type, TyKind> {
            self.load_types().iter().copied().collect()
        }
        pub fn load_iter_types_primitive(&self) -> impl Iterator<Item = (Type, TyPrimitive)> {
            unsafe {
                load_elts_relation::<(Type, TyPrimitive)>(
                    2226877674460436540u64,
                    self.database_root.join("relations/types_primitive"),
                )
            }
            .unwrap()
        }
        pub fn load_types_primitive(&self) -> std::cell::Ref<Vec<(Type, TyPrimitive)>> {
            if self.types_primitive.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, TyPrimitive)>(
                        2226877674460436540u64,
                        self.database_root.join("relations/types_primitive"),
                    )
                }
                .unwrap();
                *self.types_primitive.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_primitive.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_primitive(&self, facts: Vec<(Type, TyPrimitive)>) {
            unsafe {
                save_elts_relation::<(Type, TyPrimitive)>(
                    facts,
                    2226877674460436540u64,
                    self.database_root.join("relations/types_primitive"),
                );
            }
        }
        pub fn store_iter_types_primitive(
            &self,
            facts: impl IntoIterator<Item = (Type, TyPrimitive)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, TyPrimitive)>(
                    facts,
                    2226877674460436540u64,
                    self.database_root.join("relations/types_primitive"),
                );
            }
        }
        pub fn load_types_primitive_as_map(&self) -> std::collections::HashMap<Type, TyPrimitive> {
            self.load_types_primitive().iter().copied().collect()
        }
        pub fn load_iter_types_adt_def(
            &self,
        ) -> impl Iterator<Item = (Type, DefPath, AdtKind, bool, bool)> {
            unsafe {
                load_elts_relation::<(Type, DefPath, AdtKind, bool, bool)>(
                    8183742418156298214u64,
                    self.database_root.join("relations/types_adt_def"),
                )
            }
            .unwrap()
        }
        pub fn load_types_adt_def(
            &self,
        ) -> std::cell::Ref<Vec<(Type, DefPath, AdtKind, bool, bool)>> {
            if self.types_adt_def.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath, AdtKind, bool, bool)>(
                        8183742418156298214u64,
                        self.database_root.join("relations/types_adt_def"),
                    )
                }
                .unwrap();
                *self.types_adt_def.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_adt_def.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_adt_def(&self, facts: Vec<(Type, DefPath, AdtKind, bool, bool)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath, AdtKind, bool, bool)>(
                    facts,
                    8183742418156298214u64,
                    self.database_root.join("relations/types_adt_def"),
                );
            }
        }
        pub fn store_iter_types_adt_def(
            &self,
            facts: impl IntoIterator<Item = (Type, DefPath, AdtKind, bool, bool)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, DefPath, AdtKind, bool, bool)>(
                    facts,
                    8183742418156298214u64,
                    self.database_root.join("relations/types_adt_def"),
                );
            }
        }
        pub fn load_iter_types_adt_variant(
            &self,
        ) -> impl Iterator<Item = (Type, AdtVariantIndex, DefPath, InternedString)> {
            unsafe {
                load_elts_relation::<(Type, AdtVariantIndex, DefPath, InternedString)>(
                    2197893904118803340u64,
                    self.database_root.join("relations/types_adt_variant"),
                )
            }
            .unwrap()
        }
        pub fn load_types_adt_variant(
            &self,
        ) -> std::cell::Ref<Vec<(Type, AdtVariantIndex, DefPath, InternedString)>> {
            if self.types_adt_variant.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Type,
                        AdtVariantIndex,
                        DefPath,
                        InternedString,
                    )>(
                        2197893904118803340u64,
                        self.database_root.join("relations/types_adt_variant"),
                    )
                }
                .unwrap();
                *self.types_adt_variant.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_adt_variant.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_adt_variant(
            &self,
            facts: Vec<(Type, AdtVariantIndex, DefPath, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, AdtVariantIndex, DefPath, InternedString)>(
                    facts,
                    2197893904118803340u64,
                    self.database_root.join("relations/types_adt_variant"),
                );
            }
        }
        pub fn store_iter_types_adt_variant(
            &self,
            facts: impl IntoIterator<Item = (Type, AdtVariantIndex, DefPath, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, AdtVariantIndex, DefPath, InternedString)>(
                    facts,
                    2197893904118803340u64,
                    self.database_root.join("relations/types_adt_variant"),
                );
            }
        }
        pub fn load_iter_types_adt_field(
            &self,
        ) -> impl Iterator<
            Item = (
                Field,
                Type,
                AdtVariantIndex,
                DefPath,
                InternedString,
                TyVisibility,
                Type,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Field,
                    Type,
                    AdtVariantIndex,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    Type,
                )>(
                    7037499858270167988u64,
                    self.database_root.join("relations/types_adt_field"),
                )
            }
            .unwrap()
        }
        pub fn load_types_adt_field(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Field,
                Type,
                AdtVariantIndex,
                DefPath,
                InternedString,
                TyVisibility,
                Type,
            )>,
        > {
            if self.types_adt_field.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Field,
                        Type,
                        AdtVariantIndex,
                        DefPath,
                        InternedString,
                        TyVisibility,
                        Type,
                    )>(
                        7037499858270167988u64,
                        self.database_root.join("relations/types_adt_field"),
                    )
                }
                .unwrap();
                *self.types_adt_field.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_adt_field.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_adt_field(
            &self,
            facts: Vec<(
                Field,
                Type,
                AdtVariantIndex,
                DefPath,
                InternedString,
                TyVisibility,
                Type,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Field,
                    Type,
                    AdtVariantIndex,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    Type,
                )>(
                    facts,
                    7037499858270167988u64,
                    self.database_root.join("relations/types_adt_field"),
                );
            }
        }
        pub fn store_iter_types_adt_field(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Field,
                    Type,
                    AdtVariantIndex,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    Type,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Field,
                    Type,
                    AdtVariantIndex,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    Type,
                )>(
                    facts,
                    7037499858270167988u64,
                    self.database_root.join("relations/types_adt_field"),
                );
            }
        }
        pub fn load_iter_types_adt_field_visible_in(
            &self,
        ) -> impl Iterator<Item = (Field, DefPath)> {
            unsafe {
                load_elts_relation::<(Field, DefPath)>(
                    18004517893509822049u64,
                    self.database_root
                        .join("relations/types_adt_field_visible_in"),
                )
            }
            .unwrap()
        }
        pub fn load_types_adt_field_visible_in(&self) -> std::cell::Ref<Vec<(Field, DefPath)>> {
            if self.types_adt_field_visible_in.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Field, DefPath)>(
                        18004517893509822049u64,
                        self.database_root
                            .join("relations/types_adt_field_visible_in"),
                    )
                }
                .unwrap();
                *self.types_adt_field_visible_in.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_adt_field_visible_in.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_adt_field_visible_in(&self, facts: Vec<(Field, DefPath)>) {
            unsafe {
                save_elts_relation::<(Field, DefPath)>(
                    facts,
                    18004517893509822049u64,
                    self.database_root
                        .join("relations/types_adt_field_visible_in"),
                );
            }
        }
        pub fn store_iter_types_adt_field_visible_in(
            &self,
            facts: impl IntoIterator<Item = (Field, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(Field, DefPath)>(
                    facts,
                    18004517893509822049u64,
                    self.database_root
                        .join("relations/types_adt_field_visible_in"),
                );
            }
        }
        pub fn load_types_adt_field_visible_in_as_map(
            &self,
        ) -> std::collections::HashMap<Field, DefPath> {
            self.load_types_adt_field_visible_in()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_types_foreign(&self) -> impl Iterator<Item = (Type, DefPath)> {
            unsafe {
                load_elts_relation::<(Type, DefPath)>(
                    4142907501147052699u64,
                    self.database_root.join("relations/types_foreign"),
                )
            }
            .unwrap()
        }
        pub fn load_types_foreign(&self) -> std::cell::Ref<Vec<(Type, DefPath)>> {
            if self.types_foreign.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath)>(
                        4142907501147052699u64,
                        self.database_root.join("relations/types_foreign"),
                    )
                }
                .unwrap();
                *self.types_foreign.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_foreign.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_foreign(&self, facts: Vec<(Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    4142907501147052699u64,
                    self.database_root.join("relations/types_foreign"),
                );
            }
        }
        pub fn store_iter_types_foreign(&self, facts: impl IntoIterator<Item = (Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    4142907501147052699u64,
                    self.database_root.join("relations/types_foreign"),
                );
            }
        }
        pub fn load_types_foreign_as_map(&self) -> std::collections::HashMap<Type, DefPath> {
            self.load_types_foreign().iter().copied().collect()
        }
        pub fn load_iter_types_array(&self) -> impl Iterator<Item = (Type, Type)> {
            unsafe {
                load_elts_relation::<(Type, Type)>(
                    1131626646816840762u64,
                    self.database_root.join("relations/types_array"),
                )
            }
            .unwrap()
        }
        pub fn load_types_array(&self) -> std::cell::Ref<Vec<(Type, Type)>> {
            if self.types_array.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, Type)>(
                        1131626646816840762u64,
                        self.database_root.join("relations/types_array"),
                    )
                }
                .unwrap();
                *self.types_array.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_array.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_types_array(&self, facts: Vec<(Type, Type)>) {
            unsafe {
                save_elts_relation::<(Type, Type)>(
                    facts,
                    1131626646816840762u64,
                    self.database_root.join("relations/types_array"),
                );
            }
        }
        pub fn store_iter_types_array(&self, facts: impl IntoIterator<Item = (Type, Type)>) {
            unsafe {
                save_elts_relation::<(Type, Type)>(
                    facts,
                    1131626646816840762u64,
                    self.database_root.join("relations/types_array"),
                );
            }
        }
        pub fn load_types_array_as_map(&self) -> std::collections::HashMap<Type, Type> {
            self.load_types_array().iter().copied().collect()
        }
        pub fn load_iter_types_slice(&self) -> impl Iterator<Item = (Type, Type)> {
            unsafe {
                load_elts_relation::<(Type, Type)>(
                    6446650263542674023u64,
                    self.database_root.join("relations/types_slice"),
                )
            }
            .unwrap()
        }
        pub fn load_types_slice(&self) -> std::cell::Ref<Vec<(Type, Type)>> {
            if self.types_slice.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, Type)>(
                        6446650263542674023u64,
                        self.database_root.join("relations/types_slice"),
                    )
                }
                .unwrap();
                *self.types_slice.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_slice.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_types_slice(&self, facts: Vec<(Type, Type)>) {
            unsafe {
                save_elts_relation::<(Type, Type)>(
                    facts,
                    6446650263542674023u64,
                    self.database_root.join("relations/types_slice"),
                );
            }
        }
        pub fn store_iter_types_slice(&self, facts: impl IntoIterator<Item = (Type, Type)>) {
            unsafe {
                save_elts_relation::<(Type, Type)>(
                    facts,
                    6446650263542674023u64,
                    self.database_root.join("relations/types_slice"),
                );
            }
        }
        pub fn load_types_slice_as_map(&self) -> std::collections::HashMap<Type, Type> {
            self.load_types_slice().iter().copied().collect()
        }
        pub fn load_iter_types_raw_ptr(&self) -> impl Iterator<Item = (Type, Type, Mutability)> {
            unsafe {
                load_elts_relation::<(Type, Type, Mutability)>(
                    5411092752360582208u64,
                    self.database_root.join("relations/types_raw_ptr"),
                )
            }
            .unwrap()
        }
        pub fn load_types_raw_ptr(&self) -> std::cell::Ref<Vec<(Type, Type, Mutability)>> {
            if self.types_raw_ptr.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, Type, Mutability)>(
                        5411092752360582208u64,
                        self.database_root.join("relations/types_raw_ptr"),
                    )
                }
                .unwrap();
                *self.types_raw_ptr.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_raw_ptr.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_raw_ptr(&self, facts: Vec<(Type, Type, Mutability)>) {
            unsafe {
                save_elts_relation::<(Type, Type, Mutability)>(
                    facts,
                    5411092752360582208u64,
                    self.database_root.join("relations/types_raw_ptr"),
                );
            }
        }
        pub fn store_iter_types_raw_ptr(
            &self,
            facts: impl IntoIterator<Item = (Type, Type, Mutability)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, Type, Mutability)>(
                    facts,
                    5411092752360582208u64,
                    self.database_root.join("relations/types_raw_ptr"),
                );
            }
        }
        pub fn load_iter_types_ref(&self) -> impl Iterator<Item = (Type, Type, Mutability)> {
            unsafe {
                load_elts_relation::<(Type, Type, Mutability)>(
                    4446069966514798266u64,
                    self.database_root.join("relations/types_ref"),
                )
            }
            .unwrap()
        }
        pub fn load_types_ref(&self) -> std::cell::Ref<Vec<(Type, Type, Mutability)>> {
            if self.types_ref.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, Type, Mutability)>(
                        4446069966514798266u64,
                        self.database_root.join("relations/types_ref"),
                    )
                }
                .unwrap();
                *self.types_ref.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_ref.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_types_ref(&self, facts: Vec<(Type, Type, Mutability)>) {
            unsafe {
                save_elts_relation::<(Type, Type, Mutability)>(
                    facts,
                    4446069966514798266u64,
                    self.database_root.join("relations/types_ref"),
                );
            }
        }
        pub fn store_iter_types_ref(
            &self,
            facts: impl IntoIterator<Item = (Type, Type, Mutability)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, Type, Mutability)>(
                    facts,
                    4446069966514798266u64,
                    self.database_root.join("relations/types_ref"),
                );
            }
        }
        pub fn load_iter_types_fn_def(&self) -> impl Iterator<Item = (Type, DefPath)> {
            unsafe {
                load_elts_relation::<(Type, DefPath)>(
                    1425750154069839048u64,
                    self.database_root.join("relations/types_fn_def"),
                )
            }
            .unwrap()
        }
        pub fn load_types_fn_def(&self) -> std::cell::Ref<Vec<(Type, DefPath)>> {
            if self.types_fn_def.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath)>(
                        1425750154069839048u64,
                        self.database_root.join("relations/types_fn_def"),
                    )
                }
                .unwrap();
                *self.types_fn_def.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_fn_def.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_fn_def(&self, facts: Vec<(Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    1425750154069839048u64,
                    self.database_root.join("relations/types_fn_def"),
                );
            }
        }
        pub fn store_iter_types_fn_def(&self, facts: impl IntoIterator<Item = (Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    1425750154069839048u64,
                    self.database_root.join("relations/types_fn_def"),
                );
            }
        }
        pub fn load_types_fn_def_as_map(&self) -> std::collections::HashMap<Type, DefPath> {
            self.load_types_fn_def().iter().copied().collect()
        }
        pub fn load_iter_types_fn_ptr(&self) -> impl Iterator<Item = (Type,)> {
            unsafe {
                load_elts_relation::<(Type,)>(
                    9445280361843553498u64,
                    self.database_root.join("relations/types_fn_ptr"),
                )
            }
            .unwrap()
        }
        pub fn load_types_fn_ptr(&self) -> std::cell::Ref<Vec<(Type,)>> {
            if self.types_fn_ptr.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type,)>(
                        9445280361843553498u64,
                        self.database_root.join("relations/types_fn_ptr"),
                    )
                }
                .unwrap();
                *self.types_fn_ptr.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_fn_ptr.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_fn_ptr(&self, facts: Vec<(Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    9445280361843553498u64,
                    self.database_root.join("relations/types_fn_ptr"),
                );
            }
        }
        pub fn store_iter_types_fn_ptr(&self, facts: impl IntoIterator<Item = (Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    9445280361843553498u64,
                    self.database_root.join("relations/types_fn_ptr"),
                );
            }
        }
        pub fn load_iter_types_dynamic(&self) -> impl Iterator<Item = (Type,)> {
            unsafe {
                load_elts_relation::<(Type,)>(
                    10493468012836106481u64,
                    self.database_root.join("relations/types_dynamic"),
                )
            }
            .unwrap()
        }
        pub fn load_types_dynamic(&self) -> std::cell::Ref<Vec<(Type,)>> {
            if self.types_dynamic.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type,)>(
                        10493468012836106481u64,
                        self.database_root.join("relations/types_dynamic"),
                    )
                }
                .unwrap();
                *self.types_dynamic.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_dynamic.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_dynamic(&self, facts: Vec<(Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    10493468012836106481u64,
                    self.database_root.join("relations/types_dynamic"),
                );
            }
        }
        pub fn store_iter_types_dynamic(&self, facts: impl IntoIterator<Item = (Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    10493468012836106481u64,
                    self.database_root.join("relations/types_dynamic"),
                );
            }
        }
        pub fn load_iter_types_dynamic_trait(&self) -> impl Iterator<Item = (Type, DefPath, bool)> {
            unsafe {
                load_elts_relation::<(Type, DefPath, bool)>(
                    15245729660191565254u64,
                    self.database_root.join("relations/types_dynamic_trait"),
                )
            }
            .unwrap()
        }
        pub fn load_types_dynamic_trait(&self) -> std::cell::Ref<Vec<(Type, DefPath, bool)>> {
            if self.types_dynamic_trait.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath, bool)>(
                        15245729660191565254u64,
                        self.database_root.join("relations/types_dynamic_trait"),
                    )
                }
                .unwrap();
                *self.types_dynamic_trait.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_dynamic_trait.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_dynamic_trait(&self, facts: Vec<(Type, DefPath, bool)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath, bool)>(
                    facts,
                    15245729660191565254u64,
                    self.database_root.join("relations/types_dynamic_trait"),
                );
            }
        }
        pub fn store_iter_types_dynamic_trait(
            &self,
            facts: impl IntoIterator<Item = (Type, DefPath, bool)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, DefPath, bool)>(
                    facts,
                    15245729660191565254u64,
                    self.database_root.join("relations/types_dynamic_trait"),
                );
            }
        }
        pub fn load_iter_types_closure(&self) -> impl Iterator<Item = (Type, DefPath)> {
            unsafe {
                load_elts_relation::<(Type, DefPath)>(
                    18347839133875243449u64,
                    self.database_root.join("relations/types_closure"),
                )
            }
            .unwrap()
        }
        pub fn load_types_closure(&self) -> std::cell::Ref<Vec<(Type, DefPath)>> {
            if self.types_closure.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath)>(
                        18347839133875243449u64,
                        self.database_root.join("relations/types_closure"),
                    )
                }
                .unwrap();
                *self.types_closure.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_closure.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_closure(&self, facts: Vec<(Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    18347839133875243449u64,
                    self.database_root.join("relations/types_closure"),
                );
            }
        }
        pub fn store_iter_types_closure(&self, facts: impl IntoIterator<Item = (Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    18347839133875243449u64,
                    self.database_root.join("relations/types_closure"),
                );
            }
        }
        pub fn load_types_closure_as_map(&self) -> std::collections::HashMap<Type, DefPath> {
            self.load_types_closure().iter().copied().collect()
        }
        pub fn load_iter_types_coroutine(&self) -> impl Iterator<Item = (Type, DefPath)> {
            unsafe {
                load_elts_relation::<(Type, DefPath)>(
                    11592863599376068273u64,
                    self.database_root.join("relations/types_coroutine"),
                )
            }
            .unwrap()
        }
        pub fn load_types_coroutine(&self) -> std::cell::Ref<Vec<(Type, DefPath)>> {
            if self.types_coroutine.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath)>(
                        11592863599376068273u64,
                        self.database_root.join("relations/types_coroutine"),
                    )
                }
                .unwrap();
                *self.types_coroutine.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_coroutine.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_coroutine(&self, facts: Vec<(Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    11592863599376068273u64,
                    self.database_root.join("relations/types_coroutine"),
                );
            }
        }
        pub fn store_iter_types_coroutine(&self, facts: impl IntoIterator<Item = (Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    11592863599376068273u64,
                    self.database_root.join("relations/types_coroutine"),
                );
            }
        }
        pub fn load_types_coroutine_as_map(&self) -> std::collections::HashMap<Type, DefPath> {
            self.load_types_coroutine().iter().copied().collect()
        }
        pub fn load_iter_types_coroutine_witness(&self) -> impl Iterator<Item = (Type,)> {
            unsafe {
                load_elts_relation::<(Type,)>(
                    17595719270309805751u64,
                    self.database_root.join("relations/types_coroutine_witness"),
                )
            }
            .unwrap()
        }
        pub fn load_types_coroutine_witness(&self) -> std::cell::Ref<Vec<(Type,)>> {
            if self.types_coroutine_witness.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type,)>(
                        17595719270309805751u64,
                        self.database_root.join("relations/types_coroutine_witness"),
                    )
                }
                .unwrap();
                *self.types_coroutine_witness.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_coroutine_witness.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_coroutine_witness(&self, facts: Vec<(Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    17595719270309805751u64,
                    self.database_root.join("relations/types_coroutine_witness"),
                );
            }
        }
        pub fn store_iter_types_coroutine_witness(&self, facts: impl IntoIterator<Item = (Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    17595719270309805751u64,
                    self.database_root.join("relations/types_coroutine_witness"),
                );
            }
        }
        pub fn load_iter_types_coroutine_closure(&self) -> impl Iterator<Item = (Type, DefPath)> {
            unsafe {
                load_elts_relation::<(Type, DefPath)>(
                    10429624441391305088u64,
                    self.database_root.join("relations/types_coroutine_closure"),
                )
            }
            .unwrap()
        }
        pub fn load_types_coroutine_closure(&self) -> std::cell::Ref<Vec<(Type, DefPath)>> {
            if self.types_coroutine_closure.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath)>(
                        10429624441391305088u64,
                        self.database_root.join("relations/types_coroutine_closure"),
                    )
                }
                .unwrap();
                *self.types_coroutine_closure.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_coroutine_closure.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_coroutine_closure(&self, facts: Vec<(Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    10429624441391305088u64,
                    self.database_root.join("relations/types_coroutine_closure"),
                );
            }
        }
        pub fn store_iter_types_coroutine_closure(
            &self,
            facts: impl IntoIterator<Item = (Type, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    10429624441391305088u64,
                    self.database_root.join("relations/types_coroutine_closure"),
                );
            }
        }
        pub fn load_types_coroutine_closure_as_map(
            &self,
        ) -> std::collections::HashMap<Type, DefPath> {
            self.load_types_coroutine_closure()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_types_pat(&self) -> impl Iterator<Item = (Type,)> {
            unsafe {
                load_elts_relation::<(Type,)>(
                    15952850863293531464u64,
                    self.database_root.join("relations/types_pat"),
                )
            }
            .unwrap()
        }
        pub fn load_types_pat(&self) -> std::cell::Ref<Vec<(Type,)>> {
            if self.types_pat.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type,)>(
                        15952850863293531464u64,
                        self.database_root.join("relations/types_pat"),
                    )
                }
                .unwrap();
                *self.types_pat.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_pat.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_types_pat(&self, facts: Vec<(Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    15952850863293531464u64,
                    self.database_root.join("relations/types_pat"),
                );
            }
        }
        pub fn store_iter_types_pat(&self, facts: impl IntoIterator<Item = (Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    15952850863293531464u64,
                    self.database_root.join("relations/types_pat"),
                );
            }
        }
        pub fn load_iter_types_tuple(&self) -> impl Iterator<Item = (Type,)> {
            unsafe {
                load_elts_relation::<(Type,)>(
                    11514292614567617999u64,
                    self.database_root.join("relations/types_tuple"),
                )
            }
            .unwrap()
        }
        pub fn load_types_tuple(&self) -> std::cell::Ref<Vec<(Type,)>> {
            if self.types_tuple.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type,)>(
                        11514292614567617999u64,
                        self.database_root.join("relations/types_tuple"),
                    )
                }
                .unwrap();
                *self.types_tuple.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_tuple.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_types_tuple(&self, facts: Vec<(Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    11514292614567617999u64,
                    self.database_root.join("relations/types_tuple"),
                );
            }
        }
        pub fn store_iter_types_tuple(&self, facts: impl IntoIterator<Item = (Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    11514292614567617999u64,
                    self.database_root.join("relations/types_tuple"),
                );
            }
        }
        pub fn load_iter_types_tuple_element(
            &self,
        ) -> impl Iterator<Item = (Type, TupleFieldIndex, Type)> {
            unsafe {
                load_elts_relation::<(Type, TupleFieldIndex, Type)>(
                    5818754280475673020u64,
                    self.database_root.join("relations/types_tuple_element"),
                )
            }
            .unwrap()
        }
        pub fn load_types_tuple_element(
            &self,
        ) -> std::cell::Ref<Vec<(Type, TupleFieldIndex, Type)>> {
            if self.types_tuple_element.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, TupleFieldIndex, Type)>(
                        5818754280475673020u64,
                        self.database_root.join("relations/types_tuple_element"),
                    )
                }
                .unwrap();
                *self.types_tuple_element.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_tuple_element.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_tuple_element(&self, facts: Vec<(Type, TupleFieldIndex, Type)>) {
            unsafe {
                save_elts_relation::<(Type, TupleFieldIndex, Type)>(
                    facts,
                    5818754280475673020u64,
                    self.database_root.join("relations/types_tuple_element"),
                );
            }
        }
        pub fn store_iter_types_tuple_element(
            &self,
            facts: impl IntoIterator<Item = (Type, TupleFieldIndex, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, TupleFieldIndex, Type)>(
                    facts,
                    5818754280475673020u64,
                    self.database_root.join("relations/types_tuple_element"),
                );
            }
        }
        pub fn load_iter_types_projection(&self) -> impl Iterator<Item = (Type, DefPath, DefPath)> {
            unsafe {
                load_elts_relation::<(Type, DefPath, DefPath)>(
                    6998246431921405765u64,
                    self.database_root.join("relations/types_projection"),
                )
            }
            .unwrap()
        }
        pub fn load_types_projection(&self) -> std::cell::Ref<Vec<(Type, DefPath, DefPath)>> {
            if self.types_projection.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath, DefPath)>(
                        6998246431921405765u64,
                        self.database_root.join("relations/types_projection"),
                    )
                }
                .unwrap();
                *self.types_projection.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_projection.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_projection(&self, facts: Vec<(Type, DefPath, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath, DefPath)>(
                    facts,
                    6998246431921405765u64,
                    self.database_root.join("relations/types_projection"),
                );
            }
        }
        pub fn store_iter_types_projection(
            &self,
            facts: impl IntoIterator<Item = (Type, DefPath, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, DefPath, DefPath)>(
                    facts,
                    6998246431921405765u64,
                    self.database_root.join("relations/types_projection"),
                );
            }
        }
        pub fn load_iter_types_opaque(&self) -> impl Iterator<Item = (Type, DefPath)> {
            unsafe {
                load_elts_relation::<(Type, DefPath)>(
                    5423165845542059810u64,
                    self.database_root.join("relations/types_opaque"),
                )
            }
            .unwrap()
        }
        pub fn load_types_opaque(&self) -> std::cell::Ref<Vec<(Type, DefPath)>> {
            if self.types_opaque.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath)>(
                        5423165845542059810u64,
                        self.database_root.join("relations/types_opaque"),
                    )
                }
                .unwrap();
                *self.types_opaque.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_opaque.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_opaque(&self, facts: Vec<(Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    5423165845542059810u64,
                    self.database_root.join("relations/types_opaque"),
                );
            }
        }
        pub fn store_iter_types_opaque(&self, facts: impl IntoIterator<Item = (Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    5423165845542059810u64,
                    self.database_root.join("relations/types_opaque"),
                );
            }
        }
        pub fn load_types_opaque_as_map(&self) -> std::collections::HashMap<Type, DefPath> {
            self.load_types_opaque().iter().copied().collect()
        }
        pub fn load_iter_types_inherent(&self) -> impl Iterator<Item = (Type, DefPath)> {
            unsafe {
                load_elts_relation::<(Type, DefPath)>(
                    6917214768906050830u64,
                    self.database_root.join("relations/types_inherent"),
                )
            }
            .unwrap()
        }
        pub fn load_types_inherent(&self) -> std::cell::Ref<Vec<(Type, DefPath)>> {
            if self.types_inherent.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath)>(
                        6917214768906050830u64,
                        self.database_root.join("relations/types_inherent"),
                    )
                }
                .unwrap();
                *self.types_inherent.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_inherent.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_inherent(&self, facts: Vec<(Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    6917214768906050830u64,
                    self.database_root.join("relations/types_inherent"),
                );
            }
        }
        pub fn store_iter_types_inherent(&self, facts: impl IntoIterator<Item = (Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    6917214768906050830u64,
                    self.database_root.join("relations/types_inherent"),
                );
            }
        }
        pub fn load_types_inherent_as_map(&self) -> std::collections::HashMap<Type, DefPath> {
            self.load_types_inherent().iter().copied().collect()
        }
        pub fn load_iter_types_weak(&self) -> impl Iterator<Item = (Type, DefPath)> {
            unsafe {
                load_elts_relation::<(Type, DefPath)>(
                    886877941837007763u64,
                    self.database_root.join("relations/types_weak"),
                )
            }
            .unwrap()
        }
        pub fn load_types_weak(&self) -> std::cell::Ref<Vec<(Type, DefPath)>> {
            if self.types_weak.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath)>(
                        886877941837007763u64,
                        self.database_root.join("relations/types_weak"),
                    )
                }
                .unwrap();
                *self.types_weak.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_weak.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_types_weak(&self, facts: Vec<(Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    886877941837007763u64,
                    self.database_root.join("relations/types_weak"),
                );
            }
        }
        pub fn store_iter_types_weak(&self, facts: impl IntoIterator<Item = (Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    886877941837007763u64,
                    self.database_root.join("relations/types_weak"),
                );
            }
        }
        pub fn load_types_weak_as_map(&self) -> std::collections::HashMap<Type, DefPath> {
            self.load_types_weak().iter().copied().collect()
        }
        pub fn load_iter_types_param(&self) -> impl Iterator<Item = (Type, u32, InternedString)> {
            unsafe {
                load_elts_relation::<(Type, u32, InternedString)>(
                    14436349763986252312u64,
                    self.database_root.join("relations/types_param"),
                )
            }
            .unwrap()
        }
        pub fn load_types_param(&self) -> std::cell::Ref<Vec<(Type, u32, InternedString)>> {
            if self.types_param.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, u32, InternedString)>(
                        14436349763986252312u64,
                        self.database_root.join("relations/types_param"),
                    )
                }
                .unwrap();
                *self.types_param.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_param.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_types_param(&self, facts: Vec<(Type, u32, InternedString)>) {
            unsafe {
                save_elts_relation::<(Type, u32, InternedString)>(
                    facts,
                    14436349763986252312u64,
                    self.database_root.join("relations/types_param"),
                );
            }
        }
        pub fn store_iter_types_param(
            &self,
            facts: impl IntoIterator<Item = (Type, u32, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, u32, InternedString)>(
                    facts,
                    14436349763986252312u64,
                    self.database_root.join("relations/types_param"),
                );
            }
        }
        pub fn load_iter_traits(
            &self,
        ) -> impl Iterator<
            Item = (
                Item,
                DefPath,
                InternedString,
                TyVisibility,
                bool,
                bool,
                Safety,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Item,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    bool,
                    bool,
                    Safety,
                )>(
                    10046940701614007011u64,
                    self.database_root.join("relations/traits"),
                )
            }
            .unwrap()
        }
        pub fn load_traits(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Item,
                DefPath,
                InternedString,
                TyVisibility,
                bool,
                bool,
                Safety,
            )>,
        > {
            if self.traits.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Item,
                        DefPath,
                        InternedString,
                        TyVisibility,
                        bool,
                        bool,
                        Safety,
                    )>(
                        10046940701614007011u64,
                        self.database_root.join("relations/traits"),
                    )
                }
                .unwrap();
                *self.traits.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.traits.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_traits(
            &self,
            facts: Vec<(
                Item,
                DefPath,
                InternedString,
                TyVisibility,
                bool,
                bool,
                Safety,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Item,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    bool,
                    bool,
                    Safety,
                )>(
                    facts,
                    10046940701614007011u64,
                    self.database_root.join("relations/traits"),
                );
            }
        }
        pub fn store_iter_traits(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Item,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    bool,
                    bool,
                    Safety,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Item,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    bool,
                    bool,
                    Safety,
                )>(
                    facts,
                    10046940701614007011u64,
                    self.database_root.join("relations/traits"),
                );
            }
        }
        pub fn load_iter_trait_items(&self) -> impl Iterator<Item = (Item, DefPath, Defaultness)> {
            unsafe {
                load_elts_relation::<(Item, DefPath, Defaultness)>(
                    17980637816433357124u64,
                    self.database_root.join("relations/trait_items"),
                )
            }
            .unwrap()
        }
        pub fn load_trait_items(&self) -> std::cell::Ref<Vec<(Item, DefPath, Defaultness)>> {
            if self.trait_items.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Item, DefPath, Defaultness)>(
                        17980637816433357124u64,
                        self.database_root.join("relations/trait_items"),
                    )
                }
                .unwrap();
                *self.trait_items.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.trait_items.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_trait_items(&self, facts: Vec<(Item, DefPath, Defaultness)>) {
            unsafe {
                save_elts_relation::<(Item, DefPath, Defaultness)>(
                    facts,
                    17980637816433357124u64,
                    self.database_root.join("relations/trait_items"),
                );
            }
        }
        pub fn store_iter_trait_items(
            &self,
            facts: impl IntoIterator<Item = (Item, DefPath, Defaultness)>,
        ) {
            unsafe {
                save_elts_relation::<(Item, DefPath, Defaultness)>(
                    facts,
                    17980637816433357124u64,
                    self.database_root.join("relations/trait_items"),
                );
            }
        }
        pub fn load_iter_basic_blocks(
            &self,
        ) -> impl Iterator<Item = (BasicBlock, DefPath, BasicBlockKind)> {
            unsafe {
                load_elts_relation::<(BasicBlock, DefPath, BasicBlockKind)>(
                    7184699705572388449u64,
                    self.database_root.join("relations/basic_blocks"),
                )
            }
            .unwrap()
        }
        pub fn load_basic_blocks(
            &self,
        ) -> std::cell::Ref<Vec<(BasicBlock, DefPath, BasicBlockKind)>> {
            if self.basic_blocks.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(BasicBlock, DefPath, BasicBlockKind)>(
                        7184699705572388449u64,
                        self.database_root.join("relations/basic_blocks"),
                    )
                }
                .unwrap();
                *self.basic_blocks.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.basic_blocks.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_basic_blocks(&self, facts: Vec<(BasicBlock, DefPath, BasicBlockKind)>) {
            unsafe {
                save_elts_relation::<(BasicBlock, DefPath, BasicBlockKind)>(
                    facts,
                    7184699705572388449u64,
                    self.database_root.join("relations/basic_blocks"),
                );
            }
        }
        pub fn store_iter_basic_blocks(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock, DefPath, BasicBlockKind)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, DefPath, BasicBlockKind)>(
                    facts,
                    7184699705572388449u64,
                    self.database_root.join("relations/basic_blocks"),
                );
            }
        }
        pub fn load_iter_statements(
            &self,
        ) -> impl Iterator<Item = (Statement, BasicBlock, StatementIndex, StatementKind, Scope)>
        {
            unsafe {
                load_elts_relation::<(Statement, BasicBlock, StatementIndex, StatementKind, Scope)>(
                    17415903984110492204u64,
                    self.database_root.join("relations/statements"),
                )
            }
            .unwrap()
        }
        pub fn load_statements(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, BasicBlock, StatementIndex, StatementKind, Scope)>>
        {
            if self.statements.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Statement,
                        BasicBlock,
                        StatementIndex,
                        StatementKind,
                        Scope,
                    )>(
                        17415903984110492204u64,
                        self.database_root.join("relations/statements"),
                    )
                }
                .unwrap();
                *self.statements.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_statements(
            &self,
            facts: Vec<(Statement, BasicBlock, StatementIndex, StatementKind, Scope)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, BasicBlock, StatementIndex, StatementKind, Scope)>(
                    facts,
                    17415903984110492204u64,
                    self.database_root.join("relations/statements"),
                );
            }
        }
        pub fn store_iter_statements(
            &self,
            facts: impl IntoIterator<
                Item = (Statement, BasicBlock, StatementIndex, StatementKind, Scope),
            >,
        ) {
            unsafe {
                save_elts_relation::<(Statement, BasicBlock, StatementIndex, StatementKind, Scope)>(
                    facts,
                    17415903984110492204u64,
                    self.database_root.join("relations/statements"),
                );
            }
        }
        pub fn load_iter_statements_assign_use(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, Operand)> {
            unsafe {
                load_elts_relation::<(Statement, Type, Operand)>(
                    2143429789654940834u64,
                    self.database_root.join("relations/statements_assign_use"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_use(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type, Operand)>> {
            if self.statements_assign_use.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, Operand)>(
                        2143429789654940834u64,
                        self.database_root.join("relations/statements_assign_use"),
                    )
                }
                .unwrap();
                *self.statements_assign_use.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_use.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_use(&self, facts: Vec<(Statement, Type, Operand)>) {
            unsafe {
                save_elts_relation::<(Statement, Type, Operand)>(
                    facts,
                    2143429789654940834u64,
                    self.database_root.join("relations/statements_assign_use"),
                );
            }
        }
        pub fn store_iter_statements_assign_use(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, Operand)>(
                    facts,
                    2143429789654940834u64,
                    self.database_root.join("relations/statements_assign_use"),
                );
            }
        }
        pub fn load_iter_statements_assign_thead_local_ref(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, DefPath)> {
            unsafe {
                load_elts_relation::<(Statement, Type, DefPath)>(
                    16347330529269961148u64,
                    self.database_root
                        .join("relations/statements_assign_thead_local_ref"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_thead_local_ref(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type, DefPath)>> {
            if self.statements_assign_thead_local_ref.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, DefPath)>(
                        16347330529269961148u64,
                        self.database_root
                            .join("relations/statements_assign_thead_local_ref"),
                    )
                }
                .unwrap();
                *self.statements_assign_thead_local_ref.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_thead_local_ref.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_thead_local_ref(
            &self,
            facts: Vec<(Statement, Type, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, DefPath)>(
                    facts,
                    16347330529269961148u64,
                    self.database_root
                        .join("relations/statements_assign_thead_local_ref"),
                );
            }
        }
        pub fn store_iter_statements_assign_thead_local_ref(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, DefPath)>(
                    facts,
                    16347330529269961148u64,
                    self.database_root
                        .join("relations/statements_assign_thead_local_ref"),
                );
            }
        }
        pub fn load_iter_statements_assign_repeat(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, Operand, u64)> {
            unsafe {
                load_elts_relation::<(Statement, Type, Operand, u64)>(
                    8337461391872706546u64,
                    self.database_root
                        .join("relations/statements_assign_repeat"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_repeat(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type, Operand, u64)>> {
            if self.statements_assign_repeat.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, Operand, u64)>(
                        8337461391872706546u64,
                        self.database_root
                            .join("relations/statements_assign_repeat"),
                    )
                }
                .unwrap();
                *self.statements_assign_repeat.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_repeat.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_repeat(&self, facts: Vec<(Statement, Type, Operand, u64)>) {
            unsafe {
                save_elts_relation::<(Statement, Type, Operand, u64)>(
                    facts,
                    8337461391872706546u64,
                    self.database_root
                        .join("relations/statements_assign_repeat"),
                );
            }
        }
        pub fn store_iter_statements_assign_repeat(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, Operand, u64)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, Operand, u64)>(
                    facts,
                    8337461391872706546u64,
                    self.database_root
                        .join("relations/statements_assign_repeat"),
                );
            }
        }
        pub fn load_iter_statements_assign_ref(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, Type, BorrowKind)> {
            unsafe {
                load_elts_relation::<(Statement, Type, Type, BorrowKind)>(
                    2996089832552150050u64,
                    self.database_root.join("relations/statements_assign_ref"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_ref(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type, Type, BorrowKind)>> {
            if self.statements_assign_ref.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, Type, BorrowKind)>(
                        2996089832552150050u64,
                        self.database_root.join("relations/statements_assign_ref"),
                    )
                }
                .unwrap();
                *self.statements_assign_ref.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_ref.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_ref(&self, facts: Vec<(Statement, Type, Type, BorrowKind)>) {
            unsafe {
                save_elts_relation::<(Statement, Type, Type, BorrowKind)>(
                    facts,
                    2996089832552150050u64,
                    self.database_root.join("relations/statements_assign_ref"),
                );
            }
        }
        pub fn store_iter_statements_assign_ref(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, Type, BorrowKind)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, Type, BorrowKind)>(
                    facts,
                    2996089832552150050u64,
                    self.database_root.join("relations/statements_assign_ref"),
                );
            }
        }
        pub fn load_iter_statements_assign_address(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, Type, Mutability)> {
            unsafe {
                load_elts_relation::<(Statement, Type, Type, Mutability)>(
                    17409204682030536088u64,
                    self.database_root
                        .join("relations/statements_assign_address"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_address(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type, Type, Mutability)>> {
            if self.statements_assign_address.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, Type, Mutability)>(
                        17409204682030536088u64,
                        self.database_root
                            .join("relations/statements_assign_address"),
                    )
                }
                .unwrap();
                *self.statements_assign_address.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_address.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_address(
            &self,
            facts: Vec<(Statement, Type, Type, Mutability)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, Type, Mutability)>(
                    facts,
                    17409204682030536088u64,
                    self.database_root
                        .join("relations/statements_assign_address"),
                );
            }
        }
        pub fn store_iter_statements_assign_address(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, Type, Mutability)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, Type, Mutability)>(
                    facts,
                    17409204682030536088u64,
                    self.database_root
                        .join("relations/statements_assign_address"),
                );
            }
        }
        pub fn load_iter_statements_assign_len(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, Type)> {
            unsafe {
                load_elts_relation::<(Statement, Type, Type)>(
                    16360895989675975031u64,
                    self.database_root.join("relations/statements_assign_len"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_len(&self) -> std::cell::Ref<Vec<(Statement, Type, Type)>> {
            if self.statements_assign_len.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, Type)>(
                        16360895989675975031u64,
                        self.database_root.join("relations/statements_assign_len"),
                    )
                }
                .unwrap();
                *self.statements_assign_len.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_len.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_len(&self, facts: Vec<(Statement, Type, Type)>) {
            unsafe {
                save_elts_relation::<(Statement, Type, Type)>(
                    facts,
                    16360895989675975031u64,
                    self.database_root.join("relations/statements_assign_len"),
                );
            }
        }
        pub fn store_iter_statements_assign_len(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, Type)>(
                    facts,
                    16360895989675975031u64,
                    self.database_root.join("relations/statements_assign_len"),
                );
            }
        }
        pub fn load_iter_statements_assign_cast(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, CastKind, Operand, Type)> {
            unsafe {
                load_elts_relation::<(Statement, Type, CastKind, Operand, Type)>(
                    10219327660195364344u64,
                    self.database_root.join("relations/statements_assign_cast"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_cast(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type, CastKind, Operand, Type)>> {
            if self.statements_assign_cast.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, CastKind, Operand, Type)>(
                        10219327660195364344u64,
                        self.database_root.join("relations/statements_assign_cast"),
                    )
                }
                .unwrap();
                *self.statements_assign_cast.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_cast.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_cast(
            &self,
            facts: Vec<(Statement, Type, CastKind, Operand, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, CastKind, Operand, Type)>(
                    facts,
                    10219327660195364344u64,
                    self.database_root.join("relations/statements_assign_cast"),
                );
            }
        }
        pub fn store_iter_statements_assign_cast(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, CastKind, Operand, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, CastKind, Operand, Type)>(
                    facts,
                    10219327660195364344u64,
                    self.database_root.join("relations/statements_assign_cast"),
                );
            }
        }
        pub fn load_iter_statements_assign_binary_op(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, BinOp, Operand, Operand)> {
            unsafe {
                load_elts_relation::<(Statement, Type, BinOp, Operand, Operand)>(
                    2738073266338813909u64,
                    self.database_root
                        .join("relations/statements_assign_binary_op"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_binary_op(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type, BinOp, Operand, Operand)>> {
            if self.statements_assign_binary_op.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, BinOp, Operand, Operand)>(
                        2738073266338813909u64,
                        self.database_root
                            .join("relations/statements_assign_binary_op"),
                    )
                }
                .unwrap();
                *self.statements_assign_binary_op.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_binary_op.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_binary_op(
            &self,
            facts: Vec<(Statement, Type, BinOp, Operand, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, BinOp, Operand, Operand)>(
                    facts,
                    2738073266338813909u64,
                    self.database_root
                        .join("relations/statements_assign_binary_op"),
                );
            }
        }
        pub fn store_iter_statements_assign_binary_op(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, BinOp, Operand, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, BinOp, Operand, Operand)>(
                    facts,
                    2738073266338813909u64,
                    self.database_root
                        .join("relations/statements_assign_binary_op"),
                );
            }
        }
        pub fn load_iter_statements_assign_checked_binary_op(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, BinOp, Operand, Operand)> {
            unsafe {
                load_elts_relation::<(Statement, Type, BinOp, Operand, Operand)>(
                    8137280794281117000u64,
                    self.database_root
                        .join("relations/statements_assign_checked_binary_op"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_checked_binary_op(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type, BinOp, Operand, Operand)>> {
            if self.statements_assign_checked_binary_op.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, BinOp, Operand, Operand)>(
                        8137280794281117000u64,
                        self.database_root
                            .join("relations/statements_assign_checked_binary_op"),
                    )
                }
                .unwrap();
                *self.statements_assign_checked_binary_op.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(
                self.statements_assign_checked_binary_op.borrow(),
                |option| option.as_ref().unwrap(),
            )
        }
        pub fn store_statements_assign_checked_binary_op(
            &self,
            facts: Vec<(Statement, Type, BinOp, Operand, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, BinOp, Operand, Operand)>(
                    facts,
                    8137280794281117000u64,
                    self.database_root
                        .join("relations/statements_assign_checked_binary_op"),
                );
            }
        }
        pub fn store_iter_statements_assign_checked_binary_op(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, BinOp, Operand, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, BinOp, Operand, Operand)>(
                    facts,
                    8137280794281117000u64,
                    self.database_root
                        .join("relations/statements_assign_checked_binary_op"),
                );
            }
        }
        pub fn load_iter_statements_assign_nullary_op(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, NullOp, Type)> {
            unsafe {
                load_elts_relation::<(Statement, Type, NullOp, Type)>(
                    4054310599579763631u64,
                    self.database_root
                        .join("relations/statements_assign_nullary_op"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_nullary_op(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type, NullOp, Type)>> {
            if self.statements_assign_nullary_op.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, NullOp, Type)>(
                        4054310599579763631u64,
                        self.database_root
                            .join("relations/statements_assign_nullary_op"),
                    )
                }
                .unwrap();
                *self.statements_assign_nullary_op.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_nullary_op.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_nullary_op(
            &self,
            facts: Vec<(Statement, Type, NullOp, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, NullOp, Type)>(
                    facts,
                    4054310599579763631u64,
                    self.database_root
                        .join("relations/statements_assign_nullary_op"),
                );
            }
        }
        pub fn store_iter_statements_assign_nullary_op(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, NullOp, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, NullOp, Type)>(
                    facts,
                    4054310599579763631u64,
                    self.database_root
                        .join("relations/statements_assign_nullary_op"),
                );
            }
        }
        pub fn load_iter_statements_assign_unary_op(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, UnOp, Operand)> {
            unsafe {
                load_elts_relation::<(Statement, Type, UnOp, Operand)>(
                    2957248607918599194u64,
                    self.database_root
                        .join("relations/statements_assign_unary_op"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_unary_op(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type, UnOp, Operand)>> {
            if self.statements_assign_unary_op.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, UnOp, Operand)>(
                        2957248607918599194u64,
                        self.database_root
                            .join("relations/statements_assign_unary_op"),
                    )
                }
                .unwrap();
                *self.statements_assign_unary_op.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_unary_op.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_unary_op(
            &self,
            facts: Vec<(Statement, Type, UnOp, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, UnOp, Operand)>(
                    facts,
                    2957248607918599194u64,
                    self.database_root
                        .join("relations/statements_assign_unary_op"),
                );
            }
        }
        pub fn store_iter_statements_assign_unary_op(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, UnOp, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, UnOp, Operand)>(
                    facts,
                    2957248607918599194u64,
                    self.database_root
                        .join("relations/statements_assign_unary_op"),
                );
            }
        }
        pub fn load_iter_statements_assign_discriminant(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, Type)> {
            unsafe {
                load_elts_relation::<(Statement, Type, Type)>(
                    11565188595208472216u64,
                    self.database_root
                        .join("relations/statements_assign_discriminant"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_discriminant(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type, Type)>> {
            if self.statements_assign_discriminant.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, Type)>(
                        11565188595208472216u64,
                        self.database_root
                            .join("relations/statements_assign_discriminant"),
                    )
                }
                .unwrap();
                *self.statements_assign_discriminant.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_discriminant.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_discriminant(&self, facts: Vec<(Statement, Type, Type)>) {
            unsafe {
                save_elts_relation::<(Statement, Type, Type)>(
                    facts,
                    11565188595208472216u64,
                    self.database_root
                        .join("relations/statements_assign_discriminant"),
                );
            }
        }
        pub fn store_iter_statements_assign_discriminant(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, Type)>(
                    facts,
                    11565188595208472216u64,
                    self.database_root
                        .join("relations/statements_assign_discriminant"),
                );
            }
        }
        pub fn load_iter_statements_assign_aggregate(
            &self,
        ) -> impl Iterator<Item = (Statement, Type, AggregateKind)> {
            unsafe {
                load_elts_relation::<(Statement, Type, AggregateKind)>(
                    14946105627318220629u64,
                    self.database_root
                        .join("relations/statements_assign_aggregate"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_aggregate(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type, AggregateKind)>> {
            if self.statements_assign_aggregate.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type, AggregateKind)>(
                        14946105627318220629u64,
                        self.database_root
                            .join("relations/statements_assign_aggregate"),
                    )
                }
                .unwrap();
                *self.statements_assign_aggregate.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_aggregate.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_aggregate(
            &self,
            facts: Vec<(Statement, Type, AggregateKind)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, AggregateKind)>(
                    facts,
                    14946105627318220629u64,
                    self.database_root
                        .join("relations/statements_assign_aggregate"),
                );
            }
        }
        pub fn store_iter_statements_assign_aggregate(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type, AggregateKind)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type, AggregateKind)>(
                    facts,
                    14946105627318220629u64,
                    self.database_root
                        .join("relations/statements_assign_aggregate"),
                );
            }
        }
        pub fn load_iter_statements_assign_aggregate_operands(
            &self,
        ) -> impl Iterator<Item = (Statement, OperandIndex, Operand)> {
            unsafe {
                load_elts_relation::<(Statement, OperandIndex, Operand)>(
                    13820671585791218371u64,
                    self.database_root
                        .join("relations/statements_assign_aggregate_operands"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_aggregate_operands(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, OperandIndex, Operand)>> {
            if self.statements_assign_aggregate_operands.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, OperandIndex, Operand)>(
                        13820671585791218371u64,
                        self.database_root
                            .join("relations/statements_assign_aggregate_operands"),
                    )
                }
                .unwrap();
                *self.statements_assign_aggregate_operands.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(
                self.statements_assign_aggregate_operands.borrow(),
                |option| option.as_ref().unwrap(),
            )
        }
        pub fn store_statements_assign_aggregate_operands(
            &self,
            facts: Vec<(Statement, OperandIndex, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, OperandIndex, Operand)>(
                    facts,
                    13820671585791218371u64,
                    self.database_root
                        .join("relations/statements_assign_aggregate_operands"),
                );
            }
        }
        pub fn store_iter_statements_assign_aggregate_operands(
            &self,
            facts: impl IntoIterator<Item = (Statement, OperandIndex, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, OperandIndex, Operand)>(
                    facts,
                    13820671585791218371u64,
                    self.database_root
                        .join("relations/statements_assign_aggregate_operands"),
                );
            }
        }
        pub fn load_iter_statements_assign_shallow_init_box(
            &self,
        ) -> impl Iterator<Item = (Statement, Operand, Type)> {
            unsafe {
                load_elts_relation::<(Statement, Operand, Type)>(
                    12139292931626599282u64,
                    self.database_root
                        .join("relations/statements_assign_shallow_init_box"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_shallow_init_box(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Operand, Type)>> {
            if self.statements_assign_shallow_init_box.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Operand, Type)>(
                        12139292931626599282u64,
                        self.database_root
                            .join("relations/statements_assign_shallow_init_box"),
                    )
                }
                .unwrap();
                *self.statements_assign_shallow_init_box.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_shallow_init_box.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_shallow_init_box(
            &self,
            facts: Vec<(Statement, Operand, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Operand, Type)>(
                    facts,
                    12139292931626599282u64,
                    self.database_root
                        .join("relations/statements_assign_shallow_init_box"),
                );
            }
        }
        pub fn store_iter_statements_assign_shallow_init_box(
            &self,
            facts: impl IntoIterator<Item = (Statement, Operand, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Operand, Type)>(
                    facts,
                    12139292931626599282u64,
                    self.database_root
                        .join("relations/statements_assign_shallow_init_box"),
                );
            }
        }
        pub fn load_iter_statements_assign_copy_for_deref(
            &self,
        ) -> impl Iterator<Item = (Statement, Type)> {
            unsafe {
                load_elts_relation::<(Statement, Type)>(
                    10060289130472003458u64,
                    self.database_root
                        .join("relations/statements_assign_copy_for_deref"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_assign_copy_for_deref(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Type)>> {
            if self.statements_assign_copy_for_deref.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type)>(
                        10060289130472003458u64,
                        self.database_root
                            .join("relations/statements_assign_copy_for_deref"),
                    )
                }
                .unwrap();
                *self.statements_assign_copy_for_deref.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_assign_copy_for_deref.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_assign_copy_for_deref(&self, facts: Vec<(Statement, Type)>) {
            unsafe {
                save_elts_relation::<(Statement, Type)>(
                    facts,
                    10060289130472003458u64,
                    self.database_root
                        .join("relations/statements_assign_copy_for_deref"),
                );
            }
        }
        pub fn store_iter_statements_assign_copy_for_deref(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type)>(
                    facts,
                    10060289130472003458u64,
                    self.database_root
                        .join("relations/statements_assign_copy_for_deref"),
                );
            }
        }
        pub fn load_statements_assign_copy_for_deref_as_map(
            &self,
        ) -> std::collections::HashMap<Statement, Type> {
            self.load_statements_assign_copy_for_deref()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_statements_inline_asm_inputs(
            &self,
        ) -> impl Iterator<Item = (Statement, Operand)> {
            unsafe {
                load_elts_relation::<(Statement, Operand)>(
                    2888522879853589191u64,
                    self.database_root
                        .join("relations/statements_inline_asm_inputs"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_inline_asm_inputs(
            &self,
        ) -> std::cell::Ref<Vec<(Statement, Operand)>> {
            if self.statements_inline_asm_inputs.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Operand)>(
                        2888522879853589191u64,
                        self.database_root
                            .join("relations/statements_inline_asm_inputs"),
                    )
                }
                .unwrap();
                *self.statements_inline_asm_inputs.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_inline_asm_inputs.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_inline_asm_inputs(&self, facts: Vec<(Statement, Operand)>) {
            unsafe {
                save_elts_relation::<(Statement, Operand)>(
                    facts,
                    2888522879853589191u64,
                    self.database_root
                        .join("relations/statements_inline_asm_inputs"),
                );
            }
        }
        pub fn store_iter_statements_inline_asm_inputs(
            &self,
            facts: impl IntoIterator<Item = (Statement, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Operand)>(
                    facts,
                    2888522879853589191u64,
                    self.database_root
                        .join("relations/statements_inline_asm_inputs"),
                );
            }
        }
        pub fn load_statements_inline_asm_inputs_as_map(
            &self,
        ) -> std::collections::HashMap<Statement, Operand> {
            self.load_statements_inline_asm_inputs()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_statements_inline_asm_outputs(
            &self,
        ) -> impl Iterator<Item = (Statement, Type)> {
            unsafe {
                load_elts_relation::<(Statement, Type)>(
                    11349904904689664070u64,
                    self.database_root
                        .join("relations/statements_inline_asm_outputs"),
                )
            }
            .unwrap()
        }
        pub fn load_statements_inline_asm_outputs(&self) -> std::cell::Ref<Vec<(Statement, Type)>> {
            if self.statements_inline_asm_outputs.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Statement, Type)>(
                        11349904904689664070u64,
                        self.database_root
                            .join("relations/statements_inline_asm_outputs"),
                    )
                }
                .unwrap();
                *self.statements_inline_asm_outputs.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.statements_inline_asm_outputs.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_statements_inline_asm_outputs(&self, facts: Vec<(Statement, Type)>) {
            unsafe {
                save_elts_relation::<(Statement, Type)>(
                    facts,
                    11349904904689664070u64,
                    self.database_root
                        .join("relations/statements_inline_asm_outputs"),
                );
            }
        }
        pub fn store_iter_statements_inline_asm_outputs(
            &self,
            facts: impl IntoIterator<Item = (Statement, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Statement, Type)>(
                    facts,
                    11349904904689664070u64,
                    self.database_root
                        .join("relations/statements_inline_asm_outputs"),
                );
            }
        }
        pub fn load_statements_inline_asm_outputs_as_map(
            &self,
        ) -> std::collections::HashMap<Statement, Type> {
            self.load_statements_inline_asm_outputs()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_operands(&self) -> impl Iterator<Item = (Operand, OperandKind, Type)> {
            unsafe {
                load_elts_relation::<(Operand, OperandKind, Type)>(
                    5820950675276320800u64,
                    self.database_root.join("relations/operands"),
                )
            }
            .unwrap()
        }
        pub fn load_operands(&self) -> std::cell::Ref<Vec<(Operand, OperandKind, Type)>> {
            if self.operands.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Operand, OperandKind, Type)>(
                        5820950675276320800u64,
                        self.database_root.join("relations/operands"),
                    )
                }
                .unwrap();
                *self.operands.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.operands.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_operands(&self, facts: Vec<(Operand, OperandKind, Type)>) {
            unsafe {
                save_elts_relation::<(Operand, OperandKind, Type)>(
                    facts,
                    5820950675276320800u64,
                    self.database_root.join("relations/operands"),
                );
            }
        }
        pub fn store_iter_operands(
            &self,
            facts: impl IntoIterator<Item = (Operand, OperandKind, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(Operand, OperandKind, Type)>(
                    facts,
                    5820950675276320800u64,
                    self.database_root.join("relations/operands"),
                );
            }
        }
        pub fn load_iter_terminators(
            &self,
        ) -> impl Iterator<Item = (BasicBlock, TerminatorKind, Scope)> {
            unsafe {
                load_elts_relation::<(BasicBlock, TerminatorKind, Scope)>(
                    6839515169358340218u64,
                    self.database_root.join("relations/terminators"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators(&self) -> std::cell::Ref<Vec<(BasicBlock, TerminatorKind, Scope)>> {
            if self.terminators.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(BasicBlock, TerminatorKind, Scope)>(
                        6839515169358340218u64,
                        self.database_root.join("relations/terminators"),
                    )
                }
                .unwrap();
                *self.terminators.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_terminators(&self, facts: Vec<(BasicBlock, TerminatorKind, Scope)>) {
            unsafe {
                save_elts_relation::<(BasicBlock, TerminatorKind, Scope)>(
                    facts,
                    6839515169358340218u64,
                    self.database_root.join("relations/terminators"),
                );
            }
        }
        pub fn store_iter_terminators(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock, TerminatorKind, Scope)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, TerminatorKind, Scope)>(
                    facts,
                    6839515169358340218u64,
                    self.database_root.join("relations/terminators"),
                );
            }
        }
        pub fn load_iter_terminators_goto(&self) -> impl Iterator<Item = (BasicBlock, BasicBlock)> {
            unsafe {
                load_elts_relation::<(BasicBlock, BasicBlock)>(
                    1991393721502951889u64,
                    self.database_root.join("relations/terminators_goto"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_goto(&self) -> std::cell::Ref<Vec<(BasicBlock, BasicBlock)>> {
            if self.terminators_goto.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(BasicBlock, BasicBlock)>(
                        1991393721502951889u64,
                        self.database_root.join("relations/terminators_goto"),
                    )
                }
                .unwrap();
                *self.terminators_goto.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_goto.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_goto(&self, facts: Vec<(BasicBlock, BasicBlock)>) {
            unsafe {
                save_elts_relation::<(BasicBlock, BasicBlock)>(
                    facts,
                    1991393721502951889u64,
                    self.database_root.join("relations/terminators_goto"),
                );
            }
        }
        pub fn store_iter_terminators_goto(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, BasicBlock)>(
                    facts,
                    1991393721502951889u64,
                    self.database_root.join("relations/terminators_goto"),
                );
            }
        }
        pub fn load_terminators_goto_as_map(
            &self,
        ) -> std::collections::HashMap<BasicBlock, BasicBlock> {
            self.load_terminators_goto().iter().copied().collect()
        }
        pub fn load_iter_terminators_switch_int(
            &self,
        ) -> impl Iterator<Item = (BasicBlock, Operand)> {
            unsafe {
                load_elts_relation::<(BasicBlock, Operand)>(
                    15785734198669646692u64,
                    self.database_root.join("relations/terminators_switch_int"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_switch_int(&self) -> std::cell::Ref<Vec<(BasicBlock, Operand)>> {
            if self.terminators_switch_int.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(BasicBlock, Operand)>(
                        15785734198669646692u64,
                        self.database_root.join("relations/terminators_switch_int"),
                    )
                }
                .unwrap();
                *self.terminators_switch_int.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_switch_int.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_switch_int(&self, facts: Vec<(BasicBlock, Operand)>) {
            unsafe {
                save_elts_relation::<(BasicBlock, Operand)>(
                    facts,
                    15785734198669646692u64,
                    self.database_root.join("relations/terminators_switch_int"),
                );
            }
        }
        pub fn store_iter_terminators_switch_int(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, Operand)>(
                    facts,
                    15785734198669646692u64,
                    self.database_root.join("relations/terminators_switch_int"),
                );
            }
        }
        pub fn load_terminators_switch_int_as_map(
            &self,
        ) -> std::collections::HashMap<BasicBlock, Operand> {
            self.load_terminators_switch_int().iter().copied().collect()
        }
        pub fn load_iter_terminators_switch_int_targets(
            &self,
        ) -> impl Iterator<Item = (BasicBlock, u128, BasicBlock)> {
            unsafe {
                load_elts_relation::<(BasicBlock, u128, BasicBlock)>(
                    13713850366921519370u64,
                    self.database_root
                        .join("relations/terminators_switch_int_targets"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_switch_int_targets(
            &self,
        ) -> std::cell::Ref<Vec<(BasicBlock, u128, BasicBlock)>> {
            if self.terminators_switch_int_targets.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(BasicBlock, u128, BasicBlock)>(
                        13713850366921519370u64,
                        self.database_root
                            .join("relations/terminators_switch_int_targets"),
                    )
                }
                .unwrap();
                *self.terminators_switch_int_targets.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_switch_int_targets.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_switch_int_targets(
            &self,
            facts: Vec<(BasicBlock, u128, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, u128, BasicBlock)>(
                    facts,
                    13713850366921519370u64,
                    self.database_root
                        .join("relations/terminators_switch_int_targets"),
                );
            }
        }
        pub fn store_iter_terminators_switch_int_targets(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock, u128, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, u128, BasicBlock)>(
                    facts,
                    13713850366921519370u64,
                    self.database_root
                        .join("relations/terminators_switch_int_targets"),
                );
            }
        }
        pub fn load_iter_terminators_drop(
            &self,
        ) -> impl Iterator<Item = (BasicBlock, Type, BasicBlock)> {
            unsafe {
                load_elts_relation::<(BasicBlock, Type, BasicBlock)>(
                    5304576143060721589u64,
                    self.database_root.join("relations/terminators_drop"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_drop(&self) -> std::cell::Ref<Vec<(BasicBlock, Type, BasicBlock)>> {
            if self.terminators_drop.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(BasicBlock, Type, BasicBlock)>(
                        5304576143060721589u64,
                        self.database_root.join("relations/terminators_drop"),
                    )
                }
                .unwrap();
                *self.terminators_drop.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_drop.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_drop(&self, facts: Vec<(BasicBlock, Type, BasicBlock)>) {
            unsafe {
                save_elts_relation::<(BasicBlock, Type, BasicBlock)>(
                    facts,
                    5304576143060721589u64,
                    self.database_root.join("relations/terminators_drop"),
                );
            }
        }
        pub fn store_iter_terminators_drop(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock, Type, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, Type, BasicBlock)>(
                    facts,
                    5304576143060721589u64,
                    self.database_root.join("relations/terminators_drop"),
                );
            }
        }
        pub fn load_iter_terminators_drop_and_replace(
            &self,
        ) -> impl Iterator<Item = (BasicBlock, Type, Operand, BasicBlock, BasicBlock)> {
            unsafe {
                load_elts_relation::<(BasicBlock, Type, Operand, BasicBlock, BasicBlock)>(
                    14218106354975354834u64,
                    self.database_root
                        .join("relations/terminators_drop_and_replace"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_drop_and_replace(
            &self,
        ) -> std::cell::Ref<Vec<(BasicBlock, Type, Operand, BasicBlock, BasicBlock)>> {
            if self.terminators_drop_and_replace.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        BasicBlock,
                        Type,
                        Operand,
                        BasicBlock,
                        BasicBlock,
                    )>(
                        14218106354975354834u64,
                        self.database_root
                            .join("relations/terminators_drop_and_replace"),
                    )
                }
                .unwrap();
                *self.terminators_drop_and_replace.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_drop_and_replace.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_drop_and_replace(
            &self,
            facts: Vec<(BasicBlock, Type, Operand, BasicBlock, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, Type, Operand, BasicBlock, BasicBlock)>(
                    facts,
                    14218106354975354834u64,
                    self.database_root
                        .join("relations/terminators_drop_and_replace"),
                );
            }
        }
        pub fn store_iter_terminators_drop_and_replace(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock, Type, Operand, BasicBlock, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, Type, Operand, BasicBlock, BasicBlock)>(
                    facts,
                    14218106354975354834u64,
                    self.database_root
                        .join("relations/terminators_drop_and_replace"),
                );
            }
        }
        pub fn load_iter_terminators_call(
            &self,
        ) -> impl Iterator<
            Item = (
                BasicBlock,
                FunctionCall,
                Operand,
                Safety,
                Abi,
                Type,
                BasicBlock,
                Span,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    BasicBlock,
                    FunctionCall,
                    Operand,
                    Safety,
                    Abi,
                    Type,
                    BasicBlock,
                    Span,
                )>(
                    1605498164985976836u64,
                    self.database_root.join("relations/terminators_call"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_call(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                BasicBlock,
                FunctionCall,
                Operand,
                Safety,
                Abi,
                Type,
                BasicBlock,
                Span,
            )>,
        > {
            if self.terminators_call.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        BasicBlock,
                        FunctionCall,
                        Operand,
                        Safety,
                        Abi,
                        Type,
                        BasicBlock,
                        Span,
                    )>(
                        1605498164985976836u64,
                        self.database_root.join("relations/terminators_call"),
                    )
                }
                .unwrap();
                *self.terminators_call.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_call.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_call(
            &self,
            facts: Vec<(
                BasicBlock,
                FunctionCall,
                Operand,
                Safety,
                Abi,
                Type,
                BasicBlock,
                Span,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    BasicBlock,
                    FunctionCall,
                    Operand,
                    Safety,
                    Abi,
                    Type,
                    BasicBlock,
                    Span,
                )>(
                    facts,
                    1605498164985976836u64,
                    self.database_root.join("relations/terminators_call"),
                );
            }
        }
        pub fn store_iter_terminators_call(
            &self,
            facts: impl IntoIterator<
                Item = (
                    BasicBlock,
                    FunctionCall,
                    Operand,
                    Safety,
                    Abi,
                    Type,
                    BasicBlock,
                    Span,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    BasicBlock,
                    FunctionCall,
                    Operand,
                    Safety,
                    Abi,
                    Type,
                    BasicBlock,
                    Span,
                )>(
                    facts,
                    1605498164985976836u64,
                    self.database_root.join("relations/terminators_call"),
                );
            }
        }
        pub fn load_iter_terminators_call_arg(
            &self,
        ) -> impl Iterator<Item = (FunctionCall, CallArgIndex, Operand)> {
            unsafe {
                load_elts_relation::<(FunctionCall, CallArgIndex, Operand)>(
                    6170710286487383155u64,
                    self.database_root.join("relations/terminators_call_arg"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_call_arg(
            &self,
        ) -> std::cell::Ref<Vec<(FunctionCall, CallArgIndex, Operand)>> {
            if self.terminators_call_arg.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(FunctionCall, CallArgIndex, Operand)>(
                        6170710286487383155u64,
                        self.database_root.join("relations/terminators_call_arg"),
                    )
                }
                .unwrap();
                *self.terminators_call_arg.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_call_arg.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_call_arg(
            &self,
            facts: Vec<(FunctionCall, CallArgIndex, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(FunctionCall, CallArgIndex, Operand)>(
                    facts,
                    6170710286487383155u64,
                    self.database_root.join("relations/terminators_call_arg"),
                );
            }
        }
        pub fn store_iter_terminators_call_arg(
            &self,
            facts: impl IntoIterator<Item = (FunctionCall, CallArgIndex, Operand)>,
        ) {
            unsafe {
                save_elts_relation::<(FunctionCall, CallArgIndex, Operand)>(
                    facts,
                    6170710286487383155u64,
                    self.database_root.join("relations/terminators_call_arg"),
                );
            }
        }
        pub fn load_iter_terminators_call_const_target(
            &self,
        ) -> impl Iterator<Item = (FunctionCall, DefPath)> {
            unsafe {
                load_elts_relation::<(FunctionCall, DefPath)>(
                    5563931411730582203u64,
                    self.database_root
                        .join("relations/terminators_call_const_target"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_call_const_target(
            &self,
        ) -> std::cell::Ref<Vec<(FunctionCall, DefPath)>> {
            if self.terminators_call_const_target.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(FunctionCall, DefPath)>(
                        5563931411730582203u64,
                        self.database_root
                            .join("relations/terminators_call_const_target"),
                    )
                }
                .unwrap();
                *self.terminators_call_const_target.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_call_const_target.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_call_const_target(&self, facts: Vec<(FunctionCall, DefPath)>) {
            unsafe {
                save_elts_relation::<(FunctionCall, DefPath)>(
                    facts,
                    5563931411730582203u64,
                    self.database_root
                        .join("relations/terminators_call_const_target"),
                );
            }
        }
        pub fn store_iter_terminators_call_const_target(
            &self,
            facts: impl IntoIterator<Item = (FunctionCall, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(FunctionCall, DefPath)>(
                    facts,
                    5563931411730582203u64,
                    self.database_root
                        .join("relations/terminators_call_const_target"),
                );
            }
        }
        pub fn load_terminators_call_const_target_as_map(
            &self,
        ) -> std::collections::HashMap<FunctionCall, DefPath> {
            self.load_terminators_call_const_target()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_terminators_call_const_target_desc(
            &self,
        ) -> impl Iterator<Item = (FunctionCall, InternedString, InternedString, InternedString)>
        {
            unsafe { load_elts_relation :: < (FunctionCall , InternedString , InternedString , InternedString ,) > (382182623806615358u64 , self . database_root . join ("relations/terminators_call_const_target_desc")) } . unwrap ()
        }
        pub fn load_terminators_call_const_target_desc(
            &self,
        ) -> std::cell::Ref<Vec<(FunctionCall, InternedString, InternedString, InternedString)>>
        {
            if self.terminators_call_const_target_desc.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        FunctionCall,
                        InternedString,
                        InternedString,
                        InternedString,
                    )>(
                        382182623806615358u64,
                        self.database_root
                            .join("relations/terminators_call_const_target_desc"),
                    )
                }
                .unwrap();
                *self.terminators_call_const_target_desc.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_call_const_target_desc.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_call_const_target_desc(
            &self,
            facts: Vec<(FunctionCall, InternedString, InternedString, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(FunctionCall, InternedString, InternedString, InternedString)>(
                    facts,
                    382182623806615358u64,
                    self.database_root
                        .join("relations/terminators_call_const_target_desc"),
                );
            }
        }
        pub fn store_iter_terminators_call_const_target_desc(
            &self,
            facts: impl IntoIterator<
                Item = (FunctionCall, InternedString, InternedString, InternedString),
            >,
        ) {
            unsafe {
                save_elts_relation::<(FunctionCall, InternedString, InternedString, InternedString)>(
                    facts,
                    382182623806615358u64,
                    self.database_root
                        .join("relations/terminators_call_const_target_desc"),
                );
            }
        }
        pub fn load_iter_terminators_call_const_target_self(
            &self,
        ) -> impl Iterator<Item = (FunctionCall, Type)> {
            unsafe {
                load_elts_relation::<(FunctionCall, Type)>(
                    14690599567700872521u64,
                    self.database_root
                        .join("relations/terminators_call_const_target_self"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_call_const_target_self(
            &self,
        ) -> std::cell::Ref<Vec<(FunctionCall, Type)>> {
            if self.terminators_call_const_target_self.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(FunctionCall, Type)>(
                        14690599567700872521u64,
                        self.database_root
                            .join("relations/terminators_call_const_target_self"),
                    )
                }
                .unwrap();
                *self.terminators_call_const_target_self.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_call_const_target_self.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_call_const_target_self(&self, facts: Vec<(FunctionCall, Type)>) {
            unsafe {
                save_elts_relation::<(FunctionCall, Type)>(
                    facts,
                    14690599567700872521u64,
                    self.database_root
                        .join("relations/terminators_call_const_target_self"),
                );
            }
        }
        pub fn store_iter_terminators_call_const_target_self(
            &self,
            facts: impl IntoIterator<Item = (FunctionCall, Type)>,
        ) {
            unsafe {
                save_elts_relation::<(FunctionCall, Type)>(
                    facts,
                    14690599567700872521u64,
                    self.database_root
                        .join("relations/terminators_call_const_target_self"),
                );
            }
        }
        pub fn load_terminators_call_const_target_self_as_map(
            &self,
        ) -> std::collections::HashMap<FunctionCall, Type> {
            self.load_terminators_call_const_target_self()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_terminators_call_macro_backtrace(
            &self,
        ) -> impl Iterator<Item = (FunctionCall, InternedString)> {
            unsafe {
                load_elts_relation::<(FunctionCall, InternedString)>(
                    12477181522691837357u64,
                    self.database_root
                        .join("relations/terminators_call_macro_backtrace"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_call_macro_backtrace(
            &self,
        ) -> std::cell::Ref<Vec<(FunctionCall, InternedString)>> {
            if self.terminators_call_macro_backtrace.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(FunctionCall, InternedString)>(
                        12477181522691837357u64,
                        self.database_root
                            .join("relations/terminators_call_macro_backtrace"),
                    )
                }
                .unwrap();
                *self.terminators_call_macro_backtrace.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_call_macro_backtrace.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_call_macro_backtrace(
            &self,
            facts: Vec<(FunctionCall, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(FunctionCall, InternedString)>(
                    facts,
                    12477181522691837357u64,
                    self.database_root
                        .join("relations/terminators_call_macro_backtrace"),
                );
            }
        }
        pub fn store_iter_terminators_call_macro_backtrace(
            &self,
            facts: impl IntoIterator<Item = (FunctionCall, InternedString)>,
        ) {
            unsafe {
                save_elts_relation::<(FunctionCall, InternedString)>(
                    facts,
                    12477181522691837357u64,
                    self.database_root
                        .join("relations/terminators_call_macro_backtrace"),
                );
            }
        }
        pub fn load_terminators_call_macro_backtrace_as_map(
            &self,
        ) -> std::collections::HashMap<FunctionCall, InternedString> {
            self.load_terminators_call_macro_backtrace()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_terminators_assert(
            &self,
        ) -> impl Iterator<Item = (BasicBlock, Operand, bool, BasicBlock)> {
            unsafe {
                load_elts_relation::<(BasicBlock, Operand, bool, BasicBlock)>(
                    16835969248810489678u64,
                    self.database_root.join("relations/terminators_assert"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_assert(
            &self,
        ) -> std::cell::Ref<Vec<(BasicBlock, Operand, bool, BasicBlock)>> {
            if self.terminators_assert.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(BasicBlock, Operand, bool, BasicBlock)>(
                        16835969248810489678u64,
                        self.database_root.join("relations/terminators_assert"),
                    )
                }
                .unwrap();
                *self.terminators_assert.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_assert.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_assert(
            &self,
            facts: Vec<(BasicBlock, Operand, bool, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, Operand, bool, BasicBlock)>(
                    facts,
                    16835969248810489678u64,
                    self.database_root.join("relations/terminators_assert"),
                );
            }
        }
        pub fn store_iter_terminators_assert(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock, Operand, bool, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, Operand, bool, BasicBlock)>(
                    facts,
                    16835969248810489678u64,
                    self.database_root.join("relations/terminators_assert"),
                );
            }
        }
        pub fn load_iter_terminators_yield(
            &self,
        ) -> impl Iterator<Item = (BasicBlock, Operand, BasicBlock, BasicBlock)> {
            unsafe {
                load_elts_relation::<(BasicBlock, Operand, BasicBlock, BasicBlock)>(
                    8047909410674798880u64,
                    self.database_root.join("relations/terminators_yield"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_yield(
            &self,
        ) -> std::cell::Ref<Vec<(BasicBlock, Operand, BasicBlock, BasicBlock)>> {
            if self.terminators_yield.borrow().is_none() {
                let relation =
                    unsafe {
                        load_elts_relation_into_relation::<(
                            BasicBlock,
                            Operand,
                            BasicBlock,
                            BasicBlock,
                        )>(
                            8047909410674798880u64,
                            self.database_root.join("relations/terminators_yield"),
                        )
                    }
                    .unwrap();
                *self.terminators_yield.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_yield.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_yield(
            &self,
            facts: Vec<(BasicBlock, Operand, BasicBlock, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, Operand, BasicBlock, BasicBlock)>(
                    facts,
                    8047909410674798880u64,
                    self.database_root.join("relations/terminators_yield"),
                );
            }
        }
        pub fn store_iter_terminators_yield(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock, Operand, BasicBlock, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, Operand, BasicBlock, BasicBlock)>(
                    facts,
                    8047909410674798880u64,
                    self.database_root.join("relations/terminators_yield"),
                );
            }
        }
        pub fn load_iter_terminators_false_edges(
            &self,
        ) -> impl Iterator<Item = (BasicBlock, BasicBlock, BasicBlock)> {
            unsafe {
                load_elts_relation::<(BasicBlock, BasicBlock, BasicBlock)>(
                    12070114738280431417u64,
                    self.database_root.join("relations/terminators_false_edges"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_false_edges(
            &self,
        ) -> std::cell::Ref<Vec<(BasicBlock, BasicBlock, BasicBlock)>> {
            if self.terminators_false_edges.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(BasicBlock, BasicBlock, BasicBlock)>(
                        12070114738280431417u64,
                        self.database_root.join("relations/terminators_false_edges"),
                    )
                }
                .unwrap();
                *self.terminators_false_edges.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_false_edges.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_false_edges(
            &self,
            facts: Vec<(BasicBlock, BasicBlock, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, BasicBlock, BasicBlock)>(
                    facts,
                    12070114738280431417u64,
                    self.database_root.join("relations/terminators_false_edges"),
                );
            }
        }
        pub fn store_iter_terminators_false_edges(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock, BasicBlock, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, BasicBlock, BasicBlock)>(
                    facts,
                    12070114738280431417u64,
                    self.database_root.join("relations/terminators_false_edges"),
                );
            }
        }
        pub fn load_iter_terminators_false_unwind(
            &self,
        ) -> impl Iterator<Item = (BasicBlock, BasicBlock)> {
            unsafe {
                load_elts_relation::<(BasicBlock, BasicBlock)>(
                    12046788150434641704u64,
                    self.database_root
                        .join("relations/terminators_false_unwind"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_false_unwind(
            &self,
        ) -> std::cell::Ref<Vec<(BasicBlock, BasicBlock)>> {
            if self.terminators_false_unwind.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(BasicBlock, BasicBlock)>(
                        12046788150434641704u64,
                        self.database_root
                            .join("relations/terminators_false_unwind"),
                    )
                }
                .unwrap();
                *self.terminators_false_unwind.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_false_unwind.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_false_unwind(&self, facts: Vec<(BasicBlock, BasicBlock)>) {
            unsafe {
                save_elts_relation::<(BasicBlock, BasicBlock)>(
                    facts,
                    12046788150434641704u64,
                    self.database_root
                        .join("relations/terminators_false_unwind"),
                );
            }
        }
        pub fn store_iter_terminators_false_unwind(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, BasicBlock)>(
                    facts,
                    12046788150434641704u64,
                    self.database_root
                        .join("relations/terminators_false_unwind"),
                );
            }
        }
        pub fn load_terminators_false_unwind_as_map(
            &self,
        ) -> std::collections::HashMap<BasicBlock, BasicBlock> {
            self.load_terminators_false_unwind()
                .iter()
                .copied()
                .collect()
        }
        pub fn load_iter_terminators_inline_asm(&self) -> impl Iterator<Item = (BasicBlock,)> {
            unsafe {
                load_elts_relation::<(BasicBlock,)>(
                    9543573422760034863u64,
                    self.database_root.join("relations/terminators_inline_asm"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_inline_asm(&self) -> std::cell::Ref<Vec<(BasicBlock,)>> {
            if self.terminators_inline_asm.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(BasicBlock,)>(
                        9543573422760034863u64,
                        self.database_root.join("relations/terminators_inline_asm"),
                    )
                }
                .unwrap();
                *self.terminators_inline_asm.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_inline_asm.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_inline_asm(&self, facts: Vec<(BasicBlock,)>) {
            unsafe {
                save_elts_relation::<(BasicBlock,)>(
                    facts,
                    9543573422760034863u64,
                    self.database_root.join("relations/terminators_inline_asm"),
                );
            }
        }
        pub fn store_iter_terminators_inline_asm(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock,)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock,)>(
                    facts,
                    9543573422760034863u64,
                    self.database_root.join("relations/terminators_inline_asm"),
                );
            }
        }
        pub fn load_iter_terminators_unwind_action(
            &self,
        ) -> impl Iterator<Item = (BasicBlock, UnwindAction, BasicBlock)> {
            unsafe {
                load_elts_relation::<(BasicBlock, UnwindAction, BasicBlock)>(
                    265964774939904594u64,
                    self.database_root
                        .join("relations/terminators_unwind_action"),
                )
            }
            .unwrap()
        }
        pub fn load_terminators_unwind_action(
            &self,
        ) -> std::cell::Ref<Vec<(BasicBlock, UnwindAction, BasicBlock)>> {
            if self.terminators_unwind_action.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(BasicBlock, UnwindAction, BasicBlock)>(
                        265964774939904594u64,
                        self.database_root
                            .join("relations/terminators_unwind_action"),
                    )
                }
                .unwrap();
                *self.terminators_unwind_action.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.terminators_unwind_action.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_terminators_unwind_action(
            &self,
            facts: Vec<(BasicBlock, UnwindAction, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, UnwindAction, BasicBlock)>(
                    facts,
                    265964774939904594u64,
                    self.database_root
                        .join("relations/terminators_unwind_action"),
                );
            }
        }
        pub fn store_iter_terminators_unwind_action(
            &self,
            facts: impl IntoIterator<Item = (BasicBlock, UnwindAction, BasicBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(BasicBlock, UnwindAction, BasicBlock)>(
                    facts,
                    265964774939904594u64,
                    self.database_root
                        .join("relations/terminators_unwind_action"),
                );
            }
        }
        pub fn load_iter_selected_builds(
            &self,
        ) -> impl Iterator<Item = (Build, Package, PackageVersion, Krate, CrateHash, Edition)>
        {
            unsafe {
                load_elts_relation::<(Build, Package, PackageVersion, Krate, CrateHash, Edition)>(
                    10051876189233685301u64,
                    self.database_root.join("relations/selected_builds"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_builds(
            &self,
        ) -> std::cell::Ref<Vec<(Build, Package, PackageVersion, Krate, CrateHash, Edition)>>
        {
            if self.selected_builds.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        Package,
                        PackageVersion,
                        Krate,
                        CrateHash,
                        Edition,
                    )>(
                        10051876189233685301u64,
                        self.database_root.join("relations/selected_builds"),
                    )
                }
                .unwrap();
                *self.selected_builds.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_builds.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_builds(
            &self,
            facts: Vec<(Build, Package, PackageVersion, Krate, CrateHash, Edition)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Package, PackageVersion, Krate, CrateHash, Edition)>(
                    facts,
                    10051876189233685301u64,
                    self.database_root.join("relations/selected_builds"),
                );
            }
        }
        pub fn store_iter_selected_builds(
            &self,
            facts: impl IntoIterator<Item = (Build, Package, PackageVersion, Krate, CrateHash, Edition)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Package, PackageVersion, Krate, CrateHash, Edition)>(
                    facts,
                    10051876189233685301u64,
                    self.database_root.join("relations/selected_builds"),
                );
            }
        }
        pub fn load_iter_build_script_builds(
            &self,
        ) -> impl Iterator<Item = (Build, Package, PackageVersion, Krate, CrateHash, Edition)>
        {
            unsafe {
                load_elts_relation::<(Build, Package, PackageVersion, Krate, CrateHash, Edition)>(
                    399587277863080475u64,
                    self.database_root.join("relations/build_script_builds"),
                )
            }
            .unwrap()
        }
        pub fn load_build_script_builds(
            &self,
        ) -> std::cell::Ref<Vec<(Build, Package, PackageVersion, Krate, CrateHash, Edition)>>
        {
            if self.build_script_builds.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        Package,
                        PackageVersion,
                        Krate,
                        CrateHash,
                        Edition,
                    )>(
                        399587277863080475u64,
                        self.database_root.join("relations/build_script_builds"),
                    )
                }
                .unwrap();
                *self.build_script_builds.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.build_script_builds.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_build_script_builds(
            &self,
            facts: Vec<(Build, Package, PackageVersion, Krate, CrateHash, Edition)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Package, PackageVersion, Krate, CrateHash, Edition)>(
                    facts,
                    399587277863080475u64,
                    self.database_root.join("relations/build_script_builds"),
                );
            }
        }
        pub fn store_iter_build_script_builds(
            &self,
            facts: impl IntoIterator<Item = (Build, Package, PackageVersion, Krate, CrateHash, Edition)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Package, PackageVersion, Krate, CrateHash, Edition)>(
                    facts,
                    399587277863080475u64,
                    self.database_root.join("relations/build_script_builds"),
                );
            }
        }
        pub fn load_iter_selected_modules(&self) -> impl Iterator<Item = (Build, Module)> {
            unsafe {
                load_elts_relation::<(Build, Module)>(
                    15512244175999081383u64,
                    self.database_root.join("relations/selected_modules"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_modules(&self) -> std::cell::Ref<Vec<(Build, Module)>> {
            if self.selected_modules.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, Module)>(
                        15512244175999081383u64,
                        self.database_root.join("relations/selected_modules"),
                    )
                }
                .unwrap();
                *self.selected_modules.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_modules.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_modules(&self, facts: Vec<(Build, Module)>) {
            unsafe {
                save_elts_relation::<(Build, Module)>(
                    facts,
                    15512244175999081383u64,
                    self.database_root.join("relations/selected_modules"),
                );
            }
        }
        pub fn store_iter_selected_modules(
            &self,
            facts: impl IntoIterator<Item = (Build, Module)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Module)>(
                    facts,
                    15512244175999081383u64,
                    self.database_root.join("relations/selected_modules"),
                );
            }
        }
        pub fn load_selected_modules_as_map(&self) -> std::collections::HashMap<Build, Module> {
            self.load_selected_modules().iter().copied().collect()
        }
        pub fn load_iter_selected_mir_cfgs(
            &self,
        ) -> impl Iterator<Item = (Build, Item, DefPath, Scope)> {
            unsafe {
                load_elts_relation::<(Build, Item, DefPath, Scope)>(
                    1959701309029222908u64,
                    self.database_root.join("relations/selected_mir_cfgs"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_mir_cfgs(&self) -> std::cell::Ref<Vec<(Build, Item, DefPath, Scope)>> {
            if self.selected_mir_cfgs.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, Item, DefPath, Scope)>(
                        1959701309029222908u64,
                        self.database_root.join("relations/selected_mir_cfgs"),
                    )
                }
                .unwrap();
                *self.selected_mir_cfgs.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_mir_cfgs.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_mir_cfgs(&self, facts: Vec<(Build, Item, DefPath, Scope)>) {
            unsafe {
                save_elts_relation::<(Build, Item, DefPath, Scope)>(
                    facts,
                    1959701309029222908u64,
                    self.database_root.join("relations/selected_mir_cfgs"),
                );
            }
        }
        pub fn store_iter_selected_mir_cfgs(
            &self,
            facts: impl IntoIterator<Item = (Build, Item, DefPath, Scope)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Item, DefPath, Scope)>(
                    facts,
                    1959701309029222908u64,
                    self.database_root.join("relations/selected_mir_cfgs"),
                );
            }
        }
        pub fn load_iter_selected_scopes(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                DefPath,
                Scope,
                Scope,
                ScopeSafety,
                u32,
                BlockCheckMode,
                Span,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    DefPath,
                    Scope,
                    Scope,
                    ScopeSafety,
                    u32,
                    BlockCheckMode,
                    Span,
                )>(
                    8245861121520064924u64,
                    self.database_root.join("relations/selected_scopes"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_scopes(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                DefPath,
                Scope,
                Scope,
                ScopeSafety,
                u32,
                BlockCheckMode,
                Span,
            )>,
        > {
            if self.selected_scopes.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        DefPath,
                        Scope,
                        Scope,
                        ScopeSafety,
                        u32,
                        BlockCheckMode,
                        Span,
                    )>(
                        8245861121520064924u64,
                        self.database_root.join("relations/selected_scopes"),
                    )
                }
                .unwrap();
                *self.selected_scopes.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_scopes.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_scopes(
            &self,
            facts: Vec<(
                Build,
                DefPath,
                Scope,
                Scope,
                ScopeSafety,
                u32,
                BlockCheckMode,
                Span,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    DefPath,
                    Scope,
                    Scope,
                    ScopeSafety,
                    u32,
                    BlockCheckMode,
                    Span,
                )>(
                    facts,
                    8245861121520064924u64,
                    self.database_root.join("relations/selected_scopes"),
                );
            }
        }
        pub fn store_iter_selected_scopes(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    DefPath,
                    Scope,
                    Scope,
                    ScopeSafety,
                    u32,
                    BlockCheckMode,
                    Span,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    DefPath,
                    Scope,
                    Scope,
                    ScopeSafety,
                    u32,
                    BlockCheckMode,
                    Span,
                )>(
                    facts,
                    8245861121520064924u64,
                    self.database_root.join("relations/selected_scopes"),
                );
            }
        }
        pub fn load_iter_selected_type_defs(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                Item,
                Type,
                DefPath,
                InternedString,
                TyVisibility,
                TyKind,
                TyDefKind,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    Item,
                    Type,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                )>(
                    16972394906091325597u64,
                    self.database_root.join("relations/selected_type_defs"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_type_defs(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                Item,
                Type,
                DefPath,
                InternedString,
                TyVisibility,
                TyKind,
                TyDefKind,
            )>,
        > {
            if self.selected_type_defs.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        Item,
                        Type,
                        DefPath,
                        InternedString,
                        TyVisibility,
                        TyKind,
                        TyDefKind,
                    )>(
                        16972394906091325597u64,
                        self.database_root.join("relations/selected_type_defs"),
                    )
                }
                .unwrap();
                *self.selected_type_defs.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_type_defs.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_type_defs(
            &self,
            facts: Vec<(
                Build,
                Item,
                Type,
                DefPath,
                InternedString,
                TyVisibility,
                TyKind,
                TyDefKind,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Item,
                    Type,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                )>(
                    facts,
                    16972394906091325597u64,
                    self.database_root.join("relations/selected_type_defs"),
                );
            }
        }
        pub fn store_iter_selected_type_defs(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    Item,
                    Type,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Item,
                    Type,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                )>(
                    facts,
                    16972394906091325597u64,
                    self.database_root.join("relations/selected_type_defs"),
                );
            }
        }
        pub fn load_iter_selected_adts(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                Item,
                Type,
                DefPath,
                DefPath,
                InternedString,
                TyVisibility,
                TyKind,
                TyDefKind,
                AdtKind,
                bool,
                bool,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    Item,
                    Type,
                    DefPath,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                    AdtKind,
                    bool,
                    bool,
                )>(
                    1187541191248617688u64,
                    self.database_root.join("relations/selected_adts"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_adts(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                Item,
                Type,
                DefPath,
                DefPath,
                InternedString,
                TyVisibility,
                TyKind,
                TyDefKind,
                AdtKind,
                bool,
                bool,
            )>,
        > {
            if self.selected_adts.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        Item,
                        Type,
                        DefPath,
                        DefPath,
                        InternedString,
                        TyVisibility,
                        TyKind,
                        TyDefKind,
                        AdtKind,
                        bool,
                        bool,
                    )>(
                        1187541191248617688u64,
                        self.database_root.join("relations/selected_adts"),
                    )
                }
                .unwrap();
                *self.selected_adts.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_adts.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_adts(
            &self,
            facts: Vec<(
                Build,
                Item,
                Type,
                DefPath,
                DefPath,
                InternedString,
                TyVisibility,
                TyKind,
                TyDefKind,
                AdtKind,
                bool,
                bool,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Item,
                    Type,
                    DefPath,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                    AdtKind,
                    bool,
                    bool,
                )>(
                    facts,
                    1187541191248617688u64,
                    self.database_root.join("relations/selected_adts"),
                );
            }
        }
        pub fn store_iter_selected_adts(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    Item,
                    Type,
                    DefPath,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                    AdtKind,
                    bool,
                    bool,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Item,
                    Type,
                    DefPath,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                    AdtKind,
                    bool,
                    bool,
                )>(
                    facts,
                    1187541191248617688u64,
                    self.database_root.join("relations/selected_adts"),
                );
            }
        }
        pub fn load_iter_selected_adt_field_types(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                Item,
                Type,
                AdtVariantIndex,
                DefPath,
                DefPath,
                DefPath,
                InternedString,
                TyVisibility,
                TyKind,
                TyDefKind,
                AdtKind,
                bool,
                bool,
                InternedString,
                TyVisibility,
                Type,
                TyKind,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    Item,
                    Type,
                    AdtVariantIndex,
                    DefPath,
                    DefPath,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                    AdtKind,
                    bool,
                    bool,
                    InternedString,
                    TyVisibility,
                    Type,
                    TyKind,
                )>(
                    650694669259461563u64,
                    self.database_root
                        .join("relations/selected_adt_field_types"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_adt_field_types(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                Item,
                Type,
                AdtVariantIndex,
                DefPath,
                DefPath,
                DefPath,
                InternedString,
                TyVisibility,
                TyKind,
                TyDefKind,
                AdtKind,
                bool,
                bool,
                InternedString,
                TyVisibility,
                Type,
                TyKind,
            )>,
        > {
            if self.selected_adt_field_types.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        Item,
                        Type,
                        AdtVariantIndex,
                        DefPath,
                        DefPath,
                        DefPath,
                        InternedString,
                        TyVisibility,
                        TyKind,
                        TyDefKind,
                        AdtKind,
                        bool,
                        bool,
                        InternedString,
                        TyVisibility,
                        Type,
                        TyKind,
                    )>(
                        650694669259461563u64,
                        self.database_root
                            .join("relations/selected_adt_field_types"),
                    )
                }
                .unwrap();
                *self.selected_adt_field_types.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_adt_field_types.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_adt_field_types(
            &self,
            facts: Vec<(
                Build,
                Item,
                Type,
                AdtVariantIndex,
                DefPath,
                DefPath,
                DefPath,
                InternedString,
                TyVisibility,
                TyKind,
                TyDefKind,
                AdtKind,
                bool,
                bool,
                InternedString,
                TyVisibility,
                Type,
                TyKind,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Item,
                    Type,
                    AdtVariantIndex,
                    DefPath,
                    DefPath,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                    AdtKind,
                    bool,
                    bool,
                    InternedString,
                    TyVisibility,
                    Type,
                    TyKind,
                )>(
                    facts,
                    650694669259461563u64,
                    self.database_root
                        .join("relations/selected_adt_field_types"),
                );
            }
        }
        pub fn store_iter_selected_adt_field_types(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    Item,
                    Type,
                    AdtVariantIndex,
                    DefPath,
                    DefPath,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                    AdtKind,
                    bool,
                    bool,
                    InternedString,
                    TyVisibility,
                    Type,
                    TyKind,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Item,
                    Type,
                    AdtVariantIndex,
                    DefPath,
                    DefPath,
                    DefPath,
                    InternedString,
                    TyVisibility,
                    TyKind,
                    TyDefKind,
                    AdtKind,
                    bool,
                    bool,
                    InternedString,
                    TyVisibility,
                    Type,
                    TyKind,
                )>(
                    facts,
                    650694669259461563u64,
                    self.database_root
                        .join("relations/selected_adt_field_types"),
                );
            }
        }
        pub fn load_iter_types_unsafe_cell(&self) -> impl Iterator<Item = (Type, DefPath)> {
            unsafe {
                load_elts_relation::<(Type, DefPath)>(
                    13211623985391409428u64,
                    self.database_root.join("relations/types_unsafe_cell"),
                )
            }
            .unwrap()
        }
        pub fn load_types_unsafe_cell(&self) -> std::cell::Ref<Vec<(Type, DefPath)>> {
            if self.types_unsafe_cell.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath)>(
                        13211623985391409428u64,
                        self.database_root.join("relations/types_unsafe_cell"),
                    )
                }
                .unwrap();
                *self.types_unsafe_cell.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_unsafe_cell.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_types_unsafe_cell(&self, facts: Vec<(Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    13211623985391409428u64,
                    self.database_root.join("relations/types_unsafe_cell"),
                );
            }
        }
        pub fn store_iter_types_unsafe_cell(
            &self,
            facts: impl IntoIterator<Item = (Type, DefPath)>,
        ) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    13211623985391409428u64,
                    self.database_root.join("relations/types_unsafe_cell"),
                );
            }
        }
        pub fn load_types_unsafe_cell_as_map(&self) -> std::collections::HashMap<Type, DefPath> {
            self.load_types_unsafe_cell().iter().copied().collect()
        }
        pub fn load_iter_types_union(&self) -> impl Iterator<Item = (Type, DefPath)> {
            unsafe {
                load_elts_relation::<(Type, DefPath)>(
                    6806343431451474689u64,
                    self.database_root.join("relations/types_union"),
                )
            }
            .unwrap()
        }
        pub fn load_types_union(&self) -> std::cell::Ref<Vec<(Type, DefPath)>> {
            if self.types_union.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type, DefPath)>(
                        6806343431451474689u64,
                        self.database_root.join("relations/types_union"),
                    )
                }
                .unwrap();
                *self.types_union.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.types_union.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn store_types_union(&self, facts: Vec<(Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    6806343431451474689u64,
                    self.database_root.join("relations/types_union"),
                );
            }
        }
        pub fn store_iter_types_union(&self, facts: impl IntoIterator<Item = (Type, DefPath)>) {
            unsafe {
                save_elts_relation::<(Type, DefPath)>(
                    facts,
                    6806343431451474689u64,
                    self.database_root.join("relations/types_union"),
                );
            }
        }
        pub fn load_types_union_as_map(&self) -> std::collections::HashMap<Type, DefPath> {
            self.load_types_union().iter().copied().collect()
        }
        pub fn load_iter_unsafe_types(&self) -> impl Iterator<Item = (Type,)> {
            unsafe {
                load_elts_relation::<(Type,)>(
                    17729773816657051031u64,
                    self.database_root.join("relations/unsafe_types"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_types(&self) -> std::cell::Ref<Vec<(Type,)>> {
            if self.unsafe_types.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type,)>(
                        17729773816657051031u64,
                        self.database_root.join("relations/unsafe_types"),
                    )
                }
                .unwrap();
                *self.unsafe_types.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_types.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_types(&self, facts: Vec<(Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    17729773816657051031u64,
                    self.database_root.join("relations/unsafe_types"),
                );
            }
        }
        pub fn store_iter_unsafe_types(&self, facts: impl IntoIterator<Item = (Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    17729773816657051031u64,
                    self.database_root.join("relations/unsafe_types"),
                );
            }
        }
        pub fn load_iter_safe_wrapper_types(&self) -> impl Iterator<Item = (Type,)> {
            unsafe {
                load_elts_relation::<(Type,)>(
                    15640513272229994984u64,
                    self.database_root.join("relations/safe_wrapper_types"),
                )
            }
            .unwrap()
        }
        pub fn load_safe_wrapper_types(&self) -> std::cell::Ref<Vec<(Type,)>> {
            if self.safe_wrapper_types.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Type,)>(
                        15640513272229994984u64,
                        self.database_root.join("relations/safe_wrapper_types"),
                    )
                }
                .unwrap();
                *self.safe_wrapper_types.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.safe_wrapper_types.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_safe_wrapper_types(&self, facts: Vec<(Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    15640513272229994984u64,
                    self.database_root.join("relations/safe_wrapper_types"),
                );
            }
        }
        pub fn store_iter_safe_wrapper_types(&self, facts: impl IntoIterator<Item = (Type,)>) {
            unsafe {
                save_elts_relation::<(Type,)>(
                    facts,
                    15640513272229994984u64,
                    self.database_root.join("relations/safe_wrapper_types"),
                );
            }
        }
        pub fn load_iter_unsafe_blocks(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                DefPath,
                Scope,
                SpanExpansionKind,
                BlockCheckMode,
                Span,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    DefPath,
                    Scope,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>(
                    18310647019217452649u64,
                    self.database_root.join("relations/unsafe_blocks"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_blocks(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                DefPath,
                Scope,
                SpanExpansionKind,
                BlockCheckMode,
                Span,
            )>,
        > {
            if self.unsafe_blocks.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        DefPath,
                        Scope,
                        SpanExpansionKind,
                        BlockCheckMode,
                        Span,
                    )>(
                        18310647019217452649u64,
                        self.database_root.join("relations/unsafe_blocks"),
                    )
                }
                .unwrap();
                *self.unsafe_blocks.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_blocks.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_blocks(
            &self,
            facts: Vec<(
                Build,
                DefPath,
                Scope,
                SpanExpansionKind,
                BlockCheckMode,
                Span,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    DefPath,
                    Scope,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>(
                    facts,
                    18310647019217452649u64,
                    self.database_root.join("relations/unsafe_blocks"),
                );
            }
        }
        pub fn store_iter_unsafe_blocks(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    DefPath,
                    Scope,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    DefPath,
                    Scope,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>(
                    facts,
                    18310647019217452649u64,
                    self.database_root.join("relations/unsafe_blocks"),
                );
            }
        }
        pub fn load_iter_functions_unsafe_blocks(
            &self,
        ) -> impl Iterator<Item = (Build, Item, Scope, SpanExpansionKind, BlockCheckMode)> {
            unsafe {
                load_elts_relation::<(Build, Item, Scope, SpanExpansionKind, BlockCheckMode)>(
                    11657662045173312326u64,
                    self.database_root.join("relations/functions_unsafe_blocks"),
                )
            }
            .unwrap()
        }
        pub fn load_functions_unsafe_blocks(
            &self,
        ) -> std::cell::Ref<Vec<(Build, Item, Scope, SpanExpansionKind, BlockCheckMode)>> {
            if self.functions_unsafe_blocks.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        Item,
                        Scope,
                        SpanExpansionKind,
                        BlockCheckMode,
                    )>(
                        11657662045173312326u64,
                        self.database_root.join("relations/functions_unsafe_blocks"),
                    )
                }
                .unwrap();
                *self.functions_unsafe_blocks.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.functions_unsafe_blocks.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_functions_unsafe_blocks(
            &self,
            facts: Vec<(Build, Item, Scope, SpanExpansionKind, BlockCheckMode)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Item, Scope, SpanExpansionKind, BlockCheckMode)>(
                    facts,
                    11657662045173312326u64,
                    self.database_root.join("relations/functions_unsafe_blocks"),
                );
            }
        }
        pub fn store_iter_functions_unsafe_blocks(
            &self,
            facts: impl IntoIterator<Item = (Build, Item, Scope, SpanExpansionKind, BlockCheckMode)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Item, Scope, SpanExpansionKind, BlockCheckMode)>(
                    facts,
                    11657662045173312326u64,
                    self.database_root.join("relations/functions_unsafe_blocks"),
                );
            }
        }
        pub fn load_iter_selected_function_definitions(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                Item,
                DefPath,
                Module,
                TyVisibility,
                Safety,
                Abi,
                Type,
                bool,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    Item,
                    DefPath,
                    Module,
                    TyVisibility,
                    Safety,
                    Abi,
                    Type,
                    bool,
                )>(
                    13549393036406843162u64,
                    self.database_root
                        .join("relations/selected_function_definitions"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_function_definitions(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                Item,
                DefPath,
                Module,
                TyVisibility,
                Safety,
                Abi,
                Type,
                bool,
            )>,
        > {
            if self.selected_function_definitions.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        Item,
                        DefPath,
                        Module,
                        TyVisibility,
                        Safety,
                        Abi,
                        Type,
                        bool,
                    )>(
                        13549393036406843162u64,
                        self.database_root
                            .join("relations/selected_function_definitions"),
                    )
                }
                .unwrap();
                *self.selected_function_definitions.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_function_definitions.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_function_definitions(
            &self,
            facts: Vec<(
                Build,
                Item,
                DefPath,
                Module,
                TyVisibility,
                Safety,
                Abi,
                Type,
                bool,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Item,
                    DefPath,
                    Module,
                    TyVisibility,
                    Safety,
                    Abi,
                    Type,
                    bool,
                )>(
                    facts,
                    13549393036406843162u64,
                    self.database_root
                        .join("relations/selected_function_definitions"),
                );
            }
        }
        pub fn store_iter_selected_function_definitions(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    Item,
                    DefPath,
                    Module,
                    TyVisibility,
                    Safety,
                    Abi,
                    Type,
                    bool,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Item,
                    DefPath,
                    Module,
                    TyVisibility,
                    Safety,
                    Abi,
                    Type,
                    bool,
                )>(
                    facts,
                    13549393036406843162u64,
                    self.database_root
                        .join("relations/selected_function_definitions"),
                );
            }
        }
        pub fn load_iter_unsafe_statements(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                Statement,
                BasicBlock,
                StatementIndex,
                StatementKind,
                Scope,
                BlockCheckMode,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    Statement,
                    BasicBlock,
                    StatementIndex,
                    StatementKind,
                    Scope,
                    BlockCheckMode,
                )>(
                    8131028990822679293u64,
                    self.database_root.join("relations/unsafe_statements"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_statements(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                Statement,
                BasicBlock,
                StatementIndex,
                StatementKind,
                Scope,
                BlockCheckMode,
            )>,
        > {
            if self.unsafe_statements.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        Statement,
                        BasicBlock,
                        StatementIndex,
                        StatementKind,
                        Scope,
                        BlockCheckMode,
                    )>(
                        8131028990822679293u64,
                        self.database_root.join("relations/unsafe_statements"),
                    )
                }
                .unwrap();
                *self.unsafe_statements.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_statements.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_statements(
            &self,
            facts: Vec<(
                Build,
                Statement,
                BasicBlock,
                StatementIndex,
                StatementKind,
                Scope,
                BlockCheckMode,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Statement,
                    BasicBlock,
                    StatementIndex,
                    StatementKind,
                    Scope,
                    BlockCheckMode,
                )>(
                    facts,
                    8131028990822679293u64,
                    self.database_root.join("relations/unsafe_statements"),
                );
            }
        }
        pub fn store_iter_unsafe_statements(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    Statement,
                    BasicBlock,
                    StatementIndex,
                    StatementKind,
                    Scope,
                    BlockCheckMode,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Statement,
                    BasicBlock,
                    StatementIndex,
                    StatementKind,
                    Scope,
                    BlockCheckMode,
                )>(
                    facts,
                    8131028990822679293u64,
                    self.database_root.join("relations/unsafe_statements"),
                );
            }
        }
        pub fn load_iter_unsafe_terminators(
            &self,
        ) -> impl Iterator<Item = (Build, BasicBlock, TerminatorKind, Scope, BlockCheckMode)>
        {
            unsafe {
                load_elts_relation::<(Build, BasicBlock, TerminatorKind, Scope, BlockCheckMode)>(
                    17515917190007746298u64,
                    self.database_root.join("relations/unsafe_terminators"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_terminators(
            &self,
        ) -> std::cell::Ref<Vec<(Build, BasicBlock, TerminatorKind, Scope, BlockCheckMode)>>
        {
            if self.unsafe_terminators.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        BasicBlock,
                        TerminatorKind,
                        Scope,
                        BlockCheckMode,
                    )>(
                        17515917190007746298u64,
                        self.database_root.join("relations/unsafe_terminators"),
                    )
                }
                .unwrap();
                *self.unsafe_terminators.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_terminators.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_terminators(
            &self,
            facts: Vec<(Build, BasicBlock, TerminatorKind, Scope, BlockCheckMode)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, BasicBlock, TerminatorKind, Scope, BlockCheckMode)>(
                    facts,
                    17515917190007746298u64,
                    self.database_root.join("relations/unsafe_terminators"),
                );
            }
        }
        pub fn store_iter_unsafe_terminators(
            &self,
            facts: impl IntoIterator<Item = (Build, BasicBlock, TerminatorKind, Scope, BlockCheckMode)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, BasicBlock, TerminatorKind, Scope, BlockCheckMode)>(
                    facts,
                    17515917190007746298u64,
                    self.database_root.join("relations/unsafe_terminators"),
                );
            }
        }
        pub fn load_iter_unsafe_block_calls(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                BasicBlock,
                Scope,
                BlockCheckMode,
                FunctionCall,
                Safety,
                Abi,
                Type,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    BasicBlock,
                    Scope,
                    BlockCheckMode,
                    FunctionCall,
                    Safety,
                    Abi,
                    Type,
                )>(
                    3577624484585480604u64,
                    self.database_root.join("relations/unsafe_block_calls"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_block_calls(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                BasicBlock,
                Scope,
                BlockCheckMode,
                FunctionCall,
                Safety,
                Abi,
                Type,
            )>,
        > {
            if self.unsafe_block_calls.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        BasicBlock,
                        Scope,
                        BlockCheckMode,
                        FunctionCall,
                        Safety,
                        Abi,
                        Type,
                    )>(
                        3577624484585480604u64,
                        self.database_root.join("relations/unsafe_block_calls"),
                    )
                }
                .unwrap();
                *self.unsafe_block_calls.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_block_calls.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_block_calls(
            &self,
            facts: Vec<(
                Build,
                BasicBlock,
                Scope,
                BlockCheckMode,
                FunctionCall,
                Safety,
                Abi,
                Type,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    BasicBlock,
                    Scope,
                    BlockCheckMode,
                    FunctionCall,
                    Safety,
                    Abi,
                    Type,
                )>(
                    facts,
                    3577624484585480604u64,
                    self.database_root.join("relations/unsafe_block_calls"),
                );
            }
        }
        pub fn store_iter_unsafe_block_calls(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    BasicBlock,
                    Scope,
                    BlockCheckMode,
                    FunctionCall,
                    Safety,
                    Abi,
                    Type,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    BasicBlock,
                    Scope,
                    BlockCheckMode,
                    FunctionCall,
                    Safety,
                    Abi,
                    Type,
                )>(
                    facts,
                    3577624484585480604u64,
                    self.database_root.join("relations/unsafe_block_calls"),
                );
            }
        }
        pub fn load_iter_unsafe_block_calls_known_target(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                BasicBlock,
                Scope,
                Span,
                BlockCheckMode,
                FunctionCall,
                DefPath,
                Safety,
                Abi,
                Type,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    BasicBlock,
                    Scope,
                    Span,
                    BlockCheckMode,
                    FunctionCall,
                    DefPath,
                    Safety,
                    Abi,
                    Type,
                )>(
                    9332271943107689972u64,
                    self.database_root
                        .join("relations/unsafe_block_calls_known_target"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_block_calls_known_target(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                BasicBlock,
                Scope,
                Span,
                BlockCheckMode,
                FunctionCall,
                DefPath,
                Safety,
                Abi,
                Type,
            )>,
        > {
            if self.unsafe_block_calls_known_target.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        BasicBlock,
                        Scope,
                        Span,
                        BlockCheckMode,
                        FunctionCall,
                        DefPath,
                        Safety,
                        Abi,
                        Type,
                    )>(
                        9332271943107689972u64,
                        self.database_root
                            .join("relations/unsafe_block_calls_known_target"),
                    )
                }
                .unwrap();
                *self.unsafe_block_calls_known_target.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_block_calls_known_target.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_block_calls_known_target(
            &self,
            facts: Vec<(
                Build,
                BasicBlock,
                Scope,
                Span,
                BlockCheckMode,
                FunctionCall,
                DefPath,
                Safety,
                Abi,
                Type,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    BasicBlock,
                    Scope,
                    Span,
                    BlockCheckMode,
                    FunctionCall,
                    DefPath,
                    Safety,
                    Abi,
                    Type,
                )>(
                    facts,
                    9332271943107689972u64,
                    self.database_root
                        .join("relations/unsafe_block_calls_known_target"),
                );
            }
        }
        pub fn store_iter_unsafe_block_calls_known_target(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    BasicBlock,
                    Scope,
                    Span,
                    BlockCheckMode,
                    FunctionCall,
                    DefPath,
                    Safety,
                    Abi,
                    Type,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    BasicBlock,
                    Scope,
                    Span,
                    BlockCheckMode,
                    FunctionCall,
                    DefPath,
                    Safety,
                    Abi,
                    Type,
                )>(
                    facts,
                    9332271943107689972u64,
                    self.database_root
                        .join("relations/unsafe_block_calls_known_target"),
                );
            }
        }
        pub fn load_iter_unsafe_block_call_counts(
            &self,
        ) -> impl Iterator<Item = (Build, Scope, BlockCheckMode, u16)> {
            unsafe {
                load_elts_relation::<(Build, Scope, BlockCheckMode, u16)>(
                    9127435907518959203u64,
                    self.database_root
                        .join("relations/unsafe_block_call_counts"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_block_call_counts(
            &self,
        ) -> std::cell::Ref<Vec<(Build, Scope, BlockCheckMode, u16)>> {
            if self.unsafe_block_call_counts.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, Scope, BlockCheckMode, u16)>(
                        9127435907518959203u64,
                        self.database_root
                            .join("relations/unsafe_block_call_counts"),
                    )
                }
                .unwrap();
                *self.unsafe_block_call_counts.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_block_call_counts.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_block_call_counts(
            &self,
            facts: Vec<(Build, Scope, BlockCheckMode, u16)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Scope, BlockCheckMode, u16)>(
                    facts,
                    9127435907518959203u64,
                    self.database_root
                        .join("relations/unsafe_block_call_counts"),
                );
            }
        }
        pub fn store_iter_unsafe_block_call_counts(
            &self,
            facts: impl IntoIterator<Item = (Build, Scope, BlockCheckMode, u16)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Scope, BlockCheckMode, u16)>(
                    facts,
                    9127435907518959203u64,
                    self.database_root
                        .join("relations/unsafe_block_call_counts"),
                );
            }
        }
        pub fn load_iter_unsafe_block_no_calls(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                DefPath,
                Scope,
                SpanExpansionKind,
                BlockCheckMode,
                Span,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    DefPath,
                    Scope,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>(
                    7008873974644461880u64,
                    self.database_root.join("relations/unsafe_block_no_calls"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_block_no_calls(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                DefPath,
                Scope,
                SpanExpansionKind,
                BlockCheckMode,
                Span,
            )>,
        > {
            if self.unsafe_block_no_calls.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        DefPath,
                        Scope,
                        SpanExpansionKind,
                        BlockCheckMode,
                        Span,
                    )>(
                        7008873974644461880u64,
                        self.database_root.join("relations/unsafe_block_no_calls"),
                    )
                }
                .unwrap();
                *self.unsafe_block_no_calls.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_block_no_calls.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_block_no_calls(
            &self,
            facts: Vec<(
                Build,
                DefPath,
                Scope,
                SpanExpansionKind,
                BlockCheckMode,
                Span,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    DefPath,
                    Scope,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>(
                    facts,
                    7008873974644461880u64,
                    self.database_root.join("relations/unsafe_block_no_calls"),
                );
            }
        }
        pub fn store_iter_unsafe_block_no_calls(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    DefPath,
                    Scope,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    DefPath,
                    Scope,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>(
                    facts,
                    7008873974644461880u64,
                    self.database_root.join("relations/unsafe_block_no_calls"),
                );
            }
        }
        pub fn load_iter_adts_with_unsafe_cell_fields(
            &self,
        ) -> impl Iterator<
            Item = (
                Type,
                DefPath,
                AdtKind,
                bool,
                InternedString,
                TyVisibility,
                Type,
                TyKind,
                DefPath,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Type,
                    DefPath,
                    AdtKind,
                    bool,
                    InternedString,
                    TyVisibility,
                    Type,
                    TyKind,
                    DefPath,
                )>(
                    17785593477529193505u64,
                    self.database_root
                        .join("relations/adts_with_unsafe_cell_fields"),
                )
            }
            .unwrap()
        }
        pub fn load_adts_with_unsafe_cell_fields(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Type,
                DefPath,
                AdtKind,
                bool,
                InternedString,
                TyVisibility,
                Type,
                TyKind,
                DefPath,
            )>,
        > {
            if self.adts_with_unsafe_cell_fields.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Type,
                        DefPath,
                        AdtKind,
                        bool,
                        InternedString,
                        TyVisibility,
                        Type,
                        TyKind,
                        DefPath,
                    )>(
                        17785593477529193505u64,
                        self.database_root
                            .join("relations/adts_with_unsafe_cell_fields"),
                    )
                }
                .unwrap();
                *self.adts_with_unsafe_cell_fields.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.adts_with_unsafe_cell_fields.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_adts_with_unsafe_cell_fields(
            &self,
            facts: Vec<(
                Type,
                DefPath,
                AdtKind,
                bool,
                InternedString,
                TyVisibility,
                Type,
                TyKind,
                DefPath,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Type,
                    DefPath,
                    AdtKind,
                    bool,
                    InternedString,
                    TyVisibility,
                    Type,
                    TyKind,
                    DefPath,
                )>(
                    facts,
                    17785593477529193505u64,
                    self.database_root
                        .join("relations/adts_with_unsafe_cell_fields"),
                );
            }
        }
        pub fn store_iter_adts_with_unsafe_cell_fields(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Type,
                    DefPath,
                    AdtKind,
                    bool,
                    InternedString,
                    TyVisibility,
                    Type,
                    TyKind,
                    DefPath,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Type,
                    DefPath,
                    AdtKind,
                    bool,
                    InternedString,
                    TyVisibility,
                    Type,
                    TyKind,
                    DefPath,
                )>(
                    facts,
                    17785593477529193505u64,
                    self.database_root
                        .join("relations/adts_with_unsafe_cell_fields"),
                );
            }
        }
        pub fn load_iter_selected_function_sizes(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                Item,
                DefPath,
                TyVisibility,
                Safety,
                Abi,
                bool,
                u64,
                u64,
                u64,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    Item,
                    DefPath,
                    TyVisibility,
                    Safety,
                    Abi,
                    bool,
                    u64,
                    u64,
                    u64,
                )>(
                    17952628850175337567u64,
                    self.database_root.join("relations/selected_function_sizes"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_function_sizes(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                Item,
                DefPath,
                TyVisibility,
                Safety,
                Abi,
                bool,
                u64,
                u64,
                u64,
            )>,
        > {
            if self.selected_function_sizes.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        Item,
                        DefPath,
                        TyVisibility,
                        Safety,
                        Abi,
                        bool,
                        u64,
                        u64,
                        u64,
                    )>(
                        17952628850175337567u64,
                        self.database_root.join("relations/selected_function_sizes"),
                    )
                }
                .unwrap();
                *self.selected_function_sizes.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_function_sizes.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_function_sizes(
            &self,
            facts: Vec<(
                Build,
                Item,
                DefPath,
                TyVisibility,
                Safety,
                Abi,
                bool,
                u64,
                u64,
                u64,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Item,
                    DefPath,
                    TyVisibility,
                    Safety,
                    Abi,
                    bool,
                    u64,
                    u64,
                    u64,
                )>(
                    facts,
                    17952628850175337567u64,
                    self.database_root.join("relations/selected_function_sizes"),
                );
            }
        }
        pub fn store_iter_selected_function_sizes(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    Item,
                    DefPath,
                    TyVisibility,
                    Safety,
                    Abi,
                    bool,
                    u64,
                    u64,
                    u64,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Item,
                    DefPath,
                    TyVisibility,
                    Safety,
                    Abi,
                    bool,
                    u64,
                    u64,
                    u64,
                )>(
                    facts,
                    17952628850175337567u64,
                    self.database_root.join("relations/selected_function_sizes"),
                );
            }
        }
        pub fn load_iter_selected_build_sizes(
            &self,
        ) -> impl Iterator<Item = (Build, u64, u64, u64)> {
            unsafe {
                load_elts_relation::<(Build, u64, u64, u64)>(
                    6204349218941523339u64,
                    self.database_root.join("relations/selected_build_sizes"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_build_sizes(&self) -> std::cell::Ref<Vec<(Build, u64, u64, u64)>> {
            if self.selected_build_sizes.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, u64, u64, u64)>(
                        6204349218941523339u64,
                        self.database_root.join("relations/selected_build_sizes"),
                    )
                }
                .unwrap();
                *self.selected_build_sizes.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_build_sizes.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_build_sizes(&self, facts: Vec<(Build, u64, u64, u64)>) {
            unsafe {
                save_elts_relation::<(Build, u64, u64, u64)>(
                    facts,
                    6204349218941523339u64,
                    self.database_root.join("relations/selected_build_sizes"),
                );
            }
        }
        pub fn store_iter_selected_build_sizes(
            &self,
            facts: impl IntoIterator<Item = (Build, u64, u64, u64)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, u64, u64, u64)>(
                    facts,
                    6204349218941523339u64,
                    self.database_root.join("relations/selected_build_sizes"),
                );
            }
        }
        pub fn load_iter_selected_thir_bodies(
            &self,
        ) -> impl Iterator<Item = (Build, Item, DefPath, ThirBlock)> {
            unsafe {
                load_elts_relation::<(Build, Item, DefPath, ThirBlock)>(
                    187356987950530142u64,
                    self.database_root.join("relations/selected_thir_bodies"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_thir_bodies(
            &self,
        ) -> std::cell::Ref<Vec<(Build, Item, DefPath, ThirBlock)>> {
            if self.selected_thir_bodies.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, Item, DefPath, ThirBlock)>(
                        187356987950530142u64,
                        self.database_root.join("relations/selected_thir_bodies"),
                    )
                }
                .unwrap();
                *self.selected_thir_bodies.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_thir_bodies.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_thir_bodies(&self, facts: Vec<(Build, Item, DefPath, ThirBlock)>) {
            unsafe {
                save_elts_relation::<(Build, Item, DefPath, ThirBlock)>(
                    facts,
                    187356987950530142u64,
                    self.database_root.join("relations/selected_thir_bodies"),
                );
            }
        }
        pub fn store_iter_selected_thir_bodies(
            &self,
            facts: impl IntoIterator<Item = (Build, Item, DefPath, ThirBlock)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Item, DefPath, ThirBlock)>(
                    facts,
                    187356987950530142u64,
                    self.database_root.join("relations/selected_thir_bodies"),
                );
            }
        }
        pub fn load_iter_selected_thir_blocks(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                DefPath,
                ThirBlock,
                ThirBlock,
                ScopeSafety,
                BlockCheckMode,
                Span,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    DefPath,
                    ThirBlock,
                    ThirBlock,
                    ScopeSafety,
                    BlockCheckMode,
                    Span,
                )>(
                    2486237659217230847u64,
                    self.database_root.join("relations/selected_thir_blocks"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_thir_blocks(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                DefPath,
                ThirBlock,
                ThirBlock,
                ScopeSafety,
                BlockCheckMode,
                Span,
            )>,
        > {
            if self.selected_thir_blocks.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        DefPath,
                        ThirBlock,
                        ThirBlock,
                        ScopeSafety,
                        BlockCheckMode,
                        Span,
                    )>(
                        2486237659217230847u64,
                        self.database_root.join("relations/selected_thir_blocks"),
                    )
                }
                .unwrap();
                *self.selected_thir_blocks.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_thir_blocks.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_thir_blocks(
            &self,
            facts: Vec<(
                Build,
                DefPath,
                ThirBlock,
                ThirBlock,
                ScopeSafety,
                BlockCheckMode,
                Span,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    DefPath,
                    ThirBlock,
                    ThirBlock,
                    ScopeSafety,
                    BlockCheckMode,
                    Span,
                )>(
                    facts,
                    2486237659217230847u64,
                    self.database_root.join("relations/selected_thir_blocks"),
                );
            }
        }
        pub fn store_iter_selected_thir_blocks(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    DefPath,
                    ThirBlock,
                    ThirBlock,
                    ScopeSafety,
                    BlockCheckMode,
                    Span,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    DefPath,
                    ThirBlock,
                    ThirBlock,
                    ScopeSafety,
                    BlockCheckMode,
                    Span,
                )>(
                    facts,
                    2486237659217230847u64,
                    self.database_root.join("relations/selected_thir_blocks"),
                );
            }
        }
        pub fn load_iter_unsafe_thir_blocks(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                DefPath,
                ThirBlock,
                SpanExpansionKind,
                BlockCheckMode,
                Span,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    DefPath,
                    ThirBlock,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>(
                    1154412361575908387u64,
                    self.database_root.join("relations/unsafe_thir_blocks"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_thir_blocks(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                DefPath,
                ThirBlock,
                SpanExpansionKind,
                BlockCheckMode,
                Span,
            )>,
        > {
            if self.unsafe_thir_blocks.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        DefPath,
                        ThirBlock,
                        SpanExpansionKind,
                        BlockCheckMode,
                        Span,
                    )>(
                        1154412361575908387u64,
                        self.database_root.join("relations/unsafe_thir_blocks"),
                    )
                }
                .unwrap();
                *self.unsafe_thir_blocks.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_thir_blocks.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_thir_blocks(
            &self,
            facts: Vec<(
                Build,
                DefPath,
                ThirBlock,
                SpanExpansionKind,
                BlockCheckMode,
                Span,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    DefPath,
                    ThirBlock,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>(
                    facts,
                    1154412361575908387u64,
                    self.database_root.join("relations/unsafe_thir_blocks"),
                );
            }
        }
        pub fn store_iter_unsafe_thir_blocks(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    DefPath,
                    ThirBlock,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    DefPath,
                    ThirBlock,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>(
                    facts,
                    1154412361575908387u64,
                    self.database_root.join("relations/unsafe_thir_blocks"),
                );
            }
        }
        pub fn load_iter_unsafe_thir_stmts(
            &self,
        ) -> impl Iterator<Item = (Build, ThirStmt, ThirBlock, StatementIndex, BlockCheckMode)>
        {
            unsafe {
                load_elts_relation::<(Build, ThirStmt, ThirBlock, StatementIndex, BlockCheckMode)>(
                    10168724987398392313u64,
                    self.database_root.join("relations/unsafe_thir_stmts"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_thir_stmts(
            &self,
        ) -> std::cell::Ref<Vec<(Build, ThirStmt, ThirBlock, StatementIndex, BlockCheckMode)>>
        {
            if self.unsafe_thir_stmts.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        ThirStmt,
                        ThirBlock,
                        StatementIndex,
                        BlockCheckMode,
                    )>(
                        10168724987398392313u64,
                        self.database_root.join("relations/unsafe_thir_stmts"),
                    )
                }
                .unwrap();
                *self.unsafe_thir_stmts.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_thir_stmts.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_thir_stmts(
            &self,
            facts: Vec<(Build, ThirStmt, ThirBlock, StatementIndex, BlockCheckMode)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, ThirStmt, ThirBlock, StatementIndex, BlockCheckMode)>(
                    facts,
                    10168724987398392313u64,
                    self.database_root.join("relations/unsafe_thir_stmts"),
                );
            }
        }
        pub fn store_iter_unsafe_thir_stmts(
            &self,
            facts: impl IntoIterator<
                Item = (Build, ThirStmt, ThirBlock, StatementIndex, BlockCheckMode),
            >,
        ) {
            unsafe {
                save_elts_relation::<(Build, ThirStmt, ThirBlock, StatementIndex, BlockCheckMode)>(
                    facts,
                    10168724987398392313u64,
                    self.database_root.join("relations/unsafe_thir_stmts"),
                );
            }
        }
        pub fn load_iter_functions_unsafe_thir_blocks(
            &self,
        ) -> impl Iterator<Item = (Build, Item, ThirBlock, SpanExpansionKind, BlockCheckMode)>
        {
            unsafe {
                load_elts_relation::<(Build, Item, ThirBlock, SpanExpansionKind, BlockCheckMode)>(
                    4970716083954244550u64,
                    self.database_root
                        .join("relations/functions_unsafe_thir_blocks"),
                )
            }
            .unwrap()
        }
        pub fn load_functions_unsafe_thir_blocks(
            &self,
        ) -> std::cell::Ref<Vec<(Build, Item, ThirBlock, SpanExpansionKind, BlockCheckMode)>>
        {
            if self.functions_unsafe_thir_blocks.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        Item,
                        ThirBlock,
                        SpanExpansionKind,
                        BlockCheckMode,
                    )>(
                        4970716083954244550u64,
                        self.database_root
                            .join("relations/functions_unsafe_thir_blocks"),
                    )
                }
                .unwrap();
                *self.functions_unsafe_thir_blocks.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.functions_unsafe_thir_blocks.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_functions_unsafe_thir_blocks(
            &self,
            facts: Vec<(Build, Item, ThirBlock, SpanExpansionKind, BlockCheckMode)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Item, ThirBlock, SpanExpansionKind, BlockCheckMode)>(
                    facts,
                    4970716083954244550u64,
                    self.database_root
                        .join("relations/functions_unsafe_thir_blocks"),
                );
            }
        }
        pub fn store_iter_functions_unsafe_thir_blocks(
            &self,
            facts: impl IntoIterator<Item = (Build, Item, ThirBlock, SpanExpansionKind, BlockCheckMode)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, Item, ThirBlock, SpanExpansionKind, BlockCheckMode)>(
                    facts,
                    4970716083954244550u64,
                    self.database_root
                        .join("relations/functions_unsafe_thir_blocks"),
                );
            }
        }
        pub fn load_iter_selected_function_thir_sizes(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                Item,
                DefPath,
                TyVisibility,
                Safety,
                Abi,
                bool,
                u64,
                u64,
                u64,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    Item,
                    DefPath,
                    TyVisibility,
                    Safety,
                    Abi,
                    bool,
                    u64,
                    u64,
                    u64,
                )>(
                    18434311253541221667u64,
                    self.database_root
                        .join("relations/selected_function_thir_sizes"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_function_thir_sizes(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                Item,
                DefPath,
                TyVisibility,
                Safety,
                Abi,
                bool,
                u64,
                u64,
                u64,
            )>,
        > {
            if self.selected_function_thir_sizes.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        Item,
                        DefPath,
                        TyVisibility,
                        Safety,
                        Abi,
                        bool,
                        u64,
                        u64,
                        u64,
                    )>(
                        18434311253541221667u64,
                        self.database_root
                            .join("relations/selected_function_thir_sizes"),
                    )
                }
                .unwrap();
                *self.selected_function_thir_sizes.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_function_thir_sizes.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_function_thir_sizes(
            &self,
            facts: Vec<(
                Build,
                Item,
                DefPath,
                TyVisibility,
                Safety,
                Abi,
                bool,
                u64,
                u64,
                u64,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Item,
                    DefPath,
                    TyVisibility,
                    Safety,
                    Abi,
                    bool,
                    u64,
                    u64,
                    u64,
                )>(
                    facts,
                    18434311253541221667u64,
                    self.database_root
                        .join("relations/selected_function_thir_sizes"),
                );
            }
        }
        pub fn store_iter_selected_function_thir_sizes(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    Item,
                    DefPath,
                    TyVisibility,
                    Safety,
                    Abi,
                    bool,
                    u64,
                    u64,
                    u64,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    Item,
                    DefPath,
                    TyVisibility,
                    Safety,
                    Abi,
                    bool,
                    u64,
                    u64,
                    u64,
                )>(
                    facts,
                    18434311253541221667u64,
                    self.database_root
                        .join("relations/selected_function_thir_sizes"),
                );
            }
        }
        pub fn load_iter_selected_build_thir_sizes(
            &self,
        ) -> impl Iterator<Item = (Build, u64, u64, u64)> {
            unsafe {
                load_elts_relation::<(Build, u64, u64, u64)>(
                    17858554035766366380u64,
                    self.database_root
                        .join("relations/selected_build_thir_sizes"),
                )
            }
            .unwrap()
        }
        pub fn load_selected_build_thir_sizes(
            &self,
        ) -> std::cell::Ref<Vec<(Build, u64, u64, u64)>> {
            if self.selected_build_thir_sizes.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, u64, u64, u64)>(
                        17858554035766366380u64,
                        self.database_root
                            .join("relations/selected_build_thir_sizes"),
                    )
                }
                .unwrap();
                *self.selected_build_thir_sizes.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.selected_build_thir_sizes.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_selected_build_thir_sizes(&self, facts: Vec<(Build, u64, u64, u64)>) {
            unsafe {
                save_elts_relation::<(Build, u64, u64, u64)>(
                    facts,
                    17858554035766366380u64,
                    self.database_root
                        .join("relations/selected_build_thir_sizes"),
                );
            }
        }
        pub fn store_iter_selected_build_thir_sizes(
            &self,
            facts: impl IntoIterator<Item = (Build, u64, u64, u64)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, u64, u64, u64)>(
                    facts,
                    17858554035766366380u64,
                    self.database_root
                        .join("relations/selected_build_thir_sizes"),
                );
            }
        }
        pub fn load_iter_unsafe_thir_block_calls(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                ThirBlock,
                BlockCheckMode,
                ThirExpr,
                ThirExpr,
                Safety,
                Abi,
                Type,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    ThirBlock,
                    BlockCheckMode,
                    ThirExpr,
                    ThirExpr,
                    Safety,
                    Abi,
                    Type,
                )>(
                    13770514726142206672u64,
                    self.database_root.join("relations/unsafe_thir_block_calls"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_thir_block_calls(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                ThirBlock,
                BlockCheckMode,
                ThirExpr,
                ThirExpr,
                Safety,
                Abi,
                Type,
            )>,
        > {
            if self.unsafe_thir_block_calls.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        ThirBlock,
                        BlockCheckMode,
                        ThirExpr,
                        ThirExpr,
                        Safety,
                        Abi,
                        Type,
                    )>(
                        13770514726142206672u64,
                        self.database_root.join("relations/unsafe_thir_block_calls"),
                    )
                }
                .unwrap();
                *self.unsafe_thir_block_calls.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_thir_block_calls.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_thir_block_calls(
            &self,
            facts: Vec<(
                Build,
                ThirBlock,
                BlockCheckMode,
                ThirExpr,
                ThirExpr,
                Safety,
                Abi,
                Type,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    ThirBlock,
                    BlockCheckMode,
                    ThirExpr,
                    ThirExpr,
                    Safety,
                    Abi,
                    Type,
                )>(
                    facts,
                    13770514726142206672u64,
                    self.database_root.join("relations/unsafe_thir_block_calls"),
                );
            }
        }
        pub fn store_iter_unsafe_thir_block_calls(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    ThirBlock,
                    BlockCheckMode,
                    ThirExpr,
                    ThirExpr,
                    Safety,
                    Abi,
                    Type,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    ThirBlock,
                    BlockCheckMode,
                    ThirExpr,
                    ThirExpr,
                    Safety,
                    Abi,
                    Type,
                )>(
                    facts,
                    13770514726142206672u64,
                    self.database_root.join("relations/unsafe_thir_block_calls"),
                );
            }
        }
        pub fn load_iter_unsafe_thir_block_call_counts(
            &self,
        ) -> impl Iterator<Item = (Build, ThirBlock, BlockCheckMode, u16)> {
            unsafe {
                load_elts_relation::<(Build, ThirBlock, BlockCheckMode, u16)>(
                    16677585589365782970u64,
                    self.database_root
                        .join("relations/unsafe_thir_block_call_counts"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_thir_block_call_counts(
            &self,
        ) -> std::cell::Ref<Vec<(Build, ThirBlock, BlockCheckMode, u16)>> {
            if self.unsafe_thir_block_call_counts.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(Build, ThirBlock, BlockCheckMode, u16)>(
                        16677585589365782970u64,
                        self.database_root
                            .join("relations/unsafe_thir_block_call_counts"),
                    )
                }
                .unwrap();
                *self.unsafe_thir_block_call_counts.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_thir_block_call_counts.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_thir_block_call_counts(
            &self,
            facts: Vec<(Build, ThirBlock, BlockCheckMode, u16)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, ThirBlock, BlockCheckMode, u16)>(
                    facts,
                    16677585589365782970u64,
                    self.database_root
                        .join("relations/unsafe_thir_block_call_counts"),
                );
            }
        }
        pub fn store_iter_unsafe_thir_block_call_counts(
            &self,
            facts: impl IntoIterator<Item = (Build, ThirBlock, BlockCheckMode, u16)>,
        ) {
            unsafe {
                save_elts_relation::<(Build, ThirBlock, BlockCheckMode, u16)>(
                    facts,
                    16677585589365782970u64,
                    self.database_root
                        .join("relations/unsafe_thir_block_call_counts"),
                );
            }
        }
        pub fn load_iter_unsafe_thir_block_no_calls(
            &self,
        ) -> impl Iterator<
            Item = (
                Build,
                DefPath,
                ThirBlock,
                SpanExpansionKind,
                BlockCheckMode,
                Span,
            ),
        > {
            unsafe {
                load_elts_relation::<(
                    Build,
                    DefPath,
                    ThirBlock,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>(
                    18126364453640819788u64,
                    self.database_root
                        .join("relations/unsafe_thir_block_no_calls"),
                )
            }
            .unwrap()
        }
        pub fn load_unsafe_thir_block_no_calls(
            &self,
        ) -> std::cell::Ref<
            Vec<(
                Build,
                DefPath,
                ThirBlock,
                SpanExpansionKind,
                BlockCheckMode,
                Span,
            )>,
        > {
            if self.unsafe_thir_block_no_calls.borrow().is_none() {
                let relation = unsafe {
                    load_elts_relation_into_relation::<(
                        Build,
                        DefPath,
                        ThirBlock,
                        SpanExpansionKind,
                        BlockCheckMode,
                        Span,
                    )>(
                        18126364453640819788u64,
                        self.database_root
                            .join("relations/unsafe_thir_block_no_calls"),
                    )
                }
                .unwrap();
                *self.unsafe_thir_block_no_calls.borrow_mut() = Some(relation.into());
            }
            std::cell::Ref::map(self.unsafe_thir_block_no_calls.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn store_unsafe_thir_block_no_calls(
            &self,
            facts: Vec<(
                Build,
                DefPath,
                ThirBlock,
                SpanExpansionKind,
                BlockCheckMode,
                Span,
            )>,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    DefPath,
                    ThirBlock,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>(
                    facts,
                    18126364453640819788u64,
                    self.database_root
                        .join("relations/unsafe_thir_block_no_calls"),
                );
            }
        }
        pub fn store_iter_unsafe_thir_block_no_calls(
            &self,
            facts: impl IntoIterator<
                Item = (
                    Build,
                    DefPath,
                    ThirBlock,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                ),
            >,
        ) {
            unsafe {
                save_elts_relation::<(
                    Build,
                    DefPath,
                    ThirBlock,
                    SpanExpansionKind,
                    BlockCheckMode,
                    Span,
                )>(
                    facts,
                    18126364453640819788u64,
                    self.database_root
                        .join("relations/unsafe_thir_block_no_calls"),
                );
            }
        }
        pub fn load_strings(&self) -> std::cell::Ref<InterningTable<InternedString, String>> {
            if self.strings.borrow().is_none() {
                *self.strings.borrow_mut() = Some(
                    crate::storage::load(&self.database_root.join("interning/strings.bincode"))
                        .unwrap(),
                );
            }
            std::cell::Ref::map(self.strings.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn load_strings_as_vec(&self) -> Vec<(InternedString, String)> {
            let table: InterningTable<InternedString, String> =
                crate::storage::load(&self.database_root.join("interning/strings.bincode"))
                    .unwrap();
            table.into()
        }
        pub fn load_package_names(
            &self,
        ) -> std::cell::Ref<InterningTable<Package, InternedString>> {
            if self.package_names.borrow().is_none() {
                *self.package_names.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        5753046719037099568u64,
                        self.database_root.join("interning/package_names"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.package_names.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_package_names_as_vec(&self) -> Vec<(Package, InternedString)> {
            let table: InterningTable<Package, InternedString> = unsafe {
                InterningTable::load(
                    5753046719037099568u64,
                    self.database_root.join("interning/package_names"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_package_versions(
            &self,
        ) -> std::cell::Ref<InterningTable<PackageVersion, InternedString>> {
            if self.package_versions.borrow().is_none() {
                *self.package_versions.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        7616596403864686833u64,
                        self.database_root.join("interning/package_versions"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.package_versions.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_package_versions_as_vec(&self) -> Vec<(PackageVersion, InternedString)> {
            let table: InterningTable<PackageVersion, InternedString> = unsafe {
                InterningTable::load(
                    7616596403864686833u64,
                    self.database_root.join("interning/package_versions"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_crate_names(&self) -> std::cell::Ref<InterningTable<Krate, InternedString>> {
            if self.crate_names.borrow().is_none() {
                *self.crate_names.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        16204848724287879413u64,
                        self.database_root.join("interning/crate_names"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.crate_names.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn load_crate_names_as_vec(&self) -> Vec<(Krate, InternedString)> {
            let table: InterningTable<Krate, InternedString> = unsafe {
                InterningTable::load(
                    16204848724287879413u64,
                    self.database_root.join("interning/crate_names"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_editions(&self) -> std::cell::Ref<InterningTable<Edition, InternedString>> {
            if self.editions.borrow().is_none() {
                *self.editions.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        11029148200302886001u64,
                        self.database_root.join("interning/editions"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.editions.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn load_editions_as_vec(&self) -> Vec<(Edition, InternedString)> {
            let table: InterningTable<Edition, InternedString> = unsafe {
                InterningTable::load(
                    11029148200302886001u64,
                    self.database_root.join("interning/editions"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_names(&self) -> std::cell::Ref<InterningTable<Name, InternedString>> {
            if self.names.borrow().is_none() {
                *self.names.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        10407673464407571910u64,
                        self.database_root.join("interning/names"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.names.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn load_names_as_vec(&self) -> Vec<(Name, InternedString)> {
            let table: InterningTable<Name, InternedString> = unsafe {
                InterningTable::load(
                    10407673464407571910u64,
                    self.database_root.join("interning/names"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_relative_def_paths(
            &self,
        ) -> std::cell::Ref<InterningTable<RelativeDefId, InternedString>> {
            if self.relative_def_paths.borrow().is_none() {
                *self.relative_def_paths.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        15140222954922639867u64,
                        self.database_root.join("interning/relative_def_paths"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.relative_def_paths.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_relative_def_paths_as_vec(&self) -> Vec<(RelativeDefId, InternedString)> {
            let table: InterningTable<RelativeDefId, InternedString> = unsafe {
                InterningTable::load(
                    15140222954922639867u64,
                    self.database_root.join("interning/relative_def_paths"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_summary_keys(
            &self,
        ) -> std::cell::Ref<InterningTable<SummaryId, InternedString>> {
            if self.summary_keys.borrow().is_none() {
                *self.summary_keys.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        18304431051085712562u64,
                        self.database_root.join("interning/summary_keys"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.summary_keys.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_summary_keys_as_vec(&self) -> Vec<(SummaryId, InternedString)> {
            let table: InterningTable<SummaryId, InternedString> = unsafe {
                InterningTable::load(
                    18304431051085712562u64,
                    self.database_root.join("interning/summary_keys"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_abis(&self) -> std::cell::Ref<InterningTable<Abi, InternedString>> {
            if self.abis.borrow().is_none() {
                *self.abis.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        7090802155184187506u64,
                        self.database_root.join("interning/abis"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.abis.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn load_abis_as_vec(&self) -> Vec<(Abi, InternedString)> {
            let table: InterningTable<Abi, InternedString> = unsafe {
                InterningTable::load(
                    7090802155184187506u64,
                    self.database_root.join("interning/abis"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_def_paths(
            &self,
        ) -> std::cell::Ref<
            InterningTable<DefPath, (Krate, CrateHash, RelativeDefId, DefPathHash, SummaryId)>,
        > {
            if self.def_paths.borrow().is_none() {
                *self.def_paths.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        4088999352158381537u64,
                        self.database_root.join("interning/def_paths"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.def_paths.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn load_def_paths_as_vec(
            &self,
        ) -> Vec<(
            DefPath,
            Krate,
            CrateHash,
            RelativeDefId,
            DefPathHash,
            SummaryId,
        )> {
            let table: InterningTable<
                DefPath,
                (Krate, CrateHash, RelativeDefId, DefPathHash, SummaryId),
            > = unsafe {
                InterningTable::load(
                    4088999352158381537u64,
                    self.database_root.join("interning/def_paths"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_builds(
            &self,
        ) -> std::cell::Ref<
            InterningTable<Build, (Package, PackageVersion, Krate, CrateHash, Edition)>,
        > {
            if self.builds.borrow().is_none() {
                *self.builds.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        16225872080495055883u64,
                        self.database_root.join("interning/builds"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.builds.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn load_builds_as_vec(
            &self,
        ) -> Vec<(Build, Package, PackageVersion, Krate, CrateHash, Edition)> {
            let table: InterningTable<Build, (Package, PackageVersion, Krate, CrateHash, Edition)> = unsafe {
                InterningTable::load(
                    16225872080495055883u64,
                    self.database_root.join("interning/builds"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_span_file_names(
            &self,
        ) -> std::cell::Ref<InterningTable<SpanFileName, InternedString>> {
            if self.span_file_names.borrow().is_none() {
                *self.span_file_names.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        12030213377196136044u64,
                        self.database_root.join("interning/span_file_names"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.span_file_names.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_span_file_names_as_vec(&self) -> Vec<(SpanFileName, InternedString)> {
            let table: InterningTable<SpanFileName, InternedString> = unsafe {
                InterningTable::load(
                    12030213377196136044u64,
                    self.database_root.join("interning/span_file_names"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_crate_cfg_keys(
            &self,
        ) -> std::cell::Ref<InterningTable<CrateCfgKey, InternedString>> {
            if self.crate_cfg_keys.borrow().is_none() {
                *self.crate_cfg_keys.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        8740096367364431282u64,
                        self.database_root.join("interning/crate_cfg_keys"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.crate_cfg_keys.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_crate_cfg_keys_as_vec(&self) -> Vec<(CrateCfgKey, InternedString)> {
            let table: InterningTable<CrateCfgKey, InternedString> = unsafe {
                InterningTable::load(
                    8740096367364431282u64,
                    self.database_root.join("interning/crate_cfg_keys"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_crate_cfg_values(
            &self,
        ) -> std::cell::Ref<InterningTable<CrateCfgValue, InternedString>> {
            if self.crate_cfg_values.borrow().is_none() {
                *self.crate_cfg_values.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        12166464883024393290u64,
                        self.database_root.join("interning/crate_cfg_values"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.crate_cfg_values.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_crate_cfg_values_as_vec(&self) -> Vec<(CrateCfgValue, InternedString)> {
            let table: InterningTable<CrateCfgValue, InternedString> = unsafe {
                InterningTable::load(
                    12166464883024393290u64,
                    self.database_root.join("interning/crate_cfg_values"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_type_kinds(&self) -> std::cell::Ref<InterningTable<TyKind, InternedString>> {
            if self.type_kinds.borrow().is_none() {
                *self.type_kinds.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        10162902544379113809u64,
                        self.database_root.join("interning/type_kinds"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.type_kinds.borrow(), |option| option.as_ref().unwrap())
        }
        pub fn load_type_kinds_as_vec(&self) -> Vec<(TyKind, InternedString)> {
            let table: InterningTable<TyKind, InternedString> = unsafe {
                InterningTable::load(
                    10162902544379113809u64,
                    self.database_root.join("interning/type_kinds"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_statement_kinds(
            &self,
        ) -> std::cell::Ref<InterningTable<StatementKind, InternedString>> {
            if self.statement_kinds.borrow().is_none() {
                *self.statement_kinds.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        6273093956909890961u64,
                        self.database_root.join("interning/statement_kinds"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.statement_kinds.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_statement_kinds_as_vec(&self) -> Vec<(StatementKind, InternedString)> {
            let table: InterningTable<StatementKind, InternedString> = unsafe {
                InterningTable::load(
                    6273093956909890961u64,
                    self.database_root.join("interning/statement_kinds"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_binary_op_kind(&self) -> std::cell::Ref<InterningTable<BinOp, InternedString>> {
            if self.binary_op_kind.borrow().is_none() {
                *self.binary_op_kind.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        16753478943827638787u64,
                        self.database_root.join("interning/binary_op_kind"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.binary_op_kind.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_binary_op_kind_as_vec(&self) -> Vec<(BinOp, InternedString)> {
            let table: InterningTable<BinOp, InternedString> = unsafe {
                InterningTable::load(
                    16753478943827638787u64,
                    self.database_root.join("interning/binary_op_kind"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_nullary_op_kind(
            &self,
        ) -> std::cell::Ref<InterningTable<NullOp, InternedString>> {
            if self.nullary_op_kind.borrow().is_none() {
                *self.nullary_op_kind.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        9389093448705492600u64,
                        self.database_root.join("interning/nullary_op_kind"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.nullary_op_kind.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_nullary_op_kind_as_vec(&self) -> Vec<(NullOp, InternedString)> {
            let table: InterningTable<NullOp, InternedString> = unsafe {
                InterningTable::load(
                    9389093448705492600u64,
                    self.database_root.join("interning/nullary_op_kind"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_unary_op_kind(&self) -> std::cell::Ref<InterningTable<UnOp, InternedString>> {
            if self.unary_op_kind.borrow().is_none() {
                *self.unary_op_kind.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        14289673856492345124u64,
                        self.database_root.join("interning/unary_op_kind"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.unary_op_kind.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_unary_op_kind_as_vec(&self) -> Vec<(UnOp, InternedString)> {
            let table: InterningTable<UnOp, InternedString> = unsafe {
                InterningTable::load(
                    14289673856492345124u64,
                    self.database_root.join("interning/unary_op_kind"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_terminator_kinds(
            &self,
        ) -> std::cell::Ref<InterningTable<TerminatorKind, InternedString>> {
            if self.terminator_kinds.borrow().is_none() {
                *self.terminator_kinds.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        13371625579468820097u64,
                        self.database_root.join("interning/terminator_kinds"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.terminator_kinds.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_terminator_kinds_as_vec(&self) -> Vec<(TerminatorKind, InternedString)> {
            let table: InterningTable<TerminatorKind, InternedString> = unsafe {
                InterningTable::load(
                    13371625579468820097u64,
                    self.database_root.join("interning/terminator_kinds"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_thir_binary_op_kind(
            &self,
        ) -> std::cell::Ref<InterningTable<ThirBinOp, InternedString>> {
            if self.thir_binary_op_kind.borrow().is_none() {
                *self.thir_binary_op_kind.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        16069664154929368055u64,
                        self.database_root.join("interning/thir_binary_op_kind"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.thir_binary_op_kind.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_thir_binary_op_kind_as_vec(&self) -> Vec<(ThirBinOp, InternedString)> {
            let table: InterningTable<ThirBinOp, InternedString> = unsafe {
                InterningTable::load(
                    16069664154929368055u64,
                    self.database_root.join("interning/thir_binary_op_kind"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_thir_logical_op_kind(
            &self,
        ) -> std::cell::Ref<InterningTable<ThirLogicalOp, InternedString>> {
            if self.thir_logical_op_kind.borrow().is_none() {
                *self.thir_logical_op_kind.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        3532929431539356339u64,
                        self.database_root.join("interning/thir_logical_op_kind"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.thir_logical_op_kind.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_thir_logical_op_kind_as_vec(&self) -> Vec<(ThirLogicalOp, InternedString)> {
            let table: InterningTable<ThirLogicalOp, InternedString> = unsafe {
                InterningTable::load(
                    3532929431539356339u64,
                    self.database_root.join("interning/thir_logical_op_kind"),
                )
                .unwrap()
            };
            table.into()
        }
        pub fn load_thir_unary_op_kind(
            &self,
        ) -> std::cell::Ref<InterningTable<ThirUnOp, InternedString>> {
            if self.thir_unary_op_kind.borrow().is_none() {
                *self.thir_unary_op_kind.borrow_mut() = Some(unsafe {
                    InterningTable::load(
                        9863820750922661495u64,
                        self.database_root.join("interning/thir_unary_op_kind"),
                    )
                    .unwrap()
                });
            }
            std::cell::Ref::map(self.thir_unary_op_kind.borrow(), |option| {
                option.as_ref().unwrap()
            })
        }
        pub fn load_thir_unary_op_kind_as_vec(&self) -> Vec<(ThirUnOp, InternedString)> {
            let table: InterningTable<ThirUnOp, InternedString> = unsafe {
                InterningTable::load(
                    9863820750922661495u64,
                    self.database_root.join("interning/thir_unary_op_kind"),
                )
                .unwrap()
            };
            table.into()
        }
    }
}
