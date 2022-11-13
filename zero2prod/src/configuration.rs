#[derive(serde::Deserialize, Debug)]
pub struct Setting {
    database: DatabaseSettings,
    application_port: u16,
}

impl Setting {
    pub fn new(database: DatabaseSettings, application_port: u16) -> Self {
        Self {
            database,
            application_port,
        }
    }

    pub fn database(&self) -> &DatabaseSettings {
        &self.database
    }

    pub fn application_port(&self) -> &u16 {
        &self.application_port
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    username: String,
    password: String,
    port: u16,
    host: String,
    database_name: String,
}

impl DatabaseSettings {
    pub fn new(
        username: String,
        password: String,
        port: u16,
        host: String,
        database_name: String,
    ) -> Self {
        Self {
            username,
            password,
            port,
            host,
            database_name,
        }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }

    pub fn host(&self) -> &String {
        &self.host
    }

    pub fn database_name(&self) -> &String {
        &self.database_name
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration() -> Result<Setting, config::ConfigError> {
    let builder = config::Config::builder()
        .add_source(config::File::new("configuration", config::FileFormat::Yaml));
    match builder.build() {
        Ok(config) => config.try_deserialize::<Setting>(),
        Err(e) => Err(e),
    }
}
