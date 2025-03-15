DEFINE TABLE OVERWRITE organizator
    SCHEMAFULL;

DEFINE FIELD OVERWRITE password_hash ON organizator
    TYPE string;
DEFINE FIELD OVERWRITE username ON organizator
    TYPE string;

DEFINE INDEX OVERWRITE organizator_username ON organizator
    FIELDS username
    UNIQUE;
