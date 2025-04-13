SELECT * FROM type::table($table)
    WHERE in = type::record($in)
    LIMIT type::number($limit)
    START AT type::number($offset)
