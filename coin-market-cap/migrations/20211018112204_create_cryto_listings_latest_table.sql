-- Create a table with the latest listings of cryptocurrencies

CREATE TABLE crypto_listings_latest (
    id INTEGER REFERENCES crypto_map,
    num_market_pairs INTEGER,
    tags TEXT[],
    max_supply NUMERIC,
    circulating_supply NUMERIC,
    total_supply NUMERIC,
    platform INTEGER REFERENCES platforms (id),
    cmc_rank INTEGER,

    quote TEXT,    
    -- Currently, the changes are represented in-place.
    price NUMERIC,
    volume_24h NUMERIC,
    volume_change_24h NUMERIC,
    percent_change_1h NUMERIC,
    percent_change_24h NUMERIC,
    percent_change_7d NUMERIC,
    percent_change_30d NUMERIC,
    percent_change_60d NUMERIC,
    percent_change_90d NUMERIC,
    market_cap NUMERIC,
    market_cap_dominance NUMERIC,
    fully_diluted_market_cap NUMERIC,
    last_updated timestamptz NOT NULL,

    PRIMARY KEY (id, last_updated)
);