CREATE TABLE reservation_chat_messages (
    id SERIAL PRIMARY KEY,
    reservation_id integer NOT NULL,
    sender_id UUID NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_modified TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reservation_chat_messages_reservation_id ON reservation_chat_messages(reservation_id);
CREATE INDEX idx_reservation_chat_messages_created_at ON reservation_chat_messages(created_at);

CREATE TRIGGER update_reservation_chat_messages_modtime
BEFORE UPDATE ON reservation_chat_messages
FOR EACH ROW
EXECUTE FUNCTION update_last_modified_column();
