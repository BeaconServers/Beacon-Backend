<?php
// Start the session
session_start();
?>

<?php
if(session_id() !== '') {
    $games = fopen('games.json', 'r');
    echo fread($games, filesize("games.json"));

} else {
    echo "Not logged in";

}
?>