DEFINE TABLE OVERWRITE mentor
    SCHEMAFULL;

DEFINE FIELD OVERWRITE username ON mentor 
    TYPE string;
DEFINE FIELD OVERWRITE password_hash ON mentor 
    TYPE string;
DEFINE FIELD OVERWRITE name ON mentor 
    TYPE string;
DEFINE FIELD OVERWRITE surname ON mentor 
    TYPE string;
DEFINE FIELD OVERWRITE bio ON mentor 
    TYPE string;