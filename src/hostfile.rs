use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HostfileError {
    #[error("no address found in line \"{0}\"")]
    NoAddressFound(String),
}

pub struct Hostfile {
    entries: HashMap<String, HashSet<String>>,
}

impl Hostfile {
    pub fn new_from_str(data: &str) -> Result<Self, HostfileError> {
        let mut entries: HashMap<String, HashSet<String>> = HashMap::new();

        for line in data.split('\n') {
            let line = line.trim();

            if line.starts_with("#") || line.is_empty() {
                continue;
            }

            let mut tokens = line.split_whitespace();
            let address = tokens
                .next()
                .ok_or(HostfileError::NoAddressFound(line.to_string()))?
                .to_string();
            let hostnames = tokens.map(|s| s.to_owned());

            match entries.get_mut(&address) {
                Some(hs) => {
                    hs.extend(hostnames);
                }
                None => {
                    entries.insert(address, hostnames.collect());
                }
            }
        }

        Ok(Hostfile { entries })
    }

    pub fn to_string(&self) -> String {
        let mut lines = self
            .entries
            .iter()
            .map(|(k, v)| {
                let mut hostnames = v.iter().map(|s| s.to_owned()).collect::<Vec<String>>();

                hostnames.sort();

                format!("{} {}", k, hostnames.join(" "))
            })
            .collect::<Vec<String>>();

        lines.sort();
        lines.join("\n")
    }

    pub fn add(&mut self, address: &str, hostname: &str) {
        match self.entries.get_mut(address) {
            Some(hs) => {
                hs.insert(hostname.to_string());
            }
            None => {
                self.entries
                    .insert(address.to_string(), HashSet::from([hostname.to_string()]));
            }
        }
    }

    pub fn remove(&mut self, address: &str, hostname: &str) {
        match self.entries.get_mut(address) {
            Some(hs) => {
                hs.remove(hostname);
            }
            _ => {}
        }

        let empty_entries: Vec<String> = self
            .entries
            .iter()
            .filter_map(|e| {
                if e.1.len() == 0 {
                    Some(e.0.to_owned())
                } else {
                    None
                }
            })
            .collect();

        for ee in empty_entries {
            self.entries.remove(&ee);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_integration_1() {
        let input = "# comment line\n127.0.0.1 localhost\n";

        assert_eq!(
            Hostfile::new_from_str(&input).unwrap().to_string(),
            "127.0.0.1 localhost",
        );
    }

    #[test]
    fn test_integration_2() {
        let input = "# comment line\n127.0.0.1 localhost\n127.0.0.1 test.localdomain";

        assert_eq!(
            Hostfile::new_from_str(&input).unwrap().to_string(),
            "127.0.0.1 localhost test.localdomain",
        );
    }
    #[test]
    fn test_whitespace() {
        let input = "# comment line\n127.0.0.1\tlocalhost  localhost.localdomain\n";

        assert_eq!(
            Hostfile::new_from_str(&input).unwrap().to_string(),
            "127.0.0.1 localhost localhost.localdomain",
        );
    }

    #[test]
    fn test_add() {
        let input = "# comment line\n127.0.0.1\tlocalhost  localhost.localdomain\n";
        let mut hf = Hostfile::new_from_str(&input).unwrap();

        hf.add("192.168.0.1", "test.localdomain");

        assert_eq!(
            hf.to_string(),
            "127.0.0.1 localhost localhost.localdomain\n192.168.0.1 test.localdomain",
        );
    }

    #[test]
    fn test_remove() {
        let input = "# comment line\n127.0.0.1\tlocalhost  localhost.localdomain\n192.168.0.10 test.localdomain";
        let mut hf = Hostfile::new_from_str(&input).unwrap();

        hf.remove("192.168.0.10", "test.localdomain");

        assert_eq!(hf.to_string(), "127.0.0.1 localhost localhost.localdomain",);
    }
}
