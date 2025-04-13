SELECT * FROM type::table($table)
    WHERE name = type::string($name)
    LIMIT 1
