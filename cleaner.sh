#!/bin/bash

readonly WORK_DIR=$(dirname $(readlink --canonicalize-existing "${0}"))

time while read dir; do
	echo -n "Cleaning: ${dir}"
	(
		cd "${dir}"
		cargo clean 2> /dev/null
	)
	echo ", done"
done < <(find "${WORK_DIR}" -maxdepth 1 -type d)

exit 0
