#!/usr/bin/env bash

INPUT_FILE="input"

input=$(cat "$INPUT_FILE")
if [ "$(tail -n1 $INPUT_FILE)" != "" ]; then
	input="${input}"$'\n'
fi

elves=()
calories=0
while IFS= read -r line; do
	if [ -z "${line}" ]; then
		elves+=("$calories")
		calories=0
	else
		calories=$((calories + line))
	fi
done < <(echo "$input")

IFS=$'\n'
part1=$(echo "${elves[*]}" | sort -nr | head -n1)
part2=$(echo "${elves[*]}" | sort -nr | head -n3 | awk '{s+=$1} END {print s}')
echo "Part1: ${part1}, Part2: ${part2}"
