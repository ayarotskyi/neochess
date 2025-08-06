CREATE TABLE position (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fen VARCHAR UNIQUE NOT NULL
);