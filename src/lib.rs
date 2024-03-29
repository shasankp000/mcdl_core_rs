extern crate serde_json;
extern crate serde;

#[macro_use]

extern crate serde_derive;


pub mod main {

    pub mod config_updater {
        use std::path::Path;
        use requests_rs::requests;
        use std::env;
        use std::fs;
        use whoami::username;
     
        
        /// 
        /// ### Updates the version configuration.
        /// 
        /// Example
        /// 
        /// ``` 
        /// use minecraft_downloader_core::main::config_updater::update_configs;
        /// 
        /// update_configs("your installation directory").expect("Error message!")
        /// 
        /// ```
        pub fn update_configs(install_dir: &str) -> Result<(), serde_json::Error> {


            #[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
            #[serde(rename_all = "snake_case")]
            enum VersionKind {
                Release,
                Snapshot,
                OldAlpha,
                OldBeta,
            }

            #[derive(Debug, Serialize, Deserialize)]
            struct VersionsManifest {
                latest: Latest,
                versions: Vec<Versions>
            }
            

            #[derive(Debug, Serialize, Deserialize)]
            struct Versions {
                id: String,
                #[serde(rename = "type")]
                kind: VersionKind,
                url: String,
                time: String,
                #[serde(rename = "releaseTime")]
                release_time: String
            }

            #[derive(Debug, Serialize, Deserialize)]
            struct Latest {
                release: String,
                snapshot: String,
            }



            // type JsonMap = Vec<>;

            let os_type = env::consts::OS; // os type

            let mc_dir = format!("{}/.minecraft", install_dir);

            println!("Running initial actions....");

            println!("Detected operating system: {}", os_type);
            

            if os_type == "windows".trim() {


                let sys_username = username();

                let home_dir_str = format!("C:\\Users\\{}", sys_username);

                let version_json_dir = format!("{}/AppData/Roaming/version_jsons", home_dir_str);

                // check if dir exists
                if Path::new(&version_json_dir).exists(){
                    println!("version_jsons dir already exists, ignoring...")
                }
                // if not exists then do this
                else  {
                    fs::create_dir(&version_json_dir).expect("Failed to create dir.");
                    println!("Created {}", &version_json_dir);
                }

                if Path::new(&format!("{}/release", version_json_dir)).exists(){
                    println!("release dir already exists, ignoring...")
                }
                else  {
                    fs::create_dir(format!("{}/release", &version_json_dir)).expect("Failed to create directory!");
                    println!("Created {version_json_dir}/release");
                }

                if Path::new(&format!("{}/snapshot", version_json_dir)).exists() {
                    println!("snapshot dir already exists, ignoring...")
                }
                else  {
                    fs::create_dir(format!("{}/snapshot", &version_json_dir)).expect("Failed to create directory!");
                    println!("Created {version_json_dir}/snapshot");
                }


                
                // check for minecraft directory
                
                if Path::new(&format!("{}/versions", mc_dir)).exists() {
                    println!("versions dir already exists, ignoring...")
                }
                else  {
                    fs::create_dir(format!("{}/versions", mc_dir)).expect("Failed to create dir.");
                    println!("Created {mc_dir}/versions");
                }
    
    
                if Path::new(&format!("{}/libraries", mc_dir)).exists() {
                    println!("libraries dir already exists, ignoring...")
                }
                else  {
                    fs::create_dir(format!("{}/libraries", mc_dir)).expect("Failed to create dir.");
                    println!("Created {mc_dir}/libraries");
                }
                
    
                if Path::new(&format!("{}/assets", mc_dir)).exists() {
                    println!("assets dir already exists, ignoring...")
                }
                else  {
                    fs::create_dir(format!("{}/assets", mc_dir)).expect("Failed to create dir.");
                    println!("Created {mc_dir}/assets");
                }
    
                requests::file_downloader::async_download_file("https://launchermeta.mojang.com/mc/game/version_manifest.json", &version_json_dir).expect("Error fetching data!");
    
                let f1 = fs::read_to_string(format!("{}/version_manifest.json", version_json_dir)).expect("Error opening file!");
    
                //let json_value: serde_json::value::Value = serde_json::from_str(f1.as_str())?;
    
                let versions_manifest: VersionsManifest = serde_json::from_str(f1.as_str())?;
    
                
            
                let snapshots_and_releases= versions_manifest.versions.iter()
                    .filter(|version| matches!(version.kind, VersionKind::Release | VersionKind::Snapshot));
    
    
                for version in snapshots_and_releases {
                    if version.kind == VersionKind::Release {
                        requests::file_downloader::async_download_file(&version.url, &format!("{}/release", version_json_dir)).expect("Error downloading file!");
                    }
    
                    if version.kind == VersionKind::Snapshot {
                        requests::file_downloader::async_download_file(&version.url, &format!("{}/snapshot", version_json_dir)).expect("Error downloading file!");
                    }
                }

            }

            else if os_type == "linux".trim() {
                let sys_username = username();

                let home_dir_str = format!("/home/{}", sys_username);
                
                let version_json_dir = format!("{}/version_jsons", home_dir_str);

                if Path::new(&version_json_dir).exists() {
                    println!("version_jsons dir already exists, ignoring...")
                }
                
                else {
                    fs::create_dir(&version_json_dir).expect("Failed to create dir.");
                    println!("Created {}", &version_json_dir)
                }

                if Path::new(&format!("{}/release", version_json_dir)).exists(){
                    println!("release dir already exists, ignoring...")
                }
                else  {
                    fs::create_dir(format!("{}/release", &version_json_dir)).expect("Failed to create directory!");
                    println!("Created {version_json_dir}/release");
                }

                if Path::new(&format!("{}/snapshot", version_json_dir)).exists() {
                    println!("snapshot dir already exists, ignoring...")
                }
                else  {
                    fs::create_dir(format!("{}/snapshot", &version_json_dir)).expect("Failed to create directory!");
                    println!("Created {version_json_dir}/snapshot");
                }

                if Path::new(&mc_dir).exists() {
                    println!(".minecraft dir already exists, ignoring...")
                }
                else  {
                    fs::create_dir(&mc_dir).expect("Failed to create dir.");
                    println!("Created {}", mc_dir);
                }
    
                // check for minecraft directory
                
                if Path::new(&format!("{}/versions", mc_dir)).exists() {
                    println!("versions dir already exists, ignoring...")
                }
                else  {
                    fs::create_dir(format!("{}/versions", mc_dir)).expect("Failed to create dir.");
                    println!("Created {mc_dir}/versions");
                }
    
    
                if Path::new(&format!("{}/libraries", mc_dir)).exists() {
                    println!("libraries dir already exists, ignoring...")
                }
                else  {
                    fs::create_dir(format!("{}/libraries", mc_dir)).expect("Failed to create dir.");
                    println!("Created {mc_dir}/libraries");
                }
                
    
                if Path::new(&format!("{}/assets", mc_dir)).exists() {
                    println!("assets dir already exists, ignoring...")
                }
                else  {
                    fs::create_dir(format!("{}/assets", mc_dir)).expect("Failed to create dir.");
                    println!("Created {mc_dir}/assets");
                }
    
                requests::file_downloader::async_download_file("https://launchermeta.mojang.com/mc/game/version_manifest.json", &version_json_dir).expect("Error fetching data!");
    
                let f1 = fs::read_to_string(format!("{}/version_manifest.json", version_json_dir)).expect("Error opening file!");
    
                //let json_value: serde_json::value::Value = serde_json::from_str(f1.as_str())?;
    
                let versions_manifest: VersionsManifest = serde_json::from_str(f1.as_str())?;
    
                
            
                let snapshots_and_releases= versions_manifest.versions.iter()
                    .filter(|version| matches!(version.kind, VersionKind::Release | VersionKind::Snapshot));
    
    
                for version in snapshots_and_releases {
                    if version.kind == VersionKind::Release {
                        requests::file_downloader::async_download_file(&version.url, &format!("{}/release", version_json_dir)).expect("Error downloading file!");
                    }
    
                    if version.kind == VersionKind::Snapshot {
                        requests::file_downloader::async_download_file(&version.url, &format!("{}/snapshot", version_json_dir)).expect("Error downloading file!");
                    }
                }


            }

            Ok(())
        }

       
    }

    pub mod game_downloader {
        use std::path::Path;
        use requests_rs::requests;
        use std::env;
        use std::fs::{self};
        use whoami::username;

        use std::collections::HashMap;

        ////
        /// ###  Downloads the respective jar file for the  given game version
        ///
        /// Example: 
        /// 
        /// ```
        /// use minecraft_downloader_core::main::game_downloader::version_downloader;
        /// 
        /// version_downloader("1.19.2", "your_installation_directory", "release").expect("Error message")
        /// ```
        pub fn version_downloader(game_ver: &str, install_dir: &str, r#type: &str) -> Result<(), std::io::Error> {

            let mc_dir = format!("{}/.minecraft", install_dir);
            let os_type = env::consts::OS; // os type

            let sys_username = username();

            let mut home_dir_str = String::new();

            if os_type == "windows".trim() {
                home_dir_str = format!("C:\\Users\\{}", sys_username);
                println!("{}", home_dir_str)
            }

            else if os_type == "linux".trim() {
                home_dir_str = format!("/home/{}", sys_username);
                println!("{}", home_dir_str)
            }

             

            if r#type == "release" {

                if Path::new(&format!("{}/versions/{}", mc_dir, game_ver)).exists() {
                    println!("{} dir already exists, ignoring...", game_ver)
                }
                else {
                    fs::create_dir(format!("{}/versions/{}", mc_dir, game_ver)).expect("Failed to create dir.");
                    println!("Created {mc_dir}/versions/{game_ver}");
                }


                if os_type == "windows".trim() {

                    fs::copy(format!("{}/AppData/Roaming/version_jsons/release/{}.json", home_dir_str, game_ver), format!("{}/versions/{}/{}.json", mc_dir, game_ver, game_ver)).expect("Error copying file!");
                    println!("Copied {}.json", game_ver);

                    let a = fs::read_to_string(format!("{}/AppData/Roaming/version_jsons/release/{}.json", home_dir_str, game_ver)).expect("Error reading file!");

                    let json_value: serde_json::value::Value = serde_json::from_str(&a).expect("Error parsing!");

                    let map1 = json_value["downloads"]["client"].as_object().unwrap();


                    let jar_url = map1.get("url").unwrap().as_str().unwrap();



                    println!("{}", jar_url);


                    requests::file_downloader::async_download_file(jar_url, &format!("{}/versions/{}", mc_dir, game_ver)).expect("Error downloading file");
                    //requests::file_downloader::async_download_file(&jar_mappings_url, &format!("{}/versions/{}", mc_dir, game_ver)).expect("Error downloading file");
                    fs::rename(format!("{}/versions/{}/client.jar", mc_dir, game_ver), format!("{}/versions/{}/{}.jar", mc_dir, game_ver, game_ver)).expect("Error renaming file!");
                   
                    println!("Renamed client.jar => {}.jar", game_ver);


                }

                else if os_type == "linux".trim() {

                    fs::copy(format!("{}/version_jsons/release/{}.json", home_dir_str, game_ver), format!("{}/versions/{}/{}.json", mc_dir, game_ver, game_ver)).expect("Error copying file!");
                    println!("Copied {}.json", game_ver);

                    let a = fs::read_to_string(format!("{}/version_jsons/release/{}.json", home_dir_str, game_ver)).expect("Error reading file!");

                    let json_value: serde_json::value::Value = serde_json::from_str(&a).expect("Error parsing!");

                    let map1 = json_value["downloads"]["client"].as_object().unwrap();


                    let jar_url = map1.get("url").unwrap().as_str().unwrap();



                    println!("{}", jar_url);


                    requests::file_downloader::async_download_file(jar_url, &format!("{}/versions/{}", mc_dir, game_ver)).expect("Error downloading file");
                    
                    fs::rename(format!("{}/versions/{}/client.jar", mc_dir, game_ver), format!("{}/versions/{}/{}.jar", mc_dir, game_ver, game_ver)).expect("Error renaming file!");
                    
                    println!("Renamed client.jar => {}.jar", game_ver);

                }
                

            }

            else if r#type == "snapshot" {
                println!("Snapshot support not yet added!")
            }

            Ok(())
        }


        #[derive(Debug, Serialize, Deserialize)]
        pub struct Res {
                libraries: Vec<Libraries>,
        }
            
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Libraries {
                downloads: Downloads,
                name: String,
                #[serde(default)]
                rules: Option<Vec<Rules>>
        }
            
        #[derive(Debug, Serialize, Deserialize)]
        struct Rules {
            action: String,
            os: Os
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Os {
            name: String
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Downloads {
            artifact: Artifact,
        }
            
        #[derive(Debug, Serialize, Deserialize)]
        struct Artifact {
            path: String,
            sha1: String,
            size: i32,
            url: String
        }

        ////
        /// ###  Downloads the respective libraries for the given game version
        ///
        /// Example: 
        /// 
        /// ```
        /// use minecraft_downloader_core::main::game_downloader::lib_downloader;
        /// 
        /// lib_downloader("1.19.2", "your_installation_directory").expect("Error message")
        /// ```
        pub fn lib_downloader(game_ver: &str, install_dir: &str) -> Result<(), std::io::Error> {

            

            let mc_dir = format!("{}/.minecraft", install_dir);


            let libs_info = fs::read_to_string(format!("{}/versions/{}/{}.json", mc_dir, game_ver, game_ver)).expect("Error reading file!");

            let libs_json: Res = serde_json::from_str(&libs_info).expect("Error parsing!");

            
            if cfg!(target_os = "windows") {
                // downloading windows only libs

                let libs_iter1 = libs_json.libraries.iter()
                    .filter(|data| data.downloads.artifact.url.contains("lwjgl") && data.downloads.artifact.url.contains("windows") || data.downloads.artifact.url.contains("text2speech-1.13.9-natives-windows"));


                //println!("Windows only libraries.");

                let mut lib_url_array: Vec<&String> = Vec::new();
                let mut dirpath_array: Vec<String> = Vec::new();

                let cwd = install_dir;

                for lib in libs_iter1 {
                    // println!("{}", lib.downloads.artifact.url)
                    let path1 = Path::new(&lib.downloads.artifact.path);

                    // println!("{}", path1.parent().unwrap().to_string_lossy())
                    lib_url_array.push(&lib.downloads.artifact.url);
                    dirpath_array.push(path1.parent().unwrap().to_string_lossy().to_string());

                }


                // filter out the rest of the platform independent libs.

                let libs_iter2 = libs_json.libraries.iter()
                     .filter(|data1| data1.downloads.artifact.url.contains("logging") || data1.downloads.artifact.url.contains("blocklist") || data1.downloads.artifact.url.contains("patchy") || data1.downloads.artifact.url.contains("oshi") || data1.downloads.artifact.url.contains("jna") || data1.downloads.artifact.url.contains("slf4j") || data1.downloads.artifact.url.contains("ibm") || data1.downloads.artifact.url.contains("javabridge") && data1.downloads.artifact.url.contains("jopt-simple") || data1.downloads.artifact.url.contains("guava") || data1.downloads.artifact.url.contains("commons") || data1.downloads.artifact.url.contains("commons-io") || data1.downloads.artifact.url.contains("brigadier") || data1.downloads.artifact.url.contains("datafixerupper") || data1.downloads.artifact.url.contains("gson") || data1.downloads.artifact.url.contains("authlib") || data1.downloads.artifact.url.contains("httpcomponents") || data1.downloads.artifact.url.contains("commons-logging") || data1.downloads.artifact.url.contains("fastutil") || data1.downloads.artifact.url.contains("text2speech-1.13.9.jar") || data1.downloads.artifact.url.contains("io/netty"));


                //println!("Platform independent libraries.");

                for lib2 in libs_iter2 {
                    // println!("{}", lib2.downloads.artifact.url)

                    let path2 = Path::new(&lib2.downloads.artifact.path);

                    lib_url_array.push(&lib2.downloads.artifact.url);
                    dirpath_array.push(path2.parent().unwrap().to_string_lossy().to_string());

                    // println!("{}", path2.parent().unwrap().to_string_lossy())
                }

                
                env::set_current_dir(format!("{}/libraries", mc_dir)).expect("Error changing directories");

                for dir in dirpath_array.iter() {
                    

                    if Path::new(&format!("{}", dir)).exists() {
                        println!("{} dir already exists, ignoring...", dir)
                    }
                    else {
                       fs::create_dir_all(format!("{}", dir)).expect("Failed to create dir."); // recursively creates all directories in a path.
                       println!("Created {mc_dir}/libraries/{dir}");
                    }
   
                }
                

                for (url, dir1) in lib_url_array.iter().zip(dirpath_array.iter()) {
                    //println!("{} => {}", url, dir1)

                    requests::file_downloader::async_download_file(url, dir1).expect("Error downloading libraries!");
                }
                
                env::set_current_dir(cwd).expect("Error changing directories!");

                println!("All libraries have been downloaded successfully");
            }


            else if cfg!(target_os = "linux") {

                // download linux only libraries

                let libs_iter1 = libs_json.libraries.iter()
                    .filter(|data| data.downloads.artifact.url.contains("lwjgl") && data.downloads.artifact.url.contains("linux") || data.downloads.artifact.url.contains("text2speech-1.13.9-natives-linux"));


                //println!("Windows only libraries.");

                let mut lib_url_array: Vec<&String> = Vec::new();
                let mut dirpath_array: Vec<String> = Vec::new();

                let cwd = install_dir;

                for lib in libs_iter1 {
                    // println!("{}", lib.downloads.artifact.url)
                    let path1 = Path::new(&lib.downloads.artifact.path);

                    // println!("{}", path1.parent().unwrap().to_string_lossy())
                    lib_url_array.push(&lib.downloads.artifact.url);
                    dirpath_array.push(path1.parent().unwrap().to_string_lossy().to_string());

                }


                // filter out the rest of the platform independent libs.

                let libs_iter2 = libs_json.libraries.iter()
                     .filter(|data1| data1.downloads.artifact.url.contains("logging") || data1.downloads.artifact.url.contains("blocklist") || data1.downloads.artifact.url.contains("patchy") || data1.downloads.artifact.url.contains("oshi") || data1.downloads.artifact.url.contains("jna") || data1.downloads.artifact.url.contains("slf4j") || data1.downloads.artifact.url.contains("ibm") || data1.downloads.artifact.url.contains("javabridge") && data1.downloads.artifact.url.contains("jopt-simple") || data1.downloads.artifact.url.contains("guava") || data1.downloads.artifact.url.contains("commons") || data1.downloads.artifact.url.contains("commons-io") || data1.downloads.artifact.url.contains("brigadier") || data1.downloads.artifact.url.contains("datafixerupper") || data1.downloads.artifact.url.contains("gson") || data1.downloads.artifact.url.contains("authlib") || data1.downloads.artifact.url.contains("httpcomponents") || data1.downloads.artifact.url.contains("commons-logging") || data1.downloads.artifact.url.contains("fastutil") || data1.downloads.artifact.url.contains("text2speech-1.13.9.jar") || data1.downloads.artifact.url.contains("io/netty"));


                //println!("Platform independent libraries.");

                for lib2 in libs_iter2 {
                    // println!("{}", lib2.downloads.artifact.url)

                    let path2 = Path::new(&lib2.downloads.artifact.path);

                    lib_url_array.push(&lib2.downloads.artifact.url);
                    dirpath_array.push(path2.parent().unwrap().to_string_lossy().to_string());

                    // println!("{}", path2.parent().unwrap().to_string_lossy())
                }

                
                env::set_current_dir(format!("{}/libraries", mc_dir)).expect("Error changing directories");

                for dir in dirpath_array.iter() {
                    

                    if Path::new(&format!("{}", dir)).exists() {
                        println!("{} dir already exists, ignoring...", dir)
                    }
                    else {
                       fs::create_dir_all(format!("{}", dir)).expect("Failed to create dir."); // recursively creates all directories in a path.
                       println!("Created {mc_dir}/libraries/{dir}");
                    }
   
                }
                

                for (url, dir1) in lib_url_array.iter().zip(dirpath_array.iter()) {
                    //println!("{} => {}", url, dir1)

                    requests::file_downloader::async_download_file(url, dir1).expect("Error downloading libraries!");
                }
                
                env::set_current_dir(cwd).expect("Error changing directories!");

                println!("All libraries have been downloaded successfully");
            }
            
            Ok(())
        }


        ////
        /// ###  Downloads the respective assets for the given game version
        ///
        /// Example: 
        /// 
        /// ```
        /// use minecraft_downloader_core::main::game_downloader::assets_downloader;
        /// 
        /// assets_downloader("1.19.2", "your_installation_directory").expect("Error message")
        /// ```
        pub fn assets_downloader(game_ver: &str, install_dir: &str) -> Result<(), std::io::Error> {

            
            #[derive(Debug, Serialize, Deserialize)]
            struct Data {
                hash: String, 
                size: i32
            }

            #[derive(Debug, Serialize, Deserialize)]
            struct Objects {
                objects: HashMap<String, Data>
            }
            
            


            // Download the assets

            let mc_dir = format!("{}/.minecraft", install_dir);
            //let os_type = env::consts::OS; // os type

            let assets_info = fs::read_to_string(format!("{}/versions/{}/{}.json", mc_dir, game_ver, game_ver)).expect("Error reading file!");

            let assets_json: serde_json::value::Value = serde_json::from_str(&assets_info).expect("Error parsing!");

            let assets_map1 = assets_json["assetIndex"].as_object().unwrap();

            let assets_map2 = assets_json["logging"]["client"]["file"].as_object().unwrap();

            let assets_index_url = assets_map1.get_key_value("url").unwrap().1.as_str().unwrap();

            let logging_configs_url = assets_map2.get_key_value("url").unwrap().1.as_str().unwrap();
            //println!("{}", assets_index_url);

            if Path::new(&format!("{}/assets/indexes", mc_dir)).exists() {
                println!("indexes dir already exists, ignoring...")
            }
            else {
                fs::create_dir(format!("{}/assets/indexes", mc_dir)).expect("Failed to create dir.");
                println!("Created {mc_dir}/assets/indexes");
            }

            if Path::new(&format!("{}/assets/objects", mc_dir)).exists() {
                println!("objects dir already exists, ignoring...")
            }
            else  {
                fs::create_dir(format!("{}/assets/objects", mc_dir)).expect("Failed to create dir.");
                println!("Created {mc_dir}/assets/objects");
            }


            if Path::new(&format!("{}/assets/log_configs", mc_dir)).exists() {
                println!("log_configs dir already exists, ignoring...")
            }
            else  {
                fs::create_dir(format!("{}/assets/log_configs", mc_dir)).expect("Failed to create dir.");
                println!("Created {mc_dir}/assets/log_configs");
            }

            requests::file_downloader::async_download_file(assets_index_url, &format!("{}/assets/indexes", mc_dir)).expect("Error downloading file!");

            requests::file_downloader::async_download_file(logging_configs_url, &format!("{}/assets/log_configs", mc_dir)).expect("Error downloading file!");

            let sliced_str = &game_ver[0..4];

            let objects_info  = fs::read_to_string(format!("{}/assets/indexes/{}.json", mc_dir, sliced_str)).expect("Error reading file!");

            let parsed_objects: Objects = serde_json::from_str(&objects_info).expect("Error parsing!");


            println!("Constructing urls...");

            let mut assets_url_array: Vec<String> = Vec::new();
            let mut assets_name_array: Vec<String> = Vec::new();
            let mut sliced_name_array: Vec<String> = Vec::new();

            for val in parsed_objects.objects.values() {
                // println!("{}", val.hash)

                let each_hash = &val.hash;
                
                assets_url_array.push(format!("https://resources.download.minecraft.net/{}/{}", &each_hash[0..2], each_hash));
                assets_name_array.push(each_hash.to_string());
                sliced_name_array.push(format!("{}", &each_hash[0..2]));

            }

            println!("Creating directories....");

            for dir_name in sliced_name_array.iter() {
                
                if Path::new(&format!("{mc_dir}/assets/objects/{dir_name}")).exists() {
                    println!("{dir_name} dir already exists, ignoring...")
                }
                else {
                   fs::create_dir(format!("{mc_dir}/assets/objects/{dir_name}")).expect("Failed to create dir.");
                   println!("Created {mc_dir}/assets/objects/{dir_name}");
                }

            }

        
            println!("Downloading assets...");

            for (urls, dir1) in assets_url_array.iter().zip(sliced_name_array.iter()) {
                
                requests::file_downloader::async_download_file(urls, &format!("{}/assets/objects/{}", mc_dir, dir1)).expect("Error downloading libraries!");

            }

            println!("All assets have been downloaded successfully");

            Ok(())
        }


        // minecraft launcher related functions

        /// Downloads native libraries for the given game version to a given path.
        /// 
        /// Note: This function assumes that the game has already been installed, i.e, jarFile, libraries and assets have been downloaded
        /// 
        /// This function can be used my minecraft launcher libraries to extract the natives to a certain path and pass it to the minecraft launching command.
        /// 
        /// ```
        /// use minecraft_downloader_core::main::game_downloader::extract_natives;
        /// 
        /// extract_natives("1.19.2", "minecraft_installation_directory", "path_to_extract_natives_to", "windows/linux/osx");
        /// 
        /// ```
        pub fn extract_natives(game_ver:&str, install_dir: &str, platform:&str) {

            println!("Checking for natives directory....");
            
            let mc_dir = format!("{}/.minecraft", install_dir);

            let libs_info = fs::read_to_string(format!("{}/versions/{}/{}.json", mc_dir, game_ver, game_ver)).expect("Error reading file!");

            let libs_json: Res = serde_json::from_str(&libs_info).expect("Error parsing!");


            if cfg!(target_os = "windows") && platform == "windows".trim() {

                let libs_iter1 = libs_json.libraries.iter()
                    .filter(|data| data.downloads.artifact.url.contains("lwjgl") && data.downloads.artifact.url.contains("windows") || data.downloads.artifact.url.contains("text2speech-1.13.9-natives-windows"));


                println!("Windows only libraries.");

                let mut lib_url_array: Vec<&String> = Vec::new();
                let mut dirpath_array: Vec<String> = Vec::new();


                for lib in libs_iter1 {
                    let path1 = Path::new(&lib.downloads.artifact.path);

                    lib_url_array.push(&lib.downloads.artifact.url);
                    dirpath_array.push(path1.parent().unwrap().to_string_lossy().to_string());

                }

                for dir in dirpath_array.iter() {
                    
                    if Path::new(&format!("{mc_dir}\\versions\\{game_ver}\\natives\\{dir}")).exists() {
                        println!("{} dir already exists, ignoring...", dir)
                    }
                    else {
                       fs::create_dir_all(format!("{mc_dir}\\versions\\{game_ver}\\natives\\{dir}")).expect("Failed to create dir."); // recursively creates all directories in a path.
                       println!("Created {mc_dir}\\natives\\{dir}");
                    }
   
                }

                for (url, dir1) in lib_url_array.iter().zip(dirpath_array.iter()) {

                    requests::file_downloader::async_download_file(url, &format!("{mc_dir}\\versions\\{game_ver}\\natives\\{dir1}")).expect("Error downloading libraries!");
                }
                
                println!("All natives have been extracted successfully.");


            }

            else if cfg!(target_os = "linux") && platform == "linux".trim() {
                let libs_iter1 = libs_json.libraries.iter()
                    .filter(|data| data.downloads.artifact.url.contains("lwjgl") && data.downloads.artifact.url.contains("linux") || data.downloads.artifact.url.contains("text2speech-1.13.9-natives-linux"));


                println!("Linux only libraries.");

                let mut lib_url_array: Vec<&String> = Vec::new();
                let mut dirpath_array: Vec<String> = Vec::new();


                for lib in libs_iter1 {
                    let path1 = Path::new(&lib.downloads.artifact.path);

                    lib_url_array.push(&lib.downloads.artifact.url);
                    dirpath_array.push(path1.parent().unwrap().to_string_lossy().to_string());

                }

                for dir in dirpath_array.iter() {

                    
                    if Path::new(&format!("{mc_dir}/versions/{game_ver}/natives/{dir}")).exists() {
                        println!("{} dir already exists, ignoring...", dir)
                    }
                    else {
                        fs::create_dir_all(format!("{mc_dir}/versions/{game_ver}/natives/{dir}")).expect("Failed to create dir."); // recursively creates all directories in a path.
                        println!("Created {mc_dir}/natives/{dir}");
                    }
   
                }

                for (url, dir1) in lib_url_array.iter().zip(dirpath_array.iter()) {

                    requests::file_downloader::async_download_file(url, &format!("{mc_dir}/versions/{game_ver}/natives/{dir1}")).expect("Error downloading libraries!");
                }
                
                println!("All natives have been extracted successfully.");
            }

        }

        /// Gets the log4j2FilePath argument for the given game version
        /// 
        /// Note: This function assumes that the game has already been installed, i.e, jarFile, libraries and assets have been downloaded
        /// 
        /// This function can be used my minecraft launcher libraries to get the log4j2 coniguration file path and pass it to the minecraft launching command.
        /// 
        /// ```
        /// use minecraft_downloader_core::main::game_downloader::get_logging_arg;
        /// 
        /// get_logging_arg("1.19.2", "minecraft_installation_path")
        /// ```
        pub fn get_logging_arg(game_ver:&str, install_dir: &str) -> String {

            #[derive(Debug,Serialize,Deserialize)]
            struct GameFiles {
                logging: Logging
            }

            #[derive(Debug,Serialize,Deserialize)]
            struct Logging {
                client: Client
            }

            #[derive(Debug,Serialize,Deserialize)]
            struct Client {
                argument: String,
                file: File,
                r#type: String
            }

            #[derive(Debug,Serialize,Deserialize)]
            struct File {
                id: String,
                sha1: String,
                size: i32,
                url: String
            }


            
            let mut mc_dir = String::new();

            if cfg!(target_os = "windows") {
                mc_dir = format!("{}\\.minecraft", install_dir);
            }

            else if cfg!(target_os = "linux") {
                mc_dir = format!("{}/.minecraft", install_dir);
            }

            let game_info = fs::read_to_string(format!("{}/versions/{}/{}.json", mc_dir, game_ver, game_ver)).expect("Error reading file!");

            let game_files: GameFiles = serde_json::from_str(&game_info).expect("Error reading from string!");

            let arg = game_files.logging.client.argument[0..25].to_string();

            let xml_id = game_files.logging.client.file.id;

            let mut constructed_argument = String::new();

            if cfg!(target_os = "windows") {
                constructed_argument = format!("{arg}={install_dir}\\.minecraft\\assets\\log_configs\\{xml_id}");
            }

            else if cfg!(target_os = "linux") {
                constructed_argument = format!("{arg}={install_dir}/.minecraft/assets/log_configs/{xml_id}");
            }

            constructed_argument

        }


        /// Gets the mainClass of minecraft for the given game version
        /// 
        /// Note: This function assumes that the game has already been installed, i.e, jarFile, libraries and assets have been downloaded.
        /// 
        /// This function can be used by minecraft launcher libraries to get the mainClass and pass it to the minecraft launching command.
        /// 
        /// ```
        /// use minecraft_downloader_core::main::game_downloader::get_main_class;
        /// 
        /// get_main_class("1.19.2", "minecraft_installation_path")
        /// ```
        pub fn get_main_class(game_ver: &str, install_dir: &str) -> String {
            #[derive(Debug,Serialize,Deserialize)]
            #[serde(rename_all="camelCase")]
            struct GameFiles {
                main_class: String,
                minimum_launcher_version: i32,
                release_time: String,
                time: String,
                r#type: String
            }

            let mut mc_dir = String::new();

            if cfg!(target_os = "windows") {
                mc_dir = format!("{}\\.minecraft", install_dir);
            }

            else if cfg!(target_os = "linux") {
                mc_dir = format!("{}/.minecraft", install_dir);
            }

            let game_info = fs::read_to_string(format!("{}/versions/{}/{}.json", mc_dir, game_ver, game_ver)).expect("Error reading file!");

            let game_files: GameFiles = serde_json::from_str(&game_info).expect("Error reading from string!");

            let main_class = game_files.main_class;

            main_class
        }
        
        /// Generates the classpath string for -cp argument in minecraft jvm comamnd for the given game version
        /// 
        /// Note: This function assumes that the game has already been installed, i.e, jarFile, libraries and assets have been downloaded.
        /// 
        /// This function can be used by minecraft launcher libraries to get the classpath and pass it to the minecraft launching command.
        /// 
        /// ```
        /// use minecraft_downloader_core::main::game_downloader::get_class_path;
        /// 
        /// get_class_path("minecraft_installation_path", "1.19.2")
        /// ```
        pub fn get_class_path(install_dir: &str, game_ver: &str) -> String{

            let mut mc_dir = String::new();

            if cfg!(target_os = "windows") {
                mc_dir = format!("{}\\.minecraft", install_dir);
            }

            else if cfg!(target_os = "linux") {
                mc_dir = format!("{}/.minecraft", install_dir);
            }

            let libs_info = fs::read_to_string(format!("{}/versions/{}/{}.json", mc_dir, game_ver, game_ver)).expect("Error reading file!");

            let libs_json: Res = serde_json::from_str(&libs_info).expect("Error parsing!");

            let mut classpath = String::new();

            // Get the platform independent libs.

            let independent_str = libs_json.libraries.iter()
                .filter(|data| !data.downloads.artifact.path.contains("natives-windows") && !data.downloads.artifact.path.contains("natives-linux") && !data.downloads.artifact.path.contains("natives-windows") && !data.downloads.artifact.path.contains("natives-macos") && !data.downloads.artifact.path.contains("natives-macos-arm64") && !data.downloads.artifact.path.contains("unix") && !data.downloads.artifact.path.contains("linux-x86_64") && !data.downloads.artifact.path.contains("linux-aarch_64") && !data.downloads.artifact.path.contains("java-objc-bridge"));

            for independent_paths in independent_str {
                // println!("{}", independent_paths.downloads.artifact.path)
                classpath+=&format!("{}\\libraries\\{};", mc_dir , independent_paths.downloads.artifact.path)
            };

            
            //Get the platform dependent libs

            if cfg!(target_os = "windows") {
                let native_str = libs_json.libraries.iter()
                .filter(|data|data.downloads.artifact.path.contains("natives-windows"));

                for native_paths in native_str {
                    classpath+=&format!("{}\\libraries\\{};", mc_dir ,native_paths.downloads.artifact.path)
                }
            }

            else if cfg!(target_os = "linux") {
                let native_str = libs_json.libraries.iter()
                .filter(|data|data.downloads.artifact.path.contains("natives-linux"));

                for native_paths in native_str {
                    classpath+=&format!("{}\\libraries\\{};", mc_dir ,native_paths.downloads.artifact.path)
                }
            }
        
            classpath
            

        }

    }
}