mod commands;
mod misc;
mod utils;

use utils::uptimer::{Uptimer, UptimerKey};

use serenity::utils::MessageBuilder;
use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    client::bridge::gateway::ShardManager,
    framework::{
        standard::{
            macros::{group, help, hook},
            Args, CommandGroup, CommandResult, DispatchError, HelpOptions,
        },
        StandardFramework,
    },
    http::Http,
    model::{
        channel::Message, channel::Reaction, channel::ReactionType, event::ResumedEvent,
        gateway::Ready, guild::Member, id::ChannelId, id::GuildId, id::UserId,
    },
    prelude::*,
};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};

use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use commands::{dagpi::*, info::*, math::*, meta::*, moderation::*, owner::*};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
        println!("Using API v{}", ready.version);
        println!("Id: {}", ready.session_id);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    async fn guild_member_addition(&self, ctx: Context, guild: GuildId, mut mem: Member) {
        let cached_guild = guild.to_guild_cached(&ctx.cache).await.unwrap();
        let gid: u64 = 491175207122370581;
        if cached_guild.id.as_u64() == &gid {
            let channel = cached_guild
                .channel_id_from_name(&ctx.cache, "welcomes")
                .await
                .unwrap();
            let role = cached_guild.role_by_name("Unverified").unwrap();
            mem.add_role(&ctx, role).await.unwrap();
            let msg = MessageBuilder::new().push("Welcome").mention(&mem.user).push_bold("to Daggy Tech").push("A Server that houses projects like Dagpi,Dagbot,R.Daggy, Polraorid and More!\nTo Verify Head on over to").channel(channel).push("And read the rules to verify!\nHave a Great Time!").build();
            channel.say(ctx, msg).await.unwrap();
        }
    }

    #[allow(unused_variables)]
    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let role_channel = ChannelId::from(783311887323889675);

        if role_channel == reaction.channel_id {
            let user = reaction.user_id.unwrap();
            let cached_guild = reaction
                .guild_id
                .unwrap()
                .to_guild_cached(&ctx.cache)
                .await
                .unwrap();

            let role: (&str, u64) = match reaction.emoji.clone() {
                ReactionType::Custom { animated, id, name } => ("None", 217462890364403712),
                ReactionType::Unicode(estr) => match estr.as_str() {
                    "\u{01f9ea}" => ("Beta-Test", 783331831764221983),
                    "\u{002705}" => ("Pingable", 783332178292768788),
                    "\u{01f44c}" => ("Dagpi Notifs", 783332593318756392),
                    "\u{01f4f7}" => ("Polaroid Updates", 783332998375669799),
                    "\u{01f916}" => ("R.Daggy", 217462890364403712),
                    _ => ("None", 217462890364403712),
                },
                _ => ("None", 217462890364403712),
            };
            if role.0 != "None" && &role.1 == reaction.message_id.as_u64() {
                let mut mem = cached_guild.member(&ctx, user).await.unwrap();
                let beta_role = cached_guild.role_by_name(role.0).unwrap();
                if mem.roles.contains(&beta_role.id) {
                    match mem.remove_role(&ctx, beta_role).await {
                        Ok(_o) => {
                            match mem
                                .user
                                .direct_message(&ctx, |f| {
                                    f.content(format!("Removed role `{}`", role.0))
                                })
                                .await
                            {
                                Ok(_o) => {}
                                Err(_e) => {}
                            };
                        }
                        Err(_e) => println!("COuldn't add role"),
                    };
                } else {
                    println!("No role")
                }
            }
        }
    }

    #[allow(unused_variables)]
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let role_channel = ChannelId::from(783311887323889675);

        if role_channel == reaction.channel_id {
            let user = reaction.user_id.unwrap();
            let cached_guild = reaction
                .guild_id
                .unwrap()
                .to_guild_cached(&ctx.cache)
                .await
                .unwrap();

            let role: (&str, u64) = match reaction.emoji.clone() {
                ReactionType::Custom { animated, id, name } => ("None", 217462890364403712),
                ReactionType::Unicode(estr) => match estr.as_str() {
                    "\u{01f9ea}" => ("Beta-Test", 783331831764221983),
                    "\u{002705}" => ("Pingable", 783332178292768788),
                    "\u{01f44c}" => ("Dagpi Notifs", 783332593318756392),
                    "\u{01f4f7}" => ("Polaroid Updates", 783332998375669799),
                    "\u{01f916}" => ("R.Daggy", 217462890364403712),
                    _ => ("None", 217462890364403712),
                },
                _ => ("None", 217462890364403712),
            };
            if role.0 != "None" && &role.1 == reaction.message_id.as_u64() {
                let mut mem = cached_guild.member(&ctx, user).await.unwrap();
                let beta_role = cached_guild.role_by_name(role.0).unwrap();
                if mem.roles.contains(&beta_role.id) {
                    println!("Reapply")
                } else {
                    match mem.add_role(&ctx, beta_role).await {
                        Ok(_o) => {
                            match mem
                                .user
                                .direct_message(&ctx, |f| {
                                    f.content(format!("Gave you the role `{}`", role.0))
                                })
                                .await
                            {
                                Ok(_o) => {}
                                Err(_e) => {}
                            };
                        }
                        Err(_e) => println!("COuldn't add role"),
                    };
                }
            } else {
                reaction.delete(&ctx).await.unwrap();
            }
        };
    }
}

#[group]
#[description("Get info about R.Daggy")]
#[commands(uptime, ping, quit, latency, source, about)]
struct General;

#[group]
#[description("Commands used for dagpi")]
#[prefix = "dagpi"]
#[commands(status, approve)]
struct Dagpi;

#[group]
#[description("Fun stuff for this boring bot")]
#[commands(multiply, pride, joke)]
struct Fun;

#[group]
#[description("Top class moderation suite")]
#[commands(purge, kick, ban, unban, mute, verify, role_embed)]
struct Moderation;

#[group]
#[description("Commands for stalking!")]
#[commands(serverinfo, userinfo, commands, spotify, ide)]
struct Info;

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

#[help]
// This replaces the information that a user can pass
// a command-name as argument to gain specific information about it.
#[individual_command_tip = "```diff\n- Use `daggy help <command>` for help with a command\n+ Use `daggy help <catgeory>` for help with a category\n```"]
// Some arguments require a `{}` in order to replace it with contextual information.
// In this case our `{}` refers to a command's name.
#[command_not_found_text = "Could not find: `{}`."]
// Define the maximum Levenshtein-distance between a searched command-name
// and commands. If the distance is lower than or equal the set distance,
// it will be displayed as a suggestion.
// Setting the distance to 0 will disable suggestions.
#[max_levenshtein_distance(3)]
// When you use sub-groups, Serenity will use the `indention_prefix` to indicate
// how deeply an item is indented.
// The default value is "-", it will be changed to "+".
#[indention_prefix = "+"]
// On another note, you can set up the help-menu-filter-behaviour.
// Here are all possible settings shown on all possible options.
// First case is if a user lacks permissions for a command, we can hide the command.
#[lacking_permissions = "Hide"]
// If the user is nothing but lacking a certain role, we just display it hence our variant is `Nothing`.
#[lacking_role = "Strike"]
// The last `enum`-variant is `Strike`, which ~~strikes~~ a command.
#[wrong_channel = "Hide"]
// Serenity will automatically analyse and generate a hint/tip explaining the possible
// cases of ~~strikethrough-commands~~, but only if
// `strikethrough_commands_tip_in_{dm, guild}` aren't specified.
// If you pass in a value, it will be displayed instead.
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = misc::help::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
#[hook]
async fn before(ctx: &Context, _msg: &Message, command_name: &str) -> bool {
    // Increment the number of times this command has been run once. If
    // the command's name does not exist in the counter, add a default
    // value of 0.
    let mut data = ctx.data.write().await;
    let counter = data
        .get_mut::<CommandCounter>()
        .expect("Expected CommandCounter in TypeMap.");
    let entry = counter.entry(command_name.to_string()).or_insert(0);
    *entry += 1;

    true // if `before` returns false, command processing doesn't happen.
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => {}
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    info!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::Ratelimited(duration) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", duration.as_secs()),
                )
                .await
                .unwrap();
        }
        DispatchError::NotEnoughArguments { min, given } => {
            msg.channel_id
                .say(
                    &ctx.http,
                    &format!(
                        "This command required `{}` arguments.\nYou have only provided `{}`",
                        min, given
                    ),
                )
                .await
                .unwrap();
        }
        DispatchError::TooManyArguments { max, given } => {
            msg.channel_id
                .say(
                    &ctx.http,
                    &format!(
                        "This command only needs `{} `arguments.\nYou have only provided `{}`",
                        max, given
                    ),
                )
                .await
                .unwrap();
        }
        DispatchError::LackingPermissions(p) => {
            let mut base = String::from("You need the follwing permisison\n");
            let _p_vec = p
                .get_permission_names()
                .iter()
                .map(|f| -> String {
                    let app = format!("{}\n", f);
                    base.push_str(&app);
                    app
                })
                .collect::<Vec<String>>();
            msg.channel_id.say(&ctx.http, base).await.unwrap();
        }
        DispatchError::OnlyForOwners => {
            msg.channel_id
                .say(&ctx.http, "You don't own me looser")
                .await
                .unwrap();
        }
        DispatchError::OnlyForGuilds => {
            msg.channel_id
                .say(&ctx.http, "Use me in a guild uwu")
                .await
                .unwrap();
        }
        _ => {
            msg.channel_id
                .say(&ctx, "Unkown Error Occured")
                .await
                .unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    
    //dotenv::dotenv().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    let user: u64 = 491174779278065689;
    let id = Some(UserId::from(user));
    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).on_mention(id).prefix("daggy "))
        .before(before)
        .bucket("complicated", |b| b.delay(5).time_span(30).limit(2))
        .await
        .bucket("info", |b| b.delay(5).time_span(30).limit(6))
        .await
        .bucket("dagpi", |b| b.delay(30).time_span(60).limit(2))
        .await
        .after(after)
        .unrecognised_command(unknown_command)
        .on_dispatch_error(dispatch_error)
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&MODERATION_GROUP)
        .group(&FUN_GROUP)
        .group(&INFO_GROUP)
        .group(&DAGPI_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .intents(GatewayIntents::all())
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<UptimerKey>(Uptimer::new());
        data.insert::<utils::client::ClientKey>(utils::client::Client::new());
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
