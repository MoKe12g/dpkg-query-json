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

    fn run(){
        todo!()
    }
}
