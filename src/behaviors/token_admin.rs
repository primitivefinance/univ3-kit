use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use arbiter_core::middleware::ArbiterMiddleware;
use arbiter_engine::messager::{Message, Messager, To};
use ethers::types::H160;

use super::*;
use crate::bindings::token::ArbiterToken;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAdmin {
    #[serde(skip)]
    pub tokens: Option<HashMap<String, ArbiterToken<ArbiterMiddleware>>>,

    pub token_data: HashMap<String, TokenData>,

    #[serde(skip)]
    pub messager: Option<Messager>,

    #[serde(skip)]
    pub client: Option<Arc<ArbiterMiddleware>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: Option<H160>,
}

/// Used as an action to ask what tokens are available.
#[derive(Debug, Deserialize, Serialize)]
pub enum TokenAdminQuery {
    /// Get the address of the token.
    AddressOf(String),

    /// Mint tokens.
    MintRequest(MintRequest),

    /// Deploy request.
    DeployRequest(TokenData),
}

/// Used as an action to mint tokens.
#[derive(Debug, Deserialize, Serialize)]
pub struct MintRequest {
    /// The token to mint.
    pub token: String,
    /// The address to mint to.
    pub mint_to: H160,
    /// The amount to mint.
    pub mint_amount: u64,
}

#[async_trait::async_trait]
impl Behavior<Message> for TokenAdmin {
    async fn startup(
        &mut self,
        client: Arc<ArbiterMiddleware>,
        messager: Messager,
    ) -> Result<Option<EventStream<Message>>> {
        let mut deployed_tokens = HashMap::new();
        let mut token_addresses = HashMap::new();

        for (token_id, data) in self.token_data.iter_mut() {
            let deploy_result = ArbiterToken::deploy(
                client.clone(),
                (data.name.clone(), data.symbol.clone(), data.decimals),
            )?
            .send()
            .await?;

            let token = deploy_result;

            let token_address = token.address();
            data.address = Some(token_address);

            deployed_tokens.insert(token_id.clone(), token);
            token_addresses.insert(token_id.clone(), token_address);
        }

        self.tokens = Some(deployed_tokens);
        self.client = Some(client.clone());

        let _ = messager.send(To::All, &token_addresses).await;

        Ok(Some(messager.clone().stream().unwrap()))
    }

    async fn process(&mut self, event: Message) -> Result<ControlFlow> {
        let query: TokenAdminQuery = match serde_json::from_str(&event.data) {
            Ok(query) => query,
            Err(_) => {
                eprintln!("Failed to deserialize the event data into a TokenAdminQuery");
                return Ok(ControlFlow::Continue);
            }
        };

        match query {
            TokenAdminQuery::AddressOf(token_name) => {
                if let Some(token_data) = self.token_data.get(&token_name) {
                    let response = if let Some(v) = token_data.address {
                        v
                    } else {
                        return Err(anyhow::anyhow!("Token address is not found."));
                    };
                    if let Some(messager) = &self.messager {
                        messager
                            .send(To::Agent(event.from.clone()), response)
                            .await?;
                    } else {
                        eprintln!("Messager is not available.");
                        return Err(anyhow::anyhow!("Messager is not available."));
                    }
                } else {
                    eprintln!("Token not found: {}", token_name);
                }
                Ok(ControlFlow::Continue)
            }
            TokenAdminQuery::MintRequest(mint_request) => {
                if let Some(token) = self
                    .tokens
                    .as_ref()
                    .and_then(|tokens| tokens.get(&mint_request.token))
                {
                    match token
                        .mint(mint_request.mint_to, mint_request.mint_amount.into())
                        .send()
                        .await
                    {
                        Ok(_) => println!("Minting successful for token: {}", mint_request.token),
                        Err(e) => eprintln!(
                            "Failed to mint token: {}. Error: {:?}",
                            mint_request.token, e
                        ),
                    }
                } else {
                    eprintln!("Token not found for minting: {}", mint_request.token);
                }
                Ok(ControlFlow::Continue)
            }
            TokenAdminQuery::DeployRequest(deploy_request) => {
                let req = deploy_request.clone();
                let token = ArbiterToken::deploy(
                    self.client.clone().unwrap(),
                    (
                        deploy_request.name,
                        deploy_request.symbol,
                        deploy_request.decimals,
                    ),
                )?
                .send()
                .await?;

                self.token_data.insert(req.name.clone(), req.clone());
                self.tokens
                    .as_mut()
                    .and_then(|token_obj| token_obj.insert(req.name, token));

                Ok(ControlFlow::Continue)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use arbiter_engine::{agent::Agent, world::World};
    use futures_util::StreamExt;
    use tracing_subscriber::FmtSubscriber;

    use crate::behaviors::token_admin::{TokenAdmin, TokenData};

    #[tokio::test]
    async fn token_admin_behavior_test() {
        let subscriber = FmtSubscriber::new();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        let mut world = World::new("univ3");
        let messager = world.messager.clone();

        let token_admin_behavior = TokenAdmin {
            tokens: None,
            token_data: {
                let mut h = HashMap::new();
                h.insert(
                    "MockToken".to_string(),
                    TokenData {
                        name: "MockToken".to_string(),
                        symbol: "MTK".to_string(),
                        decimals: 18,
                        address: None,
                    },
                );
                h
            },
            messager: Some(messager.clone()),
            client: None,
        };

        let agent = Agent::builder("token_admin_agent");
        world.add_agent(agent.with_behavior(token_admin_behavior));

        world.run().await.expect("World failed to run");

        let mut stream = messager.stream().expect("Failed to get messager stream");
        if let Some(res) = stream.next().await {
            let token_res_data = &res.data;
            println!("{}", token_res_data);

            let data: String =
                serde_json::from_str(token_res_data).expect("Failed to deserialize message data");

            let parsed_data: HashMap<String, String> =
                serde_json::from_str(&data).expect("Failed to deserialize token data");

            if let Some(address) = parsed_data.get("MockToken") {
                assert_eq!("0xb00efcb70090a21d46660adf95a16ec69623f694", address);
            } else {
                panic!("MockToken not found in the parsed data");
            }
        }
    }
}
