DEFINE TABLE OVERWRITE profile
    SCHEMAFULL;

DEFINE FIELD OVERWRITE bio ON profile
    TYPE string;
DEFINE FIELD OVERWRITE name ON profile
    TYPE string;
DEFINE FIELD OVERWRITE surname ON profile
    TYPE string;
DEFINE FIELD OVERWRITE city ON profile
    TYPE string;
DEFINE FIELD OVERWRITE portfolio_urls ON profile
    TYPE set<string>;
