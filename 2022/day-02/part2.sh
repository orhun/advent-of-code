#!/usr/bin/env bash

INPUT_FILE="input"

declare -A moves=(["A"]=1 ["B"]=2 ["C"]=3 ["X"]=1 ["Y"]=2 ["Z"]=3)
declare -a win_table=('C-X' 'A-Y' 'B-Z')
declare -a lose_table=('B-X' 'C-Y' 'A-Z')
total_score=0

function find_in_table() {
	value=$1
	shift
	table=("$@")
	for i in "${!table[@]}"; do
		if [[ "${table[$i]}" == *"${value}"* ]]; then
			score=$((i + 1))
		fi
	done
}

while IFS= read -r line; do
	read -ra move <<<"$line"
	case "${moves[${move[1]}]}" in
	1)
		find_in_table "${move[0]}" "${lose_table[@]}"
		total_score=$((total_score + score))
		;;
	2)
		total_score=$((total_score + ${moves[${move[0]}]} + 3))
		;;
	3)
		find_in_table "${move[0]}" "${win_table[@]}"
		total_score=$((total_score + score + 6))
		;;
	esac
done <$INPUT_FILE

echo "$total_score"
