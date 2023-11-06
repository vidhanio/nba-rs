use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
