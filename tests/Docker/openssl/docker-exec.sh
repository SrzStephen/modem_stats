#!/bin/bash
cd "${0%/*}"
docker build . --tag opensshbuild:0.0.1 
docker run -v$(pwd):/export opensshbuild:0.0.1