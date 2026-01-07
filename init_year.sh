#!/usr/bin/env bash

YEAR=2024
SESSION="53616c7465645f5fbdfc18812ee97e5f3f1a5518fb6bb4074e50b58a2b4572cc55ed0c126a8d8aeaff6b2bc758b2b246c5394b292d5199f3c19be0786d006257"
OUTDIR="${YEAR}/inputs"

mkdir -p "$OUTDIR"

for DAY in {1..25};
do
	printf "Downloading day %02d...\n" "$DAY"

	curl -s -H "Cookie: session=${SESSION}" "https://adventofcode.com/${YEAR}/day/${DAY}/input" -o "${OUTDIR}/day$(printf "%02d" "$DAY").txt"

	sleep 1
done
