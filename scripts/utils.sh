#!/usr/bin/env bash

if [ "${CI:-false}" != "true" ]; then
	bold=$(tput bold)
	normal=$(tput sgr0)
	light_grey=$(tput setaf 250)
	blue=$(tput setaf 4)
	green=$(tput setaf 2)
	yellow=$(tput setaf 3)
	red=$(tput setaf 1)
else
	bold=""
	normal=""
	blue=""
	light_grey=""
	green=""
	yellow=""
	red=""
fi

function formatted_time {
	date +%FT%T.%3N
}

function formatted_severity {
	printf "%+6s:" $1
}

function formatted_log {
	log_severity=$1
	log_message="$2"
	echo "${bold}$(formatted_time) $(formatted_severity $log_severity) $log_message ${normal}"
}

function info {
	formatted_log INFO "$1"
}

function warn {
	formatted_log WARN "$1"
}

function success {
	echo "${green}$(formatted_log INFO "$1")"
}

function error {
	echo >&2 "${red}$(formatted_log ERROR "$1")"
}

function error-and-exit {
	error "$1"
	if [ "${2:''}" != "" ]; then
		exit $2
	else
		exit 1
	fi
}
