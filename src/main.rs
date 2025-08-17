use dpkg_query_json::{QueryFieldPackage};


fn main() {
    let fields = vec![
                        "Package".to_string(),
                      "Version".to_string(),
                      "Architecture".to_string()];

    // let fields = vec!["Package".to_string(),
    //                   "Architecture".to_string()
    // ];


    //let fields = Vec::new();
    //let packages = Vec::new();
    let packages = vec!["nginx".to_string()];
    let mut query = QueryFieldPackage::new(fields, packages);
    //query.set_root_dir(Option::from("/home/quantenregen/Schreibtisch/test-bookworm/".to_string()));
    println!("{:?}", query.json_string());

}
