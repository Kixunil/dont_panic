Don't panic!() slice
====================

This crate uses `dont_panic` crate to create drop-in replacement for slices. (Not fully drop-in
yet.) The goal is to ensure the code won't ever panic. The user of the crate must prove to the
compiler that the panicking code is unreachable by checking bounds before indexing into slice.
