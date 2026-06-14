# Livediff Glossary

- **WatcherSession**: The domain orchestrator that maintains file caches and coordinates diffs.
- **DiffEngine**: Pure module that computes string differences between old and new file contents.
- **FileSystemPort**: Abstract interface for reading files and retrieving metadata, allowing the domain to be tested without hitting a real disk.
- **FileModification**: A record of a changed file, containing its patch and metadata.
