SELECT * FROM type::table($team_table)
    WHERE 
        tour = type::record($tour_id)
        name = type::string($name)
    LIMIT 1
