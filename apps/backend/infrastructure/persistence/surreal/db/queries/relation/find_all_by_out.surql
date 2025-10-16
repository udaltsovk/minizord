SELECT * FROM type::table($table)
    WHERE out = type::record($out)
    LIMIT type::number($limit)
    START AT type::number($offset)
