use utils::shared_types::{CommandsExport, Context, Error};

/// Shows help information for commands.
///
/// Shows this help message or more detailed help for a specific command.
#[poise::command(slash_command, guild_only)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "The command to get help for."]
    #[autocomplete = "autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        show_subcommands: true,
        include_description: false,
        ..Default::default()
    };

    let command_str = command.as_deref();

    poise::builtins::help(ctx, command_str, config).await?;

    Ok(())
}

async fn autocomplete_command(ctx: Context<'_>, partial: &str) -> Vec<String> {
    ctx.framework()
        .options()
        .commands
        .iter()
        .filter_map(move |cmd| {
            let name = &cmd.name;
            if name.starts_with(partial) {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect()
}

pub static EXPORT: CommandsExport = &[help];
