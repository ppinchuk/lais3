chmod 777 /var/run/docker.sock
sudo service docker start
./compile.sh
./compile.sh cargo clean
