DEFINE TABLE OVERWRITE participant
    SCHEMAFULL;

DEFINE FIELD OVERWRITE bio ON participant
    TYPE string;
DEFINE FIELD OVERWRITE email ON participant
    TYPE string;
DEFINE FIELD OVERWRITE name ON participant
    TYPE string;
DEFINE FIELD OVERWRITE password_hash ON participant
    TYPE string;
DEFINE FIELD OVERWRITE portfolio_urls ON participant
    TYPE set<string>;
DEFINE FIELD OVERWRITE surname ON participant
    TYPE string;