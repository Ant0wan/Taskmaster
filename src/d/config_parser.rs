use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub struct IniConfig {
    sections: HashMap<String, HashMap<String, String>>,
}

impl IniConfig {
    pub fn new() -> Self {
        IniConfig {
            sections: HashMap::new(),
        }
    }

    pub fn add_section(&mut self, section_name: String) {
        self.sections.insert(section_name, HashMap::new());
    }

    pub fn add_key_value(&mut self, section_name: String, key: String, value: String) {
        let section = self
            .sections
            .entry(section_name)
            .or_insert_with(HashMap::new);
        section.insert(key, value);
    }
}

pub fn parse_ini_file(filename: &str) -> Result<IniConfig, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let mut config = IniConfig::new();
    let mut current_section = String::new();

    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].to_string();
            config.add_section(current_section.clone());
        } else if let Some(idx) = line.find('=') {
            let key = line[..idx].trim().to_string();
            let value = line[idx + 1..].trim().to_string();
            config.add_key_value(current_section.clone(), key, value);
        } else {
            eprintln!("Invalid line: {}", line);
        }
    }

    Ok(config)
}