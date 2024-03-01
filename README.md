<h1 style="text-align: center">HarbUI</h1>

<p align="center">
    <img src="https://github.com/mediclab/harbui/assets/1334139/13cddfda-0228-4de9-a0c6-4ca7bdcc2028">
</p>

<p align="center">
    <img src="https://img.shields.io/github/actions/workflow/status/mediclab/harbui/docker.yml">
    <img src="https://img.shields.io/docker/pulls/mediclab/harbui">
    <img src="https://img.shields.io/github/license/mediclab/harbui">
</p>

### HarbUI - Docker Registry UI

Docker Registry UI supports manifests mediaTypes:

* vnd.docker.distribution.manifest.list.v2+json
* vnd.docker.distribution.manifest.v2+json
* vnd.oci.image.index.v1+json
* vnd.oci.image.manifest.v1+json

Example docker-compose.yml file:

```
services:
  harbui:
    image: mediclab/harbui:latest
    ports:
      - 8000:8000
    environment:
      REGISTRY_HOST: registry.example.com
      SECRET_KEY: "<YOUR_GENERATED_SECRET_KEY>"
```

Environment variables:

| env                          | required | default | info                                                                                    |
|------------------------------|----------|---------|-----------------------------------------------------------------------------------------|
| REGISTRY_HOST                | true     | None    | Host of your Self-Hosted Docker Registry                                                |
| SECRET_KEY                   | true     | None    | Secret key for secure framework things. Can be generated with `openssl rand -base64 32` |
| REGISTRY_UNSECURED           | false    | false   | Use HTTPS on registry requests                                                          |
| HARBUI_DELETING_ALLOWED      | false    | false   | Allow deleting images from HarbUI                                                       |
| REGISTRY_HTTP_BASIC_USER     | false    | None    | If your registry API closed by HTTP-Basic Auth you can provide credinitials             |
| REGISTRY_HTTP_BASIC_PASSWORD | false    | None    | If your registry API closed by HTTP-Basic Auth you can provide credinitials             |

### Next:

1. Pagination (for tags not working - [issue](https://github.com/distribution/distribution/issues/1936))
2. Authorization
3. Image details page

<img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/mediclab/harbui">
