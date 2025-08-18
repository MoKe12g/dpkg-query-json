use dpkg_query_json::dpkg_list_package_files;
use dpkg_query_json::dpkg_list_packages::DpkgListPackages;
use dpkg_query_json::dpkg_options::DpkgOptions;

// Example for using query_field_package
fn main() {

    // Attributes you can use to get the information you want from dpkg-query
    let fields = vec![
        "Package".to_string(),
        "Version".to_string(),
        "Conffiles".to_string(),
        "Provides".to_string(),
        "Status".to_string(),
        "Origin".to_string()];

    // Features

    // List packages (--list [Paketname-Muster]) -> dpkg_list_packages
    let packages = vec![];
    let mut query = DpkgListPackages::new(fields, packages);
    // it is possible to set another root directory
    query.set_options(DpkgOptions::new().set_root_dir("/home/quantenregen/Schreibtisch/test-bookworm/".to_string()));
    println!("{:?}", query.json());

    // List files installed from packages (--listfiles [paketname])
    let files = dpkg_list_package_files::ListPackageFiles::new("bash".parse().unwrap()).exec().unwrap();
    println!("{:?}", files);

    // TODO: List packets that have configuration files installed


    // TODO: Get hashes for specified files from specified packages

}
