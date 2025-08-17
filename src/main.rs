use std::option::Option;
use dpkg_query_json::dpkg_options::DpkgOptions;
use dpkg_query_json::query_field_package::QueryFieldPackage;

// Example for using query_field_package
fn main() {
    let fields = vec![
                        "Package".to_string(),
                      "Version".to_string(),
                      "Conffiles".to_string(),
                        "Provides".to_string(),
                    "Status".to_string(),
    "Origin".to_string()];

    // let fields = vec!["Package".to_string(),
    //                   "Architecture".to_string()
    // ];


    //let fields = Vec::new();
    //let packages = Vec::new();
    let packages = vec!["bash".to_string()];
    let mut query = QueryFieldPackage::new(fields, packages);
    query.set_options(DpkgOptions::new().set_root_dir("/home/quantenregen/Schreibtisch/test-bookworm/".to_string()));
    println!("{:?}", query.json());

    // TODO: List packages (--list [Paketname-Muster])
    // TODO: List files installed from packages (--listfiles [paketname])
    // TODO: List packets that have configuration files installed
    // TODO: Get hashes for specified files from specified packages
    
}
