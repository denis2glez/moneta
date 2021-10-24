-- Create map of resources to CoinMarketCap IDs

CREATE TABLE crypto_platform (
    -- crypto_map's derived blockchain id 
    id INTEGER PRIMARY KEY,
    -- crypto_map's base blockchain id (not necessarily unique)
    platform INTEGER,
    token_address TEXT
);

CREATE TABLE crypto_map (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    -- Not necessarily unique!
    symbol TEXT,
    slug TEXT,
    rank INTEGER,
    is_active BOOLEAN,
    first_historical_data timestamptz NOT NULL,
    last_historical_data timestamptz NOT NULL,
    platform INTEGER REFERENCES crypto_platform (id)
);