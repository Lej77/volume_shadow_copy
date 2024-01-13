# volume_shadow_copy

<!-- Badge style inspired by https://github.com/dnaka91/advent-of-code/blob/de37024ba3b385694e14f79c849370c0f605f054/README.md -->

<!-- [![Build Status][build-img]][build-url] -->
[![Documentation][doc-img]][doc-url]

<!--
[build-img]: https://img.shields.io/github/actions/workflow/status/Lej77/volume_shadow_copy/ci.yml?branch=main&style=for-the-badge
[build-url]: https://github.com/Lej77/volume_shadow_copy/actions/workflows/ci.yml
 -->
<!-- https://shields.io/badges/static-badge -->
[doc-img]: https://img.shields.io/badge/docs.rs-volume_shadow_copy-4d76ae?style=for-the-badge
[doc-url]: https://lej77.github.io/volume_shadow_copy/volume_shadow_copy/index.html

Volume Shadow Copy Service API wrapper. Can be used to make a VSS
"requester" that uses VSS to make or restore backups.

## Why it is useful

There are many advantages to using the Volume Shadow Copy Service to preform
backups instead of just reading files directly. For example it ensures that
files are not modified while the they are being read and it can also notify
registered VSS "writers" that a backup is being preformed so that they can
prepares their data as appropriate, such as completing all open
transactions, rolling transaction logs, and flushing caches.

Using the Volume Shadow Copy Service should also allow copying files that
have a [share mode] that doesn't [allow them to be read]. Even if a file can
be read, opening it can still cause issues since another process could try
to open it with a [share mode that doesn't allow another reader] and
therefore fail their open operation [causing issues for that other process].

[share mode]:
    https://doc.rust-lang.org/std/os/windows/fs/trait.OpenOptionsExt.html#tymethod.share_mode
[allow them to be read]:
    https://stackoverflow.com/questions/3560651/whats-the-least-invasive-way-to-read-a-locked-file-in-c-sharp-perhaps-in-unsaf
[share mode that doesn't allow another reader]:
    https://stackoverflow.com/questions/11855245/unable-to-read-same-file-in-parallel
[causing issues for that other process]:
    https://stackoverflow.com/questions/10473442/why-cant-i-open-a-file-for-reading-if-theoretically-i-should-be-allowed

## References

This command-line tool could maybe be used to manage volume shadow copies:
[VShadow Tool and Sample - Win32 apps | Microsoft
Docs](https://docs.microsoft.com/en-us/windows/win32/vss/vshadow-tool-and-sample)

High level overview of Volume Shadow Copy Service: [Volume Shadow Copy
Service | Microsoft
Docs](https://docs.microsoft.com/en-us/windows-server/storage/file-server/volume-shadow-copy-service)

High level overview of Volume Shadow Copy Service: [Volume Shadow Copy
Service - Win32 apps | Microsoft
Docs](https://docs.microsoft.com/en-us/windows/win32/vss/volume-shadow-copy-service-portal)

Hight level overview of a "requestor": [Requesters - Win32 apps | Microsoft
Docs](https://docs.microsoft.com/en-us/windows/win32/vss/requestors)

Overview of what operations to preform when making a backup: [Overview of
Processing a Backup Under VSS - Win32 apps | Microsoft
Docs](https://docs.microsoft.com/en-us/windows/win32/vss/overview-of-processing-a-backup-under-vss)

Documentation for "vsbackup.h" header that defines the API that "requestors"
uses: [Vsbackup.h header - Win32 apps | Microsoft
Docs](https://docs.microsoft.com/en-us/windows/win32/api/vsbackup/)

List with relevant header files: [System Services - Win32 apps | Microsoft
Docs](https://docs.microsoft.com/en-us/windows/win32/api/_base/)

Specification  for the Shadow Copy Management Protocol: [[MS-SCMP]: Shadow
Copy Management Protocol | Microsoft
Docs](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-scmp/a1ab0e30-2dc1-49bb-8c46-4616ea09cc54)

## License

This project is released under either:

- [MIT
  License](https://github.com/Lej77/volume_shadow_copy/blob/master/LICENSE-MIT)
- [Apache License (Version
  2.0)](https://github.com/Lej77/volume_shadow_copy/blob/master/LICENSE-APACHE)

at your choosing.

### Copied content

The above license might not apply to content copied from other sources. This
includes some documentation comments in the code as well as error info
inside the `errors.md` file used to generate [error types](errors). The
example code in `examples/make-backup-snapshot.rs` was inspired by code from
the [backup program `restic`] and so might be affected by its license.

[backup program `restic`]:
    https://github.com/restic/restic/blob/db8a95899114ef5131818462d057cac202189b3a/internal/fs/vss_windows.go#L763-L777

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
