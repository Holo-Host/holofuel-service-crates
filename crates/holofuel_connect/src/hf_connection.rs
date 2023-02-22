use super::holo_config::{self, HappsFile, APP_PORT};
use anyhow::{anyhow, Context, Result};
use holochain_client::{AgentPubKey, AppWebsocket};
use holochain_conductor_api::{AppInfo, CellInfo, ProvisionedCell, ZomeCall};
use holochain_keystore::MetaLairClient;
use holochain_types::prelude::{
    ExternIO, FunctionName, Nonce256Bits, Timestamp, ZomeCallUnsigned, ZomeName,
};

pub struct HolofuelAgent {
    app_websocket: AppWebsocket,
    // admin_websocket: AdminWebsocket,
    keystore: MetaLairClient,
    holofuel_id: String,
}

impl HolofuelAgent {
    pub async fn connect() -> Result<Self> {
        let app_websocket = AppWebsocket::connect(format!("ws://localhost:{}/", APP_PORT))
            .await
            .context("failed to connect to holochain's app interface")?;
        // let admin_websocket = AdminWebsocket::connect(format!("ws://localhost:{}/", ADMIN_PORT))
        //     .await
        //     .context("failed to connect to holochain's app interface")?;
        let passphrase =
            sodoken::BufRead::from(holo_config::default_password()?.as_bytes().to_vec());
        let keystore = holochain_keystore::lair_keystore::spawn_lair_keystore(
            url2::url2!("{}", holo_config::get_lair_url()?),
            passphrase,
        )
        .await?;

        let app_file = HappsFile::build()?;
        let holofuel = app_file.holofuel().unwrap();

        Ok(Self {
            app_websocket,
            // admin_websocket,
            keystore,
            holofuel_id: holofuel.id(),
        })
    }

    pub async fn get_cell(&mut self) -> Result<(ProvisionedCell, AgentPubKey)> {
        match self
            .app_websocket
            .app_info(self.holofuel_id.clone())
            .await
            .map_err(|err| anyhow!("{:?}", err))?
        {
            Some(AppInfo {
                // This works on the assumption that the core apps has HHA in the first position of the vec
                cell_info,
                agent_pub_key,
                ..
            }) => {
                let cell = match &cell_info.get("holofuel").unwrap()[0] {
                    CellInfo::Provisioned(c) => c.clone(),
                    _ => return Err(anyhow!("unable to find holofuel")),
                };
                Ok((cell, agent_pub_key))
            }
            _ => Err(anyhow!("holofuel is not installed")),
        }
    }

    pub async fn zome_call(
        &mut self,
        zome_name: ZomeName,
        fn_name: FunctionName,
        payload: ExternIO,
    ) -> Result<ExternIO> {
        let (cell, agent_pubkey) = self.get_cell().await?;
        let (nonce, expires_at) = fresh_nonce()?;
        let zome_call_unsigned = ZomeCallUnsigned {
            cell_id: cell.cell_id,
            zome_name,
            fn_name,
            payload,
            cap_secret: None,
            provenance: agent_pubkey,
            nonce,
            expires_at,
        };
        let signed_zome_call =
            ZomeCall::try_from_unsigned_zome_call(&self.keystore, zome_call_unsigned).await?;

        let response = self
            .app_websocket
            .call_zome(signed_zome_call)
            .await
            .map_err(|err| anyhow!("{:?}", err))?;
        Ok(response)
    }
}

use std::time::Duration;
/// generates nonce for zome calls
fn fresh_nonce() -> Result<(Nonce256Bits, Timestamp)> {
    let mut bytes = [0; 32];
    getrandom::getrandom(&mut bytes)?;
    let nonce = Nonce256Bits::from(bytes);
    // Rather arbitrary but we expire nonces after 5 mins.
    let expires: Timestamp = (Timestamp::now() + Duration::from_secs(60 * 5))?;
    Ok((nonce, expires))
}
