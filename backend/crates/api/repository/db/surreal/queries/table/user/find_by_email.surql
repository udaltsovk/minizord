SELECT * FROM type::table($table)
    WHERE email = type::string($email)
    LIMIT 1
