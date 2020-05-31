# Beacon Backend
## Welcome to the superior backend!
### Setup
#### Windows:
1. Install <a href="https://www.apachefriends.org/index.html">XAMPP</a>, an easy to use PHP development environment.
2. During installation, make sure to enable Apache and MySQL services (these should be enabled by default)
3. Upon installation, open `C:\xampp `and run `xampp-control.exe`
4. Clone the backend into the htdocs folder (run `cd C:\xampp\htdocs` and then run `git clone https://github.com/Susorodni/Beacon.git`)
5. Checkout the backend branch (run `git checkout backend`)
6. Open `C:\xampp\phpMyAdmin\config.inc.php` and look for the lines that say `$cfg['Servers'][$i]['user']` and `$cfg['Servers'][$i]['password']` and change the username and password to `root`
7. Install <a href="https://www.postman.com/">Postman</a>, an API development toolkit.
8. Run Postman and import `Beacon.postman_collection.json` from `C:\xampp\htdocs\Beacon`
9.  Start the MySQL services from the XAMPP control panel
10. Impor the MySQL database by running `C:\xampp\mysql\bin\mysql.exe  -u root -p beacon < C:\xampp\htdocs\Beacon\beacon.sql`
11. Open `C:\xampp\apache\conf\httpd.conf` and at the very bottom of the file add a new line that that says `AccessFileName htaccess.txt`
12. Create a new file in the `C:\xampp\apache\conf` directory called htaccess.txt, and on the very first line type `php_value session.gc_maxlifetime 31557600`
13. Start the Apache service from the XAMPP control panel.
14. That's it! To interact with the database, either run POST requests from Postman or send POST requests from the frontend

