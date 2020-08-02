<?php
require 'server.php';

if(isset($_SESSION["username"])) {
    $games = fopen('games.json', 'r');
    echo fread($games, filesize("games.json"));

} else {
    echo "Not logged in";

}
?>