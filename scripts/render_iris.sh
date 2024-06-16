#!/bin/bash -e

./target/debug/examples/iris -v "SepalWidth"
echo ""

./target/debug/examples/iris -v "PetalLength"
echo ""

./target/debug/examples/iris -v "PetalWidth"
echo ""
