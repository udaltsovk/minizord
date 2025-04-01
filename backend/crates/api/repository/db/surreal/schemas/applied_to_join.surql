DEFINE TABLE OVERWRITE applied_to_join
    SCHEMAFULL
    TYPE RELATION FROM user TO team ENFORCED;

DEFINE FIELD OVERWRITE application ON applied_to_join
    TYPE string;
