#!/bin/bash
set -e
set -o pipefail

uvx ruff check .
uvx ruff format --check .

echo "✅ All checks passed successfully!"