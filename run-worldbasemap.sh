#!/bin/bash

./target/release/tiledownloader-cli.exe \
	-o worldbasemap-16.mbtiles \
	--url "https://basemaps.arcgis.com/arcgis/rest/services/World_Basemap_v2/VectorTileServer/tile/{z}/{y}/{x}.pbf" \
	--name "WorldBasemap-16.mbtiles" \
	--bbox 23.995,-83.247,25.572,-80.228 \
	-z 16
