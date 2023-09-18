#!/bin/bash
#
# Simple script to generate pending migrations and write them on `src/schema.rs`

diesel migration run
diesel print-schema > src/schema.rs
