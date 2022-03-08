//! Helpful imports for creating library functionality.

pub use std::fmt::{self, Debug, Formatter};
pub use std::hash::Hash;
pub use std::num::NonZeroUsize;
pub use std::sync::Arc;

pub use typst_macros::class;

pub use crate::diag::{with_alternative, At, StrResult, TypResult};
pub use crate::eval::{
    Arg, Args, Array, Cast, Construct, Dict, Func, Layout, LayoutNode, Merge, Property,
    Regions, Scope, Set, Show, ShowNode, Smart, StyleChain, StyleMap, StyleVec, Template,
    Value,
};
pub use crate::frame::*;
pub use crate::geom::*;
pub use crate::syntax::{Span, Spanned};
pub use crate::util::{EcoString, OptionExt};
pub use crate::Context;