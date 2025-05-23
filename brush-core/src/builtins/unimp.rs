use crate::{builtins, commands, trace_categories};

use clap::Parser;

/// (UNIMPLEMENTED COMMAND)
#[derive(Parser)]
pub(crate) struct UnimplementedCommand {
    #[clap(allow_hyphen_values = true)]
    args: Vec<String>,

    #[clap(skip)]
    declarations: Vec<commands::CommandArg>,
}

impl builtins::Command for UnimplementedCommand {
    async fn execute(
        &self,
        context: commands::ExecutionContext<'_>,
    ) -> Result<crate::builtins::ExitCode, crate::error::Error> {
        tracing::warn!(target: trace_categories::UNIMPLEMENTED,
            "unimplemented built-in: {} {}",
            context.command_name,
            self.args.join(" ")
        );
        Ok(builtins::ExitCode::Unimplemented)
    }
}

impl builtins::DeclarationCommand for UnimplementedCommand {
    fn set_declarations(&mut self, declarations: Vec<commands::CommandArg>) {
        self.declarations = declarations;
    }
}
