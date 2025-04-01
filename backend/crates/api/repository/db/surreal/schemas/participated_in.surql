DEFINE TABLE OVERWRITE participated_in
    SCHEMAFULL
    TYPE RELATION FROM user TO tour ENFORCED;

DEFINE FIELD OVERWRITE score ON participated_in
    TYPE float;
DEFINE FIELD OVERWRITE specialization ON participated_in
    TYPE record<specialization>;
DEFINE FIELD OVERWRITE technologies ON participated_in
    TYPE set<record<technology>>;
