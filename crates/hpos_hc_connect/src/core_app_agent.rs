use super::holo_config::{self, HappsFile, APP_PORT};
use crate::utils::fresh_nonce;
use anyhow::{anyhow, Context, Result};
use holochain_client::{AgentPubKey, AppWebsocket};
use holochain_conductor_api::{AppInfo, CellInfo, ProvisionedCell, ZomeCall};
use holochain_keystore::{AgentPubKeyExt, MetaLairClient};
use holochain_types::prelude::{ExternIO, FunctionName, Signature, ZomeCallUnsigned, ZomeName};
use std::sync::Arc;

pub struct CoreAppAgent {
    app_websocket: AppWebsocket,
    // admin_websocket: AdminWebsocket,
    keystore: MetaLairClient,
    core_app_id: String,
}

impl CoreAppAgent {
    /// connects to a holofuel agent that is running on a hpos server
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
        let core_app = app_file.core_app().unwrap();

        Ok(Self {
            app_websocket,
            // admin_websocket,
            keystore,
            core_app_id: core_app.id(),
        })
    }

    /// get cell details of the hha agent
    pub async fn get_cell(
        &mut self,
        role_name: CoreAppRoleName,
    ) -> Result<(ProvisionedCell, AgentPubKey)> {
        match self
            .app_websocket
            .app_info(self.core_app_id.clone())
            .await
            .map_err(|err| anyhow!("{:?}", err))?
        {
            Some(AppInfo {
                // This works on the assumption that the core apps has HHA in the first position of the vec
                cell_info,
                agent_pub_key,
                ..
            }) => {
                let cell = match &cell_info.get(role_name.id()).unwrap()[0] {
                    CellInfo::Provisioned(c) => c.clone(),
                    _ => return Err(anyhow!("unable to find {}", role_name.id())),
                };
                Ok((cell, agent_pub_key))
            }
            _ => Err(anyhow!("holofuel is not installed")),
        }
    }
    /// Sign byte payload with holofuel agent's private key
    pub async fn sign_raw(&mut self, data: Arc<[u8]>) -> Result<Signature> {
        // We are signing all calls using the core-app agent because we are assuming both the cells have the same agent-key
        let (_, agent_pubkey) = self.get_cell(CoreAppRoleName::HHA).await?;
        Ok(agent_pubkey.sign_raw(&self.keystore, data).await?)
    }

    /// make a zome call to the holofuel agent that is running on a hpos server
    pub async fn zome_call(
        &mut self,
        role_name: CoreAppRoleName,
        zome_name: ZomeName,
        fn_name: FunctionName,
        payload: ExternIO,
    ) -> Result<ExternIO> {
        let (cell, agent_pubkey) = self.get_cell(role_name).await?;
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

pub enum CoreAppRoleName {
    HHA,
    Holofuel,
}
impl CoreAppRoleName {
    fn id(&self) -> &str {
        match self {
            CoreAppRoleName::HHA => "core-app",
            CoreAppRoleName::Holofuel => "holofuel",
        }
    }
}
