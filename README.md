# postcode_api_rust

This is a basic rust api that takes in a postcode and provides a longitude and latitude
for a given postcode.

## Installation

In order to install the package you can clone the repo onto your local machine.
<br><br>
You will need to add an .env file, the .env.example can provide some basic setup.
<br><br>
When you run the api it will install the relevant packages for you.

## Usage

In order to start running the project you can run which will start the api if you've followed the steps of installation.
```
cargo run
```
<br><br>
In order to run the tests for the api you can run:
```
cargo test
```

## Routes

There are three routes provided on this api:
I'd recommend using postman but curl requests are also provided below.

GET - "/healthz"         -health check
<br>
GET - "/postcode/{postcode goes here}"   -singular postcode search

<br>

```
curl --location --request GET 'http://localhost:8080/postcode/WC2N 5DU'
```
<br>

POST - "/postcodes" for multiple post codes
<br><br>
``
curl --location --request POST 'http://localhost:8080/postcodes' \
--header 'Content-Type: application/json' \
--data-raw '["OX49 5NU", "M32 0JG", "NE30 1DP"]'
``

<br><br>
Raw data for a postman request: 

`["OX49 5NU", "M32 0JG", "NE30 1DP"]`

<br>
<br>

## Improvements to be made
If I had further time -
<br>
Add better Error handling, for now it only returns 500 responses if any problems, which is not ideal in this scenario.
Internally we have some errors but these need to be mapped into a Tide Error Response. To get that data out.
<br>
<br>
I would have implemented further testing by introducing further mock postcode clients.
<br>
An OAS Swagger spec so that it is easily callable and visible to see specs.
<br>
Further clients to get different postcode data such as weather.
<br>
<br>
I think a GQL server would be useful here instead of REST. Especially if you chain more and more info onto the postcode info.
