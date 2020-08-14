<?php
require 'server.php';

$username = $_POST["username"];
$email = $_POST["email"];
$password = $_POST["password"];

$password = password_hash($password, PASSWORD_ARGON2ID);

$sql = "INSERT INTO users (username, password, email)
VALUES ('$username', '$password', '$email')";

if ($conn->query($sql) === TRUE) {
    echo "New record created successfully";
} else {
    echo "Error: " . $sql . "<br>" . $conn->error;
}

$conn->close();
?>