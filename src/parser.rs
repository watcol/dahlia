//! Variable parsers and combinators.

// traits
mod once;
mod reusable;

pub use once::{BaseParserOnce, ParserOnce};
pub use reusable::{BaseParser, Parser};

// combinators
mod boxed;

pub use boxed::BoxedParser;

// parsers
mod any;
mod condition;
mod position;
mod value;

pub use any::{any, Any};
pub use condition::{is, is_not, is_not_once, is_once, Condition, ConditionOnce};
pub use position::{position, Position};
pub use value::{value, value_clone, Value, ValueClone};
