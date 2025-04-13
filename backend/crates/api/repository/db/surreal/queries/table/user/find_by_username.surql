SELECT * FROM type::table($table)
    WHERE username = type::string($username)
    LIMIT 1
