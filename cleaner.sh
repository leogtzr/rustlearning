#!/bin/bash

readonly WORK_DIR=$(dirname $(readlink --canonicalize-existing "${0}"))

while read dir; do
	(
		cd "${dir}"
		cargo clean 2> /dev/null
	)
done < <(find "${WORK_DIR}" -maxdepth 1 -type d)

exit 0