# GeoProxy

## What is it?

This is a simple reverse http proxy written in Rust.
It uses simplistic geo indexing to select appropriate backend on the basis of provided `Geolocation` header.
The proxy aims to be as transparent as possible, leaving request/response mostly untouched.

## Quickstart

First, build the proxy:

```shell

docker build -t geoproxy .

```

Then edit provided `config.json` file, adding polygons/backends mappings as desired.

Next, run on your host network (assuming, that you also have statsd running on `127.0.0.1:8125/UDP`):

```shell

docker run -d \
    --name geoproxy \
    --net host \
    -v $PWD/config.json:/config.json \
    geoproxy --statsd 127.0.0.1:8125 --config /config.json

```

## Geolocation

### Routing with the header

- the format of the header is `Geolocation: [x, y]`
- if the header is not provided, or there's no polygon under provided point the default backend is used.
- in case some polygons overlap, the backend used will be selected arbitrarily


```shell

$> curl -i -H "Geolocation: [2.0, 3.0]" http://localhost:8000/

HTTP/1.1 200 OK
x-backend: 1
content-length: 2
date: Wed, 19 Jun 2019 18:04:50 GMT

OK
```

## Statsd support

Statsd support is disabled by default, pass `-s host:port` via the command line to enable.

## Configuration file format

```json
{
  "backends": [
    {
      "areas": [
        {
          "exterior": [
            {
              "x": 0,
              "y": 0
            },
            {
              "x": 0,
              "y": 5
            },
            {
              "x": 5,
              "y": 5
            },
            {
              "x": 5,
              "y": 0
            }
          ],
          "interiors": []
        },
        {
          "exterior": [
            {
              "x": 20,
              "y": 0
            },
            {
              "x": 20,
              "y": 5
            },
            {
              "x": 25,
              "y": 5
            },
            {
              "x": 25,
              "y": 0
            }
          ],
          "interiors": []
        }
      ],
      "backend": {
        "base_url": "http://backend1"
      }
    },
    {
      "areas": [
        {
          "exterior": [
            {
              "x": 0,
              "y": 0
            },
            {
              "x": 0,
              "y": -5
            },
            {
              "x": -5,
              "y": -5
            },
            {
              "x": -5,
              "y": 0
            }
          ],
          "interiors": []
        },
        {
          "exterior": [
            {
              "x": -20,
              "y": 0
            },
            {
              "x": -20,
              "y": -5
            },
            {
              "x": -25,
              "y": -5
            },
            {
              "x": -25,
              "y": 0
            }
          ],
          "interiors": []
        }
      ],
      "backend": {
        "base_url": "http://backend2"
      }
    }
  ],
  "default_backend": {
    "base_url": "http://default_backend"
  }
}
```
