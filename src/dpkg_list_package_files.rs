use crate::dpkg_options::DpkgOptions;
use std::io::Error;
use std::process::Command;

pub struct ListPackageFiles {
    package_name_pattern: String,
    options: Option<DpkgOptions>,
}

impl ListPackageFiles {
    pub fn new(package_name_pattern: String) -> ListPackageFiles {
        ListPackageFiles { package_name_pattern, options: None }
    }

    pub fn with_options(package_name_pattern: String, options: DpkgOptions) -> ListPackageFiles {
        ListPackageFiles { package_name_pattern, options: Some(options) }
    }

    pub fn exec(&self) -> Result<Vec<String>, Error> {
        let mut command = Command::new("dpkg-query");

        // applying dpkg Options
        match &self.options {
            Some(options) => command.args(options.build()),
            _ => &mut command // TOOD: Why does that have to be here?
        };

        command.args(["--listfiles", &self.package_name_pattern]);

        println!("Executing {:?}", &command);
        match command.output() {
            Ok(data) => { Ok(String::from_utf8_lossy(&data.stdout).split("\n").filter(move |x| { !x.is_empty() }).map(|s| s.to_string()).collect()) }
            Err(e) => Err(e)
        }
    }
}
