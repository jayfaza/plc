use crate::errors::{
    ConfigParseError, ConfigReadError, CouldNotCreateConfig, CouldNotLoadConfig,
    CouldNotWriteConfig,
};
use crate::utils::calonicalize;
use error_stack::{Report, ResultExt};
use serde::{Deserialize, Serialize};
use shellexpand::tilde;
use std::fs::read_to_string;

#[derive(Deserialize, Serialize)]
pub struct ConfigToml {
    pub included_extensions: Vec<String>,
}

pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Self {}
    }
    fn read_from_path(&self, path: &str) -> Result<String, Report<ConfigReadError>> {
        Ok(read_to_string(path).change_context(ConfigReadError)?)
    }

    fn parse_config(&self, config_string: String) -> Result<ConfigToml, Report<ConfigParseError>> {
        Ok(toml::from_str(config_string.as_str())
            .attach("failed to parse config")
            .change_context(ConfigParseError)?)
    }

    pub fn load_config(&self) -> Result<ConfigToml, Report<CouldNotLoadConfig>> {
        let raw_config = self
            .read_from_path(
                &calonicalize(&tilde("~/.config/plc/plc.toml"))
                    .change_context(CouldNotLoadConfig)?,
            )
            .attach("failed to load plc.toml")
            .change_context(CouldNotLoadConfig)?;
        Ok(self
            .parse_config(raw_config)
            .change_context(CouldNotLoadConfig)?)
    }

    fn create_config(&self) -> Result<(), Report<CouldNotCreateConfig>> {
        let configs_dir =
            &calonicalize(&tilde("~/.config").to_string()).change_context(CouldNotCreateConfig)?;
        let dir_path_str = &format!("{}/plc", configs_dir);
        let file_path_str = &format!("{}/plc.toml", dir_path_str);

        let file = std::path::Path::new(file_path_str);

        let dir = std::path::Path::new(dir_path_str);

        std::fs::create_dir(dir)
            .attach_with(|| format!("failed to create config dir: {}", dir.display()))
            .change_context(CouldNotCreateConfig)?;

        std::fs::File::create(file)
            .attach_with(|| format!("could not create config: {}", file.display()))
            .change_context(CouldNotCreateConfig)?;
        Ok(())
    }

    pub fn create_default_config(&self) -> Result<(), Report<CouldNotCreateConfig>> {
        self.create_config()?;
        self.write_to_config(self.default_config_entry())
            .change_context(CouldNotCreateConfig)
    }

    fn write_to_config(&self, data: String) -> Result<(), Report<CouldNotWriteConfig>> {
        let configs_dir = &calonicalize(&tilde("~/.config")).change_context(CouldNotWriteConfig)?;
        let config_file = calonicalize(&format!("{}/plc/plc.toml", configs_dir))
            .change_context(CouldNotWriteConfig)?;
        Ok(std::fs::write(config_file, data)
            .attach_with(|| format!("failed to write to plc.toml"))
            .change_context(CouldNotWriteConfig)?)
    }

    fn default_config_entry(&self) -> String {
        let extensions: Vec<String> = vec![
            "rs".to_string(),
            "py".to_string(),
            "lua".to_string(),
            "c".to_string(),
            "cpp".to_string(),
            "sh".to_string(),
            "js".to_string(),
        ];
        let config = ConfigToml {
            included_extensions: extensions,
        };
        toml::to_string(&config).unwrap()
    }
}
