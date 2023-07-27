#!/bin/bash -e
rm -rf build
idf.py set-target "esp32"
idf.py build
idf.py flash

