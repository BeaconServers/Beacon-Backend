# This file is just for being able to start and stop the entire backend easily

import threading
import os
import sys
import signal
from python import generate_server_password as gen_pass


def main():
    print("Starting LAMPP")
    os.system("/opt/lampp/xampp restart")

    print("To shutdown the backend, press CTRL + C")


def start_steam():
    print("Starting Counter Strike Source server...")
    os.system("sudo -u steam /home/steam/CSS/srcds_run -console -game cstrike -maxplayers 20 -port 27015 +ip "
              "127.0.0.1 +map de_dust2 "
              "+sv_password password")


main_t = threading.Thread(target=main)
gen_pass_t = threading.Thread(target=gen_pass.generate_passwords)
steam_t = threading.Thread(target=start_steam)

if __name__ == '__main__':
    main_t.start()
    gen_pass_t.start()
    steam_t.start()


def exit_handler(sig, frame):
    print("Shutting down password generator...")
    gen_pass_t.keep_running = False
    gen_pass_t.join()
    main_t.join()
    steam_t.join()

    print("Shutting down back end...")
    print("Backing up MySQL database")
    os.system("/opt/lampp/bin/mysqldump -u root beacon > /opt/lampp/htdocs/Beacon/beacon.sql")
    print("Backup complete!")
    print("Shutting down LAMPP...")
    os.system('/opt/lampp/xampp stop')

    sys.exit("Goodbye, and thanks for all the fish!")


signal.signal(signal.SIGINT, exit_handler)
