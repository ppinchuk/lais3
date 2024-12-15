#!/bin/sh

cp target/debug/solution .
luxai-s3 main.py main.py --output replay.json
