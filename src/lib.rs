//! # dpkg-query-json
//!
//!
//! A crate for parsing "dpkg-query" in json.
//!
//! # Examples
//!
//!
//! Minimum length of fields 2. Default fields `Package` `Version`. List available below.
//!
//! if the list of packages is empty, all are returned
//!
//!
//! #### Map<String, Value>
//!```
//! use dpkg_query_json::query_field_package::QueryFieldPackage;
//! let fields = vec![String::from("Package"),
//! String::from("Version"),
//! String::from("Architecture")];
//! let packages = vec![String::from("dpkg")];
//! QueryFieldPackage::new(fields, packages).json(); //Map<String, Value>
//!
//! ```
//!
//!
//! ```{"dpkg": Object({"Architecture": String("amd64"), "Version": String("1.19.7ubuntu3")})}```
//!
//!
//!
//!------------------------------------
//!
//!
//!#### String
//!```
//! use dpkg_query_json::query_field_package::QueryFieldPackage;
//! let fields = vec![String::from("Package"),
//! String::from("Version"),
//! String::from("Architecture")];
//! let packages = vec![String::from("dpkg")];
//! QueryFieldPackage::new(fields, packages).json_string(); //String
//!
//! ```
//!
//! ```"{\"dpkg\":{\"Architecture\":\"amd64\",\"Version\":\"1.19.7ubuntu3\"}}"```
//!


//! # Package information fields
//!
//! Architecture
//!
//! Bugs
//!
//! Conffiles
//!
//! Config-Version
//!
//! Conflicts
//!
//! Breaks
//!
//! Depends
//!
//! Description
//!
//! Enhances
//!
//! Essential
//!
//! Filename
//!
//! Installed-Size
//!
//! MD5sum
//!
//! MSDOS-Filename
//!
//! Maintainer
//!
//! Origin
//!
//! Package
//!
//! Pre-Depends
//!
//! Priority
//!
//! Provides
//!
//! Recommends
//!
//! Replaces
//!
//! Revision
//!
//! Section
//!
//! Size
//!
//! Source
//!
//! Status
//!
//! Suggests
//!
//! Version
//!

pub mod dpkg_options;
pub mod query_field_package;
pub mod dpkg_list_packages;
pub mod dpkg_list_package_files;
