DEFINE TABLE OVERWRITE member_of
    SCHEMAFULL
    TYPE RELATION FROM participant TO team ENFORCED;

DEFINE FIELD OVERWRITE specialization ON member_of
    TYPE record<specialization>;