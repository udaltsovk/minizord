SELECT * FROM type::table($table)
    WHERE 
        tour = type::record($tour_id)
    LIMIT $limit
    START AT $offset
