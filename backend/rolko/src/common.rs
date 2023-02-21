// Config
use config::Config;

static CONFIG_FILE: &str = "config_dev.toml";

fn load_config() {
    let settings = Config::builder()
        .add_source(config::File::with_name(CONFIG_FILE))
        .build()
        .unwrap();
}