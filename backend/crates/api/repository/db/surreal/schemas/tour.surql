DEFINE TABLE OVERWRITE tour
    SCHEMAFULL;

DEFINE FIELD OVERWRITE name ON tour
    TYPE string;
DEFINE FIELD OVERWRITE starts_at ON tour
    TYPE datetime;
DEFINE FIELD OVERWRITE ends_at ON tour
    TYPE datetime;
DEFINE FIELD OVERWRITE max_members ON tour
    TYPE int;
DEFINE FIELD OVERWRITE required_specializations ON tour
    TYPE set<record<specialization>>;