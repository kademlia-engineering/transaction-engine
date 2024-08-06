# Transaction Payment Processing Example

The data we work with in this scenario comes from bitcoind’s rpc call `listsinceblock`. A frequently used approach to detect incoming deposits is to periodically call `listsinceblock` and process the returned data. This test contains 2 json files that represent the data from 2 separate calls to this endpoint. Your task is to write code that processes those files and detects all valid incoming deposits.

**Goal**: Process transactions and filter them for valid deposits.

**Note**: A deposit is considered valid when it has at least 6 confirmations.

Known customer addresses are:
* Wesley Crusher: mvd6qFeVkqH6MNAS2Y2cLifbdaX5XUkbZJ
* Leonard McCoy: mmFFG4jqAtw9MoCC88hw5FNfreQWuEHADp
* Jonathan Archer: mzzg8fvHXydKs8j9D2a8t7KpSXpGgAnk4n
* Jadzia Dax: 2N1SP7r92ZZJvYKG2oNtzPwYnzw62up7mTo
* Montgomery Scott: mutrAf4usv3HKNdpLwVD4ow2oLArL6Rez8
* James T. Kirk: miTHhiX3iFhVnAEecLjybxvV5g8mKYTtnM
* Spock: mvcyJMiAcSXKAEsQxbW9TYZ369rsMG6rVV

## How to run

`docker-compose up`

1. Read all transactions from `transactions-1.json` and `transactions-2.json` and store all deposits in a database of your choice.
2. Read deposits from the database that are good to credit to users and print the following 10 lines on stdout:

    ```
    Deposited for Wesley Crusher: count=n sum=x.xxxxxxxx
    Deposited for Leonard McCoy: count=n sum=x.xxxxxxxx
    Deposited for Jonathan Archer: count=n sum=x.xxxxxxxx
    Deposited for Jadzia Dax: count=n sum=x.xxxxxxxx
    Deposited for Montgomery Scott: count=n sum=x.xxxxxxxx
    Deposited for James T. Kirk: count=n sum=x.xxxxxxxx
    Deposited for Spock: count=n sum=x.xxxxxxxx
    Deposited without reference: count=n sum=x.xxxxxxxx
    Smallest valid deposit: x.xxxxxxxx
    Largest valid deposit: x.xxxxxxxx
    ```
