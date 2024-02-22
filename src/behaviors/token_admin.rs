use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Result};
use arbiter_core::{middleware::ArbiterMiddleware};
use arbiter_engine::{
    machine::{Behavior, EventStream},
    messager::Messager
};
use arbiter_engine::machine::ControlFlow;
use arbiter_engine::messager::{Message, To};
use ethers::types::H160;

use super::*;
use crate::bindings::token::ArbiterToken;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAdmin{
    #[serde(skip)]
    pub tokens: Option<HashMap<String, ArbiterToken<ArbiterMiddleware>>>,
    pub token_data: HashMap<String, TokenData>,
    #[serde(skip)]
    pub messager: Option<Messager>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<H160>
}

/// Used as an action to ask what tokens are available.
#[derive(Debug, Deserialize, Serialize)]
pub enum TokenAdminQuery {
    /// Get the address of the token.
    AddressOf(String),
    /// Mint tokens.
    MintRequest(MintRequest)
}

/// Used as an action to mint tokens.
#[derive(Debug, Deserialize, Serialize)]
pub struct MintRequest {
    /// The token to mint.
    pub token: String,
    /// The address to mint to.
    pub mint_to: H160,
    /// The amount to mint.
    pub mint_amount: u64
}

#[async_trait::async_trait]
impl Behavior<Message> for TokenAdmin {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<Message>>> {
        let mut deployed_tokens = HashMap::new();
        let mut token_addresses = HashMap::new(); // For messaging purposes

        for (token_id, data) in self.token_data.iter_mut() {
            // Handle the deployment result
            let deploy_result = ArbiterToken::deploy(
                client.clone(),
                (
                    data.name.clone(),
                    data.symbol.clone(),
                    data.decimals,
                ),
            )?.send().await?;

            // Send the deployment transaction and await its confirmation
            let token = deploy_result;

            // Assuming `address` is an async method to get the deployed contract's address
            let token_address = token.address();
            data.address = Some(token_address);

            deployed_tokens.insert(token_id.clone(), token);
            token_addresses.insert(token_id.clone(), token_address); // Collect addresses for messaging
        }

        self.tokens = Some(deployed_tokens);

        // Serialize and send token information to all agents
        let message_content = serde_json::to_string(&token_addresses)?;

        messager
            .send(To::All, serde_json::to_string(&message_content)?)
            .await;

        // Return None as no event stream is required for startup
        Ok(None)
    }

    async fn process(
        &mut self,
        event: Message
    ) -> Result<ControlFlow> {
        // First, deserialize the incoming message to a TokenAdminQuery
        let query: TokenAdminQuery = match serde_json::from_str(&event.data) {
            Ok(query) => query,
            Err(_) => {
                // Log or handle the error as appropriate for your application
                eprintln!("Failed to deserialize the event data into a TokenAdminQuery");
                return Ok(ControlFlow::Continue);
            }
        };

        // Process the query
        match query {
            TokenAdminQuery::AddressOf(token_name) => {
                if let Some(token_data) = self.token_data.get(&token_name) {
                    let response = serde_json::to_string(&token_data.address)
                        .map_err(|_| anyhow::anyhow!("Failed to serialize token address"))?;
                    if let Some(messager) = &self.messager {
                        messager.send(To::Agent(event.from.clone()), response).await?;
                    } else {
                        eprintln!("Messager is not available.");
                        return Err(anyhow::anyhow!("Messager is not available."));
                    }
                } else {
                    eprintln!("Token not found: {}", token_name);
                }
                Ok(ControlFlow::Continue)
            },
            TokenAdminQuery::MintRequest(mint_request) => {
                if let Some(token) = self.tokens.as_ref().and_then(|tokens| tokens.get(&mint_request.token)) {
                    match token.mint(mint_request.mint_to, mint_request.mint_amount.into()).send().await {
                        Ok(_) => println!("Minting successful for token: {}", mint_request.token),
                        Err(e) => eprintln!("Failed to mint token: {}. Error: {:?}", mint_request.token, e),
                    }
                } else {
                    eprintln!("Token not found for minting: {}", mint_request.token);
                }
                Ok(ControlFlow::Continue)
            }
        }
    }
}