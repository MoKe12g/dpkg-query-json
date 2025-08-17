use crate::dpkg_options::DpkgOptions;
use serde_json::{json, Map, Value};
use std::io::Error;
use std::process::Command;

//#[derive(Debug)]
pub struct QueryFieldPackage {
    fields: Vec<String>,
    packages: Vec<String>,
    options: Option<DpkgOptions>,
}

impl QueryFieldPackage {
    pub fn new(fields: Vec<String>, packages: Vec<String>) -> Self {
        QueryFieldPackage {
            fields,
            packages,
            options: None,
        }
    }

    pub fn from_options(fields: Vec<String>, packages: Vec<String>, dpkg_options: DpkgOptions) -> Self {
        QueryFieldPackage {
            fields,
            packages,
            options: Some(dpkg_options),
        }
    }

    pub fn set_options(&mut self, options: DpkgOptions) {
        self.options = Some(options);
    }

    fn exec(&mut self) -> Result<String, Error> {

        // set default fields if empty
        if self.fields.len() <= 1 {
            self.fields.clear();
            self.fields = vec![String::from("Package"), String::from("Version")];
        }

        let mut command = Command::new("dpkg-query");
        command.arg("-W");
        if self.fields.len() > 0 {
            let mut modified_fields = Vec::with_capacity(29);
            for str in self.fields.iter() {
                modified_fields.push("${".to_owned() + &str + "}");
            }
            command.arg("-f").arg(format!("'{}\t\n'", modified_fields.join("<==>")));
        }

        // applying dpkg Options
        match &self.options {
            Some(options) => command.args(options.build()),
            _ => &mut command // TOOD: Why does that have to be here?
        };

        if self.packages.len() > 0 {
            command.args(&self.packages);
        }

        println!("Executing {:?}", &command);
        match command.output() {
            Ok(data) => { Ok(String::from_utf8_lossy(&data.stdout).to_string()) }
            Err(e) => Err(e)
        }
    }

    fn parse_to_json(&mut self) -> Result<Map<String, Value>, Error> {
        let mut data_json = Map::new();

        for line in self.exec()?.split("\t\n") {
            let mut d = Map::new();
            let split_line = line.split("<==>").collect::<Vec<&str>>();
            for (i, line) in split_line[1..].iter().enumerate() {
                d.insert((self.fields[i + 1]).to_string(), json!(line));
            }
            &data_json.insert(split_line[0].to_string(), Value::from(d));
        }

        Ok(data_json)
    }

    pub fn json(mut self) -> Map<String, Value> {
        self.parse_to_json().unwrap_or_else(|err| {
            let mut x = Map::new();
            x.insert(String::from("error"), Value::from(err.to_string()));
            x
        })
    }

    pub fn json_string(mut self) -> String {
        serde_json::to_string(&self.parse_to_json().unwrap_or_else(|err| {
            let mut x = Map::new();
            x.insert(String::from("error"), Value::from(err.to_string()));
            x
        })).unwrap()
    }
}
