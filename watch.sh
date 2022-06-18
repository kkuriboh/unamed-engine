#!/bin/bash

while true;
do inotifywait -e modify * && make;
done
