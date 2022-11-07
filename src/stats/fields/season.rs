use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SeasonSince1946 {
    #[serde(rename = "1946-47")]
    From1946To47,

    #[serde(rename = "1947-48")]
    From1947To48,

    #[serde(rename = "1948-49")]
    From1948To49,

    #[serde(rename = "1949-50")]
    From1949To50,

    #[serde(rename = "1950-51")]
    From1950To51,

    #[serde(rename = "1951-52")]
    From1951To52,

    #[serde(rename = "1952-53")]
    From1952To53,

    #[serde(rename = "1953-54")]
    From1953To54,

    #[serde(rename = "1954-55")]
    From1954To55,

    #[serde(rename = "1955-56")]
    From1955To56,

    #[serde(rename = "1956-57")]
    From1956To57,

    #[serde(rename = "1957-58")]
    From1957To58,

    #[serde(rename = "1958-59")]
    From1958To59,

    #[serde(rename = "1959-60")]
    From1959To60,

    #[serde(rename = "1960-61")]
    From1960To61,

    #[serde(rename = "1961-62")]
    From1961To62,

    #[serde(rename = "1962-63")]
    From1962To63,

    #[serde(rename = "1963-64")]
    From1963To64,

    #[serde(rename = "1964-65")]
    From1964To65,

    #[serde(rename = "1965-66")]
    From1965To66,

    #[serde(rename = "1966-67")]
    From1966To67,

    #[serde(rename = "1967-68")]
    From1967To68,

    #[serde(rename = "1968-69")]
    From1968To69,

    #[serde(rename = "1969-70")]
    From1969To70,

    #[serde(rename = "1970-71")]
    From1970To71,

    #[serde(rename = "1971-72")]
    From1971To72,

    #[serde(rename = "1972-73")]
    From1972To73,

    #[serde(rename = "1973-74")]
    From1973To74,

    #[serde(rename = "1974-75")]
    From1974To75,

    #[serde(rename = "1975-76")]
    From1975To76,

    #[serde(rename = "1976-77")]
    From1976To77,

    #[serde(rename = "1977-78")]
    From1977To78,

    #[serde(rename = "1978-79")]
    From1978To79,

    #[serde(rename = "1979-80")]
    From1979To80,

    #[serde(rename = "1980-81")]
    From1980To81,

    #[serde(rename = "1981-82")]
    From1981To82,

    #[serde(rename = "1982-83")]
    From1982To83,

    #[serde(rename = "1983-84")]
    From1983To84,

    #[serde(rename = "1984-85")]
    From1984To85,

    #[serde(rename = "1985-86")]
    From1985To86,

    #[serde(rename = "1986-87")]
    From1986To87,

    #[serde(rename = "1987-88")]
    From1987To88,

    #[serde(rename = "1988-89")]
    From1988To89,

    #[serde(rename = "1989-90")]
    From1989To90,

    #[serde(rename = "1990-91")]
    From1990To91,

    #[serde(rename = "1991-92")]
    From1991To92,

    #[serde(rename = "1992-93")]
    From1992To93,

    #[serde(rename = "1993-94")]
    From1993To94,

    #[serde(rename = "1994-95")]
    From1994To95,

    #[serde(rename = "1995-96")]
    From1995To96,

    #[serde(rename = "1996-97")]
    From1996To97,

    #[serde(rename = "1997-98")]
    From1997To98,

    #[serde(rename = "1998-99")]
    From1998To99,

    #[serde(rename = "1999-00")]
    From1999To00,

    #[serde(rename = "2000-01")]
    From2000To01,

    #[serde(rename = "2001-02")]
    From2001To02,

    #[serde(rename = "2002-03")]
    From2002To03,

    #[serde(rename = "2003-04")]
    From2003To04,

    #[serde(rename = "2004-05")]
    From2004To05,

    #[serde(rename = "2005-06")]
    From2005To06,

    #[serde(rename = "2006-07")]
    From2006To07,

    #[serde(rename = "2007-08")]
    From2007To08,

    #[serde(rename = "2008-09")]
    From2008To09,

    #[serde(rename = "2009-10")]
    From2009To10,

    #[serde(rename = "2010-11")]
    From2010To11,

    #[serde(rename = "2011-12")]
    From2011To12,

    #[serde(rename = "2012-13")]
    From2012To13,

    #[serde(rename = "2013-14")]
    From2013To14,

    #[serde(rename = "2014-15")]
    From2014To15,

    #[serde(rename = "2015-16")]
    From2015To16,

    #[serde(rename = "2016-17")]
    From2016To17,

    #[serde(rename = "2017-18")]
    From2017To18,

    #[serde(rename = "2018-19")]
    From2018To19,

    #[serde(rename = "2019-20")]
    From2019To20,

    #[serde(rename = "2020-21")]
    From2020To21,

    #[serde(rename = "2021-22")]
    From2021To22,

    #[default]
    #[serde(rename = "2022-23")]
    From2022To23,
}

impl SeasonSince1946 {
    #[must_use]
    pub const fn next(&self) -> Option<Self> {
        match self {
            Self::From1946To47 => Some(Self::From1947To48),
            Self::From1947To48 => Some(Self::From1948To49),
            Self::From1948To49 => Some(Self::From1949To50),
            Self::From1949To50 => Some(Self::From1950To51),
            Self::From1950To51 => Some(Self::From1951To52),
            Self::From1951To52 => Some(Self::From1952To53),
            Self::From1952To53 => Some(Self::From1953To54),
            Self::From1953To54 => Some(Self::From1954To55),
            Self::From1954To55 => Some(Self::From1955To56),
            Self::From1955To56 => Some(Self::From1956To57),
            Self::From1956To57 => Some(Self::From1957To58),
            Self::From1957To58 => Some(Self::From1958To59),
            Self::From1958To59 => Some(Self::From1959To60),
            Self::From1959To60 => Some(Self::From1960To61),
            Self::From1960To61 => Some(Self::From1961To62),
            Self::From1961To62 => Some(Self::From1962To63),
            Self::From1962To63 => Some(Self::From1963To64),
            Self::From1963To64 => Some(Self::From1964To65),
            Self::From1964To65 => Some(Self::From1965To66),
            Self::From1965To66 => Some(Self::From1966To67),
            Self::From1966To67 => Some(Self::From1967To68),
            Self::From1967To68 => Some(Self::From1968To69),
            Self::From1968To69 => Some(Self::From1969To70),
            Self::From1969To70 => Some(Self::From1970To71),
            Self::From1970To71 => Some(Self::From1971To72),
            Self::From1971To72 => Some(Self::From1972To73),
            Self::From1972To73 => Some(Self::From1973To74),
            Self::From1973To74 => Some(Self::From1974To75),
            Self::From1974To75 => Some(Self::From1975To76),
            Self::From1975To76 => Some(Self::From1976To77),
            Self::From1976To77 => Some(Self::From1977To78),
            Self::From1977To78 => Some(Self::From1978To79),
            Self::From1978To79 => Some(Self::From1979To80),
            Self::From1979To80 => Some(Self::From1980To81),
            Self::From1980To81 => Some(Self::From1981To82),
            Self::From1981To82 => Some(Self::From1982To83),
            Self::From1982To83 => Some(Self::From1983To84),
            Self::From1983To84 => Some(Self::From1984To85),
            Self::From1984To85 => Some(Self::From1985To86),
            Self::From1985To86 => Some(Self::From1986To87),
            Self::From1986To87 => Some(Self::From1987To88),
            Self::From1987To88 => Some(Self::From1988To89),
            Self::From1988To89 => Some(Self::From1989To90),
            Self::From1989To90 => Some(Self::From1990To91),
            Self::From1990To91 => Some(Self::From1991To92),
            Self::From1991To92 => Some(Self::From1992To93),
            Self::From1992To93 => Some(Self::From1993To94),
            Self::From1993To94 => Some(Self::From1994To95),
            Self::From1994To95 => Some(Self::From1995To96),
            Self::From1995To96 => Some(Self::From1996To97),
            Self::From1996To97 => Some(Self::From1997To98),
            Self::From1997To98 => Some(Self::From1998To99),
            Self::From1998To99 => Some(Self::From1999To00),
            Self::From1999To00 => Some(Self::From2000To01),
            Self::From2000To01 => Some(Self::From2001To02),
            Self::From2001To02 => Some(Self::From2002To03),
            Self::From2002To03 => Some(Self::From2003To04),
            Self::From2003To04 => Some(Self::From2004To05),
            Self::From2004To05 => Some(Self::From2005To06),
            Self::From2005To06 => Some(Self::From2006To07),
            Self::From2006To07 => Some(Self::From2007To08),
            Self::From2007To08 => Some(Self::From2008To09),
            Self::From2008To09 => Some(Self::From2009To10),
            Self::From2009To10 => Some(Self::From2010To11),
            Self::From2010To11 => Some(Self::From2011To12),
            Self::From2011To12 => Some(Self::From2012To13),
            Self::From2012To13 => Some(Self::From2013To14),
            Self::From2013To14 => Some(Self::From2014To15),
            Self::From2014To15 => Some(Self::From2015To16),
            Self::From2015To16 => Some(Self::From2016To17),
            Self::From2016To17 => Some(Self::From2017To18),
            Self::From2017To18 => Some(Self::From2018To19),
            Self::From2018To19 => Some(Self::From2019To20),
            Self::From2019To20 => Some(Self::From2020To21),
            Self::From2020To21 => Some(Self::From2021To22),
            Self::From2021To22 => Some(Self::From2022To23),
            Self::From2022To23 => None,
        }
    }
}

impl IntoIterator for SeasonSince1946 {
    type Item = Self;
    type IntoIter = SeasonSince1946Iterator;

    fn into_iter(self) -> Self::IntoIter {
        SeasonSince1946Iterator { next: Some(self) }
    }
}

#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SeasonSince1946Iterator {
    next: Option<SeasonSince1946>,
}

impl Iterator for SeasonSince1946Iterator {
    type Item = SeasonSince1946;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        self.next = self.next.and_then(|s| s.next());
        next
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SeasonSince1996 {
    #[serde(rename = "1996-97")]
    From1996To97,

    #[serde(rename = "1997-98")]
    From1997To98,

    #[serde(rename = "1998-99")]
    From1998To99,

    #[serde(rename = "1999-00")]
    From1999To00,

    #[serde(rename = "2000-01")]
    From2000To01,

    #[serde(rename = "2001-02")]
    From2001To02,

    #[serde(rename = "2002-03")]
    From2002To03,

    #[serde(rename = "2003-04")]
    From2003To04,

    #[serde(rename = "2004-05")]
    From2004To05,

    #[serde(rename = "2005-06")]
    From2005To06,

    #[serde(rename = "2006-07")]
    From2006To07,

    #[serde(rename = "2007-08")]
    From2007To08,

    #[serde(rename = "2008-09")]
    From2008To09,

    #[serde(rename = "2009-10")]
    From2009To10,

    #[serde(rename = "2010-11")]
    From2010To11,

    #[serde(rename = "2011-12")]
    From2011To12,

    #[serde(rename = "2012-13")]
    From2012To13,

    #[serde(rename = "2013-14")]
    From2013To14,

    #[serde(rename = "2014-15")]
    From2014To15,

    #[serde(rename = "2015-16")]
    From2015To16,

    #[serde(rename = "2016-17")]
    From2016To17,

    #[serde(rename = "2017-18")]
    From2017To18,

    #[serde(rename = "2018-19")]
    From2018To19,

    #[serde(rename = "2019-20")]
    From2019To20,

    #[serde(rename = "2020-21")]
    From2020To21,

    #[serde(rename = "2021-22")]
    From2021To22,

    #[default]
    #[serde(rename = "2022-23")]
    From2022To23,
}

super::convert! {
    SeasonSince1996 => SeasonSince1946 {
        From1996To97,
        From1997To98,
        From1998To99,
        From1999To00,
        From2000To01,
        From2001To02,
        From2002To03,
        From2003To04,
        From2004To05,
        From2005To06,
        From2006To07,
        From2007To08,
        From2008To09,
        From2009To10,
        From2010To11,
        From2011To12,
        From2012To13,
        From2013To14,
        From2014To15,
        From2015To16,
        From2016To17,
        From2017To18,
        From2018To19,
        From2019To20,
        From2020To21,
        From2021To22,
        From2022To23,
    }
}
