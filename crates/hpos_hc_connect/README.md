# hpos-holofuel-connect

This crate can be used to connect to holofuel running on a hpos profile that is installed by configure-holochain

### Needed enviroment variables

```
HOLOCHAIN_DEFAULT_PASSWORD=<password to unlock holochain conductor>
CORE_HAPP_FILE=<path to a config.json file used for the configure-holochain service>
HOLOCHAIN_WORKING_DIR=<path to holochains working dir>
DEV_UID_OVERRIDE=<network-seed that is used to create new hash spaces with different holo-nixpkgs builds>
```

### Example:

```rust
    use hpos_hc_connect::HolofuelAgent;
    let mut agent: HolofuelAgent = HolofuelAgent::connect().await?;
    let result: ExternIO = agent
        .zome_call(
            ZomeName::from("transactor"),
            FunctionName::from("get_ledger"),
            ExternIO::encode(())?,
        )
        .await?;
```
