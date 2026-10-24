# Open Lab Data Format (OLDF) Specification

**OLDF** is an open, portable container format for laboratory data. One `.oldf` file can hold structures, images, spectra, sequences, tables, the way you chose to look at them, and a record of where every piece came from.

> **Status:** v0.1, experimental draft. Anything can change before 1.0. The container layout and the manifest below are the first working version, not a finished standard.

## Quick map

| I want to… | Go to |
|---|---|
| Understand the goal and the approach | [What I want to achieve for this v0.1](#what-i-want-to-achieve-for-this-v01), [How I will work](#how-i-will-work) |
| See what an OLDF file looks like inside | [File structure](#file-structure) |
| Know what kinds of data fit in | [What can go inside](#what-can-go-inside) |
| Write or read a manifest | [Manifest](#manifest), [Datasets](#datasets) |
| Tell measured data from computed data | [Roles: raw, derived and simulated](#roles-raw-derived-and-simulated) |
| Implement a writer or a reader | [Writing rules](#writing-rules), [Reader requirements](#reader-requirements) |
| Check safety concerns | [Security considerations](#security-considerations) |
| Look at a complete example | [Example](#example) |

## Table of contents

- [What I want to achieve for this v0.1](#what-i-want-to-achieve-for-this-v01)
- [How I will work](#how-i-will-work)
- [File structure](#file-structure)
- [What can go inside](#what-can-go-inside)
- [Manifest](#manifest)
- [Datasets](#datasets)
- [Roles: raw, derived and simulated](#roles-raw-derived-and-simulated)
- [Profiles](#profiles)
- [Writing rules](#writing-rules)
- [Reader requirements](#reader-requirements)
- [Security considerations](#security-considerations)
- [Versioning and compatibility](#versioning-and-compatibility)
- [Out of scope for v0.1](#out-of-scope-for-v01)
- [Tooling](#tooling)
- [Example](#example)
- [Naming conventions](#naming-conventions)

---

## What I want to achieve for this v0.1

Lab data usually lives in many places at once: an instrument file in a vendor format, an image in a folder, a spreadsheet of results, a structure downloaded from a database, and a notebook that says how it all fits together. When someone shares the work, half of that gets lost.

For v0.1 I want a small, solid base:

1. **One file for one piece of work.** Data of different kinds travel together with the information needed to understand them.
2. **Self-describing.** The first bytes identify the format, and a manifest lists everything inside, so a program knows what it has before reading any data.
3. **Honest about origin.** Measured data and computed data are marked as such, and computed data records the tool, version and parameters that produced it.
4. **Verifiable.** Every file has a hash, so corruption is detected instead of silently accepted.
5. **Easy to open without special tools.** An OLDF file is a ZIP archive with a manifest. Any ZIP tool can inspect it.
6. **Small enough to implement.** One person should be able to write a reader in a few days.

Not goals for v0.1: replacing existing formats such as PDB, mmCIF, DICOM or mzML. OLDF does not redefine them. It holds them, alongside everything else.

## How I will work

- **Specification first, in small steps.** I write down only what the first implementation needs, and I grow the spec from real use rather than from guesses.
- **A real client validates the design.** The reference library, the command line tool and a molecule viewer are built against this document, and when the document and the code disagree, I fix the one that is wrong.
- **Open by default.** Specification, reference implementation and examples are public, with permissive licenses, so anyone can implement OLDF without asking.
- **Compatibility is a promise.** From 1.0, files written by an older version stay readable. Before 1.0, I say clearly what may change.
- **No lock-in.** Original data is always kept inside the file, so everything computed can be recomputed.
- **Security is part of the design**, not a later patch (see [Security considerations](#security-considerations)).

---

## File structure

An OLDF file is a **ZIP archive** with the extension `.oldf`.

```
experiment.oldf
├── mimetype              identifies the format (first entry, stored uncompressed)
├── manifest.json         index and metadata of the whole file
├── data/                 the data itself, raw and derived
│   ├── structure.pdb
│   ├── structure_bonds.csv
│   ├── gel_001.png
│   └── gel_001_bands.csv
├── scenes/               how to display the data (camera, styles, selections)
│   └── default.json
├── provenance/           optional: extra records about origin and processing
└── notes/                optional: protocols, comments, free text
```

What each part is for:

| Entry | Required | Purpose |
|---|---|---|
| `mimetype` | yes | Plain text `application/x-oldf+zip`. Must be the **first** entry, **stored without compression** and without an extra field, so the text starts at byte 38 of the file. A reader can recognize an OLDF file by reading a few bytes. |
| `manifest.json` | yes | UTF-8 JSON at the archive root. It lists every dataset and describes the experiment. |
| `data/` | recommended | The datasets. Any layout is allowed inside, as long as every file is listed in the manifest. |
| `scenes/` | optional | Display state for viewers. |
| `provenance/` | optional | Additional origin records. |
| `notes/` | optional | Human-readable material. |

The folder names are conventions. What the manifest says is what counts.

## What can go inside

Anything that is a file can be a dataset. The manifest says what each file is through its `profile` and `media_type` (see [Datasets](#datasets) and [Profiles](#profiles)). Typical contents:

| Kind of data | Examples of files |
|---|---|
| Molecular structures | PDB, mmCIF, BinaryCIF |
| Computed structure data | bond tables, secondary structure per residue, surface meshes (glTF) |
| Images | gels, microscopy, plates (PNG, TIFF) |
| Measurements and tables | spectra, plate readings, results (CSV, Parquet) |
| Sequences | FASTA, FASTQ, alignments |
| Volumes and trajectories | density maps, molecular dynamics frames (Zarr, HDF5) |
| Display state | camera, representations, selections, labels (`scene.json`) |
| Context | instrument and sample descriptions, protocols, notes |

One file can mix several of these. For example, a structure, the gel image where its size was estimated, the band table extracted from that image, and the view used in a figure.

---

## Manifest

`manifest.json` is a JSON object. Duplicate keys are not allowed.

| Field | Type | Required | Description |
|---|---|---|---|
| `format` | string | yes | Always `"oldf"`. |
| `format_version` | string | yes | Version of this specification, as `MAJOR.MINOR.PATCH`. |
| `id` | string | yes | A UUID that identifies this file. |
| `created` | string | yes | Creation time, ISO 8601 in UTC. |
| `created_by` | object | yes | `software` and `version` of the tool that wrote the file. |
| `license` | string | no | An SPDX license identifier for the contents, such as `CC-BY-4.0`. |
| `experiment` | object | no | `title`, `description`, `lab`, `operators`. |
| `instruments` | array | no | Instruments used: `id`, `manufacturer`, `model`, `serial`, `software_version`. |
| `samples` | array | no | Samples: `id`, `description`, plus free fields. |
| `datasets` | array | yes | Every data file in the archive. See below. |
| `provenance` | object | no | Original files and conversions that led to this file. |
| `extensions` | object | no | Extra fields from third parties, under a reverse-DNS key such as `"org.example.myfield"`. |

Conventions that apply everywhere in the manifest:

- Dates and times use ISO 8601 in UTC.
- Units are written explicitly, using [UCUM](https://ucum.org/) codes.
- Things refer to each other by `id` (`instrument`, `sample`, `derived_from`), never by repeating paths.
- Personal data, such as operator names, is optional. Think about what you share before you publish a file.

## Datasets

Each item of `datasets` describes one file in the archive.

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | string | yes | Unique within the manifest. |
| `path` | string | yes | Path inside the archive. Relative, with `/` as separator. |
| `profile` | string | yes | What kind of data this is, such as `structure.pdb`. See [Profiles](#profiles). |
| `media_type` | string | yes | The MIME type of the file. |
| `role` | string | yes | `raw`, `derived` or `simulated`. |
| `sha256` | string | yes | SHA-256 of the file contents, in lowercase hexadecimal. |
| `size` | integer | yes | Size of the file in bytes. |
| `derived_from` | array | if derived | `id`s of the datasets it was computed from. |
| `method` | object | if derived or simulated | `tool`, `version`, `algorithm`, `parameters`. |
| `instrument` | string | no | `id` of an entry in `instruments`. |
| `sample` | string | no | `id` of an entry in `samples`. |
| `acquired` | string | no | When the data was measured. |
| `columns` | array | no | For tables: `name`, `type`, optional `unit`. |

## Roles: raw, derived and simulated

This is the most important distinction in the format.

- **`raw`**: as it came from the instrument or the source. Never modified.
- **`derived`**: computed from other datasets in the same file. It must say which ones (`derived_from`) and how (`method`).
- **`simulated`**: produced by a model rather than measured. It must say how (`method`), so it can never be mistaken for a measurement.

Derived data is an optimization, not a source of truth. A reader that doesn't trust or can't reproduce the `method` should recompute from the `raw` data, which is always included.

## Profiles

A **profile** names a kind of data and what is expected of it, written as `family.kind`, for example `structure.pdb` or `electrophoresis.image`.

In v0.1 the profile is just a label that tells a reader what the dataset is. The required fields of each profile are not defined yet. Names in use so far:

- `structure.pdb`, `structure.mmcif`, `structure.bonds`, `structure.scene`
- `electrophoresis.image`, `electrophoresis.bands`

New profiles are added as real use cases appear. Third parties should use their own prefix, such as `org.example.spectrum`, to avoid clashes.

---

## Writing rules

A writer **must**:

1. Write `mimetype` first, stored without compression, with no extra field, containing exactly `application/x-oldf+zip`.
2. Write `manifest.json` as valid UTF-8 JSON.
3. List **every** other file of the archive in `datasets`, and nothing else. No orphan files, no listed file missing.
4. Compute `sha256` and `size` from the exact bytes stored.
5. Use paths that are relative, use `/`, contain no `..` and are not absolute. Paths must be unique, including when compared without regard to upper or lower case, so the file opens correctly on every operating system.
6. Use UTF-8 names. No encryption, no symbolic links, no multi-part archives. ZIP64 is allowed.
7. Write the file atomically: to a temporary file first, and rename it when complete.

A writer **should**:

- Store data that is already compressed (PNG, Parquet, JPEG) without compressing it again.
- Produce **deterministic output**: entries in a fixed order after `mimetype` and `manifest.json`, fixed timestamps (or `SOURCE_DATE_EPOCH`), fixed permissions. The same content then gives the same file and the same hash.

## Reader requirements

A reader **must**:

1. Check `mimetype` before reading anything else.
2. Reject a file whose major `format_version` it does not support.
3. Ignore fields it doesn't know, except inside `extensions` it chooses to interpret.
4. Check that ids are unique and that every reference (`instrument`, `sample`, `derived_from`) points to something that exists.
5. Check that every path in the manifest exists in the archive, and the other way around.
6. Verify `sha256` and `size` before trusting a dataset's contents.
7. Apply the limits described in [Security considerations](#security-considerations).

A reader **should** report errors in terms a person can act on, such as "dataset `d-2` has no `method`", rather than raw parser messages.

## Security considerations

OLDF files come from other people, so a reader treats them as untrusted input.

- **Path traversal:** never extract to a path that escapes the target folder.
- **Zip bombs:** limit the total decompressed size, the compression ratio, and the number of entries.
- **Memory:** validate sizes before allocating. Limit the size and nesting depth of `manifest.json`.
- **No network access:** a reader must not download anything because of what a file says, including schemas or referenced URLs.
- **Duplicate keys and odd encodings** in JSON are errors, not something to guess about.
- **Cached or derived data** inside a file is untrusted too. Check ranges and sizes after reading it.
- **Privacy:** files may contain personal or confidential data. Writers should make such fields optional and easy to remove.

## Versioning and compatibility

The specification uses semantic versioning.

- `0.x`: unstable. Breaking changes may happen between minor versions, and each one is noted.
- From `1.0`: the major number changes only for incompatible changes. Minor versions only add things that older readers can safely ignore.
- A file declares its version in `format_version`. Readers decide what they accept based on it.

## Out of scope for v0.1

These are planned or possible, but not part of this version:

- Digital signatures and key management.
- Encryption.
- A published JSON Schema. It will come after the first working implementation, and will be generated from the reference code so the two cannot drift apart.
- Formal definitions of profiles, including their required fields.
- Registration of the media type.
- Partial updates and indexed access to very large datasets.
- Governance of the standard.

---

## Tooling

This repository is organized as a Cargo workspace:

```
crates/
├── oldf-core/    reference library: read, write, validate
└── oldf-cli/     the `oldf` command line tool
```

Planned commands for `oldf`: `validate`, `info`, `create` and `convert`.

Bindings to other languages are built on top of `oldf-core`. The molecule viewer is a separate project: it will open OLDF files, and also plain PDB and mmCIF.

## Example

A small synthetic example, with a water molecule and a gel drawn by a script, is described by a manifest like this (abridged):

```json
{
  "format": "oldf",
  "format_version": "0.1.0",
  "id": "968f89fd-aa83-52d0-8eb1-8b1727e3b0cf",
  "created": "2026-10-24T12:00:00Z",
  "created_by": { "software": "make_example.py", "version": "0.0.1" },
  "datasets": [
    {
      "id": "d-1",
      "path": "data/water.pdb",
      "profile": "structure.pdb",
      "media_type": "chemical/x-pdb",
      "role": "raw",
      "sha256": "4e9dcccee6e45be03a5867ff16fecaebc7d3a94b63c52bb83f325877c3e40626",
      "size": 273
    },
    {
      "id": "d-2",
      "path": "data/water_bonds.csv",
      "profile": "structure.bonds",
      "media_type": "text/csv",
      "role": "derived",
      "derived_from": ["d-1"],
      "method": {
        "tool": "make_example.py",
        "version": "0.0.1",
        "algorithm": "distance",
        "parameters": { "tolerance_angstrom": 0.4 }
      },
      "sha256": "fc10146d6f7adeeff77f038eb91b052e32c08c02a4f2ebb1c01f91c1f6464f88",
      "size": 57
    }
  ]
}
```

You can inspect any `.oldf` file with a normal ZIP tool:

```bash
unzip -l example.oldf            # list the contents
unzip -p example.oldf manifest.json   # print the manifest
```

The complete synthetic example archive will live in `examples/`.

## Naming conventions

| Thing | Spelling |
|---|---|
| Name of the format in text | OLDF, Open Lab Data Format |
| File extension | `.oldf` |
| Value of `format` and the `mimetype` | `oldf`, `application/x-oldf+zip` (provisional, not registered) |
| Crates and command line tool | `oldf-core`, `oldf-cli`, `oldf` |