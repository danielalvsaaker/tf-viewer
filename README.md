# tf-viewer
Selfhosted web dashboard for activity tracking and analyzing. Visualize your activities and view your statistics.  
Easy to setup, with minimal interaction.


#### Features
- Multi-user support
- User and gear statistics
- Supports FIT file type (will support TCX and GPX in the future)
- Embedded database, works out of the box
- Easy to setup, deploy and maintain
- Fast, can run on Raspberry Pi (tested on 3A+)

#### Docker setup
- Create a folder for persisting data
- `docker pull danielalvsaaker/tf-viewer:latest`
- `docker run -d -e TZ=<your timezone> -v <persisting data folder>:/data -p 8080:8080 danielalvsaaker/tf-viewer`
- Go to http://localhost:8080 in your browser
- Check out  the file `example-config.toml` in this repo, change and move the file to `<persisting data folder>/config.toml` if necessary

#### Setup from source
- Make sure Rust is installed (see https://rustup.rs/)
- `git clone https://github.com/danielalvsaaker/tf-viewer && cd tf-viewer`
- `cargo install --release`
- If your $PATH was set by the Rust installer, you should be able to run your binary as `tf-viewer` in your shell

#### Built on
- [sled](https://github.com/spacejam/sled): High-performance embedded key-value database
- [actix-web](https://actix.rs): Fast async web framework

#### Note
The database may require migration after changes to data structures or to the underlying [on-disk-format](https://github.com/spacejam/sled#known-issues-warnings).
Migration scripts will in such case be provided.
