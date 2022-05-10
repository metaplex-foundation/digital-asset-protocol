#!/usr/bin/env bash
set -e
flatc -r -o ./src/generated --gen-object-api ./asset.fbs
flatc --ts -o ./ts-src/generated --gen-object-api   ./asset.fbs
