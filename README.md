# Beacon Backend
## Welcome to the superior git repo ;)!
### Setup
1. Go to [the Docker installation instructions](https://docs.docker.com/compose/install/) and choose your platform and install docker-compose
2. Clone this git repositry anywhere on your computer
3. Simply run `docker-compose up --build --remove-orphans` from the root directory of the repository (you may need to run the command with sudo on Linux)
4. Due to quirks in the way Docker works (see 5bcf6bba6debdae6a6b6f601290f73d085a28c53), running `docker-compose up` for the first time will download CS:GO during the run phase, instead of during the build stage. What this means to anyone who cares is that the first time you run SteamCMD, it will take a little while before the entire backend is up and running, even though the web server will be up. Basically, the first time you run the backend, it won't be fully working until CS:GO is downloaded, so don't open any issues about not being able to access the game servers, even though the web server is up ;)

You don't need the `--remove-orphans` option on docker-compose up, however, it automatically removes docker images that aren't being used (like for example, if the version of a docker image changes in development). 

Likewise, the `--build` option tells Docker to build the latest version of the package, if anything has been updated. While that isn't necessary either, it's strongly recommended unless you know what you're doing.

### **Warnings** (PLEASE READ)
This repo is **NOT** production ready. Do **NOT** run this on any computer that is exposed to the internet, it can and will be hacked! This doesn't mean that the servers are insecure to run locally, however, there are just no security patches applied, that's why this is the developer branch. Running this server on your regular old laptop or PC at home is just fine. By exposed to the internet, I mean with port forwarding enabled. If you don't know what this means, then you're fine ;). 


### Notes:
* The entire backend is very, very large (when running the game servers). As of writing, running both the web server stuff (Apache + PHP + MySQL) and a CS:GO server takes up **27 GB** of hard drive space. I know that this is a lot, and that those running low on storage space will probably not be able to run it. The solution in the future will be to make running the game servers optional, i.e. by providing an argument to docker-compose. That way our boys in blue at the frontend can mess w. backend web stuff, without having to sacrifice a ton of disk space.

* As for memory and cpu time, it's not very intensive, with or without SteamCMD It runs fine on my 2.6GHZ, 4 GB of RAM laptop, with no noticable performance drop, though as of writing it uses 7% of my CPU time, and 10% of memory after running for about a minute, though it uses around 80% on startup (still no noticable performance drop).
