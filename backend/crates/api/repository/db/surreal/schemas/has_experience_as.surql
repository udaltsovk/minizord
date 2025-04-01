DEFINE TABLE OVERWRITE has_experience_as
    SCHEMAFULL
    TYPE RELATION FROM user TO specialization ENFORCED;

DEFINE FIELD OVERWRITE level ON has_experience_as
    TYPE int;
