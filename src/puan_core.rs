/// Enum definition for Bound
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bound {
    #[prost(int64, tag = "1")]
    pub lower: i64,
    #[prost(int64, tag = "2")]
    pub upper: i64,
}
/// Message definition for Primitive variable
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Primitive {
    #[prost(message, optional, tag = "1")]
    pub bound: ::core::option::Option<Bound>,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// Message definition for Variable
/// A Variable is either one of composite or primitive
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Variable {
    #[prost(oneof = "variable::Part", tags = "1, 2")]
    pub part: ::core::option::Option<variable::Part>,
}
/// Nested message and enum types in `Variable`.
pub mod variable {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Part {
        #[prost(message, tag = "1")]
        Composite(super::Composite),
        #[prost(message, tag = "2")]
        Primitive(super::Primitive),
    }
}
/// Message definition for Composite variable
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Composite {
    #[prost(enumeration = "Direction", tag = "1")]
    pub direction: i32,
    #[prost(int64, tag = "2")]
    pub bias: i64,
    #[prost(message, repeated, tag = "3")]
    pub variables: ::prost::alloc::vec::Vec<Variable>,
}
/// Enum definition for Direction
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Direction {
    Positive = 0,
    Negative = 1,
}
impl Direction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Direction::Positive => "Positive",
            Direction::Negative => "Negative",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Positive" => Some(Self::Positive),
            "Negative" => Some(Self::Negative),
            _ => None,
        }
    }
}
