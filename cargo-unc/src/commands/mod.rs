use strum::{EnumDiscriminants, EnumIter, EnumMessage};

pub mod abi_command;
pub mod build_command;
pub mod create_dev_account;
pub mod deploy;
pub mod new;

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = unc_cli_rs::GlobalContext)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(disable_back)]
#[non_exhaustive]
/// What are you up to? (select one of the options with the up-down arrows on your keyboard and press Enter)
pub enum UncCommand {
    #[strum_discriminants(strum(
        message = "new                 -  Initializes a new project to create a contract"
    ))]
    /// Initializes a new project to create a contract
    New(self::new::New),
    #[strum_discriminants(strum(
        message = "build               -  Build a UNC contract with embed ABI (opt out by passing `--no-embed-abi`)"
    ))]
    /// Build a UNC contract with embed ABI (opt out by passing `--no-embed-abi`)
    Build(self::build_command::BuildCommand),
    #[strum_discriminants(strum(
        message = "abi                 -  Generates ABI for the contract"
    ))]
    /// Generates ABI for the contract
    Abi(self::abi_command::AbiCommand),
    #[strum_discriminants(strum(
        message = "create-dev-account  -  Create a development account using a faucet service sponsor to receive some UNC tokens (testnet only).
                         To create an account on a different network, use UNC CLI [https://unc.cli.rs]"
    ))]
    /// Create a development account using the faucet service sponsor to receive some UNC tokens (testnet only)
    /// To create an account on a different network, use UNC CLI [https://unc.cli.rs]
    CreateDevAccount(self::create_dev_account::CreateAccount),
    #[strum_discriminants(strum(message = "deploy              -  Add a new contract code"))]
    /// Add a new contract code
    Deploy(self::deploy::Contract),
}
