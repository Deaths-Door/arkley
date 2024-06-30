## Intsall npm and expo for React Native
## https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-18-04#installing-node-js-from-default-repositories-with-apt
#sudo apt install -y nodejs
sudo apt install -y npm

curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash

nvm install v20.5.0
nvm use v20.5.0

npm install -g npx

cd arkley_ui
npm install 
##npm install -g create-expo-app