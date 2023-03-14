#!/bin/bash

./target/release/tiledownloader-cli.exe \
	-o usgs-imagery-16.mbtiles \
	--url "https://basemap.nationalmap.gov/arcgis/rest/services/USGSImageryOnly/MapServer/tile/{z}/{y}/{x}" \
	--name "USGS-Imagery-16.mbtiles" \
	--bbox 23.995,-83.247,25.572,-80.228 \
	-z 16
