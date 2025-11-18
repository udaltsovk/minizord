DEFINE TABLE OVERWRITE team
    SCHEMAFULL;

DEFINE FIELD OVERWRITE lead ON team
    TYPE record<user>;
DEFINE FIELD OVERWRITE name ON team
    TYPE string;
DEFINE FIELD OVERWRITE tour ON team
    TYPE record<tour>;
