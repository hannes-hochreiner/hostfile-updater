use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};
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
    pub fn new_from_str<S: AsRef<str>>(data: S) -> Result<Self, HostfileError> {
        let mut entries: HashMap<String, HashSet<String>> = HashMap::new();

        for line in data.as_ref().split('\n') {
            let line = line.trim();

            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            let mut tokens = line.split_whitespace();
            let address = tokens
                .next()
                .ok_or_else(|| HostfileError::NoAddressFound(line.to_string()))?
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

    pub fn add<S: AsRef<str>>(&mut self, address: S, hostnames: &[S]) {
        let address = address.as_ref();

        match self.entries.get_mut(address) {
            Some(hs) => {
                for host in hostnames {
                    hs.insert(host.as_ref().to_string());
                }
            }
            None => {
                self.entries.insert(
                    address.to_string(),
                    hostnames
                        .iter()
                        .map(|s| String::from(s.as_ref()))
                        .collect::<HashSet<String>>(),
                );
            }
        }
    }

    pub fn remove<S: AsRef<str>>(&mut self, address: S, hostnames: &[S]) {
        let address = address.as_ref();

        if let Some(hs) = self.entries.get_mut(address) {
            for host in hostnames {
                hs.remove(host.as_ref());
            }
        }

        let empty_entries: Vec<String> = self
            .entries
            .iter()
            .filter_map(|e| {
                if e.1.is_empty() {
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

impl Display for Hostfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        write!(f, "{}", lines.join("\n"))
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

        hf.add("192.168.0.1", &vec!["test.localdomain"]);

        assert_eq!(
            hf.to_string(),
            "127.0.0.1 localhost localhost.localdomain\n192.168.0.1 test.localdomain",
        );
    }

    #[test]
    fn test_remove() {
        let input = "# comment line\n127.0.0.1\tlocalhost  localhost.localdomain\n192.168.0.10 test.localdomain";
        let mut hf = Hostfile::new_from_str(&input).unwrap();

        hf.remove("192.168.0.10", &vec!["test.localdomain"]);

        assert_eq!(hf.to_string(), "127.0.0.1 localhost localhost.localdomain",);
    }

    #[test]
    fn test_multiple() {
        let input = "# comment line\n127.0.0.1\tlocalhost  localhost.localdomain\n";
        let mut hf = Hostfile::new_from_str(&input).unwrap();

        hf.add(
            "192.168.0.1",
            &vec!["test1.localdomain", "test2.localdomain"],
        );

        assert_eq!(
            hf.to_string(),
            "127.0.0.1 localhost localhost.localdomain\n192.168.0.1 test1.localdomain test2.localdomain",
        );
    }
}
