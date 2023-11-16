import influxdb_client, os, time
from influxdb_client import InfluxDBClient, Point, WritePrecision
from influxdb_client.client.write_api import SYNCHRONOUS

token = "KQyL5kf_jbtDsYXh5qDB8Krj_DHPpCBFXrhA2uptgux_VrsWHeQByGSG66-2RB1t72K-vBjxKCXRxjNSQfCTrw=="
org = "a"
url = "http://localhost:8086"
client = influxdb_client.InfluxDBClient(url=url, token=token, org=org)
bucket="demo"

  
  
query_api = client.query_api()

query = """from(bucket: "demo")
 |> range(start: -10m)
 |> filter(fn: (r) => r._measurement == "measurement1")"""
tables = query_api.query(query, org="a")

for table in tables:
  for record in table.records:
    print(record)
    
query = """from(bucket: "demo")
 |> range(start: -10m)
 |> filter(fn: (r) => r._measurement == "measurement1")
 |> mean()"""
tables = query_api.query(query, org="a")

for table in tables:
    for record in table.records:
        print(record)