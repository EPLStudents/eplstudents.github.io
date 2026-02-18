#!/usr/bin/env bash
set -xe

ROUTES=$(cat <<-END
/
END
)

rm -rf static
mkdir -p static
cp -r public static/public
cp CNAME static/CNAME


# array of (route, source) pairs
array=(
	"/discord/ discord.html"
	"/discord-sinf/ discord-sinf.html"
  	"/drive/ drive.html"
  	"/drive-contributions/ drive-contributions.html"
  	"/sharepoint-epl/ sharepoint-epl.html"
)

for item in "${array[@]}"; do
  route=$(echo $item | cut -d' ' -f1)
  source=$(echo $item | cut -d' ' -f2)
  
  mkdir -p "static$route"
  cp "$source" "static$route/index.html"
done

killall eplstudents-website || true
cargo build
cargo run &
sleep 2

port=$(lsof -i -P -n | grep LISTEN  | grep eplstuden | cut -d: -f2 | cut -d' ' -f1)
echo "Port: $port"

for route in $ROUTES; do
	mkdir -p static$route
	curl http://localhost:$port$route -o 'static'$route'index.html'
done

jobs=$(jobs -p | cut -d+ -f2 | cut -d' ' -f2)
if [ $jobs ]; then 
	kill $jobs
fi
