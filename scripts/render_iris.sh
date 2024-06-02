#!/bin/bash -e

./target/debug/examples/iris barchart
echo ""
echo ""

./target/debug/examples/iris barchart -v
echo ""
echo ""

./target/debug/examples/iris barchart --breakdown
echo ""
echo ""

./target/debug/examples/iris barchart --breakdown -v
echo ""
echo ""

./target/debug/examples/iris histogram
echo ""
echo ""

./target/debug/examples/iris histogram -v
echo ""
echo ""

./target/debug/examples/iris histogram --breakdown
echo ""
echo ""

./target/debug/examples/iris histogram --breakdown -v
