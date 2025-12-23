# csv_combiner

A tool to combine CSV, TSV, and other delimited data files with flexible options for handling duplicates and missing data.

## Overview

**csv_combiner** helps data scientists and analysts organize multiple data files into a single coherent output. If you're tired of manually copy-pasting to merge files in repetitive workflows, this tool should make life easier.

## Features

- Combine files with different columns
- Handle missing columns with a custom fill value
- Remove or merge duplicate entries based on key columns
- Support for custom delimiters (CSV, TSV, etc.)
- Cross-platform CLI tool (or at the very least my best attempt at cross-platform)

## Status

**v0.1.0** - CLI with all core features enabled. See [README_CLI.md](README_CLI.md) for usage details.

## About

This project is a learning experience to explore Rust, GitHub, and LLM coding assistants while building practical tooling. While similar tools exist, this particular flavor of CSV merging may be useful for others facing the same data organization challenges I've had.  Feedback is welcome and constructive feedback is appreciated. 