DEFINE TABLE OVERWRITE reviewed
    SCHEMAFULL
    TYPE RELATION FROM user TO user ENFORCED;

DEFINE FIELD OVERWRITE review ON reviewed
    TYPE string;
DEFINE FIELD OVERWRITE score ON reviewed
    TYPE int;
