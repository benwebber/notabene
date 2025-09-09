# notabene

A fast linter for changelogs in the [Keep a Changelog] format.

**notabene** provides the `nb` binary.

```
cargo install --features=cli notabene
```

## Usage

```
nb check [FILE]
```

By default, `nb` tries to read `CHANGELOG.md` in the current directory.

## Rules

### E001

The title is missing.

### E002

The title is not plain text.

### E003

There is a duplicate `h1` in the document.

### E004

The `h2` is not a valid unreleased or release section heading.

### E100

The document does not have an unreleased section.

### E102

There is more than one unreleased section heading in the document.

### E201

The date is not in ISO 8601 format.

### E202

The yanked token does not match `[YANKED]`.

### E300

The change section heading is not a known change type.

### E301

There is more than one change section with the same change type.

### E400

A section is unexpectedly empty (e.g. a release with no changes).

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
