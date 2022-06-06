#!/bin/bash

#TODO: Update to my configs and reference DistroTube
#
# Script name: dm-confedit
# Description: Choose from a list of configuration files to edit.
# Dependencies: dmenu, emacsclient
# GitLab: https://www.gitlab.com/dwt1/dmscripts
# License: https://www.gitlab.com/dwt1/dmscripts/LICENSE
# Contributors: Derek Taylor
#               Simon Ingelsson
#               HostGrady

# Set with the flags "-e", "-u","-o pipefail" cause the script to fail
# if certain things happen, which is a good thing.  Otherwise, we can
# get hidden bugs that are hard to discover.

set -euo pipefail

DMENU="dmenu -i -l 20 -p"
BROWSER="firefox"

main() {

	source "${HOME}/.config/quickGHMenu/repo_list"
	for i in "${!github_repo_list[@]}"; do
		[[ -f ${github_repo_list["${i}"]} ]] && _clean_list["${i}"]=${github_repo_list["${i}"]}
	done

	# Piping the above array (cleaned) into dmenu.
	# We use "printf '%s\n'" to format the array one item to a line.
	choice="$(printf '%s\n' "${!github_repo_list[@]}" | ${DMENU} "Open Project:" "$@")"

	if [ "$choice" ]; then
		repo=$(printf '%s\n' "${github_repo_list["${choice}"]}")
		$BROWSER "$repo"
	else
		echo "Program terminated." && exit 0
	fi
}

[[ "${BASH_SOURCE[0]}" == "${0}" ]] && main "$@"
