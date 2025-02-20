CREATE OR REPLACE FUNCTION generate_random_id(length integer)
RETURNS INTEGER AS $$
DECLARE
  id INTEGER := 0;
  i INTEGER;
BEGIN
  FOR i IN 1..length LOOP
    id := id * 10 + floor(random() * 10)::INTEGER;
  END LOOP;
  RETURN id;
END;
$$ LANGUAGE plpgsql;
