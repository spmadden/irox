#!/bin/bash

./target/debug/tiledownloader-cli.exe \
	-o google-hd-10.mbtiles \
	--url "http://mt{s}.google.com/vt/lyrs=s@176103410&x={x}&y={y}&z={z}&s=Galileo&scale=2&hl=en" \
	--name "Google-HD-10.mbtiles" \
	--bbox 23.995,-83.247,25.572,-80.228 \
	--server-parts 0,1,2,3 \
	--referrer "https://www.google.com/maps/" \
	-z 20
