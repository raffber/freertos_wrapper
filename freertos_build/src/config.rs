use anyhow::Result;
use handlebars::Handlebars;
use serde::Serialize;

const TEMPLATE: &str = include_str!("FreeRTOSConfig.h.hbs");

#[derive(Serialize, Debug)]
pub struct Config {
    pub clock_freq: u32,
    pub tick_rate: u32,

    pub max_prios: u32,
}

pub struct ConfigBuilder {
    config: Config,
}

impl Config {
    pub fn new(clock_freq: u32) -> ConfigBuilder {
        ConfigBuilder {
            config: Config {
                clock_freq,
                tick_rate: 1000,
                max_prios: 5,
            },
        }
    }

    pub fn render(&self) -> Result<String> {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("config", TEMPLATE)
            .unwrap();
        Ok(handlebars.render("config", self)?)
    }
}

impl ConfigBuilder {
    pub fn build(self) -> Config {
        self.config
    }

    pub fn tick_rate(mut self, tick_rate: u32) -> Self {
        self.config.tick_rate = tick_rate;
        self
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    fn check(cfg: &Config, expr: &str) {
        let regex = Regex::new(expr).unwrap();
        let rendered = cfg.render().unwrap();
        regex
            .find(&rendered)
            .unwrap_or_else(|| panic!("Expected to find {} in config {}", expr, rendered));
    }

    #[test]
    fn check_template() {
        let cfg = Config::new(100e6 as u32).build();
        check(&cfg, "configCPU_CLOCK_HZ\\s+100000000");
    }
}
