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


main_t = threading.Thread(target=main)
gen_pass_t = threading.Thread(target=gen_pass.generate_passwords)

if __name__ == '__main__':
    main_t.start()
    gen_pass_t.start()


def exit_handler(sig, frame):
    print("Shutting down password generator...")
    gen_pass_t.keep_running = False
    gen_pass_t.join()

    print("Shutting down back end...")
    print("Backing up MySQL database")
    os.system("/opt/lampp/bin/mysqldump -u root beacon > /opt/lampp/htdocs/Beacon/beacon.sql")
    print("Backup complete!")
    print("Shutting down LAMPP...")
    os.system('/opt/lampp/xampp stop')

    sys.exit(0)


signal.signal(signal.SIGINT, exit_handler)
