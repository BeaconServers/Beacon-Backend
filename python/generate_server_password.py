#!/usr/bin/env python
# Generates a secure password by generating a thousand bytes, then finding the sha512 hash
import schedule
import time
from hashlib import sha512
from os import urandom
import signal
import sys
import threading

games = ["css", "tf2"]


# Upon the signal to kill the program, the exit handler does some cleanup
def exit_handler(sig, frame):
    print('Stopping password generator...')
    schedule.clear()
    sys.exit(0)


signal.signal(signal.SIGINT, exit_handler)


# Gets a truly random 1024 bytes and hashes it to get an unguessable password, and stores it in game_passwords directory
def generate_new_password(games):
    for game in games:
        randomness = urandom(1024)
        password = sha512(randomness).hexdigest()
        current_minute = time.strftime("%M")

        with open('game_passwords/' + game + '.pwd', 'w') as f:
            f.write(password[0:50] + '\n' + current_minute)


generate_new_password(['css'])
schedule.every(1).minutes.do(generate_new_password, games)


def generate_passwords():
    print("Generating passwords")
    t = threading.currentThread()
    while getattr(t, "keep_running", True):
        time.sleep(5)
        schedule.run_pending()

    print("Password generator stopped!")
    schedule.clear()
