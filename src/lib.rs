use std::path::PathBuf;
use zed_extension_api::{self as zed, Command, LanguageServerId, Worktree};

/// Zed extension to integrate Steep LSP with fallback to Bundler (only if Gemfile present), global, or no-op
struct SteepExtension;

impl zed::Extension for SteepExtension {
    fn new() -> Self {
        SteepExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Command> {
        if worktree.read_text_file("Steepfile").is_err() {
            eprintln!("Steep LSP: No Steepfile found in project root, Steep language server will not start");
            return Err("No Steepfile found in project root".to_string());
        }

        let root = PathBuf::from(worktree.root_path());
        let gemfile = root.join("Gemfile");

        if gemfile.exists() && worktree.which("bundle").is_some() {
            eprintln!("Steep LSP: Starting via bundle exec steep langserver");
            return Ok(Command {
                command: "/usr/bin/env".to_string(),
                args: vec![
                    "bundle".into(),
                    "exec".into(),
                    "steep".into(),
                    "langserver".into(),
                ],
                env: worktree.shell_env(),
            });
        }

        if let Some(steep_path) = worktree.which("steep") {
            eprintln!(
                "Steep LSP: Starting via global steep installation at {}",
                steep_path
            );
            return Ok(Command {
                command: steep_path,
                args: vec!["langserver".into()],
                env: worktree.shell_env(),
            });
        }

        eprintln!("Steep LSP: No steep executable found. Install steep via 'gem install steep' or add to Gemfile");
        Err("steep executable not found. Please install steep gem.".to_string())
    }
}

zed::register_extension!(SteepExtension);
