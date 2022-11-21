# mcdl_core_rs
A Minecraft downloader written in rust


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


# Changelog v0.1.2

Added some new functions related to minecraft launcher, which will also be used by my next upcoming crate

## Functions

## extract_natives

Downloads native libraries for the given game version to a given path.
Note: This function assumes that the game has already been installed, i.e, jarFile, libraries and assets have been downloaded
This function can be used my minecraft launcher libraries to extract the natives to a certain path and pass it to the minecraft launching command.

```use minecraft_downloader_core::main::game_downloader::extract_natives;```
 
```extract_natives("1.19.2", "minecraft_installation_directory", "path_to_extract_natives_to", "windows/linux/osx");```

Implemented only for windows at the moment.

## get_logging_arg

Gets the log4j2FilePath argument for the given game version

Note: This function assumes that the game has already been installed, i.e, jarFile, libraries and assets have been downloaded

This function can be used my minecraft launcher libraries to get the log4j2 coniguration file path and pass it to the minecraft launching command.

```use minecraft_downloader_core::main::game_downloader::get_logging_arg;```

```get_logging_arg("1.19.2", "minecraft_installation_path")```

## get_main_class

Gets the mainClass of minecraft for the given game version.

Note: This function assumes that the game has already been installed, i.e, jarFile, libraries and assets have been downloaded.

This function can be used by minecraft launcher libraries to get the mainClass and pass it to the minecraft launching command.

```use minecraft_downloader_core::main::game_downloader::get_main_class;```

```get_main_class("1.19.2", "minecraft_installation_path")```

## get_class_path

Generates the classpath for -cp argument in minecraft command for given version and returns in the form of a string

Note: This function assumes that the game has already been installed, i.e, jarFile, libraries and assets have been downloaded.

This function can be used by minecraft launcher libraries to get the mainClass and pass it to the minecraft launching command.

```use minecraft_downloader_core::main::game_downloader::get_class_path;```

```get_class_path("minecraft_installation_path", "1.19.2")```

This project uses https://github.com/shasankp000/requests_rs (updated to v0.1.5)
