#!/bin/sh
/steamcmd/steamcmd.sh +runscript /home/steam/csgo_install.txt
cd /home/steam/csgo/
/home/steam/csgo/srcds_run -game csgo -console -usercon +game_type 0 +game_mode 0 +mapgroup mg_active +map de_dust2
