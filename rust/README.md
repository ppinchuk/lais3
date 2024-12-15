chmod 777 /var/run/docker.sock
  OR
sudo docker ps
sudo chown $USER /var/run/docker.sock


sudo service docker start

./compile.sh
./compile.sh cargo clean
