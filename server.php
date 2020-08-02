<?php
session_start();
$servername = "localhost";
$serverUsername = "root";
$serverPassword = "";
$dbname = "beacon";

// Create connection
$conn = new mysqli($servername, $serverUsername, $serverPassword, $dbname);
// Check connection
if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}
?>