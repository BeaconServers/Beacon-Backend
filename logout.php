<?php
require 'server.php';

session_destroy();
unset($_SESSION["username"]);
unset($_SESSION["password"]);

echo "Successfully logged out";
?>