macro_rules! endpoint {
    {
        $name:ident($params:ident): $endpoint:literal => {
            $(
                $df:ident: $dv:expr,
            )*
        } => {
            $(
                $rn:ident: $rs:ident($rl:literal) {
                    $(
                        $(#[$rfattr:meta])*
                        $rf:ident: $rty:ty,
                    )*
                },
            )*
        }
    } => {
        #[allow(missing_copy_implementations)]
        #[derive(Clone, Debug, Default, PartialEq, Eq, ::serde::Serialize, ::serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        #[serde(rename_all = "PascalCase")]
        pub struct $name {
            params: $params,
        }

        impl $crate::Endpoint for $name {
            type Parameters = $params;
            type ResultSets = ResultSets;

            fn new(params: Self::Parameters) -> Self {
                Self { params }
            }

            fn endpoint(&self) -> ::std::borrow::Cow<'static, str> {
                $endpoint.into()
            }

            fn parameters(&self) -> Self::Parameters {
                Self::Parameters {
                    $(
                        $df: $dv,
                    )*
                    ..self.params
                }
            }
        }

        $(
            #[allow(missing_copy_implementations)]
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, Debug, Default, PartialEq, ::serde::Serialize)]
            pub struct $rs {
                $(
                    $(#[$rfattr])*
                    pub $rf: $rty
                ),*
            }
        )*

        #[allow(missing_copy_implementations)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Debug, Default, PartialEq, ::serde::Serialize)]
        #[serde(deny_unknown_fields)]
        #[serde(rename_all = "camelCase")]
        pub struct ResultSets {
            $(
                pub $rn: Vec<$rs>,
            )*
        }

        impl ::std::convert::TryFrom<Vec<$crate::BasicResultSet>> for ResultSets {
            type Error = String;

            #[allow(unused_mut)]
            #[allow(unused_variables)]
            fn try_from(basic: Vec<$crate::BasicResultSet>) -> Result<Self, Self::Error> {
                basic
                    .into_iter()
                    .try_fold(
                        Self::default(),
                        |mut result_sets, mut rs| {
                            let mut index_hm = rs
                                .headers
                                .iter()
                                .enumerate()
                                .map(|(i, h)| (h.to_lowercase(), i))
                                .collect::<::std::collections::HashMap<_, _>>();


                            match rs.name.as_str() {
                                $(
                                    $rl => {
                                        $(
                                            index_hm.entry(stringify!($rf).to_owned()).or_insert_with_key(|rf| {
                                                rs.headers.push(rf.to_owned());
                                                rs.headers.len() - 1
                                            });
                                        )*

                                        rs.row_set.iter_mut().for_each(|row| row.resize(rs.headers.len(), ::serde_json::Value::Null));

                                        result_sets.$rn = rs.row_set
                                            .into_iter()
                                            .map(|row| Ok($rs {
                                                $(
                                                    $rf: row
                                                        .get(*index_hm
                                                            .get(stringify!($rf).to_lowercase().as_str())
                                                            .expect("every header should be present in `index_hm`"))
                                                        .cloned()
                                                        .map(::serde_json::from_value)
                                                        .expect("every field should be present in `row`")
                                                        .map_err(|e| format!("failed to parse field `{}`: {}", stringify!($rf), e))?,
                                                )*
                                            }))
                                            .collect::<Result<_, String>>()?;

                                        Ok(result_sets)
                                    },
                                )*
                                _ => Err(format!("unknown result set: `{}`", rs.name)),
                            }
                        },
                    )
            }
        }

        impl<'de> ::serde::Deserialize<'de> for ResultSets {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let raw = $crate::serde::vec_or_single::VecOrSingle::<$crate::BasicResultSet>::deserialize(deserializer)?.into_vec();

                raw.try_into().map_err(::serde::de::Error::custom)
            }
        }
    };
}
pub(crate) use endpoint;
