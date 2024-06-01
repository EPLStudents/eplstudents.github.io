#!/bin/sh
set -xe

ROUTES=$(cat <<-END
/
END
)

rm -rf static
mkdir -p static
cp -r public static/public
cp CNAME static/CNAME

mkdir static/discord/
cp discord.html static/discord/index.html

mkdir static/discord_SINF/
cp discord_sinf.html static/discord_SINF/index.html


killall eplstudents-website || true
cargo run &
sleep 2

for route in $ROUTES; do
	mkdir -p static$route
	fetch http://localhost:8000$route -o static$route/index.html
done

kill $(jobs -p)