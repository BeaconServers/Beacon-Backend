<?php
require 'server.php';

if(!isset($_SESSION["username"])) {
    $username = $_POST["username"];
    $password = $_POST["password"];

    $res = $conn->query("SELECT * FROM users");
    if ($res->num_rows > 0) {
        // output data of each row
        while ($row = $res->fetch_assoc()) {
            if ($row["username"] == $username and password_verify($password, $row["password"])) {
                $_SESSION["username"] = $username;
                $_SESSION["password"] = $password;

                echo session_id();
                break;
            }
        }
    } else {
        echo "Your username or password is incorrect!";
    }
} else {
    echo "Already signed in! \n";
    echo session_id();
}
?>
