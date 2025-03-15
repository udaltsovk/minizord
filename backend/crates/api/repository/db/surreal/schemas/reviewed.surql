DEFINE TABLE OVERWRITE reviewed
    SCHEMAFULL
    TYPE RELATION FROM participant | organizator | mentor TO participant | mentor ENFORCED;

DEFINE FIELD OVERWRITE review ON reviewed
    TYPE string;
DEFINE FIELD OVERWRITE score ON reviewed
    TYPE int;