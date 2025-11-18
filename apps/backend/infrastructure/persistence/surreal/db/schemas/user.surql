DEFINE TABLE OVERWRITE user
    SCHEMAFULL;

DEFINE FIELD OVERWRITE email ON user
    TYPE string;
DEFINE FIELD OVERWRITE username ON user
    TYPE string;
DEFINE FIELD OVERWRITE password_hash ON user
    TYPE string;
DEFINE FIELD OVERWRITE role ON user
    TYPE string;
DEFINE FIELD OVERWRITE profile ON user
    TYPE option<record<profile>>;

DEFINE INDEX OVERWRITE unique_email ON user
    FIELDS email
    UNIQUE;
DEFINE INDEX OVERWRITE unique_username ON user
    FIELDS username
    UNIQUE;
DEFINE INDEX OVERWRITE unique_profile ON user
    FIELDS profile
    UNIQUE;
