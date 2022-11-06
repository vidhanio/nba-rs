//! All possible fields which may be used in the request or response of a
//! [`crate::stats::Endpoint`].

#![allow(missing_docs)]

mod league_id;
mod per_mode;
mod position;
mod season;
mod season_type;
mod yes_or_no;

pub use league_id::*;
pub use per_mode::*;
pub use position::*;
pub use season::*;
pub use season_type::*;
pub use yes_or_no::*;

macro_rules! convert {
    {
        $subset:ident => $superset:ident {
            $($variant:ident,)*
        }
    } => {
        impl From<$subset> for $superset {
            fn from(value: $subset) -> Self {
                match value {
                    $(
                        $subset::$variant => Self::$variant,
                    )*
                }
            }
        }

        impl TryFrom<$superset> for $subset {
            type Error = ();

            fn try_from(value: $superset) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $superset::$variant => Ok(Self::$variant),
                    )*
                    _ => Err(()),
                }
            }
        }
    };
}
use convert;
