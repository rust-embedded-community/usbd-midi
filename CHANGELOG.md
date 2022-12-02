# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Swapped out bespoke MIDI types for shared [midi-types](https://crates.io/crates/midi-types).
  * And in turn [midi-convert](https://crates.io/crates/midi-convert) for parsing/rendering.
  * **NOTE**: this crate uses a different note octave numbering convention, you may need to
  change the octave number of your note constants if you use them.
