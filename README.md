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

## core_app_cli
```
core_app_cli 0.1.0

USAGE:
    core_app_cli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    b       Gets your balance, fees, promised and available Fuel
    help    Prints this message or the help of the given subcommand(s)
    pay     Pay your first pending invoice
    pr      Gets profile details
    tx      Gets the list of all your transactions
```

## holofuel-init

This crate initializes the holochain init function and also set a profile name for the holofuel account

### Expected Environment variables

```
FEE_COLLECTOR_PUBKEY=<public key of the fee collector for the holofuel app>
EXPECT_PUBKEY=<test is this key is used on the server>
```

## hpos_hc_connect

This crate can be used to connect to holofuel running on a hpos profile that is installed by configure-holochain

### Expected environment variables

````
HOLOCHAIN_DEFAULT_PASSWORD=<password to unlock holochain conductor>
CORE_HAPP_FILE=<path to a config.json file used for the configure-holochain service>
DEV_UID_OVERRIDE=<network-seed that is used to create new hash spaces with different holo-nixpkgs builds>
LAIR_CONNECTION_URL=<string uri to lcoation of lair keystore> *OPTIONAL*
HOLOCHAIN_WORKING_DIR=<path to holochains working dir> *OPTIONAL is LAIR_CONNECTION_URL is not provided*
```

### Example:

```rust
    use hpos_hc_connect::HolofuelAgent;
    let mut agent: HolofuelAgent = HolofuelAgent::connect(None).await?;
    let result: ExternIO = agent
        .zome_call(
            ZomeName::from("transactor"),
            FunctionName::from("get_ledger"),
            ExternIO::encode(())?,
        )
        .await?;
````
