#!/bin/bash

./target/debug/tiledownloader-cli.exe \
	-o nro-osm-15.mbtiles \
	--url "http://saturn.newrobotorder.com/openstreetmap/{z}/{x}/{y}.png" \
	--name "NRO-OSM-10" \
	--bbox 23.995,-83.247,25.572,-80.228 \
	-z 11
