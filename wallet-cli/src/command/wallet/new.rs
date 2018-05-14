use wallet_crypto::{bip39};
use command::{HasCommand};
use clap::{ArgMatches, Arg, App};
use config::{Config};

use super::util::{generate_entropy};
use super::Wallet;

pub struct CommandNewWallet;

impl HasCommand for CommandNewWallet {
    type Output = Option<Config>;
    type Config = Config;

    const COMMAND : &'static str = "generate";

    fn clap_options<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("generate a new wallet")
                .arg(Arg::with_name("LANGUAGE")
                    .long("language")
                    .takes_value(true)
                    .value_name("LANGUAGE")
                    .possible_values(&["english"])
                    .help("use the given language for the mnemonic")
                    .required(false)
                    .default_value(r"english")
                )
                .arg(Arg::with_name("NO PAPER WALLET")
                    .long("no-paper-wallet")
                    .takes_value(false)
                    .help("if this option is set, the interactive mode won't ask you about generating a paperwallet")
                    .required(false)
                )
                .arg(Arg::with_name("MNEMONIC SIZE")
                    .long("number-of-mnemonic-words")
                    .takes_value(true)
                    .value_name("MNEMONIC_SIZE")
                    .possible_values(&["12", "15", "18", "21", "24"])
                    .help("set the number of the mnemonic words")
                    .required(false)
                    .default_value(r"15")
                )
                .arg(Arg::with_name("PASSWORD")
                    .long("--password")
                    .takes_value(true)
                    .value_name("PASSWORD")
                    .help("set the password from the CLI instead of prompting for it. It is quite unsafe as the password can be visible from your shell history.")
                    .required(false)
                )
    }
    fn run(config: Config, args: &ArgMatches) -> Self::Output {
        let mut cfg = config;
        assert!(cfg.wallet.is_none());
        let language    = value_t!(args.value_of("LANGUAGE"), String).unwrap(); // we have a default value
        let mnemonic_sz = value_t!(args.value_of("MNEMONIC SIZE"), bip39::Type).unwrap();
        let password    = value_t!(args.value_of("PASSWORD"), String).ok();
        let without_paper_wallet = args.is_present("NO PAPER WALLET");
        let seed = generate_entropy(language, password, mnemonic_sz, without_paper_wallet);
        cfg.wallet = Some(Wallet::generate(seed));
        let _storage = cfg.get_storage().unwrap();
        Some(cfg) // we need to update the config's wallet
    }
}