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

killall eplstudents-website || true
cargo run &
sleep 2

port=$(lsof -i -P -n | grep LISTEN  | grep eplstuden | cut -d: -f2 | cut -d' ' -f1)
echo "Port: $port"
lsof -i -P -n | grep LISTEN

#for route in $ROUTES; do
#	mkdir -p static$route
#	curl http://localhost:$port$route -o static$route/index.html
#done

#kill $(jobs -p)