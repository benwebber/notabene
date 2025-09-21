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

There is a duplicate `h1` in the document.

### E003

The document does not have an unreleased section.

### E004

There is more than one unreleased section heading in the document.

### E005

The unreleased section is not the first section in the document.

### E100

The title is not plain text.

### E101

The `h2` is not a valid unreleased or release section heading.

### E102

A section is unexpectedly empty (e.g. a release with no changes).

### E103

The change section heading is not a known change type.

### E104

There is more than one change section with the same change type.

### E200

The release is not in reverse chronological order.

### E201

There is more than one release for this version in the document.

### E202

The release is missing a date

### E203

The date is not in ISO 8601 format.

### E204

The yanked token does not match `[YANKED]`.

### E300

The target reference does not exist.

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
