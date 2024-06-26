## Currc, a currency conversion tool

This is a currency conversion tool that sources currency exchange rates from https://www.exchangerate-api.com/.

## Setup instructions

1. Obtain an api key from https://www.exchangerate-api.com/ by creating a free account and paste you api key in line 6 in ./src/api_access.rs.

## Usage:

Usage: `currc [OPTIONS] <COMMAND>`

Commands:
 - `i`     Enter interactive mode
 - `convert`  Get exchange rate from base to target currency, if an amount is provided convert the amount from base to target currency
 - `allrates`   Get all exchange rates for a currency
 - `help`  Print this message or the help of the given subcommand(s)

Options:
 - `-h, --help`  Print help

#### Enter interactive mode

Usage: `currc i`

Options:
 - `-h, --help`  Print help

#### Get exchange rate from base to target currency, if an amount is provided convert the amount from base to target currency

Usage: `currc convert <BASE> <TARGET> [AMOUNT]`

Arguments:
 - `<BASE>`    Base currency
 - `<TARGET>`  Target currency
 - `[AMOUNT]`  Amount to convert

Options:
  -h, --help  Print help

#### Get all exchange rates for a currency

Usage: `currc allrates <CURR>`

Arguments:
 - `<CURR>`  Currency code

Options:
 - `-h, --help`  Print help

## Additional features

Currc caches API call results in a text file. If cached data for a currency is expired, it is redownloaded from the API and the cache is updated.
