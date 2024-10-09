# bauxite-store

> Bauxite store is a library inspired by [PNPM](https://pnpm.io) to manage Minecraft files (mods, modloaders, etc) in a more efficient way.

## Specifications

### File Storage and Management

Bauxite Store employs an efficient method for managing Minecraft files by leveraging a store directory and symbolic links. Here's how it works:

1. **Hash-Based Storage**: Files are stored in a designated store directory using their hash values. This ensures that each file is uniquely identified and prevents duplication.

   > **Example**: If a mod file has a hash of `abc123`, it will be stored in the store directory as `<store>/files/ab/abc123`.

2. **Symbolic Linking**: Instead of copying files directly into the game folder, Bauxite Store creates symbolic links. These links point to the actual files in the store directory, saving disk space and reducing redundancy.

   > **Example**: A symbolic link in the game folder might look like `<game folder>/mods/mod1.jar -> <store>/files/ab/abc123`.

3. **Reference Tracking**: The store maintains a reference list of all locations where each file is linked. This is particularly useful for cleaning up the store, as it allows the system to identify and remove files that are no longer in use.

   > **Example**: If `abc123` is linked in multiple locations, the store will keep track of all these references. When a file is no longer needed, the store can safely remove it without affecting other links.

4. **Files tracking**: The store keeps track of all files that are stored in it. This is useful for checking if a file is already stored in the store or not. This is done by storing the path of the file as the key, and both the hash, the size, the permissions and the version of the file as the value.

   > **Example**: If a file is stored in the store, the store will keep track of the file like this:
   >
   > ```json
   > {
   >   "mods/mod1.jar@1.0.0": {
   >     "hash": "abc123",
   >     "size": 123456,
   >     "permissions": 640
   >   }
   > }
   > ```

## Inspirations

- **[PNPM](https://pnpm.io)**: PNPM is a package manager for JavaScript that uses hard links and symlinks to save disk space and reduce redundancy. Bauxite Store is inspired by PNPM's approach to managing files efficiently.
- **[Symbolic Links](https://en.wikipedia.org/wiki/Symbolic_link)**: Symbolic links are a powerful feature of Unix-like operating systems that allow files to be referenced by multiple paths. Bauxite Store leverages symbolic links to create a lightweight and efficient file management system.
- **[Ideas for clever uses of symbolic links](https://www.planetminecraft.com/forums/pmc/discussion/ideas-for-clever-uses-of-symbolic-links-623052/)**: Symbolic links have a wide range of applications beyond file management. Here is an example of how symbolic links can be used in Minecraft modding.
