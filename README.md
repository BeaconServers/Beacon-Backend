# Beacon Backend
## Welcome to the superior backend!
### Setup
#### Windows (manual):
1. Install <a href="https://www.apachefriends.org/index.html">XAMPP</a>, an easy to use PHP development environment.
2. During installation, make sure to enable Apache and MySQL services (these should be enabled by default)
3. Upon installation, open `C:\xampp `and run `xampp-control.exe`
4. Clone the backend into the htdocs folder (run `cd C:\xampp\htdocs` and then run `git clone -b backend https://github.com/Susorodni/Beacon.git`)
5. (If you're developing the backend) Install <a href="http://insomnia.rest/download/">Insomnia</a>, an API development toolkit.
6. Run Insomnia and import `beacon.json`
7.  Start the MySQL services from the XAMPP control panel
8. Import the MySQL database by running `C:\xampp\mysql\bin\mysql.exe -u root -p beacon < C:\xampp\htdocs\Beacon\beacon.sql`
9. Start the Apache service from the XAMPP control panel (`C:\xampp\xampp-control.exe`).
10. Download [SteamCMD](https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip)
11. Extract the zip file to a folder (we'll use `C:\steamcmd` as an example)
12. Open command prompt and run `cd C:\steamcmd` then run `steamcmd`
13. Run `login anonymous`
14. Run `force_install_dir C:\steamcmd\css` (or whatever folder you extracted steamcmd to  `\css`)
15. Run `app_update 232330 validate`
16. Run quit
17. Replace `C:\xampp\htdocs` with `C:\xampp\htdocs\Beacon\Web` 
18. Install the backend's python requirements by running `pip3 install -r req.txt`
18. To run the backend, run `bootstrap_windows.py` (not ready yet) 
19. That's it! To interact with the database, either run POST requests from Insomnia or send POST requests from the frontend

#### Ubuntu (script):
1. Install the requirements (which the latest Ubuntu releases should come with) by running `pip3 install -r req.txt`
2. Simply download and run `setup_ubuntu.py`, be patient as it can take a while.
