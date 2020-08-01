<?php
// Start the session
session_start();
?>

<?php
if(isset($_SESSION["username"])) {
    $servername = "localhost";
    $serverUsername = "root";
    $serverPassword = "";
    $dbname = "beacon";

    // Create connection
    $conn = new mysqli($servername, $serverUsername, $serverPassword, $dbname);

    $games = fopen('games.json', 'r');
    echo fread($games, filesize("games.json"));

} else {
    echo "Not logged in";

}
?>