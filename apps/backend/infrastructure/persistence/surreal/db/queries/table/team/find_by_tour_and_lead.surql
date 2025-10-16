SELECT * FROM type::table($table)
    WHERE 
        tour = type::record($tour_id)
        lead = type::record($lead_id)
    LIMIT 1
