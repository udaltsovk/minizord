SELECT * FROM type::table($table)
    WHERE type::field($field) = type::string($value)
    LIMIT 1
