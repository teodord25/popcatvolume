#!/bin/bash


if [[ $1 == list ]]; then
    pactl list sinks | grep '^[[:space:]]Volume:'
elif [[ $1 == vol ]]; then
    # front-left of 0th sink
    pactl list sinks | grep '^[[:space:]]Volume:' | grep -o '[0-9]*%' | sed -n '1p'
fi
