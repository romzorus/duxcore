use crate::error::Error;
use config::{self, Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DuxConfigAgent {
    pub source: Source
}

impl DuxConfigAgent {
    pub fn default() -> DuxConfigAgent {
        DuxConfigAgent {
            source: Source {
                method: Some("local".to_string()),
                url: None,
                branch: None,
                path: Some("/tmp/dux/tasklist.yaml".to_string())
            }
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

#[derive(Deserialize)]
pub struct Source {
    pub method: Option<String>, // local | http | git
    pub url: Option<String>, // http (url of file) | git (url of repo (.git))
    pub branch: Option<String>, // git (branch to pull)
    pub path: Option<String>, // local (path to tasklist file) | git (file to consider once the local repo is synced)
}
