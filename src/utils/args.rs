use std::{cmp::max, collections::HashMap, env};

pub struct Args {
    arg_map: HashMap<String, String>,
}

impl Args {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let arg_map = args
            .into_iter()
            .filter_map(|arg| match arg.split_once("=") {
                Some((key, val)) => Some((key.to_string(), val.to_string())),
                None => None,
            })
            .collect::<HashMap<_, _>>();

        Self { arg_map }
    }

    pub fn get_batch_size(&self) -> usize {
        let default = 1;
        let batch_size = self
            .arg_map
            .get("batch_size")
            .map(|s| max(1, s.parse::<usize>().unwrap_or(default)));

        batch_size.unwrap_or(default)
    }
}
