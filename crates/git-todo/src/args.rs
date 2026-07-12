use clap::{Parser, Subcommand};

/// Represents CLI arguments.
#[derive(Clone, Debug, Parser)]
#[command(name = "git-todo", about = "Manage TODO lists for git.")]
pub struct Args {
    /// The branch to operate on.
    #[arg(short, long, global = true, help = "The branch to operate on")]
    pub branch: Option<String>,

    /// The repo path.
    #[arg(short, long, help = "Git repository path", default_value = ".")]
    pub repo: String,

    /// CLI subcommand.
    #[command(subcommand)]
    pub subcommand: Sub,
}

/// Represents CLI subcommands.
#[derive(Clone, Debug, Subcommand)]
pub enum Sub {
    /// Subcommand for adding a new item.
    #[command(about = "Add a new TODO item", alias = "a")]
    Add {
        /// Item content.
        item: String,
    },

    /// Subcommand for checking TODO items status.
    #[command(about = "Checks if there are still TODO items undone", alias = "ch")]
    Check {
        /// Specifies whether to suppress output.
        #[arg(short, long, help = "Do not output anything")]
        quiet: bool,
    },

    /// Subcommand for clearing TODO items.
    #[command(about = "Clears TODO items", alias = "cl")]
    Clear {
        /// Specifies whether to clear only done items.
        #[arg(short, long, help = "Only clear done items")]
        done: bool,
    },

    /// Subcommand for completing items.
    #[command(about = "Complete TODO items", alias = "c")]
    Complete {
        /// Item indexes to complete.
        indexes: Vec<usize>,
    },

    /// Subcommand for deleting items.
    #[command(about = "Delete TODO items", alias = "d")]
    Delete {
        /// Item indexes to delete.
        indexes: Vec<usize>,
    },

    /// Subcommand for listing items.
    #[command(about = "List TODO items", alias = "l")]
    List {
        /// Specifies whether to show only undone items.
        #[arg(short, long, help = "Only show undone items")]
        undone: bool,
    },
}
