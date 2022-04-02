#!/bin/bash

openapi-generator generate -g rust-server -i specification.yaml -o openapi_client
