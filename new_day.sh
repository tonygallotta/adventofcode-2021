#!/bin/bash
day_num=${1-$(date +'%d')}
project=day${day_num}
cargo new ${project}
cp template/main.rs ${project}/src/
touch ${project}/test.txt
curl "https://adventofcode.com/2021/day/${day_num}/input" \
  -H 'cookie: session=53616c7465645f5f2afd136378ba4549a128cd4cd40bc08a1e4745cba68fa4ae0595a1f49ca94ab3492f830d3871deb4' \
  > ${project}/input.txt

