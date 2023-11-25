CREATE OR REPLACE FUNCTION update_last_modified_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.last_modified = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
