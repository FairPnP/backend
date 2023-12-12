CREATE TABLE user_ratings (
  id SERIAL PRIMARY KEY,
  user_id UUID NOT NULL,
  rated_by_user_id UUID NOT NULL,
  star_rating integer NOT NULL,
  last_modified timestamp NOT NULL DEFAULT (now()),
  created_at timestamp NOT NULL DEFAULT (now())
);

CREATE INDEX idx_user_ratings_on_user_id ON user_ratings(user_id);
CREATE INDEX idx_user_ratings_on_rated_by_user_id ON user_ratings(rated_by_user_id);

CREATE TRIGGER update_user_rating_modtime
BEFORE UPDATE ON user_ratings
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
