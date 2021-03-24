This is a WIP database directory. We are figuring out how to transfer from a PostgreSQL database
to an Elasticsearch one.

## Instructions

```
# Install elasticsearch using your system's package manager
pacman -S elastic search

# Create keystore
elasticsearch-keystore create

# Start the service each time you want to work with elasticsearch or enable
# it if you want it to start at boot
systemctl start elasticsearch.service
systemctl enable elasticsearch.service

# Check if it works
curl http://127.0.0.1:9200
```

Kill yourself
And then proceed with testing this Rust module.
Currently it should create an index, and if it is successful or if the index
already exists, just push an example json in there and later try to fetch whether 
the index was successfully populated. or smth like that.
