# tf-viewer
Selfhosted web dashboard for activity tracking and analyzing. Visualize your activities and view your statistics.  
Easy to setup, with minimal interaction.


#### Features
- Multi-user support
- User and gear statistics
- Supports FIT file type (will support TCX and GPX in the future)
- Embedded database, works out of the box
- Easy to setup, deploy and maintain
- Fast

#### Built on
- [sled](https://github.com/spacejam/sled): High-performance embedded key-value database
- [actix-web](https://actix.rs): Fast async web framework

#### Note
The database may require migration after changes to data structures or to the underlying [on-disk-format](https://github.com/spacejam/sled#known-issues-warnings).
Migration scripts will in such case be provided.
