pub mod fields {
    pub use crate::{
        serde::{serde_none_as_empty_string, serde_optional_infallible},
        stats::fields::{
            LeagueId, PerModeGeneralTraditional as PerMode, Season2022To1996 as Season, SeasonType,
            YesOrNo,
        },
    };

    use serde::{Deserialize, Serialize};
    use serde_repr::{Deserialize_repr, Serialize_repr};

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum MeasureType {
        #[default]
        #[serde(rename = "Base")]
        Base,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
    #[repr(u8)]
    pub enum LastNGames {
        #[default]
        AllGames = 0,

        LastGame = 1,

        Last2Games = 2,

        Last3Games = 3,

        Last4Games = 4,

        Last5Games = 5,

        Last6Games = 6,

        Last7Games = 7,

        Last8Games = 8,

        Last9Games = 9,

        Last10Games = 10,

        Last11Games = 11,

        Last12Games = 12,

        Last13Games = 13,

        Last14Games = 14,

        Last15Games = 15,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
    #[repr(u8)]
    pub enum Month {
        #[default]
        AllMonths = 0,

        January = 4,

        February = 5,

        March = 6,

        April = 7,

        May = 8,

        June = 9,

        July = 10,

        August = 11,

        September = 12,

        October = 1,

        November = 2,

        December = 3,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SeasonSegment {
        #[serde(rename = "Pre All-Star")]
        PreAllStar,

        #[serde(rename = "Post All-Star")]
        PostAllStar,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Position {
        #[default]
        AllPositions,

        #[serde(rename = "F")]
        Forward,

        #[serde(rename = "C")]
        Center,

        #[serde(rename = "G")]
        Guard,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum StarterOrBench {
        #[serde(rename = "Starters")]
        Starters,

        #[serde(rename = "Bench")]
        Bench,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Experience {
        #[serde(rename = "Rookie")]
        Rookie,

        #[serde(rename = "Sophomore")]
        Sophomore,

        #[serde(rename = "Veteran")]
        Veteran,
    }

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

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DraftPick {
        #[serde(rename = "1st Round")]
        Pick1stRound,

        #[serde(rename = "2nd Round")]
        Pick2ndRound,

        #[serde(rename = "1st Pick")]
        Pick1stPick,

        #[serde(rename = "Lottery Pick")]
        LotteryPick,

        #[serde(rename = "Top 5 Pick")]
        Top5Pick,

        #[serde(rename = "Top 10 Pick")]
        Top10Pick,

        #[serde(rename = "Top 15 Pick")]
        Top15Pick,

        #[serde(rename = "Top 20 Pick")]
        Top20Pick,

        #[serde(rename = "Top 25 Pick")]
        Top25Pick,

        #[serde(rename = "Picks 11 Thru 20")]
        Picks11Thru20,

        #[serde(rename = "Picks 21 Thru 30")]
        Picks21Thru30,

        #[serde(rename = "Undrafted")]
        Undrafted,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum College {
        #[serde(rename = "High School")]
        HighSchool,

        #[serde(rename = "None")]
        NoCollege,

        #[serde(rename = "Acadia (CAN)")]
        AcadiaCan,

        #[serde(rename = "Adams State")]
        AdamsState,

        #[serde(rename = "Adelphi")]
        Adelphi,

        #[serde(rename = "Air Force")]
        AirForce,

        #[serde(rename = "Akron")]
        Akron,

        #[serde(rename = "Alabama")]
        Alabama,

        #[serde(rename = "Alabama A&M")]
        AlabamaAM,

        #[serde(rename = "Alabama-Birmingham")]
        AlabamaBirmingham,

        #[serde(rename = "Alabama-Huntsville")]
        AlabamaHuntsville,

        #[serde(rename = "Alabama State")]
        AlabamaState,

        #[serde(rename = "Albany")]
        Albany,

        #[serde(rename = "Albany State (GA)")]
        AlbanyStateGa,

        #[serde(rename = "Alcorn State")]
        AlcornState,

        #[serde(rename = "Alliance")]
        Alliance,

        #[serde(rename = "American")]
        American,

        #[serde(rename = "American International")]
        AmericanInternational,

        #[serde(rename = "Angelo State")]
        AngeloState,

        #[serde(rename = "Appalachian State")]
        AppalachianState,

        #[serde(rename = "Arizona")]
        Arizona,

        #[serde(rename = "Arizona State")]
        ArizonaState,

        #[serde(rename = "Arkansas")]
        Arkansas,

        #[serde(rename = "Arkansas-Fort Smith")]
        ArkansasFortSmith,

        #[serde(rename = "Arkansas-Little Rock")]
        ArkansasLittleRock,

        #[serde(rename = "Arkansas-Pine Bluff")]
        ArkansasPineBluff,

        #[serde(rename = "Arkansas State")]
        ArkansasState,

        #[serde(rename = "Army")]
        Army,

        #[serde(rename = "Assumption")]
        Assumption,

        #[serde(rename = "Auburn")]
        Auburn,

        #[serde(rename = "Auburn-Montgomery")]
        AuburnMontgomery,

        #[serde(rename = "Augsburg")]
        Augsburg,

        #[serde(rename = "Augustana (IL)")]
        AugustanaIl,

        #[serde(rename = "Augustana (SD)")]
        AugustanaSd,

        #[serde(rename = "Augusta State")]
        AugustaState,

        #[serde(rename = "Aurora")]
        Aurora,

        #[serde(rename = "Austin Peay")]
        AustinPeay,

        #[serde(rename = "Averett")]
        Averett,

        #[serde(rename = "Azusa Pacific")]
        AzusaPacific,

        #[serde(rename = "Ball State")]
        BallState,

        #[serde(rename = "Baltimore")]
        Baltimore,

        #[serde(rename = "Barton")]
        Barton,

        #[serde(rename = "Barton Community College")]
        BartonCommunityCollege,

        #[serde(rename = "Baylor")]
        Baylor,

        #[serde(rename = "Belmont")]
        Belmont,

        #[serde(rename = "Belmont Abbey")]
        BelmontAbbey,

        #[serde(rename = "Beloit")]
        Beloit,

        #[serde(rename = "Bemidji State")]
        BemidjiState,

        #[serde(rename = "Benedict")]
        Benedict,

        #[serde(rename = "Bethel (TN)")]
        BethelTn,

        #[serde(rename = "Binghamton")]
        Binghamton,

        #[serde(rename = "Biola")]
        Biola,

        #[serde(rename = "Blinn")]
        Blinn,

        #[serde(rename = "Bloomfield")]
        Bloomfield,

        #[serde(rename = "Bloomsburg")]
        Bloomsburg,

        #[serde(rename = "Bluefield")]
        Bluefield,

        #[serde(rename = "Boise State")]
        BoiseState,

        #[serde(rename = "Boston College")]
        BostonCollege,

        #[serde(rename = "Boston University")]
        BostonUniversity,

        #[serde(rename = "Bowie State")]
        BowieState,

        #[serde(rename = "Bowling Green")]
        BowlingGreen,

        #[serde(rename = "Bradley")]
        Bradley,

        #[serde(rename = "Brevard Community College")]
        BrevardCommunityCollege,

        #[serde(rename = "Brewton-Parker")]
        BrewtonParker,

        #[serde(rename = "Bridgeport")]
        Bridgeport,

        #[serde(rename = "Brigham Young")]
        BrighamYoung,

        #[serde(rename = "Brigham Young-Hawaii")]
        BrighamYoungHawaii,

        #[serde(rename = "Brooklyn")]
        Brooklyn,

        #[serde(rename = "Brown")]
        Brown,

        #[serde(rename = "Bryant")]
        Bryant,

        #[serde(rename = "Bucknell")]
        Bucknell,

        #[serde(rename = "Buffalo")]
        Buffalo,

        #[serde(rename = "Buffalo State")]
        BuffaloState,

        #[serde(rename = "Butler")]
        Butler,

        #[serde(rename = "Butler Community College")]
        ButlerCommunityCollege,

        #[serde(rename = "C.W. Post")]
        CWPost,

        #[serde(rename = "California")]
        California,

        #[serde(rename = "California (PA)")]
        CaliforniaPa,

        #[serde(rename = "California-Berkeley")]
        CaliforniaBerkeley,

        #[serde(rename = "California-Davis")]
        CaliforniaDavis,

        #[serde(rename = "California-Irvine")]
        CaliforniaIrvine,

        #[serde(rename = "California-Los Angeles")]
        CaliforniaLosAngeles,

        #[serde(rename = "California-Riverside")]
        CaliforniaRiverside,

        #[serde(rename = "California-Santa Barbara")]
        CaliforniaSantaBarbara,

        #[serde(rename = "Cal State-Bakersfield")]
        CalStateBakersfield,

        #[serde(rename = "Cal State-Dominguez Hills")]
        CalStateDominguezHills,

        #[serde(rename = "Cal State-Fullerton")]
        CalStateFullerton,

        #[serde(rename = "Cal State-Los Angeles")]
        CalStateLosAngeles,

        #[serde(rename = "Cal State-Northridge")]
        CalStateNorthridge,

        #[serde(rename = "Cal State-Poly Pomona")]
        CalStatePolyPomona,

        #[serde(rename = "Cal State-San Bernardino")]
        CalStateSanBernardino,

        #[serde(rename = "Campbell")]
        Campbell,

        #[serde(rename = "Campbellsville")]
        Campbellsville,

        #[serde(rename = "Canisius")]
        Canisius,

        #[serde(rename = "Case Western")]
        CaseWestern,

        #[serde(rename = "Catholic")]
        Catholic,

        #[serde(rename = "Centenary (LA)")]
        CentenaryLa,

        #[serde(rename = "Central Arkansas")]
        CentralArkansas,

        #[serde(rename = "Central Connecticut State")]
        CentralConnecticutState,

        #[serde(rename = "Central Florida")]
        CentralFlorida,

        #[serde(rename = "Central Michigan")]
        CentralMichigan,

        #[serde(rename = "Central Missouri")]
        CentralMissouri,

        #[serde(rename = "Central Oklahoma")]
        CentralOklahoma,

        #[serde(rename = "Central State (OH)")]
        CentralStateOh,

        #[serde(rename = "Central Washington")]
        CentralWashington,

        #[serde(rename = "Charleston (WV)")]
        CharlestonWv,

        #[serde(rename = "Cheyney")]
        Cheyney,

        #[serde(rename = "Chicago State")]
        ChicagoState,

        #[serde(rename = "Christopher Newport")]
        ChristopherNewport,

        #[serde(rename = "Cincinnati")]
        Cincinnati,

        #[serde(rename = "City College of New York")]
        CityCollegeOfNewYork,

        #[serde(rename = "Claflin")]
        Claflin,

        #[serde(rename = "Clark Atlanta")]
        ClarkAtlanta,

        #[serde(rename = "Clayton State")]
        ClaytonState,

        #[serde(rename = "Clemson")]
        Clemson,

        #[serde(rename = "Cleveland State")]
        ClevelandState,

        #[serde(rename = "Colgate")]
        Colgate,

        #[serde(rename = "College of Charleston")]
        CollegeOfCharleston,

        #[serde(rename = "College of New Jersey")]
        CollegeOfNewJersey,

        #[serde(rename = "Colorado")]
        Colorado,

        #[serde(rename = "Colorado Christian")]
        ColoradoChristian,

        #[serde(rename = "Colorado State")]
        ColoradoState,

        #[serde(rename = "Colorado State-Pueblo")]
        ColoradoStatePueblo,

        #[serde(rename = "Columbia")]
        Columbia,

        #[serde(rename = "Columbus State")]
        ColumbusState,

        #[serde(rename = "Concordia")]
        Concordia,

        #[serde(rename = "Concordia-Irvine")]
        ConcordiaIrvine,

        #[serde(rename = "Connecticut")]
        Connecticut,

        #[serde(rename = "Coppin State")]
        CoppinState,

        #[serde(rename = "Cornell")]
        Cornell,

        #[serde(rename = "Corpus Christi")]
        CorpusChristi,

        #[serde(rename = "Creighton")]
        Creighton,

        #[serde(rename = "Culver-Stockton")]
        CulverStockton,

        #[serde(rename = "Dakota Wesleyan")]
        DakotaWesleyan,

        #[serde(rename = "Dartmouth")]
        Dartmouth,

        #[serde(rename = "Davidson")]
        Davidson,

        #[serde(rename = "Davis & Elkins")]
        DavisElkins,

        #[serde(rename = "Dayton")]
        Dayton,

        #[serde(rename = "Delaware")]
        Delaware,

        #[serde(rename = "Delaware State")]
        DelawareState,

        #[serde(rename = "Delta State")]
        DeltaState,

        #[serde(rename = "Denver")]
        Denver,

        #[serde(rename = "DePaul")]
        DePaul,

        #[serde(rename = "DePauw")]
        DePauw,

        #[serde(rename = "Detroit Mercy")]
        DetroitMercy,

        #[serde(rename = "Dillard")]
        Dillard,

        #[serde(rename = "District of Columbia")]
        DistrictOfColumbia,

        #[serde(rename = "Drake")]
        Drake,

        #[serde(rename = "Drexel")]
        Drexel,

        #[serde(rename = "Duke")]
        Duke,

        #[serde(rename = "Duquesne")]
        Duquesne,

        #[serde(rename = "East Carolina")]
        EastCarolina,

        #[serde(rename = "East Central")]
        EastCentral,

        #[serde(rename = "Eastern Illinois")]
        EasternIllinois,

        #[serde(rename = "Eastern Kentucky")]
        EasternKentucky,

        #[serde(rename = "Eastern Michigan")]
        EasternMichigan,

        #[serde(rename = "Eastern New Mexico")]
        EasternNewMexico,

        #[serde(rename = "Eastern Oklahoma State (Junior College)")]
        EasternOklahomaStateJuniorCollege,

        #[serde(rename = "Eastern Washington")]
        EasternWashington,

        #[serde(rename = "East Tennessee State")]
        EastTennesseeState,

        #[serde(rename = "Eckerd")]
        Eckerd,

        #[serde(rename = "Elizabeth City State")]
        ElizabethCityState,

        #[serde(rename = "Elon")]
        Elon,

        #[serde(rename = "Evangel")]
        Evangel,

        #[serde(rename = "Evansville")]
        Evansville,

        #[serde(rename = "Fairfield")]
        Fairfield,

        #[serde(rename = "Fairleigh Dickinson")]
        FairleighDickinson,

        #[serde(rename = "Fayetteville State")]
        FayettevilleState,

        #[serde(rename = "Ferris State")]
        FerrisState,

        #[serde(rename = "Findlay")]
        Findlay,

        #[serde(rename = "Florida")]
        Florida,

        #[serde(rename = "Florida A&M")]
        FloridaAM,

        #[serde(rename = "Florida Atlantic")]
        FloridaAtlantic,

        #[serde(rename = "Florida Community College")]
        FloridaCommunityCollege,

        #[serde(rename = "Florida Gulf Coast")]
        FloridaGulfCoast,

        #[serde(rename = "Florida International")]
        FloridaInternational,

        #[serde(rename = "Florida State")]
        FloridaState,

        #[serde(rename = "Fordham")]
        Fordham,

        #[serde(rename = "Fort Valley State")]
        FortValleyState,

        #[serde(rename = "Franklin")]
        Franklin,

        #[serde(rename = "Fresno State")]
        FresnoState,

        #[serde(rename = "Friends")]
        Friends,

        #[serde(rename = "Furman")]
        Furman,

        #[serde(rename = "Gannon")]
        Gannon,

        #[serde(rename = "Gardner-Webb")]
        GardnerWebb,

        #[serde(rename = "George Mason")]
        GeorgeMason,

        #[serde(rename = "Georgetown")]
        Georgetown,

        #[serde(rename = "Georgetown College")]
        GeorgetownCollege,

        #[serde(rename = "George Washington")]
        GeorgeWashington,

        #[serde(rename = "Georgia")]
        Georgia,

        #[serde(rename = "Georgia Southern")]
        GeorgiaSouthern,

        #[serde(rename = "Georgia State")]
        GeorgiaState,

        #[serde(rename = "Georgia Tech")]
        GeorgiaTech,

        #[serde(rename = "Gonzaga")]
        Gonzaga,

        #[serde(rename = "Grambling")]
        Grambling,

        #[serde(rename = "Grand Canyon")]
        GrandCanyon,

        #[serde(rename = "Green Bay")]
        GreenBay,

        #[serde(rename = "Grinnell College")]
        GrinnellCollege,

        #[serde(rename = "Guilford")]
        Guilford,

        #[serde(rename = "Hamline")]
        Hamline,

        #[serde(rename = "Hampton")]
        Hampton,

        #[serde(rename = "Hardin-Simmons")]
        HardinSimmons,

        #[serde(rename = "Hartford")]
        Hartford,

        #[serde(rename = "Harvard")]
        Harvard,

        #[serde(rename = "Hawaii")]
        Hawaii,

        #[serde(rename = "High Point")]
        HighPoint,

        #[serde(rename = "Hillsborough Community College")]
        HillsboroughCommunityCollege,

        #[serde(rename = "Hillsdale")]
        Hillsdale,

        #[serde(rename = "Hofstra")]
        Hofstra,

        #[serde(rename = "Holy Cross")]
        HolyCross,

        #[serde(rename = "Holy Family")]
        HolyFamily,

        #[serde(rename = "Houston")]
        Houston,

        #[serde(rename = "Houston Baptist")]
        HoustonBaptist,

        #[serde(rename = "Howard")]
        Howard,

        #[serde(rename = "Humboldt State")]
        HumboldtState,

        #[serde(rename = "Huntington")]
        Huntington,

        #[serde(rename = "Idaho")]
        Idaho,

        #[serde(rename = "Idaho State")]
        IdahoState,

        #[serde(rename = "Illinois")]
        Illinois,

        #[serde(rename = "Illinois-Chicago")]
        IllinoisChicago,

        #[serde(rename = "Illinois State")]
        IllinoisState,

        #[serde(rename = "Illinois Wesleyan")]
        IllinoisWesleyan,

        #[serde(rename = "Incarnate Word")]
        IncarnateWord,

        #[serde(rename = "Indiana")]
        Indiana,

        #[serde(rename = "Indianapolis")]
        Indianapolis,

        #[serde(rename = "Indiana Purdue-Fort Wayne")]
        IndianaPurdueFortWayne,

        #[serde(rename = "Indiana Purdue-Indianapolis")]
        IndianaPurdueIndianapolis,

        #[serde(rename = "Indiana State")]
        IndianaState,

        #[serde(rename = "Indiana Tech")]
        IndianaTech,

        #[serde(rename = "Indian Hills Community College")]
        IndianHillsCommunityCollege,

        #[serde(rename = "Iona")]
        Iona,

        #[serde(rename = "Iowa")]
        Iowa,

        #[serde(rename = "Iowa State")]
        IowaState,

        #[serde(rename = "Jackson State")]
        JacksonState,

        #[serde(rename = "Jacksonville")]
        Jacksonville,

        #[serde(rename = "Jacksonville State")]
        JacksonvilleState,

        #[serde(rename = "James Madison")]
        JamesMadison,

        #[serde(rename = "Johnson C. Smith")]
        JohnsonCSmith,

        #[serde(rename = "Kansas")]
        Kansas,

        #[serde(rename = "Kansas State")]
        KansasState,

        #[serde(rename = "Kennesaw State")]
        KennesawState,

        #[serde(rename = "Kent State")]
        KentState,

        #[serde(rename = "Kentucky")]
        Kentucky,

        #[serde(rename = "Kentucky State")]
        KentuckyState,

        #[serde(rename = "Kentucky Wesleyan")]
        KentuckyWesleyan,

        #[serde(rename = "Kenyon")]
        Kenyon,

        #[serde(rename = "King's (NY)")]
        KingSNy,

        #[serde(rename = "Lafayette")]
        Lafayette,

        #[serde(rename = "Lamar")]
        Lamar,

        #[serde(rename = "La Salle")]
        LaSalle,

        #[serde(rename = "Lawrence Tech")]
        LawrenceTech,

        #[serde(rename = "Lebanon Valley")]
        LebanonValley,

        #[serde(rename = "Lee")]
        Lee,

        #[serde(rename = "Lehigh")]
        Lehigh,

        #[serde(rename = "Le Moyne")]
        LeMoyne,

        #[serde(rename = "Lewis")]
        Lewis,

        #[serde(rename = "Liberty")]
        Liberty,

        #[serde(rename = "Lincoln (MO)")]
        LincolnMo,

        #[serde(rename = "Lipscomb")]
        Lipscomb,

        #[serde(rename = "Long Beach State")]
        LongBeachState,

        #[serde(rename = "Long Island-Brooklyn")]
        LongIslandBrooklyn,

        #[serde(rename = "Longwood")]
        Longwood,

        #[serde(rename = "Los Angeles Community College")]
        LosAngelesCommunityCollege,

        #[serde(rename = "Louisiana-Lafayette")]
        LouisianaLafayette,

        #[serde(rename = "Louisiana-Monroe")]
        LouisianaMonroe,

        #[serde(rename = "Louisiana State")]
        LouisianaState,

        #[serde(rename = "Louisiana Tech")]
        LouisianaTech,

        #[serde(rename = "Louisville")]
        Louisville,

        #[serde(rename = "Loyola-Chicago")]
        LoyolaChicago,

        #[serde(rename = "Loyola-Maryland")]
        LoyolaMaryland,

        #[serde(rename = "Loyola-Marymount")]
        LoyolaMarymount,

        #[serde(rename = "Maine")]
        Maine,

        #[serde(rename = "Manhattan")]
        Manhattan,

        #[serde(rename = "Marist")]
        Marist,

        #[serde(rename = "Marquette")]
        Marquette,

        #[serde(rename = "Marshall")]
        Marshall,

        #[serde(rename = "Maryland")]
        Maryland,

        #[serde(rename = "Maryland-Eastern Shore")]
        MarylandEasternShore,

        #[serde(rename = "Massachusetts")]
        Massachusetts,

        #[serde(rename = "Master's")]
        MasterS,

        #[serde(rename = "McNeese State")]
        McNeeseState,

        #[serde(rename = "Memphis")]
        Memphis,

        #[serde(rename = "Mercer")]
        Mercer,

        #[serde(rename = "Meridian Community College")]
        MeridianCommunityCollege,

        #[serde(rename = "Merrimack")]
        Merrimack,

        #[serde(rename = "Metro State")]
        MetroState,

        #[serde(rename = "Miami (FL)")]
        MiamiFl,

        #[serde(rename = "Miami (OH)")]
        MiamiOh,

        #[serde(rename = "Michigan")]
        Michigan,

        #[serde(rename = "Michigan State")]
        MichiganState,

        #[serde(rename = "Middle Tennessee State")]
        MiddleTennesseeState,

        #[serde(rename = "Midland")]
        Midland,

        #[serde(rename = "Midwestern State")]
        MidwesternState,

        #[serde(rename = "Miles")]
        Miles,

        #[serde(rename = "Millersville")]
        Millersville,

        #[serde(rename = "Minnesota")]
        Minnesota,

        #[serde(rename = "Minnesota-Duluth")]
        MinnesotaDuluth,

        #[serde(rename = "Minnesota State-Mankato")]
        MinnesotaStateMankato,

        #[serde(rename = "Mississippi")]
        Mississippi,

        #[serde(rename = "Mississippi Gulf Community College")]
        MississippiGulfCommunityCollege,

        #[serde(rename = "Mississippi State")]
        MississippiState,

        #[serde(rename = "Mississippi Valley State")]
        MississippiValleyState,

        #[serde(rename = "Missouri")]
        Missouri,

        #[serde(rename = "Missouri-Kansas City")]
        MissouriKansasCity,

        #[serde(rename = "Missouri State")]
        MissouriState,

        #[serde(rename = "Missouri Western State")]
        MissouriWesternState,

        #[serde(rename = "Monmouth")]
        Monmouth,

        #[serde(rename = "Montana")]
        Montana,

        #[serde(rename = "Montana State")]
        MontanaState,

        #[serde(rename = "Montevallo")]
        Montevallo,

        #[serde(rename = "Morehead State")]
        MoreheadState,

        #[serde(rename = "Morehouse")]
        Morehouse,

        #[serde(rename = "Morgan State")]
        MorganState,

        #[serde(rename = "Mountain State")]
        MountainState,

        #[serde(rename = "Mount St. Mary's")]
        MountStMaryS,

        #[serde(rename = "Mount Union")]
        MountUnion,

        #[serde(rename = "Mt. San Antonio")]
        MtSanAntonio,

        #[serde(rename = "Muhlenberg")]
        Muhlenberg,

        #[serde(rename = "Murray State")]
        MurrayState,

        #[serde(rename = "Navy")]
        Navy,

        #[serde(rename = "Nebraska")]
        Nebraska,

        #[serde(rename = "Nebraska-Kearney")]
        NebraskaKearney,

        #[serde(rename = "Nebraska-Omaha")]
        NebraskaOmaha,

        #[serde(rename = "Nevada")]
        Nevada,

        #[serde(rename = "Nevada-Las Vegas")]
        NevadaLasVegas,

        #[serde(rename = "Nevada-Reno")]
        NevadaReno,

        #[serde(rename = "New Jersey Institute of Technology")]
        NewJerseyInstituteOfTechnology,

        #[serde(rename = "New Mexico")]
        NewMexico,

        #[serde(rename = "New Mexico Highlands")]
        NewMexicoHighlands,

        #[serde(rename = "New Mexico State")]
        NewMexicoState,

        #[serde(rename = "New Mexico Tech")]
        NewMexicoTech,

        #[serde(rename = "New Orleans")]
        NewOrleans,

        #[serde(rename = "New York University")]
        NewYorkUniversity,

        #[serde(rename = "Niagara")]
        Niagara,

        #[serde(rename = "Nicholls State")]
        NichollsState,

        #[serde(rename = "Norfolk State")]
        NorfolkState,

        #[serde(rename = "North Carolina")]
        NorthCarolina,

        #[serde(rename = "North Carolina A&T")]
        NorthCarolinaAT,

        #[serde(rename = "North Carolina-Asheville")]
        NorthCarolinaAsheville,

        #[serde(rename = "North Carolina Central")]
        NorthCarolinaCentral,

        #[serde(rename = "North Carolina-Charlotte")]
        NorthCarolinaCharlotte,

        #[serde(rename = "North Carolina-Greensboro")]
        NorthCarolinaGreensboro,

        #[serde(rename = "North Carolina State")]
        NorthCarolinaState,

        #[serde(rename = "North Carolina-Wilmington")]
        NorthCarolinaWilmington,

        #[serde(rename = "North Dakota")]
        NorthDakota,

        #[serde(rename = "North Dakota State")]
        NorthDakotaState,

        #[serde(rename = "Northeastern")]
        Northeastern,

        #[serde(rename = "Northeastern State")]
        NortheasternState,

        #[serde(rename = "Northeast Mississippi Community College")]
        NortheastMississippiCommunityCollege,

        #[serde(rename = "Northern Arizona")]
        NorthernArizona,

        #[serde(rename = "Northern Colorado")]
        NorthernColorado,

        #[serde(rename = "Northern Illinois")]
        NorthernIllinois,

        #[serde(rename = "Northern Iowa")]
        NorthernIowa,

        #[serde(rename = "Northern Kentucky")]
        NorthernKentucky,

        #[serde(rename = "North Park")]
        NorthPark,

        #[serde(rename = "North Texas")]
        NorthTexas,

        #[serde(rename = "Northwestern")]
        Northwestern,

        #[serde(rename = "Northwestern Oklahoma")]
        NorthwesternOklahoma,

        #[serde(rename = "Northwestern Oklahoma State")]
        NorthwesternOklahomaState,

        #[serde(rename = "Northwestern State")]
        NorthwesternState,

        #[serde(rename = "Northwest Florida State")]
        NorthwestFloridaState,

        #[serde(rename = "Northwest Nazarene")]
        NorthwestNazarene,

        #[serde(rename = "Northwood")]
        Northwood,

        #[serde(rename = "Notre Dame")]
        NotreDame,

        #[serde(rename = "Oakland")]
        Oakland,

        #[serde(rename = "Ohio")]
        Ohio,

        #[serde(rename = "Ohio State")]
        OhioState,

        #[serde(rename = "Ohio Wesleyan")]
        OhioWesleyan,

        #[serde(rename = "Oklahoma")]
        Oklahoma,

        #[serde(rename = "Oklahoma Baptist")]
        OklahomaBaptist,

        #[serde(rename = "Oklahoma City")]
        OklahomaCity,

        #[serde(rename = "Oklahoma Science and Arts")]
        OklahomaScienceAndArts,

        #[serde(rename = "Oklahoma State")]
        OklahomaState,

        #[serde(rename = "Oklahoma Wesleyan")]
        OklahomaWesleyan,

        #[serde(rename = "Old Dominion")]
        OldDominion,

        #[serde(rename = "Oral Roberts")]
        OralRoberts,

        #[serde(rename = "Oregon")]
        Oregon,

        #[serde(rename = "Oregon State")]
        OregonState,

        #[serde(rename = "Ouachita Baptist")]
        OuachitaBaptist,

        #[serde(rename = "Pacific")]
        Pacific,

        #[serde(rename = "Paine")]
        Paine,

        #[serde(rename = "Penn State")]
        PennState,

        #[serde(rename = "Pennsylvania")]
        Pennsylvania,

        #[serde(rename = "Pennsylvania-Kutztown")]
        PennsylvaniaKutztown,

        #[serde(rename = "Pepperdine")]
        Pepperdine,

        #[serde(rename = "Pfeiffer")]
        Pfeiffer,

        #[serde(rename = "Phillips")]
        Phillips,

        #[serde(rename = "Pikeville")]
        Pikeville,

        #[serde(rename = "Pittsburgh")]
        Pittsburgh,

        #[serde(rename = "Portland")]
        Portland,

        #[serde(rename = "Portland State")]
        PortlandState,

        #[serde(rename = "Potsdam")]
        Potsdam,

        #[serde(rename = "Prairie View A&M")]
        PrairieViewAM,

        #[serde(rename = "Princeton")]
        Princeton,

        #[serde(rename = "Providence")]
        Providence,

        #[serde(rename = "Puget Sound")]
        PugetSound,

        #[serde(rename = "Purdue")]
        Purdue,

        #[serde(rename = "Queens (NY)")]
        QueensNy,

        #[serde(rename = "Quincy")]
        Quincy,

        #[serde(rename = "Rhode Island")]
        RhodeIsland,

        #[serde(rename = "Rice")]
        Rice,

        #[serde(rename = "Richmond")]
        Richmond,

        #[serde(rename = "Rider")]
        Rider,

        #[serde(rename = "Robert Morris (IL)")]
        RobertMorrisIl,

        #[serde(rename = "Rochester (NY)")]
        RochesterNy,

        #[serde(rename = "Rockhurst")]
        Rockhurst,

        #[serde(rename = "Rutgers")]
        Rutgers,

        #[serde(rename = "Sacramento State")]
        SacramentoState,

        #[serde(rename = "Sacred Heart")]
        SacredHeart,

        #[serde(rename = "Saginaw Valley")]
        SaginawValley,

        #[serde(rename = "Saint Augustine's")]
        SaintAugustineS,

        #[serde(rename = "Saint Francis (PA)")]
        SaintFrancisPa,

        #[serde(rename = "Saint Joseph's")]
        SaintJosephS,

        #[serde(rename = "Saint Louis")]
        SaintLouis,

        #[serde(rename = "Saint Mary's (CA)")]
        SaintMarySCa,

        #[serde(rename = "Saint Mary's (MN)")]
        SaintMarySMn,

        #[serde(rename = "Saint Peter's")]
        SaintPeterS,

        #[serde(rename = "Saint Rose")]
        SaintRose,

        #[serde(rename = "Saint Vincent")]
        SaintVincent,

        #[serde(rename = "Salem International")]
        SalemInternational,

        #[serde(rename = "Sam Houston State")]
        SamHoustonState,

        #[serde(rename = "San Diego")]
        SanDiego,

        #[serde(rename = "San Diego State")]
        SanDiegoState,

        #[serde(rename = "San Francisco")]
        SanFrancisco,

        #[serde(rename = "San Jose State")]
        SanJoseState,

        #[serde(rename = "Santa Clara")]
        SantaClara,

        #[serde(rename = "Scranton")]
        Scranton,

        #[serde(rename = "Seattle")]
        Seattle,

        #[serde(rename = "Seton Hall")]
        SetonHall,

        #[serde(rename = "Seward County Community College")]
        SewardCountyCommunityCollege,

        #[serde(rename = "Shaw")]
        Shaw,

        #[serde(rename = "Shippensburg")]
        Shippensburg,

        #[serde(rename = "Siena")]
        Siena,

        #[serde(rename = "Slippery Rock")]
        SlipperyRock,

        #[serde(rename = "South Alabama")]
        SouthAlabama,

        #[serde(rename = "South Carolina")]
        SouthCarolina,

        #[serde(rename = "South Carolina-Aiken")]
        SouthCarolinaAiken,

        #[serde(rename = "South Carolina State")]
        SouthCarolinaState,

        #[serde(rename = "South Carolina Upstate")]
        SouthCarolinaUpstate,

        #[serde(rename = "South Dakota")]
        SouthDakota,

        #[serde(rename = "South Dakota State")]
        SouthDakotaState,

        #[serde(rename = "Southeastern Illinois")]
        SoutheasternIllinois,

        #[serde(rename = "Southeastern Louisiana")]
        SoutheasternLouisiana,

        #[serde(rename = "Southeastern Oklahoma State")]
        SoutheasternOklahomaState,

        #[serde(rename = "Southeast Missouri State")]
        SoutheastMissouriState,

        #[serde(rename = "Southern")]
        Southern,

        #[serde(rename = "Southern California")]
        SouthernCalifornia,

        #[serde(rename = "Southern Illinois")]
        SouthernIllinois,

        #[serde(rename = "Southern Methodist")]
        SouthernMethodist,

        #[serde(rename = "Southern Mississippi")]
        SouthernMississippi,

        #[serde(rename = "Southern Nazarene")]
        SouthernNazarene,

        #[serde(rename = "Southern Utah")]
        SouthernUtah,

        #[serde(rename = "South Florida")]
        SouthFlorida,

        #[serde(rename = "Southwest Baptist")]
        SouthwestBaptist,

        #[serde(rename = "Springfield")]
        Springfield,

        #[serde(rename = "St. Ambrose")]
        StAmbrose,

        #[serde(rename = "St. Anselm")]
        StAnselm,

        #[serde(rename = "St. Bonaventure")]
        StBonaventure,

        #[serde(rename = "St. Cloud State")]
        StCloudState,

        #[serde(rename = "St. Francis Brooklyn")]
        StFrancisBrooklyn,

        #[serde(rename = "St. John's (NY)")]
        StJohnSNy,

        #[serde(rename = "St. Mary's (CA)")]
        StMarySCa,

        #[serde(rename = "St. Mary's (TX)")]
        StMarySTx,

        #[serde(rename = "St. Peter's")]
        StPeterS,

        #[serde(rename = "St. Thomas (FL)")]
        StThomasFl,

        #[serde(rename = "Stanford")]
        Stanford,

        #[serde(rename = "Stephen F. Austin")]
        StephenFAustin,

        #[serde(rename = "Stetson")]
        Stetson,

        #[serde(rename = "Stony Brook")]
        StonyBrook,

        #[serde(rename = "Syracuse")]
        Syracuse,

        #[serde(rename = "Tampa")]
        Tampa,

        #[serde(rename = "Tarleton State")]
        TarletonState,

        #[serde(rename = "Temple")]
        Temple,

        #[serde(rename = "Tennessee")]
        Tennessee,

        #[serde(rename = "Tennessee-Chattanooga")]
        TennesseeChattanooga,

        #[serde(rename = "Tennessee-Martin")]
        TennesseeMartin,

        #[serde(rename = "Tennessee State")]
        TennesseeState,

        #[serde(rename = "Tennessee Tech")]
        TennesseeTech,

        #[serde(rename = "Texas")]
        Texas,

        #[serde(rename = "Texas A&M")]
        TexasAM,

        #[serde(rename = "Texas A&M-Commerce")]
        TexasAMCommerce,

        #[serde(rename = "Texas A&M-Corpus Christi")]
        TexasAMCorpusChristi,

        #[serde(rename = "Texas-Arlington")]
        TexasArlington,

        #[serde(rename = "Texas Christian")]
        TexasChristian,

        #[serde(rename = "Texas-El Paso")]
        TexasElPaso,

        #[serde(rename = "Texas-Pan American")]
        TexasPanAmerican,

        #[serde(rename = "Texas-San Antonio")]
        TexasSanAntonio,

        #[serde(rename = "Texas Southern")]
        TexasSouthern,

        #[serde(rename = "Texas State")]
        TexasState,

        #[serde(rename = "Texas Tech")]
        TexasTech,

        #[serde(rename = "Texas Wesleyan")]
        TexasWesleyan,

        #[serde(rename = "Thomas More")]
        ThomasMore,

        #[serde(rename = "Toledo")]
        Toledo,

        #[serde(rename = "Towson")]
        Towson,

        #[serde(rename = "Trinity Valley Community College")]
        TrinityValleyCommunityCollege,

        #[serde(rename = "Troy State")]
        TroyState,

        #[serde(rename = "Truman State")]
        TrumanState,

        #[serde(rename = "Tulane")]
        Tulane,

        #[serde(rename = "Tulsa")]
        Tulsa,

        #[serde(rename = "Tuskegee")]
        Tuskegee,

        #[serde(rename = "UCLA")]
        Ucla,

        #[serde(rename = "Utah")]
        Utah,

        #[serde(rename = "Utah State")]
        UtahState,

        #[serde(rename = "Utah Valley")]
        UtahValley,

        #[serde(rename = "Valdosta State")]
        ValdostaState,

        #[serde(rename = "Valparaiso")]
        Valparaiso,

        #[serde(rename = "Vanderbilt")]
        Vanderbilt,

        #[serde(rename = "Venezuela")]
        Venezuela,

        #[serde(rename = "Vermont")]
        Vermont,

        #[serde(rename = "Villanova")]
        Villanova,

        #[serde(rename = "Virginia")]
        Virginia,

        #[serde(rename = "Virginia Commonwealth")]
        VirginiaCommonwealth,

        #[serde(rename = "Virginia Military Institute")]
        VirginiaMilitaryInstitute,

        #[serde(rename = "Virginia Tech")]
        VirginiaTech,

        #[serde(rename = "Virginia Union")]
        VirginiaUnion,

        #[serde(rename = "Voorhees")]
        Voorhees,

        #[serde(rename = "Vorhees")]
        Vorhees,

        #[serde(rename = "Wake Forest")]
        WakeForest,

        #[serde(rename = "Walsh")]
        Walsh,

        #[serde(rename = "Washington")]
        Washington,

        #[serde(rename = "Washington & Jefferson")]
        WashingtonJefferson,

        #[serde(rename = "Washington State")]
        WashingtonState,

        #[serde(rename = "Wayne State (MI)")]
        WayneStateMi,

        #[serde(rename = "Weber State")]
        WeberState,

        #[serde(rename = "Western Carolina")]
        WesternCarolina,

        #[serde(rename = "Western Illinois")]
        WesternIllinois,

        #[serde(rename = "Western Kentucky")]
        WesternKentucky,

        #[serde(rename = "Western Michigan")]
        WesternMichigan,

        #[serde(rename = "Western Washington")]
        WesternWashington,

        #[serde(rename = "West Florida")]
        WestFlorida,

        #[serde(rename = "West Georgia")]
        WestGeorgia,

        #[serde(rename = "Westminster (PA)")]
        WestminsterPa,

        #[serde(rename = "West Texas A&M")]
        WestTexasAM,

        #[serde(rename = "West Virginia")]
        WestVirginia,

        #[serde(rename = "West Virginia State")]
        WestVirginiaState,

        #[serde(rename = "West Virginia Tech")]
        WestVirginiaTech,

        #[serde(rename = "West Virginia Wesleyan")]
        WestVirginiaWesleyan,

        #[serde(rename = "Wheaton (IL)")]
        WheatonIl,

        #[serde(rename = "Whitworth")]
        Whitworth,

        #[serde(rename = "Wichita State")]
        WichitaState,

        #[serde(rename = "Wilberforce")]
        Wilberforce,

        #[serde(rename = "William & Mary")]
        WilliamMary,

        #[serde(rename = "William Jessup")]
        WilliamJessup,

        #[serde(rename = "William Paterson")]
        WilliamPaterson,

        #[serde(rename = "William Penn")]
        WilliamPenn,

        #[serde(rename = "Wingate")]
        Wingate,

        #[serde(rename = "Winona State")]
        WinonaState,

        #[serde(rename = "Winston-Salem State")]
        WinstonSalemState,

        #[serde(rename = "Winthrop (SC)")]
        WinthropSc,

        #[serde(rename = "Wisconsin")]
        Wisconsin,

        #[serde(rename = "Wisconsin-Eau Claire")]
        WisconsinEauClaire,

        #[serde(rename = "Wisconsin-Green Bay")]
        WisconsinGreenBay,

        #[serde(rename = "Wisconsin-Milwaukee")]
        WisconsinMilwaukee,

        #[serde(rename = "Wisconsin-Parkside")]
        WisconsinParkside,

        #[serde(rename = "Wisconsin-River Falls")]
        WisconsinRiverFalls,

        #[serde(rename = "Wisconsin-Stevens Point")]
        WisconsinStevensPoint,

        #[serde(rename = "Wisconsin-Whitewater")]
        WisconsinWhitewater,

        #[serde(rename = "Wofford")]
        Wofford,

        #[serde(rename = "Wooster")]
        Wooster,

        #[serde(rename = "Wright State")]
        WrightState,

        #[serde(rename = "Wyoming")]
        Wyoming,

        #[serde(rename = "Xavier")]
        Xavier,

        #[serde(rename = "Xavier (LA)")]
        XavierLa,

        #[serde(rename = "Yale")]
        Yale,

        #[serde(rename = "Yonsei (KOR)")]
        YonseiKor,

        #[serde(rename = "Youngstown State")]
        YoungstownState,
    }

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

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Height {
        #[serde(rename = "LT 6-0")]
        LessThan6Feet0Inches,

        #[serde(rename = "GT 6-0")]
        GreaterThan6Feet0Inches,

        #[serde(rename = "LT 6-4")]
        LessThan6Feet4Inches,

        #[serde(rename = "GT 6-4")]
        GreaterThan6Feet4Inches,

        #[serde(rename = "LT 6-7")]
        LessThan6Feet7Inches,

        #[serde(rename = "GT 6-7")]
        GreaterThan6Feet7Inches,

        #[serde(rename = "LT 6-10")]
        LessThan6Feet10Inches,

        #[serde(rename = "GT 6-10")]
        GreaterThan6Feet10Inches,

        #[serde(rename = "LT 7-0")]
        LessThan7Feet0Inches,

        #[serde(rename = "GT 7-0")]
        GreaterThan7Feet0Inches,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Weight {
        #[serde(rename = "LT 200")]
        LessThan200Pounds,

        #[serde(rename = "GT 200")]
        GreaterThan200Pounds,

        #[serde(rename = "LT 225")]
        LessThan225Pounds,

        #[serde(rename = "GT 225")]
        GreaterThan225Pounds,

        #[serde(rename = "LT 250")]
        LessThan250Pounds,

        #[serde(rename = "GT 250")]
        GreaterThan250Pounds,

        #[serde(rename = "LT 275")]
        LessThan275Pounds,

        #[serde(rename = "GT 275")]
        GreaterThan275Pounds,

        #[serde(rename = "LT 300")]
        LessThan300Pounds,

        #[serde(rename = "GT 300")]
        GreaterThan300Pounds,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
    #[repr(u32)]
    pub enum Team {
        #[default]
        AllTeams = 0,

        AtlantaHawks = 1_610_612_737,

        BostonCeltics = 1_610_612_738,

        BrooklynNets = 1_610_612_751,

        CharlotteHornets = 1_610_612_766,

        ChicagoBulls = 1_610_612_741,

        ClevelandCavaliers = 1_610_612_739,

        DallasMavericks = 1_610_612_742,

        DenverNuggets = 1_610_612_743,

        DetroitPistons = 1_610_612_765,

        GoldenStateWarriors = 1_610_612_744,

        HoustonRockets = 1_610_612_745,

        IndianaPacers = 1_610_612_754,

        LaClippers = 1_610_612_746,

        LosAngelesLakers = 1_610_612_747,

        MemphisGrizzlies = 1_610_612_763,

        MiamiHeat = 1_610_612_748,

        MilwaukeeBucks = 1_610_612_749,

        MinnesotaTimberwolves = 1_610_612_750,

        NewOrleansPelicans = 1_610_612_740,

        NewYorkKnicks = 1_610_612_752,

        OklahomaCityThunder = 1_610_612_760,

        OrlandoMagic = 1_610_612_753,

        Philadelphia76ers = 1_610_612_755,

        PhoenixSuns = 1_610_612_756,

        PortlandTrailBlazers = 1_610_612_757,

        SacramentoKings = 1_610_612_758,

        SanAntonioSpurs = 1_610_612_759,

        TorontoRaptors = 1_610_612_761,

        UtahJazz = 1_610_612_762,

        WashingtonWizards = 1_610_612_764,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Division {
        #[serde(rename = "Atlantic")]
        Atlantic,

        #[serde(rename = "Central")]
        Central,

        #[serde(rename = "Southeast")]
        Southeast,

        #[serde(rename = "Northwest")]
        Northwest,

        #[serde(rename = "Pacific")]
        Pacific,

        #[serde(rename = "Southwest")]
        Southwest,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Conference {
        #[serde(rename = "East")]
        East,

        #[serde(rename = "West")]
        West,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Outcome {
        #[serde(rename = "W")]
        Win,

        #[serde(rename = "L")]
        Loss,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Location {
        #[serde(rename = "Home")]
        Home,

        #[serde(rename = "Road")]
        Road,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ShotClockRange {
        #[serde(rename = "24-22")]
        From24To22,

        #[serde(rename = "22-18 Very Early")]
        From22To18VeryEarly,

        #[serde(rename = "18-15 Early")]
        From18To15Early,

        #[serde(rename = "15-7 Average")]
        From15To7Average,

        #[serde(rename = "7-4 Late")]
        From7To4Late,

        #[serde(rename = "4-0 Very Late")]
        From4To0VeryLate,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
    #[repr(u8)]
    pub enum Quarter {
        #[default]
        AllQuarters = 0,

        FirstQuarter = 1,

        SecondQuarter = 2,

        ThirdQuarter = 3,

        FourthQuarter = 4,

        Overtime1 = 5,

        Overtime2 = 6,

        Overtime3 = 7,

        Overtime4 = 8,

        Overtime5 = 9,

        Overtime6 = 10,

        Overtime7 = 11,

        Overtime8 = 12,

        Overtime9 = 13,

        Overtime10 = 14,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Half {
        #[serde(rename = "First Half")]
        FirstHalf,

        #[serde(rename = "Second Half")]
        SecondHalf,

        #[serde(rename = "Overtime")]
        Overtime,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
    #[repr(u8)]
    pub enum PlayoffRound {
        #[default]
        AllPlayoffRounds = 0,

        ConferenceQuarterFinals = 1,

        ConferenceSemiFinals = 2,

        ConferenceFinals = 3,

        Finals = 4,
    }

    pub mod serde_optional_date {
        use chrono::NaiveDateTime;
        use serde::{self, de, Deserialize, Deserializer, Serializer};

        const FORMAT: &str = "%m/%d/%Y";

        /// # Errors
        ///
        /// Returns an error if serialization fails.
        pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            crate::serde::serde_none_as_empty_string(
                &date.map(|d| d.format(FORMAT).to_string()),
                serializer,
            )
        }

        /// # Errors
        ///
        /// Returns an error if deserialization fails.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            Option::<String>::deserialize(deserializer)?.map_or_else(
                || Ok(None),
                |date| {
                    NaiveDateTime::parse_from_str(&date, FORMAT).map_or_else(
                        |e| Err(de::Error::custom(format!("invalid date: {e}"))),
                        |date| Ok(Some(date)),
                    )
                },
            )
        }
    }
}

use std::convert::Infallible;

use chrono::NaiveDateTime;
use fields::{
    serde_none_as_empty_string, serde_optional_date, serde_optional_infallible, College,
    Conference, Country, Division, DraftPick, DraftYear, Experience, Height, LastNGames, LeagueId,
    Location, MeasureType, Month, Outcome, PerMode, PlayoffRound, Position, Quarter, Season,
    SeasonSegment, SeasonType, ShotClockRange, StarterOrBench, Team, Weight, YesOrNo,
};

crate::endpoint! {
    PlayersTraditional("leaguedashplayerstats") {
        measure_type: MeasureType,

        per_mode: PerMode,

        plus_minus: YesOrNo,

        pace_adjust: YesOrNo,

        rank: YesOrNo,

        #[serde(rename = "LeagueID")]
        league_id: LeagueId,

        season: Season,

        season_type: SeasonType,

        #[serde(rename = "PORound")]
        po_round: PlayoffRound,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        outcome: Option<Outcome>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        location: Option<Location>,

        month: Month,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        season_segment: Option<SeasonSegment>,

        #[serde(with = "serde_optional_date")]
        date_from: Option<NaiveDateTime>,

        #[serde(with = "serde_optional_date")]
        date_to: Option<NaiveDateTime>,

        #[serde(rename = "OpponentTeamID")]
        opponent_team_id: Team,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        vs_conference: Option<Conference>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        vs_division: Option<Division>,

        #[serde(rename = "TeamID")]
        team_id: Team,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        conference: Option<Conference>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        division: Option<Division>,

        #[serde(with = "serde_optional_infallible")]
        game_segment: Option<Infallible>,

        period: Quarter,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        shot_clock_range: Option<ShotClockRange>,

        last_n_games: LastNGames,

        #[serde(with = "serde_optional_infallible")]
        game_scope: Option<Infallible>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        player_experience: Option<Experience>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        player_position: Option<Position>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        starter_bench: Option<StarterOrBench>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        draft_year: Option<DraftYear>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        draft_pick: Option<DraftPick>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        college: Option<College>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        country: Option<Country>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        height: Option<Height>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        weight: Option<Weight>,

        #[serde(serialize_with = "serde_none_as_empty_string")]
        two_way: Option<YesOrNo>,
    } => {
        league_dash_player_stats: LeagueDashPlayerStatsRow("LeagueDashPlayerStats") {
            player_id: u32,
            player_name: String,
            nickname: String,
            team_id: u32,
            team_abbreviation: String,
            age: f64,
            gp: u32,
            w: u32,
            l: u32,
            w_pct: f64,
            min: f64,
            fgm: f64,
            fga: f64,
            fg_pct: f64,
            fg3m: f64,
            fg3a: f64,
            fg3_pct: f64,
            ftm: f64,
            fta: f64,
            ft_pct: f64,
            oreb: f64,
            dreb: f64,
            reb: f64,
            ast: f64,
            tov: f64,
            stl: f64,
            blk: f64,
            blka: f64,
            pf: f64,
            pfd: f64,
            pts: f64,
            plus_minus: f64,
            nba_fantasy_pts: f64,
            dd2: u32,
            td3: u32,
            wnba_fantasy_pts: f64,
            gp_rank: u32,
            w_rank: u32,
            l_rank: u32,
            w_pct_rank: u32,
            min_rank: u32,
            fgm_rank: u32,
            fga_rank: u32,
            fg_pct_rank: u32,
            fg3m_rank: u32,
            fg3a_rank: u32,
            fg3_pct_rank: u32,
            ftm_rank: u32,
            fta_rank: u32,
            ft_pct_rank: u32,
            oreb_rank: u32,
            dreb_rank: u32,
            reb_rank: u32,
            ast_rank: u32,
            tov_rank: u32,
            stl_rank: u32,
            blk_rank: u32,
            blka_rank: u32,
            pf_rank: u32,
            pfd_rank: u32,
            pts_rank: u32,
            plus_minus_rank: u32,
            nba_fantasy_pts_rank: u32,
            dd2_rank: u32,
            td3_rank: u32,
            wnba_fantasy_pts_rank: u32,
            cfid: u32,
            cfparams: String,
        },
    }
}
