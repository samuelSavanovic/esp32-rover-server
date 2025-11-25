#!/usr/bin/env bash

if [ -z "$1" ]; then
    echo "Usage: $0 <distance_mm>"
    exit 1
fi

DIST=$1

BYTE1=$(printf "%02x" $(( DIST        & 0xFF )))
BYTE2=$(printf "%02x" $(( (DIST >> 8) & 0xFF )))
BYTE3=$(printf "%02x" $(( (DIST >>16) & 0xFF )))
BYTE4=$(printf "%02x" $(( (DIST >>24) & 0xFF )))

KIND="01"

HEX="${KIND}${BYTE1}${BYTE2}${BYTE3}${BYTE4}"

echo "Sending telemetry:"
echo "  kind=0x01"
echo "  distance_mm=$DIST"
echo "  hex=$HEX"

printf "\\x${KIND}\\x${BYTE1}\\x${BYTE2}\\x${BYTE3}\\x${BYTE4}" | websocat -b ws://127.0.0.1:9000/ws
