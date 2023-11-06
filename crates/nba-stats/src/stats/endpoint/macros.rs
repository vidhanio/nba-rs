macro_rules! endpoint {
    { $endpoint:literal

        $name:ident($params:ident
            $(
            {
                $($df:ident: $dv:expr,)*
            }
            )?
        ) {
            $(
            $rn:ident[$rl:literal]: $rs:ident {
                $(
                $(#[$rfattr:meta])*
                $rf:ident: $rty:ty,
                )*
            },
            )*
        }
    } => {
        // #[allow(missing_copy_implementations)]
        // #[derive(Clone, Debug, Default, PartialEq, Eq, ::serde::Serialize, ::serde::Deserialize)]
        // #[serde(deny_unknown_fields)]
        // #[serde(rename_all(deserialize = "PascalCase"))]
        // pub struct $name {
        //     params: $params,
        // }

        // impl $crate::Endpoint for $name {
        //     type Parameters = $params;
        //     type ResultSets = ResultSets;

        //     fn new(params: Self::Parameters) -> Self {
        //         #[allow(clippy::unnecessary_struct_initialization)]
        //         Self {
        //             params: Self::Parameters {
        //                 $($(
        //                 $df: $dv,
        //                 )*)?
        //                 ..params
        //             }
        //         }
        //     }

        //     fn endpoint(&self) -> ::std::borrow::Cow<'static, str> {
        //         $endpoint.into()
        //     }

        //     fn parameters(&self) -> Self::Parameters {
        //         self.params.clone()
        //     }
        // }

        // $(
        // #[allow(missing_copy_implementations)]
        // #[allow(clippy::derive_partial_eq_without_eq)]
        // #[derive(Clone, Debug, Default, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
        // #[serde(deny_unknown_fields)]
        // #[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
        // pub struct $rs {
        //     $(
        //     $(#[$rfattr])*
        //     pub $rf: $rty
        //     ),*
        // }
        // )*

        // #[allow(missing_copy_implementations)]
        // #[allow(clippy::derive_partial_eq_without_eq)]
        // #[derive(Clone, Debug, Default, PartialEq, ::serde::Serialize)]
        // pub struct ResultSets {
        //     $(
        //     pub $rn: Vec<$rs>,
        //     )*
        // }

        // impl TryFrom<Vec<$crate::BasicResultSet>> for ResultSets {
        //     type Error = String;

        //     #[allow(unused_mut)]
        //     #[allow(unused_variables)]
        //     fn try_from(basic_result_sets: Vec<$crate::BasicResultSet>) -> Result<Self, Self::Error> {
        //         basic_result_sets
        //             .into_iter()
        //             .try_fold(Self::default(), |mut result_sets, mut rs| {
        //                 // map headers to indices
        //                 let mut header_map = rs
        //                     .headers
        //                     .iter()
        //                     .enumerate()
        //                     .collect::<::std::collections::HashMap<_, _>>();

        //                 // match result set
        //                 match rs.name.as_str() {
        //                     $(
        //                     $rl => {
        //                         result_sets.$rn = rs
        //                             .row_set
        //                             .into_iter()
        //                             .map(|row| {
        //                                 let map = row
        //                                     .into_iter()
        //                                     .enumerate()
        //                                     .try_fold(::serde_json::Map::new(), |mut map, (i, value)| {
        //                                         let header = header_map
        //                                             .get(&i)
        //                                             .ok_or_else(|| format!("index out of bounds for header: `{i}`"))?;

        //                                         map.insert((*header).to_owned(), value);

        //                                         Ok::<_, String>(map)
        //                                     })?;

        //                                 ::serde_json::from_value(::serde_json::Value::Object(map))
        //                                     .map_err(|e| format!("failed to deserialize row `{}`: {}", rs.name, e))

        //                             })
        //                             .collect::<Result<_, String>>()?;

        //                         Ok(result_sets)
        //                     }
        //                     )*
        //                     _ => Err(format!("unknown result set: `{}`", rs.name)),
        //                 }
        //             })
        //     }
        // }

        // impl<'de> ::serde::Deserialize<'de> for ResultSets {
        //     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        //     where
        //         D: ::serde::Deserializer<'de>,
        //     {
        //         let raw =
        //             $crate::serde::vec_or_single::VecOrSingle::<$crate::BasicResultSet>::deserialize(
        //                 deserializer,
        //             )?
        //             .into_vec();

        //         raw.try_into().map_err(::serde::de::Error::custom)
        //     }
        // }

        // #[cfg(test)]
        // mod tests {
        //     use $crate::Endpoint;

        //     use super::*;

        //     #[::tokio::test]
        //     async fn works() {
        //         let endpoint = $name::new(::std::default::Default::default());

        //         dbg!(endpoint.to_request());

        //         endpoint
        //             .send()
        //             .await
        //             .unwrap();
        //     }
        // }
    }
}
pub(crate) use endpoint;
