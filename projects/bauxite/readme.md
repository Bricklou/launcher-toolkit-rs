# bauxite

> Bauxite is a "small" (kinda) rust library to download, install and launch Minecraft with others utilities (modloaders, mods, etc).

## Specificities

In opposite to some already existing libraries, some radical choices
have been made:

- **Symlink first**: to keep the disk usage as low as possible, the library will try to symlink files instead of copying them.
  The idea will be to retrieves files directly from the `.minecraft` folder, and symlink them in the output folder.
  The same goes for mods, modloaders, etc. This idea came from [PNPM](https://pnpm.io/), a package manager for NodeJS that uses symlinks to
  avoid files duplication and save disk space.
- **Modular**: it should be easy to add new features, like modloaders, mods, etc. The library should be able to download and install them
  without any problem.
- **Hooks**: often, libraries only download and install files, but many servers require more advanced setups like anti-cheat check. This library
  should be able to run some hooks during installation and before launching the game, to ensure if everything is okay.
- **No GUI**: the library should be usable in a CLI environment, and should not require any GUI to work. This is why the library will not
