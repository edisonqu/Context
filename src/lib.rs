mod api_call;

use reqwest;
use std::error;
use anyhow::anyhow;
use serenity::async_trait;
use serenity::Client;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::interaction::Interaction;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot {
    hugging_face_api: String,
    client: reqwest::Client,
    guild_id: GuildId
}

#[async_trait]
impl EventHandler for Bot {

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = GuildId::set_application_commands(&self.guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| { command.name("hello").description("to say hello") })
                .create_application_command(|command| {
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
            })
        }).await.unwrap();

        info!("{:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {

        if let Interaction::ApplicationCommand(command) = interaction {

            let response_content = match command.data.name.as_str() {
                "hello" => "hello".to_owned(),
                "ask" => {
                let mut create_interaction_responses = command.create_interaction_response(&ctx.http, |response| {
                        response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                    });

                    if let Err(why) = create_interaction_response.await {
                        eprintln!("{}", why)
                    }
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
                    println!("{}",question);
                    let result = api_call::question_and_context(question, context).await;
                    print!("{:?}",result);
                    match result {

                        Ok((resp)) => {
                            let answer = resp["answer"].as_str().unwrap();
                            format!("{}", answer.trim_matches('"'),
                        )},
                        Err(err) => {
                            format!("Err: {}", err)
                        }
                    }
                }
                command => unreachable!("Unknown Command: {}", command)
            };

            let create_interaction_response = command.create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|message| message.content(response_content))
            });

            if let Err(why) = create_interaction_response.await {
                eprintln!("{}", why)
            }
        }
    }
}

//    let question = "Why do we assume that it is well-formed?";
//     let context = "Now in the interaction handler, we can add a new branch to the match tree. We pull out the option/argument corresponding to place and extract its value. Because of the restrictions made when setting the option we can assume that it is well-formed (unless Discord sends a bad request) and thus the unwraps here. After we have the arguments of the command we call the get_forecast function and format the results into a string to return.";
//
//
//     match question_and_context(question, context).await {
//         Ok(response) => println!("Response: {:?}", response["answer"]),
//         Err(error) => println!("Error: {:?}", error),
//     }

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
    let guild_id = if let Some(guild_id) = secret_store.get("DISCORD_GUILD_ID") {
        guild_id
    } else {
        return Err(anyhow!("'DISCORD_GUILD_ID' was not found").into());
    };


    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot {
            hugging_face_api: api_key,
            client: reqwest::Client::new(),
            guild_id: GuildId(guild_id.parse().unwrap())
        })
        .await
        .expect("Err creating client");

    Ok(client)
}
