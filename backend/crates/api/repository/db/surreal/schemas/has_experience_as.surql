DEFINE TABLE OVERWRITE has_experience_as
    SCHEMAFULL
    TYPE RELATION FROM participant | mentor TO specialization ENFORCED;

DEFINE FIELD OVERWRITE level ON has_experience_as
    TYPE int;