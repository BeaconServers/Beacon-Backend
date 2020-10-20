<?php
require 'server.php';

$username = $_POST["username"];
$email = $_POST["email"];
$password = $_POST["password"];

$password = password_hash($password, PASSWORD_DEFAULT);

$sql = "INSERT INTO users (username, password, email)
VALUES ('$username', '$password', '$email')";

if (filter_var($email, FILTER_VALIDATE_EMAIL)) {
	if ($conn->query($sql) === TRUE) {
    	echo "Registered successfully";
	} else {
    	echo "Error: " . $sql . "<br>" . $conn->error;
	}
} else {
	echo "Invalid email";
}

$conn->close();
?>
