
# localhost

An NGINX or Apache link webserver implentated in rust.

## Installation

from source
```bash
  git clone https://github.com/romananas/localhost
  cd localhost
  cargo run
```

## Usage 

launching the server from a config file
```bash
localhost --config="exemple.toml"
```

for config exemple see the server/config.toml exemple file

## TODO

- [ ] implement POST request (wip)
- [ ] implement DELETE request
- [ ] implement PUT requests
- [ ] implement config for download and upload folders

## License

[MIT](https://choosealicense.com/licenses/mit/)