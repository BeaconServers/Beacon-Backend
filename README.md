# Beacon Backend
## Welcome to the superior git repo ;)!
### Setup
1. Go to (the Docker installation instructions)[FROM php:7.2.7-fpm-alpine3.7] and choose your platform and install docker-compose
2. Clone this git repositry anywhere on your computer
3. Simply run `docker-compose up --remove-orphans` from the root directory of the repository (you may need to run the command with sudo on Linux)
You don't need the `--remove-orphans` option on docker-compose up, however, it automatically removes docker images that aren't being used (like for example, if the version of a docker image changes in development). 

### **Warnings** (PLEASE READ)
This repo is **NOT** production ready. Do **NOT** run this on any computer that is exposed to the internet, it can and will be hacked! This doesn't mean that the servers are insecure to run locally, however, there are just no security patches applied, that's why this is the developer branch. Running this server on your regular old laptop or PC at home is just fine.


### Notes:
* The repo in this state does not run the games servers (yet), but it will. Right now, just use it to mess with logging in and registering users, but it will have the ability to run and interface with game servers soon (just a few commits away).

* Due to the nature of Docker, running the backend takes up quite a bit of space, and uses a good chunk of memory (compared to running natively that is). At the time of writing, the docker images take up around 2GB, and after installing Ubuntu images for the games server (not done yet), I expect it to take at least 10GB. As for memory, it takes up maybe half a GB to run the server. This isn't much, and even my laptop can handle it easily, just be warned when running this on low-end machines that it may slow your computer.
