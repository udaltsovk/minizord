object::from_entries((
    SELECT type::field($field) as __field, count(type::field($field)) 
        FROM type::table($table) 
        GROUP BY __field
).map(|$stat| $stat
    .entries()
    .map(|$entry| $entry.reverse()[0])
))
