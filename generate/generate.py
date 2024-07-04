#!/usr/bin/env python3

import argparse
import json
import sys

from pydantic import TypeAdapter

from lib.generators import rust
from lib.schema import Node


def main():
    ap = argparse.ArgumentParser(allow_abbrev=False)
    ap.add_argument("schema_file", metavar="schema-file", type=str)
    ap.add_argument("generator", type=str, choices={"go", "python", "rust"})
    ap.add_argument("-o", "--output-directory", type=str, default="output/", required=False)
    ap.add_argument("--enable-async-client", action="store_true", required=False)

    args = ap.parse_args()

    schema = TypeAdapter(list[Node]).validate_python(json.load(open(args.schema_file)))
    match args.generator:
        case "go":
            print("Not yet implemented")
        case "python":
            print("Not yet implemented")
        case "rust":
            rust.Generator(output_directory=args.output_directory, async_=args.enable_async_client).process(schema)


if __name__ == "__main__":
    sys.exit(main())
