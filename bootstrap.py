# This file is just for being able to start and stop the entire backend easily
import time
import threading
import os
import pexpect
import sys
import signal
from python import generate_server_password as gen_pass

last_minute = 000
run_steam = True


def main():
    print("Starting LAMPP")
    os.system("/opt/lampp/xampp restart")

    print("To shutdown the backend, press CTRL + C")


def start_steam():
    t = threading.current_thread()
    print("Starting Counter Strike Source server...")

    steam = pexpect.spawn('sudo -u steam /home/steam/CSS/srcds_run -console -game cstrike -maxplayers 20 -port '
                          '27015 +ip 0.0.0.0 +map de_dust2 +sv_password password')

    while True:
        if run_steam:
            with open('game_passwords/css.pwd', 'r') as f:
                global last_minute
                password = f.readline().strip()
                current_minute = f.readline().strip()

                if current_minute != last_minute:
                    print("Setting password to " + password)
                    steam.sendline('sv_password ' + password)
                    last_minute = current_minute

            time.sleep(1)
        else:
            print("Sending signal to kill steam...")
            steam.kill(0)
            break


steam_t = threading.Thread(target=start_steam)
main_t = threading.Thread(target=main)
gen_pass_t = threading.Thread(target=gen_pass.generate_passwords)

if __name__ == '__main__':
    main_t.start()
    gen_pass_t.start()
    steam_t.start()


def exit_handler(sig, frame):
    print("Shutting down back end...")
    print("Shutting down password generator...")
    gen_pass_t.keep_running = False
    gen_pass_t.join()
    print("Shutting down steam...")
    global run_steam
    run_steam = False
    steam_t.join()
    main_t.join()

    print("Backing up MySQL database")
    os.system("/opt/lampp/bin/mysqldump -u root beacon > /opt/lampp/htdocs/Beacon/beacon.sql")
    print("Backup complete!")
    print("Shutting down LAMPP...")
    os.system('/opt/lampp/xampp stop')

    sys.exit("Goodbye, and thanks for all the fish!")


signal.signal(signal.SIGINT, exit_handler)
