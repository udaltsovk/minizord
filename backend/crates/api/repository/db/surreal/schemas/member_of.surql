DEFINE TABLE OVERWRITE member_of
    SCHEMAFULL
    TYPE RELATION FROM user TO team ENFORCED;

DEFINE FIELD OVERWRITE accepted ON member_of
    TYPE bool;
DEFINE FIELD OVERWRITE specialization ON member_of
    TYPE record<specialization>;
