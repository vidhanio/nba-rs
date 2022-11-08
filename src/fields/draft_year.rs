use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DraftYear {
    #[serde(rename = "2020")]
    Year2020,

    #[serde(rename = "2019")]
    Year2019,

    #[serde(rename = "2018")]
    Year2018,

    #[serde(rename = "2017")]
    Year2017,

    #[serde(rename = "2016")]
    Year2016,

    #[serde(rename = "2015")]
    Year2015,

    #[serde(rename = "2014")]
    Year2014,

    #[serde(rename = "2013")]
    Year2013,

    #[serde(rename = "2012")]
    Year2012,

    #[serde(rename = "2011")]
    Year2011,

    #[serde(rename = "2010")]
    Year2010,

    #[serde(rename = "2009")]
    Year2009,

    #[serde(rename = "2008")]
    Year2008,

    #[serde(rename = "2007")]
    Year2007,

    #[serde(rename = "2006")]
    Year2006,

    #[serde(rename = "2005")]
    Year2005,

    #[serde(rename = "2004")]
    Year2004,

    #[serde(rename = "2003")]
    Year2003,

    #[serde(rename = "2002")]
    Year2002,

    #[serde(rename = "2001")]
    Year2001,

    #[serde(rename = "2000")]
    Year2000,

    #[serde(rename = "1999")]
    Year1999,

    #[serde(rename = "1998")]
    Year1998,

    #[serde(rename = "1997")]
    Year1997,

    #[serde(rename = "1996")]
    Year1996,

    #[serde(rename = "1995")]
    Year1995,

    #[serde(rename = "1994")]
    Year1994,

    #[serde(rename = "1993")]
    Year1993,

    #[serde(rename = "1992")]
    Year1992,

    #[serde(rename = "1991")]
    Year1991,

    #[serde(rename = "1990")]
    Year1990,

    #[serde(rename = "1989")]
    Year1989,

    #[serde(rename = "1988")]
    Year1988,

    #[serde(rename = "1987")]
    Year1987,

    #[serde(rename = "1986")]
    Year1986,

    #[serde(rename = "1985")]
    Year1985,

    #[serde(rename = "1984")]
    Year1984,

    #[serde(rename = "1983")]
    Year1983,

    #[serde(rename = "1982")]
    Year1982,

    #[serde(rename = "1981")]
    Year1981,

    #[serde(rename = "1980")]
    Year1980,

    #[serde(rename = "1979")]
    Year1979,

    #[serde(rename = "1978")]
    Year1978,

    #[serde(rename = "1977")]
    Year1977,

    #[serde(rename = "1976")]
    Year1976,

    #[serde(rename = "1975")]
    Year1975,

    #[serde(rename = "1974")]
    Year1974,

    #[serde(rename = "1973")]
    Year1973,

    #[serde(rename = "1972")]
    Year1972,

    #[serde(rename = "1971")]
    Year1971,

    #[serde(rename = "1970")]
    Year1970,

    #[serde(rename = "1969")]
    Year1969,

    #[serde(rename = "1968")]
    Year1968,

    #[serde(rename = "1967")]
    Year1967,

    #[serde(rename = "1966")]
    Year1966,

    #[serde(rename = "1965")]
    Year1965,

    #[serde(rename = "1964")]
    Year1964,

    #[serde(rename = "1963")]
    Year1963,

    #[serde(rename = "1962")]
    Year1962,

    #[serde(rename = "1961")]
    Year1961,

    #[serde(rename = "1960")]
    Year1960,

    #[serde(rename = "1959")]
    Year1959,

    #[serde(rename = "1958")]
    Year1958,

    #[serde(rename = "1957")]
    Year1957,

    #[serde(rename = "1956")]
    Year1956,

    #[serde(rename = "1955")]
    Year1955,

    #[serde(rename = "1954")]
    Year1954,

    #[serde(rename = "1953")]
    Year1953,

    #[serde(rename = "1952")]
    Year1952,

    #[serde(rename = "1951")]
    Year1951,

    #[serde(rename = "1950")]
    Year1950,

    #[serde(rename = "1949")]
    Year1949,

    #[serde(rename = "1948")]
    Year1948,

    #[serde(rename = "1947")]
    Year1947,
}
