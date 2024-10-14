```shell
sea-orm-cli migrate -u postgresql://postgres:admin@localhost:5432/chiji-db?schema=public
sea-orm-cli generate entity -u postgresql://postgres:admin@localhost:5432/chiji-db?schema=public -o entity/src
```
