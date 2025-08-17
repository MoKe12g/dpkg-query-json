use crate::dpkg_options::DpkgOptions;

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

    fn run(){
        todo!()
    }
}
