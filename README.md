# holofuel service crates

## holofuel-cli

A cli that runs on hpos to make holofuel zome-calls

```
hf 0.1.0

USAGE:
    hf <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    a       Gets the list of your actionable transactions
    b       Gets your balance, fees, promised and available Fuel
    c       Gets the list of your completed transactions
    help    Prints this message or the help of the given subcommand(s)
    p       Gets the list of your pending transactions

```

## holofuel-init

This crate initializes the holochain init function and also set a profile name for the holofuel account

### Expected Enviroment var

```
FEE_COLLECTOR_PUBKEY=<public key of the fee collector for the holofuel app>
EXPECT_PUBKEY=<test is this key is used on the server>
```

## hpos_hc_connect

This crate provides the core functionality to spin up a holochain environment with a `core_happ` (hha and hf) bundle as a host would from the hpos and provide simplified signed zome calls. This is primarly inteneded as a testing utility. 

### Expected Enviroment vars

```
HOLOCHAIN_DEFAULT_PASSWORD=<string password to use when spinning up holochain>
CORE_HAPP_FILE=<string uri to address of hpos `core_happ` yaml file >
LAIR_CONNECTION_URL=<string uri to lcoation of lair keystore> *OPTIONAL*
HOLOCHAIN_WORKING_DIR=<string uri to location of holochain director> *OPTIONAL*

```
