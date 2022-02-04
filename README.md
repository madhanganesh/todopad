# Commands to run locally for development
## API
```
cd api
source _scripts/dev.sh
go run main.go
API Server started in port 8080
```

## Web
```
cd web
npm run dev
navigate to http://localhost:5500
```

# DO scripts
## nginx conf
/etc/nginx/sites-available/default
```
server {
    listen [::]:443 ssl ipv6only=on; 
	listen 443 ssl; 
    ssl_certificate /etc/letsencrypt/live/gokarma.in/fullchain.pem; 
    ssl_certificate_key /etc/letsencrypt/live/gokarma.in/privkey.pem; 
    include /etc/letsencrypt/options-ssl-nginx.conf; 
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem; 

	root /var/www/gokarma.in/html;
	index index.html index.htm index.nginx-debian.html;
    server_name www.gokarma.in gokarma.in; 

	location / {
		# First attempt to serve request as file, then
		# as directory, then fall back to displaying a 404.
		try_files $uri $uri/ =404;
	}

	location /api/ {
		proxy_pass http://localhost:8080/;
	}
}

server {
	if ($host = www.gokarma.in) {
        return 301 https://$host$request_uri;
    }


    if ($host = gokarma.in) {
        return 301 https://$host$request_uri;
    }

	listen 80 ;
	listen [::]:80 ;
   	server_name www.gokarma.in gokarma.in;
    return 404; # managed by Certbot
}
```
sudo systemctl restart nginx
sudo systemctl reload nginx

## go service
/lib/systemd/system/goweb.service
```
	[Unit]
	Description=goweb

	[Service]
	Type=simple
	Restart=always
	RestartSec=5s
	ExecStart=/root/apps/bin/todopad-api
	StandardOutput=syslog
	StandardError=syslog
	SyslogIdentifier=gowebservice
	Environment="PORT=8080"
	Environment="SECRET_KEY=@wedidit#foryou*"
	Environment="DATABASE_URL=/root/apps/bin/todopad.sqlite"
	Environment="MIGRATIONS_DIR=/root/apps/bin"

	[Install]
	WantedBy=multi-user.target
```
sudo systemctl goweb restart

# References
Nginx Golang
https://www.digitalocean.com/community/tutorials/how-to-deploy-a-go-web-application-using-nginx-on-ubuntu-18-04
