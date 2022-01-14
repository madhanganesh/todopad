# build and deploy API
cd api
docker run --rm -v "$PWD":/app -w /app -e GOOS=linux -e GOARCH=amd64 golang:1.17 go build -v
ssh -i ~/.ssh/dorsa root@64.227.179.181 exec systemctl stop goweb 
scp -i ~/.ssh/dorsa ./api root@64.227.179.181:/root/apps/bin/todopad-api
scp -i ~/.ssh/dorsa -r ./_scripts/migrations/  root@64.227.179.181:/root/apps/bin
ssh -i ~/.ssh/dorsa root@64.227.179.181 exec systemctl start goweb 
cd ..

# build and deploy web (SPA)
cd web
npm run build
scp -i ~/.ssh/dorsa -r ./public/* root@64.227.179.181:/var/www/gokarma.in/html/
cd ..
