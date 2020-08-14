# This file is just for being able to start and stop the entire backend easily
import time
import threading
import os
import sys
import signal
from python import generate_server_password as gen_pass
from python import set_server_password as set_pass


def main():
    print("Starting LAMPP")
    os.system("/opt/lampp/xampp restart")

    print("To shutdown the backend, press CTRL + C")


steam_t = threading.Thread(target=set_pass.start_steam)
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
    steam_t.run_steam = False
    steam_t.join()
    main_t.join()

    print("Backing up MySQL database")
    os.system("/opt/lampp/bin/mysqldump -u root beacon > /opt/lampp/htdocs/Beacon/beacon.sql")
    print("Backup complete!")
    print("Shutting down LAMPP...")
    os.system('/opt/lampp/xampp stop')

    sys.exit("Goodbye, and thanks for all the fish!")


signal.signal(signal.SIGINT, exit_handler)
