#!/bin/sh
/steamcmd/steamcmd.sh +login anonymous +force_install_dir /home/steam/csgo +app_update 740 +exit
cd /home/steam/csgo/
/home/steam/csgo/srcds_run -game csgo -console -usercon +game_type 0 +game_mode 0 +mapgroup mg_active +map de_dust2
