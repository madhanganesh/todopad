# build and deploy API
cd api
docker run --rm -v "$PWD":/app -w /app -e GOOS=linux -e GOARCH=amd64 golang:1.17 go build -v
ssh -i ~/.ssh/dorsa root@139.59.57.174 exec systemctl stop todopadapi 
scp -i ~/.ssh/dorsa ./api root@139.59.57.174:/root/todopad/api
scp -i ~/.ssh/dorsa -r ./_scripts/migrations/  root@139.59.57.174:/root/todopad/
ssh -i ~/.ssh/dorsa root@139.59.57.174 exec systemctl start todopadapi 
cd ..

# build and deploy web (SPA)
cd web
npm run build
scp -i ~/.ssh/dorsa -r ./public/* root@139.59.57.174:/var/www/html/
cd ..
