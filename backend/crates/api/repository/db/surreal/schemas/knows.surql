DEFINE TABLE OVERWRITE knows 
    SCHEMAFULL
    TYPE RELATION FROM participant | mentor TO technology ENFORCED;

DEFINE FIELD OVERWRITE level ON knows
    TYPE int;