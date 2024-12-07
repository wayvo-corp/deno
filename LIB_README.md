# Steps to perform after forking deno

Create a new branch from a deno release branch on github web

Cherry pick the commit from the previous lib branch to the new branch

1. Update Cargo.toml

   Remove all members except cli

2. Update cli/Cargo.toml

   Add lib entry and remove the bin entry
   Change the package name from deno to deno_lib

3. Add cli/lib.rs

4. Update cli/worker.rs

   Add fn create_module_loader & create_web_worker_callback with param ExtensionCb
   and add the following code

5. Update cli/lsp/mod.ts

   pub mod client;
   pub mod diagnostics;
   pub mod lsp_custom;
   pub mod testing;

6. Update cli/build.rs

   Comment out all the bin targets

7. Update cli/lib.rs

   Check and add any new libs from main.rs

# Deno

A fork of Deno to allow the cli (specifically the module loader) to be embedded in rust.
