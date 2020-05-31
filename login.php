<?php
// Start the session
session_start();
?>

<?php
if(session_id() == '') {
    $servername = "localhost";
    $serverUsername = "root";
    $serverPassword = "root";
    $dbname = "beacon";

    $username = $_POST["username"];
    $password = $_POST["password"];

// Create connection
    $conn = new mysqli($servername, $serverUsername, $serverPassword, $dbname);
// Check connection
    if ($conn->connect_error) {
        die("Connection failed: " . $conn->connect_error);
    }


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
    echo session_id();
}
?>