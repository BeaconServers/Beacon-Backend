import pexpect
import time
import threading

last_minute = 000


def start_steam():
    print("Starting Counter Strike Source server...")

    steam = pexpect.spawn('sudo -u steam /home/steam/CSS/srcds_run -console -game cstrike -maxplayers 20 -port '
                          '27015 +ip 0.0.0.0 +map de_dust2 +sv_password password')

    t = threading.currentThread()

    while getattr(t, "run_steam", True):
        with open('game_passwords/css.pwd', 'r') as f:
            global last_minute
            password = f.readline().strip()
            current_minute = f.readline().strip()

            if current_minute != last_minute:
                print("Setting password to " + password)
                steam.sendline('sv_password ' + password)
                last_minute = current_minute

        time.sleep(1)

    print("Sending signal to kill steam...")
    steam.kill(0)

