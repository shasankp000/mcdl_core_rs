# mcdl_core_rs
A Minecraft downloader written in rust,


## Features 

As of this moment in the initial release of the downloader, the downloader only supports ```release``` versions of vanilla mc. More support like downloading ```snapshots```, modloaders like ```fabric```, ```forge``` etc, will be added in updates.

src/main.rs for the manual tests I did

## Documentation

## Configuration updater

```minecraft_downloader_core::main::config_updater::update_configs```

This updates the configuration files for release and snapshot vanilla versions, although I haven't yet coded in the snapshot support.

Usage: 
``` update_configs("your installation directory").expect("Error message!") ``` 

Where installation directory is the directory where .minecraft folder will be generated.

## Game downloader

## jarFile downloader.

```minecraft_downloader_core::main::game_downloader::version_downloader```

This downloads the respective jar file for the given game version 

Usage: 
```version_downloader("1.19.2", "your_installation_directory", "release").expect("Error message")```

Here installation directory will be the same as used in the function above.

## libraries downloader

```minecraft_downloader_core::main::game_downloader::lib_downloader```

Downloads the respective libraries for the given game version

Natives parsing (platform dependent libraries for windows at the moment only are downloaded alongside the platform independent libraries

Usage:
```lib_downloader("1.19.2", "your_installation_directory").expect("Error message")```

Here installation directory will be the same as used in the function above.

## assets downloader

Downloads the respective assets for the given game version

```minecraft_downloader_core::main::game_downloader::assets_downloader```

Usage: 

```assets_downloader("1.19.2", "your_installation_directory").expect("Error message")```

Here installation directory will be the same as used in the function above.

This project uses https://github.com/shasankp000/requests_rs
