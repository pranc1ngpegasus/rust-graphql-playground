SELECT
  id
  , created_at
FROM organizations
WHERE
  id = ANY($1)
;
