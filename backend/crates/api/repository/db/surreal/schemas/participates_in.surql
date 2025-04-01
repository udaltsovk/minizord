DEFINE TABLE OVERWRITE participates_in
    SCHEMAFULL
    TYPE RELATION FROM user TO tour ENFORCED;

DEFINE FIELD OVERWRITE result ON participates_in
    TYPE option<record<participated_in>>;
