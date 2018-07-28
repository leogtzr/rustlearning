#!/bin/bash

readonly WORK_DIR=$(dirname $(readlink --canonicalize-existing "${0}"))

while read dir; do
	echo -n "Cleaning: ${dir}"
	(
		cd "${dir}" || continue
		cargo clean 2> /dev/null
	)
	echo ", done"
done < <(LC_ALL=C find "${WORK_DIR}" -maxdepth 1 -type d)

exit 0
