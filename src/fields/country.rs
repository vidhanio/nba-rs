use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Country {
    #[serde(rename = "US")]
    UnitedStates,

    #[serde(rename = "CA")]
    Canada,

    #[serde(rename = "AF")]
    Afghanistan,

    #[serde(rename = "AX")]
    AlandIslands,

    #[serde(rename = "AL")]
    Albania,

    #[serde(rename = "DZ")]
    Algeria,

    #[serde(rename = "AS")]
    AmericanSamoa,

    #[serde(rename = "AD")]
    Andorra,

    #[serde(rename = "AO")]
    Angola,

    #[serde(rename = "AI")]
    Anguilla,

    #[serde(rename = "AQ")]
    Antarctica,

    #[serde(rename = "AG")]
    AntiguaAndBarbuda,

    #[serde(rename = "AR")]
    Argentina,

    #[serde(rename = "AM")]
    Armenia,

    #[serde(rename = "AW")]
    Aruba,

    #[serde(rename = "AU")]
    Australia,

    #[serde(rename = "AT")]
    Austria,

    #[serde(rename = "AZ")]
    Azerbaijan,

    #[serde(rename = "BS")]
    Bahamas,

    #[serde(rename = "BH")]
    Bahrain,

    #[serde(rename = "BD")]
    Bangladesh,

    #[serde(rename = "BB")]
    Barbados,

    #[serde(rename = "BY")]
    Belarus,

    #[serde(rename = "BE")]
    Belgium,

    #[serde(rename = "BZ")]
    Belize,

    #[serde(rename = "BJ")]
    Benin,

    #[serde(rename = "BM")]
    Bermuda,

    #[serde(rename = "BT")]
    Bhutan,

    #[serde(rename = "BO")]
    Bolivia,

    #[serde(rename = "BQ")]
    Bonaire,

    #[serde(rename = "BA")]
    BosniaAndHerzegovina,

    #[serde(rename = "BW")]
    Botswana,

    #[serde(rename = "BV")]
    BouvetIsland,

    #[serde(rename = "BR")]
    Brazil,

    #[serde(rename = "IO")]
    BritishIndianOceanTerritory,

    #[serde(rename = "BN")]
    Brunei,

    #[serde(rename = "BG")]
    Bulgaria,

    #[serde(rename = "BF")]
    BurkinaFaso,

    #[serde(rename = "BI")]
    Burundi,

    #[serde(rename = "KH")]
    Cambodia,

    #[serde(rename = "CM")]
    Cameroon,

    #[serde(rename = "CV")]
    CapeVerde,

    #[serde(rename = "KY")]
    CaymanIslands,

    #[serde(rename = "CF")]
    CentralAfricanRepublic,

    #[serde(rename = "TD")]
    Chad,

    #[serde(rename = "CL")]
    Chile,

    #[serde(rename = "CN")]
    MainlandChina,

    #[serde(rename = "CX")]
    ChristmasIsland,

    #[serde(rename = "CC")]
    CocosKeelingIslands,

    #[serde(rename = "CO")]
    Colombia,

    #[serde(rename = "KM")]
    Comoros,

    #[serde(rename = "CG")]
    CongoRepublicOfThe,

    #[serde(rename = "CD")]
    CongoDemocraticRepublicOfThe,

    #[serde(rename = "CK")]
    CookIslands,

    #[serde(rename = "CR")]
    CostaRica,

    #[serde(rename = "CI")]
    CoteDIvoire,

    #[serde(rename = "HR")]
    Croatia,

    #[serde(rename = "CU")]
    Cuba,

    #[serde(rename = "AN")]
    Curacao,

    #[serde(rename = "CY")]
    Cyprus,

    #[serde(rename = "CZ")]
    CzechRepublic,

    #[serde(rename = "DK")]
    Denmark,

    #[serde(rename = "DJ")]
    Djibouti,

    #[serde(rename = "DM")]
    Dominica,

    #[serde(rename = "DO")]
    DominicanRepublic,

    #[serde(rename = "EC")]
    Ecuador,

    #[serde(rename = "EG")]
    Egypt,

    #[serde(rename = "SV")]
    ElSalvador,

    #[serde(rename = "GQ")]
    EquatorialGuinea,

    #[serde(rename = "ER")]
    Eritrea,

    #[serde(rename = "EE")]
    Estonia,

    #[serde(rename = "ET")]
    Ethiopia,

    #[serde(rename = "FK")]
    FalklandIslandsMalvinas,

    #[serde(rename = "FO")]
    FaroeIslands,

    #[serde(rename = "FJ")]
    Fiji,

    #[serde(rename = "FI")]
    Finland,

    #[serde(rename = "FR")]
    France,

    #[serde(rename = "GF")]
    FrenchGuiana,

    #[serde(rename = "PF")]
    FrenchPolynesia,

    #[serde(rename = "TF")]
    FrenchSouthernTerritories,

    #[serde(rename = "GA")]
    Gabon,

    #[serde(rename = "GM")]
    Gambia,

    #[serde(rename = "GE")]
    Georgia,

    #[serde(rename = "DE")]
    Germany,

    #[serde(rename = "GH")]
    Ghana,

    #[serde(rename = "GI")]
    Gibraltar,

    #[serde(rename = "GR")]
    Greece,

    #[serde(rename = "GL")]
    Greenland,

    #[serde(rename = "GD")]
    Grenada,

    #[serde(rename = "GP")]
    Guadeloupe,

    #[serde(rename = "GU")]
    Guam,

    #[serde(rename = "GT")]
    Guatemala,

    #[serde(rename = "GG")]
    Guernsey,

    #[serde(rename = "GN")]
    Guinea,

    #[serde(rename = "GW")]
    GuineaBissau,

    #[serde(rename = "GY")]
    Guyana,

    #[serde(rename = "HT")]
    Haiti,

    #[serde(rename = "HM")]
    HeardIslandAndMcdonaldIslands,

    #[serde(rename = "VA")]
    HolySeeVaticanCityState,

    #[serde(rename = "HN")]
    Honduras,

    #[serde(rename = "HK")]
    HongKong,

    #[serde(rename = "HU")]
    Hungary,

    #[serde(rename = "IS")]
    Iceland,

    #[serde(rename = "IN")]
    India,

    #[serde(rename = "ID")]
    Indonesia,

    #[serde(rename = "IR")]
    Iran,

    #[serde(rename = "IQ")]
    Iraq,

    #[serde(rename = "IE")]
    Ireland,

    #[serde(rename = "IM")]
    IsleOfMan,

    #[serde(rename = "IL")]
    Israel,

    #[serde(rename = "IT")]
    Italy,

    #[serde(rename = "JM")]
    Jamaica,

    #[serde(rename = "JP")]
    Japan,

    #[serde(rename = "JE")]
    Jersey,

    #[serde(rename = "JO")]
    Jordan,

    #[serde(rename = "KZ")]
    Kazakhstan,

    #[serde(rename = "KE")]
    Kenya,

    #[serde(rename = "KI")]
    Kiribati,

    #[serde(rename = "XK")]
    Kosovo,

    #[serde(rename = "KW")]
    Kuwait,

    #[serde(rename = "KG")]
    Kyrgyzstan,

    #[serde(rename = "LA")]
    Laos,

    #[serde(rename = "LV")]
    Latvia,

    #[serde(rename = "LB")]
    Lebanon,

    #[serde(rename = "LS")]
    Lesotho,

    #[serde(rename = "LR")]
    Liberia,

    #[serde(rename = "LY")]
    Libya,

    #[serde(rename = "LI")]
    Liechtenstein,

    #[serde(rename = "LT")]
    Lithuania,

    #[serde(rename = "LU")]
    Luxembourg,

    #[serde(rename = "MO")]
    Macao,

    #[serde(rename = "MG")]
    Madagascar,

    #[serde(rename = "MW")]
    Malawi,

    #[serde(rename = "MY")]
    Malaysia,

    #[serde(rename = "MV")]
    Maldives,

    #[serde(rename = "ML")]
    Mali,

    #[serde(rename = "MT")]
    Malta,

    #[serde(rename = "MH")]
    MarshallIslands,

    #[serde(rename = "MQ")]
    Martinique,

    #[serde(rename = "MR")]
    Mauritania,

    #[serde(rename = "MU")]
    Mauritius,

    #[serde(rename = "YT")]
    Mayotte,

    #[serde(rename = "MX")]
    Mexico,

    #[serde(rename = "FM")]
    MicronesiaFederatedStatesOf,

    #[serde(rename = "MD")]
    Moldova,

    #[serde(rename = "MC")]
    Monaco,

    #[serde(rename = "MN")]
    Mongolia,

    #[serde(rename = "ME")]
    Montenegro,

    #[serde(rename = "MS")]
    Montserrat,

    #[serde(rename = "MA")]
    Morocco,

    #[serde(rename = "MZ")]
    Mozambique,

    #[serde(rename = "MM")]
    Myanmar,

    #[serde(rename = "NA")]
    Namibia,

    #[serde(rename = "NR")]
    Nauru,

    #[serde(rename = "NP")]
    Nepal,

    #[serde(rename = "NL")]
    Netherlands,

    #[serde(rename = "AN")]
    NetherlandsAntilles,

    #[serde(rename = "NC")]
    NewCaledonia,

    #[serde(rename = "NZ")]
    NewZealand,

    #[serde(rename = "NI")]
    Nicaragua,

    #[serde(rename = "NE")]
    Niger,

    #[serde(rename = "NG")]
    Nigeria,

    #[serde(rename = "NU")]
    Niue,

    #[serde(rename = "NF")]
    NorfolkIsland,

    #[serde(rename = "KP")]
    NorthKorea,

    #[serde(rename = "MK")]
    NorthMacedonia,

    #[serde(rename = "MP")]
    NorthernMarianaIslands,

    #[serde(rename = "NO")]
    Norway,

    #[serde(rename = "OM")]
    Oman,

    #[serde(rename = "PK")]
    Pakistan,

    #[serde(rename = "PW")]
    Palau,

    #[serde(rename = "PS")]
    PalestinianTerritories,

    #[serde(rename = "PA")]
    Panama,

    #[serde(rename = "PG")]
    PapuaNewGuinea,

    #[serde(rename = "PY")]
    Paraguay,

    #[serde(rename = "PE")]
    Peru,

    #[serde(rename = "PH")]
    Philippines,

    #[serde(rename = "PN")]
    Pitcairn,

    #[serde(rename = "PL")]
    Poland,

    #[serde(rename = "PT")]
    Portugal,

    #[serde(rename = "PR")]
    PuertoRico,

    #[serde(rename = "QA")]
    Qatar,

    #[serde(rename = "RE")]
    Reunion,

    #[serde(rename = "RO")]
    Romania,

    #[serde(rename = "RU")]
    Russia,

    #[serde(rename = "RW")]
    Rwanda,

    #[serde(rename = "SH")]
    SaintHelena,

    #[serde(rename = "KN")]
    SaintKittsAndNevis,

    #[serde(rename = "LC")]
    SaintLucia,

    #[serde(rename = "MF")]
    SaintMartin,

    #[serde(rename = "PM")]
    SaintPierreAndMiquelon,

    #[serde(rename = "VC")]
    SaintVincentAndTheGrenadines,

    #[serde(rename = "WS")]
    Samoa,

    #[serde(rename = "SM")]
    SanMarino,

    #[serde(rename = "ST")]
    SaoTomeAndPrincipe,

    #[serde(rename = "SA")]
    SaudiArabia,

    #[serde(rename = "SN")]
    Senegal,

    #[serde(rename = "RS")]
    Serbia,

    #[serde(rename = "SC")]
    Seychelles,

    #[serde(rename = "SL")]
    SierraLeone,

    #[serde(rename = "SG")]
    Singapore,

    #[serde(rename = "SK")]
    Slovakia,

    #[serde(rename = "SI")]
    Slovenia,

    #[serde(rename = "SB")]
    SolomonIslands,

    #[serde(rename = "SO")]
    Somalia,

    #[serde(rename = "ZA")]
    SouthAfrica,

    #[serde(rename = "GS")]
    SouthGeorgiaAndTheSouthSandwichIslands,

    #[serde(rename = "KR")]
    SouthKorea,

    #[serde(rename = "SS")]
    SouthSudan,

    #[serde(rename = "ES")]
    Spain,

    #[serde(rename = "LK")]
    SriLanka,

    #[serde(rename = "SD")]
    Sudan,

    #[serde(rename = "SR")]
    Suriname,

    #[serde(rename = "SJ")]
    SavlbardAndJanMayen,

    #[serde(rename = "SZ")]
    Swaziland,

    #[serde(rename = "SE")]
    Sweden,

    #[serde(rename = "CH")]
    Switzerland,

    #[serde(rename = "SY")]
    Syria,

    #[serde(rename = "TW")]
    Taiwan,

    #[serde(rename = "TJ")]
    Tajikistan,

    #[serde(rename = "TZ")]
    Tanzania,

    #[serde(rename = "TH")]
    Thailand,

    #[serde(rename = "TL")]
    TimorLeste,

    #[serde(rename = "TG")]
    Togo,

    #[serde(rename = "TK")]
    Tokelau,

    #[serde(rename = "TO")]
    Tonga,

    #[serde(rename = "TT")]
    TrinidadAndTobago,

    #[serde(rename = "TN")]
    Tunisia,

    #[serde(rename = "TR")]
    Turkey,

    #[serde(rename = "TM")]
    Turkmenistan,

    #[serde(rename = "TC")]
    TurksAndCaicosIslands,

    #[serde(rename = "TV")]
    Tuvalu,

    #[serde(rename = "UG")]
    Uganda,

    #[serde(rename = "UA")]
    Ukraine,

    #[serde(rename = "AE")]
    UnitedArabEmirates,

    #[serde(rename = "GB")]
    UnitedKingdom,

    #[serde(rename = "UM")]
    UnitedStatesMinorOutlyingIslands,

    #[serde(rename = "UY")]
    Uruguay,

    #[serde(rename = "UZ")]
    Uzbekistan,

    #[serde(rename = "VU")]
    Vanuatu,

    #[serde(rename = "VE")]
    Venezuela,

    #[serde(rename = "VN")]
    Vietnam,

    #[serde(rename = "VG")]
    VirginIslandsBritish,

    #[serde(rename = "VI")]
    VirginIslandsUS,

    #[serde(rename = "WF")]
    WallisAndFutuna,

    #[serde(rename = "EH")]
    WesternSahara,

    #[serde(rename = "YE")]
    Yemen,

    #[serde(rename = "ZM")]
    Zambia,

    #[serde(rename = "ZW")]
    Zimbabwe,
}
