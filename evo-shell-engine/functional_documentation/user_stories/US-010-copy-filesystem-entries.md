# US-010 — Copy filesystem entries to a destination

## User Story

As an agent or shell engine consumer,
I want to copy one or more filesystem entries (files or directories) into a destination directory,
so that the files and directory structures are duplicated recursively without altering the active filesystem scope.

## Description

`evo-shell-engine` provides domain capabilities for setting filesystem scope, iterating entries, filtering, projecting, and entering subdirectories.
This user story adds the operational capability to copy single files, multiple files, or entire directory trees into a target directory.

The copy capability operates on paths resolved relative to an active `FilesystemScope`. It is fail-fast, does not mutate the active `FilesystemScope`, rejects self-copying (copying a directory into itself or onto itself), rejects silent overwriting of existing targets, and rejects symlinks.

## Operational Rules

1. **Relative Path Resolution:** Sources and destination path are resolved relative to the given `FilesystemScope`.
2. **File Copying:** Copying a file places a copy into `destination/source_filename`.
3. **Recursive Directory Copying:** Copying a directory recursively creates `destination/source_dirname` and duplicates all nested children and subdirectories using `std::fs`.
4. **Destination Requirements:** The destination path must exist and must be a directory.
5. **No Silent Overwrite:** If a target file or subdirectory already exists inside the destination, a typed error `CopyError::DestinationAlreadyExists` is returned.
6. **Self-Copy Rejection:** Copying a directory onto itself or into a descendant subdirectory (e.g. `copy(docs, docs/sub)`) is rejected with `CopyError::RecursiveSelfCopy`.
7. **Symlinks:** Symlinks are rejected with `CopyError::UnsupportedSourceType`. They are not followed recursively.
8. **Fail-Fast:** Execution stops immediately upon encountering the first error.
9. **Scope Invariance:** The active `FilesystemScope` is never modified by the copy operation.

## Out of Scope

- Move or delete operations.
- Overwriting existing files or directories (no `--overwrite` or `--force`).
- Complex ACL / ownership / extended attribute preservation.
- Transactional rollback on partial failures.
