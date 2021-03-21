# extremely-simple-ad-api-with-actix

## Build/Run
There're two `docker-compose` YAML files in this repository.  
To avoid conflicting the names of docker networks, `-p <project_name>` option is used here.

### Build/Run on docker only
All you need is `docker-compose`.

```sh
docker-compose build
docker-compose -p extremely-simple-ad-api-with-actix-all up

# To stop...
# Push Ctrl-C
```

### Build/Run app with cargo, Run redis on docker
Besides `docker-compose`, `cargo` must be installed in local machine.

```sh
docker-compose -p extremely-simple-ad-api-with-actix-dev --file docker-compose.dev.yml up -d
cd app
cargo build --release
cargo run --release

# To stop...
# Push Ctrl-C
cd ../
docker-compose -p extremely-simple-ad-api-with-actix-dev --file docker-compose.dev.yml down
```

## API
### GET `/deliver`
```sh
$ curl --silent localhost:8080/deliver?num=3 | jq
{
  "success": true,
  "errors": [
    ""
  ],
  "message": {
    "ads": [
      {
        "id": "55ce12a7-0b4c-4802-8757-9c906dc5e837"
      },
      {
        "id": "95944ac8-c095-4d55-b538-5fe3a30afffd"
      },
      {
        "id": "e77dee4c-3980-41f6-b3be-fb34e7fc7017"
      }
    ]
  }
}
```

### POST `/cv`
```sh
$ curl --silent -XPOST localhost:8080/cv?id=55ce12a7-0b4c-4802-8757-9c906dc5e837 | jq
{
  "success": true,
  "errors": [
    ""
  ],
  "message": {
    "id": "55ce12a7-0b4c-4802-8757-9c906dc5e837"
  }
}
```
