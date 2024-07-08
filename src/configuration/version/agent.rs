use crate::error::Error;
use config::{self, Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DuxConfigAgent {
    // Placeholder
}

impl DuxConfigAgent {
    pub fn default() -> DuxConfigAgent {
        DuxConfigAgent {
            // Placeholder
        }
    }

    pub fn from(path: Option<String>) -> Result<DuxConfigAgent, Error> {
        let config_file_path = match path {
            Some(content) => content,
            None => "/etc/dux/dux.conf".to_string(),
        };

        let config_builder = Config::builder()
            .add_source(File::new(config_file_path.as_str(), FileFormat::Ini))
            .build();

        match config_builder {
            Ok(config_content) => {
                let dux_config = config_content.try_deserialize::<DuxConfigAgent>();

                match dux_config {
                    Ok(config) => Ok(config),
                    Err(e) => {
                        // TODO : in this case, the user provided a config file but the parsing failed -> use default values instead of stopping everythin ?
                        Err(Error::FailureToParseContent(format!("{e}")))
                    }
                }
            }
            Err(_e) => {
                // TODO : Log some warning with 'e' content
                Ok(DuxConfigAgent::default())
            }
        }
    }
}
