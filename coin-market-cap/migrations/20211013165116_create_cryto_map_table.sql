-- Create map of resources to CoinMarketCap IDs
CREATE TABLE crypto_map (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    -- Not necessarily unique!
    symbol VARCHAR (3),
    slug TEXT,
    rank INTEGER,
    is_active BOOLEAN,
    first_historical_data timestamptz NOT NULL,
    last_historical_data timestamptz NOT NULL,
    -- Cryto's id or null otherwise.
    platform INTEGER
);