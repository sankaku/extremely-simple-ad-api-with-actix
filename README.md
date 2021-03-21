# extremely-simple-ad-api-with-actix
Very simple ad api using actix-web.

Not for practical use. This is meant to be a target of stress test exercise.

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
Delivers ads.

The IDs of delivered ads are UUIDs and recorded in Redis as hashes.  
The key of Redis hash is like `id:55ce12a7-0b4c-4802-8757-9c906dc5e837`.

- `num`  
  [_Optional_, _Default_ = 5] Integer. The number of ads to be returned.

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
Tracks a conversion.

- `id`  
  [_Mandatory_] UUID. An ID of the fetched ads in `/deliver`.

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

## License
MIT License