<?php
$servername = "localhost";
$serverUsername = "root";
$serverPassword = "root";
$dbname = "beacon";

$username = $_POST["username"];
$email = $_POST["email"];
$password = $_POST["password"];

$password = password_hash($password, PASSWORD_DEFAULT);

// Create connection
$conn = new mysqli($servername, $serverUsername, $serverPassword, $dbname);
// Check connection
if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}

$sql = "INSERT INTO users (username, password, email)
VALUES ('$username', '$password', '$email')";

if ($conn->query($sql) === TRUE) {
    echo "New record created successfully";
} else {
    echo "Error: " . $sql . "<br>" . $conn->error;
}

$conn->close();
?>