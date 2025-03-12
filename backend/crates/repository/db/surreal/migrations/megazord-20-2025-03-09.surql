-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: has_experience_as
-- ------------------------------

DEFINE TABLE has_experience_as TYPE RELATION IN participant | mentor OUT specialization SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON has_experience_as TYPE record<participant | mentor> PERMISSIONS FULL;
DEFINE FIELD level ON has_experience_as TYPE int PERMISSIONS FULL;
DEFINE FIELD out ON has_experience_as TYPE record<specialization> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: knows
-- ------------------------------

DEFINE TABLE knows TYPE RELATION IN participant | mentor OUT technology SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON knows TYPE record<participant | mentor> PERMISSIONS FULL;
DEFINE FIELD level ON knows TYPE int PERMISSIONS FULL;
DEFINE FIELD out ON knows TYPE record<technology> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: member_of
-- ------------------------------

DEFINE TABLE member_of TYPE RELATION IN participant OUT team SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON member_of TYPE record<participant> PERMISSIONS FULL;
DEFINE FIELD out ON member_of TYPE record<team> PERMISSIONS FULL;
DEFINE FIELD specialization ON member_of TYPE record<specialization> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: mentor
-- ------------------------------

DEFINE TABLE mentor TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD bio ON mentor TYPE string PERMISSIONS FULL;
DEFINE FIELD name ON mentor TYPE string PERMISSIONS FULL;
DEFINE FIELD password_hash ON mentor TYPE string PERMISSIONS FULL;
DEFINE FIELD surname ON mentor TYPE string PERMISSIONS FULL;
DEFINE FIELD username ON mentor TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE: mentors
-- ------------------------------

DEFINE TABLE mentors TYPE RELATION IN mentor OUT team SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON mentors TYPE record<mentor> PERMISSIONS FULL;
DEFINE FIELD out ON mentors TYPE record<team> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: organizator
-- ------------------------------

DEFINE TABLE organizator TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD password_hash ON organizator TYPE string PERMISSIONS FULL;
DEFINE FIELD username ON organizator TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE: participant
-- ------------------------------

DEFINE TABLE participant TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD bio ON participant TYPE string PERMISSIONS FULL;
DEFINE FIELD email ON participant TYPE string PERMISSIONS FULL;
DEFINE FIELD name ON participant TYPE string PERMISSIONS FULL;
DEFINE FIELD password_hash ON participant TYPE string PERMISSIONS FULL;
DEFINE FIELD portfolio_urls ON participant TYPE set<string> PERMISSIONS FULL;
DEFINE FIELD portfolio_urls[*] ON participant TYPE string PERMISSIONS FULL;
DEFINE FIELD surname ON participant TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE: participated_in
-- ------------------------------

DEFINE TABLE participated_in TYPE RELATION IN participant | mentor OUT tour SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON participated_in TYPE record<participant | mentor> PERMISSIONS FULL;
DEFINE FIELD out ON participated_in TYPE record<tour> PERMISSIONS FULL;
DEFINE FIELD score ON participated_in TYPE float PERMISSIONS FULL;
DEFINE FIELD specialization ON participated_in TYPE record<specialization> PERMISSIONS FULL;
DEFINE FIELD technologies ON participated_in TYPE set<record<technology>> PERMISSIONS FULL;
DEFINE FIELD technologies[*] ON participated_in TYPE record<technology> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: reviewed
-- ------------------------------

DEFINE TABLE reviewed TYPE RELATION IN participant | organizator | mentor OUT participant | mentor SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON reviewed TYPE record<participant | organizator | mentor> PERMISSIONS FULL;
DEFINE FIELD out ON reviewed TYPE record<participant | mentor> PERMISSIONS FULL;
DEFINE FIELD review ON reviewed TYPE string PERMISSIONS FULL;
DEFINE FIELD score ON reviewed TYPE int PERMISSIONS FULL;



-- ------------------------------
-- TABLE: specialization
-- ------------------------------

DEFINE TABLE specialization TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;




-- ------------------------------
-- TABLE: team
-- ------------------------------

DEFINE TABLE team TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD lead ON team TYPE record<participant> PERMISSIONS FULL;
DEFINE FIELD name ON team TYPE string PERMISSIONS FULL;
DEFINE FIELD tour ON team TYPE record<tour> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: technology
-- ------------------------------

DEFINE TABLE technology TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;




-- ------------------------------
-- TABLE: tour
-- ------------------------------

DEFINE TABLE tour TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD ends_at ON tour TYPE datetime PERMISSIONS FULL;
DEFINE FIELD name ON tour TYPE string PERMISSIONS FULL;
DEFINE FIELD starts_at ON tour TYPE datetime PERMISSIONS FULL;



-- ------------------------------
-- TABLE: uses
-- ------------------------------

DEFINE TABLE uses TYPE RELATION IN team OUT technology SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON uses TYPE record<team> PERMISSIONS FULL;
DEFINE FIELD out ON uses TYPE record<technology> PERMISSIONS FULL;



