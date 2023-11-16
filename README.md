# api center
cd webAppRust
sudo docker run -it -p 8000:8000 web_app_rust4:latest

# mongo db
cd data
sudo docker run -d -p 27017:27017 -v mongodbvol:/data/db --name mongodbvol1 mongo-db12

# fill the db
cd worker
python3 mongo_server.py