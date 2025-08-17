use std::process::Command;
use crate::dpkg_options::DpkgOptions;

pub struct ListPackages {
    package_name_pattern: String,
    options: Option<DpkgOptions>,
}

impl ListPackages {
    pub fn new(package_name_pattern: String) -> ListPackages {
        ListPackages { package_name_pattern, options: None }
    }

    pub fn with_options(package_name_pattern: String, options: DpkgOptions) -> ListPackages {
        ListPackages { package_name_pattern, options: Some(options) }
    }

    fn run() {
        /*let mut command = Command::new("dpkg-query");
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
        };*/
    }
}
