# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/timrabl/rs-py-import-helper/compare/v0.1.0...v0.2.0) - 2025-10-27

### ♻️  Refactor

- remove duplicate custom_import_sort function

### 🐛 Bug Fixes

- use .first() instead of .get(0) to satisfy clippy
- wrap sort_by closures to handle String to &str conversion
- *(ai)* update src/core.rs
- *(ai)* update src/utils/formatting.rs
- update tests to handle multi-line import formatting
- *(fmt)* run cargo fmt --all

### 🚀 Features

- [**breaking**] add clear() and reset() methods to ImportHelper

## [0.1.0](https://github.com/timrabl/rs-py-import-helper/releases/tag/v0.1.0) - 2025-10-09

### ⚙️  Miscellaneous Tasks

- update project metadata and documentation

### 🐛 Bug Fixes

- *(ci)* resolve GitHub Actions and clippy errors

### 📚 Documentation

- add comprehensive project documentation

### 📦 Other

- add development tooling and automation configuration
- Initial commit

### 🔁 CI/CD

- add GitHub Actions workflows for automated testing and releases

### 🚀 Features

- add examples and performance benchmarks
- add Python import helper library with categorization and formatting

### 🧪 Testing

- add comprehensive test suite with 73 tests
