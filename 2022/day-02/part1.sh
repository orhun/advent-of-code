#!/usr/bin/env bash

INPUT_FILE="input"

declare -A moves=(["A"]=1 ["B"]=2 ["C"]=3 ["X"]=1 ["Y"]=2 ["Z"]=3)
declare -a win_table=('C-X' 'A-Y' 'B-Z')
total_score=0

while IFS= read -r line; do
	read -ra move <<<"$line"
	if [ "${moves[${move[0]}]}" == "${moves[${move[1]}]}" ]; then
		total_score=$((total_score + ${moves[${move[1]}]} + 3))
		continue
	fi
	win=0
	for i in "${win_table[@]}"; do
		if [ "$i" == "${move[0]}-${move[1]}" ]; then
			win=6
			break
		fi
	done
	total_score=$((total_score + ${moves[${move[1]}]} + win))
done <$INPUT_FILE

echo "$total_score"
