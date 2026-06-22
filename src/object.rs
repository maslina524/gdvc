use std::collections::HashMap;
use std::fmt::Display;

pub struct GameObject {
    pub props: HashMap<u16, String>
}

impl GameObject {
    pub fn from(string: &str) -> Option<Self> {
        let mut props = HashMap::new();

        let string = string.trim_end_matches(';');
        let parts = string.split(",").collect::<Vec<&str>>();
        for chunk in parts.chunks(2) {
            if chunk.len() != 2 { continue; }
            let k = chunk[0].parse::<u16>().ok()?;
            let v = chunk[1].to_owned();

            props.insert(k, v);
        }

        Some(Self { props })
    }
}

impl Display for GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        for (k, v) in &self.props {
            ret.push_str(&format!("{},{},", k, v));
        }
        ret.pop();
        ret.push(';');

        write!(f, "{ret}")
    }
}