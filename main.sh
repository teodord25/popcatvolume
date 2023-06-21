#!/bin/bash


if [[ $1 == list ]]; then
    pactl list sinks | grep '^[[:space:]]Volume:'
elif [[ $1 == vol ]]; then
    # front-left of 0th sink
    pactl list sinks | grep '^[[:space:]]Volume:' | grep -o '[0-9]*%' | sed -n '1p'

elif [[ $1 == show ]]; then
    if [[ -z $2 ]]; then
        echo "missing filename"
        exit 1
    fi

    kitty +kitten icat images/$2.png

fi
