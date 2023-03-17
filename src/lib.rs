mod api_call;

use reqwest;
use std::error;
use anyhow::anyhow;
use serenity::async_trait;
use serenity::Client;
use serenity::model::application::command;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::application::command::Command;
use serenity::model::prelude::interaction::Interaction;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot {
    hugging_face_api: String,
    client: reqwest::Client,
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = Command::create_global_application_command( &ctx.http, |command| {
                    command
                        .name("ask")
                        .description("Ask a question with context, receive an answer")
                        .create_option(|option| {
                            option
                                .name("context")
                                .description("Provide the context for your question")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|option| {
                            option
                                .name("question")
                                .description("Ask a question about the context given")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })

        }).await.unwrap();

        info!("{:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "ask" => {
                    let result = command.defer(&ctx.http).await;
                    println!("{:?}", result);
                    let context_wrapped = command
                        .data
                        .options
                        .iter()
                        .find(|option| option.name == "context")
                        .cloned();

                    let question_wrapped = command
                        .data
                        .options
                        .iter()
                        .find(|option| option.name == "question")
                        .cloned();

                    let context_value = context_wrapped.unwrap().value.unwrap();
                    let question_value = question_wrapped.unwrap().value.unwrap();

                    let context = context_value.as_str().unwrap();
                    let question = question_value.as_str().unwrap();
                    println!("{}", question);
                    let result = api_call::question_and_context(question, context, &self.hugging_face_api).await;
                    print!("{:?}", result);
                    match result {
                        Ok((resp)) => {
                            if resp.get("answer").is_some() {
                                let answer = resp["answer"].as_str().unwrap();
                                format!("{}", answer.trim_matches('"'))
                            }
                            else if resp.get("error").is_some(){
                                let answer = resp["error"].as_str().unwrap();
                                format!("{}", answer.trim_matches('"'))
                            }else {
                                format!("Error: {}. If the problem persists, please contact ed#1234 or add an issue at https://github.com/edisonqu/Context.", resp)
                            }
                        }
                        Err(err) => {
                            format!("Err: {}", err)
                        }
                    }
                }
                command => unreachable!("Unknown Command: {}", command)
            };

            let create_interaction_response = command.edit_original_interaction_response(&ctx.http, |response| {
                response.content(response_content)
            });
            if let Err(why) = create_interaction_response.await {
                eprintln!("{}", why)
            }
        }
    }
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    println!("{:?}", secret_store.get("DISCORD_TOKEN"));
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let api_key = if let Some(api_key) = secret_store.get("HUGGING_FACE_API") {
        api_key
    } else {
        return Err(anyhow!("'HUGGING_FACE_API' was not found").into());
    };


    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot {
            hugging_face_api: api_key,
            client: reqwest::Client::new(),
        })
        .await
        .expect("Err creating client");

    Ok(client)
}
