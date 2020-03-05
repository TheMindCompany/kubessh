use structopt::StructOpt;
#[derive(Debug, StructOpt, Clone)]
pub enum Completions {
    /// Bash completion script.
    #[structopt(name = "bash")]
    Bash(Bash),

    /// Fish completion script.
    #[structopt(name = "fish")]
    Fish(Fish),

    /// Zsh completion script.
    #[structopt(name = "zsh")]
    Zsh(Zsh),

    /// PowerShell completion script.
    #[structopt(name = "powershell")]
    PowerShell(PowerShell),

    /// Elvish completion script.
    #[structopt(name = "elvish")]
    Elvish(Elvish)
}

#[derive(Debug, StructOpt, Clone)]
pub struct Bash {
    /// Bash completion script.
    #[structopt(default_value = "bash")]
    pub name: String,
}

#[derive(Debug, StructOpt, Clone)]
pub struct Fish {
    /// Fish completion script.
    #[structopt(default_value = "fish")]
    pub name: String,
}

#[derive(Debug, StructOpt, Clone)]
pub struct Zsh {
    /// Zsh completion script.
    #[structopt(default_value = "zsh")]
    pub name: String,
}

#[derive(Debug, StructOpt, Clone)]
pub struct PowerShell {
    /// PowerShell completion script.
    #[structopt(default_value = "powershell")]
    pub name: String,
}

#[derive(Debug, StructOpt, Clone)]
pub struct Elvish {
    /// Elvish completion script.
    #[structopt(default_value = "elvish")]
    pub name: String,
}
