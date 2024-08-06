-- Known Clients table
DROP TABLE known_clients;
CREATE TABLE known_clients (
    name VARCHAR(64) NOT NULL UNIQUE,
    address VARCHAR(255) NOT NULL UNIQUE
);

-- Transactions table
DROP TABLE transactions;
CREATE TABLE transactions (
    involves_watchonly BOOLEAN NOT NULL,
    account VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    category VARCHAR(255) NOT NULL,
    amount NUMERIC(18, 8) NOT NULL,
    label VARCHAR(255),
    confirmations INTEGER NOT NULL,
    blockhash VARCHAR(64) NOT NULL,
    blockindex INTEGER NOT NULL,
    blocktime BIGINT NOT NULL,
    txid VARCHAR(64) NOT NULL UNIQUE,
    vout INTEGER NOT NULL,
    walletconflicts TEXT[],
    time BIGINT NOT NULL,
    timereceived BIGINT NOT NULL,
    bip125_replaceable VARCHAR(255) NOT NULL
);

-- SELECT get_total_confirmed_amount('your_wallet_address');
-- Given a wallet address, return the deposit sum for all transactions
-- that have at least 6 confirmations
CREATE OR REPLACE FUNCTION get_total_confirmed_amount(wallet_address VARCHAR(255))
RETURNS NUMERIC(18, 8)
LANGUAGE plpgsql
AS $$
DECLARE
    total_amount NUMERIC(18, 8) := 0;
BEGIN
    SELECT SUM(amount)
    INTO total_amount
    FROM transactions
    WHERE address = wallet_address AND confirmations >= 6;

    RETURN total_amount;
END;
$$;

-- SELECT get_confirmed_transaction_count('your_wallet_address');
-- Given a wallet address, return the transaction count for all transactions
-- that have at least 6 confirmations
CREATE OR REPLACE FUNCTION get_confirmed_transaction_count(wallet_address VARCHAR(255))
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    transaction_count INTEGER := 0;
BEGIN
    SELECT COUNT(*)
    INTO transaction_count
    FROM transactions
    WHERE address = wallet_address AND confirmations >= 6;

    RETURN transaction_count;
END;
$$;

-- SELECT get_total_confirmed_amount_excluding_known_clients();
-- Return the deposit sum for all transactions
-- that have at least 6 confirmations and are not from known clients
CREATE OR REPLACE FUNCTION get_total_confirmed_amount_excluding_known_clients()
RETURNS NUMERIC(18, 8)
LANGUAGE plpgsql
AS $$
DECLARE
    total_amount NUMERIC(18, 8) := 0;
BEGIN
    SELECT SUM(amount)
    INTO total_amount
    FROM transactions
    WHERE confirmations >= 6
    AND address NOT IN (SELECT address FROM known_clients);

    RETURN total_amount;
END;
$$;

-- SELECT get_confirmed_transaction_count_excluding_known_clients();
-- Given a wallet address, return the transaction count for all transactions
-- that have at least 6 confirmations
CREATE OR REPLACE FUNCTION get_confirmed_transaction_count_excluding_known_clients()
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    transaction_count INTEGER := 0;
BEGIN
    SELECT COUNT(*)
    INTO transaction_count
    FROM transactions
    WHERE confirmations >= 6
    AND address NOT IN (SELECT address FROM known_clients);

    RETURN transaction_count;
END;
$$;

-- SELECT get_smallest_confirmed_amount();
-- Returns the smallest deposit with at least 6 confirmations
CREATE OR REPLACE FUNCTION get_smallest_confirmed_amount()
RETURNS NUMERIC(18, 8)
LANGUAGE plpgsql
AS $$
DECLARE
    smallest_amount NUMERIC(18, 8);
BEGIN
    SELECT MIN(amount)
    INTO smallest_amount
    FROM transactions
    WHERE confirmations >= 6;

    RETURN smallest_amount;
END;
$$;

-- SELECT get_max_confirmed_amount();
-- Returns the largest deposit with at least 6 confirmations
CREATE OR REPLACE FUNCTION get_max_confirmed_amount()
RETURNS NUMERIC(18, 8)
LANGUAGE plpgsql
AS $$
DECLARE
    max_amount NUMERIC(18, 8);
BEGIN
    SELECT MAX(amount)
    INTO max_amount
    FROM transactions
    WHERE confirmations >= 6;

    RETURN max_amount;
END;
$$;

-- Procedure for creating a known client entry
CREATE OR REPLACE PROCEDURE insert_known_client(
    p_name VARCHAR(64),
    p_address VARCHAR(255)
)
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO known_clients (name, address) VALUES (p_name, p_address);
END;
$$;

-- Procedure for creating a transaction entry
CREATE OR REPLACE PROCEDURE insert_transaction(
    p_involves_watchonly BOOLEAN,
    p_account VARCHAR(255),
    p_address VARCHAR(255),
    p_category VARCHAR(255),
    p_amount NUMERIC(20, 10),
    p_label VARCHAR(255),
    p_confirmations INTEGER,
    p_blockhash VARCHAR(64),
    p_blockindex INTEGER,
    p_blocktime BIGINT,
    p_txid VARCHAR(64),
    p_vout INTEGER,
    p_walletconflicts TEXT[],
    p_time BIGINT,
    p_timereceived BIGINT,
    p_bip125_replaceable VARCHAR(255)
)
LANGUAGE plpgsql
AS $$
BEGIN
    BEGIN
        INSERT INTO transactions (
            involves_watchonly, 
            account, 
            address, 
            category, 
            amount, 
            label, 
            confirmations, 
            blockhash, 
            blockindex, 
            blocktime, 
            txid, 
            vout, 
            walletconflicts, 
            time, 
            timereceived, 
            bip125_replaceable
        ) VALUES (
            p_involves_watchonly, 
            p_account, 
            p_address, 
            p_category, 
            p_amount, 
            p_label, 
            p_confirmations, 
            p_blockhash, 
            p_blockindex, 
            p_blocktime, 
            p_txid, 
            p_vout, 
            p_walletconflicts, 
            p_time, 
            p_timereceived, 
            p_bip125_replaceable
        );
    EXCEPTION WHEN unique_violation THEN
        -- Ignore duplicate key violation and do nothing
        RAISE NOTICE 'Duplicate entry detected for txid: %', p_txid;
    END;
END;
$$;
