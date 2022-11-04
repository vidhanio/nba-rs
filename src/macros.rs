macro_rules! endpoint {
    {
        $name:ident($endpoint:literal) {
            $(
                $(#[$sfattr:meta])*
                $sf:ident: $sty:ty
            ),* $(,)?
        } => {
            $(
                $rn:ident: $rs:ident($rl:literal) {
                    $(
                        $(#[$rfattr:meta])*
                        $rf:ident: $rty:ty
                    ),* $(,)?
                }
            ),* $(,)?
        }
    } => {
        #[allow(missing_copy_implementations)]
        #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        #[serde(rename_all = "PascalCase")]
        pub struct $name {
            $(
                $(#[$sfattr])*
                pub $sf: $sty
            ),*
        }

        #[::async_trait::async_trait]
        impl $crate::Endpoint for $name {
            const ENDPOINT: &'static str = $endpoint;

            type ResultSets = ResultSets;
        }

        $(
            #[allow(missing_copy_implementations)]
            #[derive(Clone, Debug, Default, ::serde::Serialize)]
            pub struct $rs {
                $(
                    $(#[$rfattr])*
                    pub $rf: $rty
                ),*
            }
        )*

        #[derive(Clone, Debug, Default, ::serde::Serialize)]
        #[serde(deny_unknown_fields)]
        #[serde(rename_all = "camelCase")]
        pub struct ResultSets {
            $(
                pub $rn: Vec<$rs>,
            )*
        }

        impl ::std::convert::TryFrom<Vec<$crate::BasicResultSet>> for ResultSets {
            type Error = String;

            fn try_from(basic: Vec<$crate::BasicResultSet>) -> Result<Self, Self::Error> {
                basic
                    .into_iter()
                    .try_fold(
                        Self::default(),
                        |mut result_sets, rs| {
                            match rs.name.as_str() {
                                $(
                                    $rl => {
                                        rs.columns().try_for_each(|(header, col, len)| {
                                            Ok(match header.to_lowercase().as_str() {
                                                $(
                                                    stringify!($rf) => {
                                                        result_sets.$rn.resize_with(len, Default::default);
                                                        col.enumerate().try_for_each(|(i, val)| {
                                                            result_sets.$rn[i].$rf = ::serde_json::from_value(val.clone())
                                                                .map_err(|e| format!("{}: {}", stringify!($rf), e))?;

                                                            Ok::<_, String>(())
                                                        })?;
                                                    }
                                                )*
                                                _ => {
                                                    return Err(format!(
                                                        "unknown column header: {}",
                                                        header
                                                    ));
                                                }
                                            })
                                        })
                                    }
                                )*
                                _ => return Err(format!("unknown result set: {}", rs.name)),
                            }?;

                            Ok(result_sets)
                        },
                    )
            }
        }

        impl<'de> ::serde::Deserialize<'de> for ResultSets {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let raw = Vec::<$crate::BasicResultSet>::deserialize(deserializer)?;

                raw.try_into().map_err(::serde::de::Error::custom)
            }
        }
    };
}
pub(crate) use endpoint;
